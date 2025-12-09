use anyhow::{Context, Result};
use arboard::Clipboard;
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>), // PNG/JPEG data
    Empty,
}

impl ClipboardContent {
    pub fn mime_type(&self) -> &str {
        match self {
            ClipboardContent::Text(_) => "text/plain",
            ClipboardContent::Image(_) => "image/png",
            ClipboardContent::Empty => "application/x-empty",
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, ClipboardContent::Empty)
    }
}

pub struct ClipboardSync {
    clipboard: Clipboard,
    last_content: ClipboardContent,
    last_check: Instant,
    enabled: bool,
}

impl ClipboardSync {
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new()
            .context("Failed to initialize clipboard")?;

        Ok(Self {
            clipboard,
            last_content: ClipboardContent::Empty,
            last_check: Instant::now(),
            enabled: true,
        })
    }

    /// Enable or disable clipboard synchronization
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.last_content = ClipboardContent::Empty;
        }
    }

    /// Check if clipboard has changed since last check
    /// Returns Some(content) if changed, None if unchanged
    pub fn check_for_changes(&mut self) -> Option<ClipboardContent> {
        if !self.enabled {
            return None;
        }

        // Rate limit checks
        if self.last_check.elapsed() < POLL_INTERVAL {
            return None;
        }

        self.last_check = Instant::now();

        // Try to get text content first
        if let Ok(text) = self.clipboard.get_text() {
            let content = ClipboardContent::Text(text);

            if content != self.last_content {
                tracing::debug!("Clipboard changed: text ({} chars)",
                    if let ClipboardContent::Text(ref t) = content { t.len() } else { 0 }
                );
                self.last_content = content.clone();
                return Some(content);
            }
        }

        // Try to get image content
        #[cfg(feature = "clipboard-image")]
        if let Ok(img) = self.clipboard.get_image() {
            // Convert arboard::ImageData to PNG bytes
            let png_data = self.image_to_png(&img).ok()?;
            let content = ClipboardContent::Image(png_data);

            if content != self.last_content {
                tracing::debug!("Clipboard changed: image");
                self.last_content = content.clone();
                return Some(content);
            }
        }

        None
    }

    /// Set clipboard content from remote
    pub fn set_content(&mut self, content: ClipboardContent) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        match content {
            ClipboardContent::Text(ref text) => {
                self.clipboard.set_text(text)
                    .context("Failed to set clipboard text")?;

                tracing::debug!("Clipboard set: text ({} chars)", text.len());
            }

            ClipboardContent::Image(ref data) => {
                #[cfg(feature = "clipboard-image")]
                {
                    let img = self.png_to_image(data)
                        .context("Failed to decode image data")?;

                    self.clipboard.set_image(img)
                        .context("Failed to set clipboard image")?;

                    tracing::debug!("Clipboard set: image");
                }

                #[cfg(not(feature = "clipboard-image"))]
                {
                    tracing::warn!("Image clipboard not supported");
                    return Err(anyhow::anyhow!("Image clipboard not supported"));
                }
            }

            ClipboardContent::Empty => {
                self.clipboard.clear()
                    .context("Failed to clear clipboard")?;

                tracing::debug!("Clipboard cleared");
            }
        }

        // Update our tracking to prevent re-sync
        self.last_content = content;
        self.last_check = Instant::now();

        Ok(())
    }

    /// Get current clipboard content without tracking
    pub fn get_content(&mut self) -> Result<ClipboardContent> {
        if let Ok(text) = self.clipboard.get_text() {
            return Ok(ClipboardContent::Text(text));
        }

        #[cfg(feature = "clipboard-image")]
        if let Ok(img) = self.clipboard.get_image() {
            let png_data = self.image_to_png(&img)?;
            return Ok(ClipboardContent::Image(png_data));
        }

        Ok(ClipboardContent::Empty)
    }

    /// Clear clipboard
    pub fn clear(&mut self) -> Result<()> {
        self.clipboard.clear()
            .context("Failed to clear clipboard")?;

        self.last_content = ClipboardContent::Empty;
        Ok(())
    }

    #[cfg(feature = "clipboard-image")]
    fn image_to_png(&self, img: &arboard::ImageData) -> Result<Vec<u8>> {
        use image::{ImageBuffer, RgbaImage};

        let rgba_img: RgbaImage = ImageBuffer::from_raw(
            img.width as u32,
            img.height as u32,
            img.bytes.to_vec(),
        ).context("Failed to create image buffer")?;

        let mut png_data = Vec::new();
        rgba_img.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageOutputFormat::Png)
            .context("Failed to encode PNG")?;

        Ok(png_data)
    }

    #[cfg(feature = "clipboard-image")]
    fn png_to_image(&self, data: &[u8]) -> Result<arboard::ImageData<'static>> {
        use image::GenericImageView;

        let img = image::load_from_memory(data)
            .context("Failed to decode image")?;

        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();

        Ok(arboard::ImageData {
            width: width as usize,
            height: height as usize,
            bytes: rgba.into_raw().into(),
        })
    }
}

/// Monitor clipboard changes in background
pub struct ClipboardMonitor {
    sync: ClipboardSync,
    callback: Box<dyn FnMut(ClipboardContent) + Send>,
}

impl ClipboardMonitor {
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: FnMut(ClipboardContent) + Send + 'static,
    {
        Ok(Self {
            sync: ClipboardSync::new()?,
            callback: Box::new(callback),
        })
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.sync.set_enabled(enabled);
    }

    /// Run one check iteration (call from main loop)
    pub fn tick(&mut self) {
        if let Some(content) = self.sync.check_for_changes() {
            (self.callback)(content);
        }
    }

    /// Set clipboard content from remote
    pub fn set_content(&mut self, content: ClipboardContent) -> Result<()> {
        self.sync.set_content(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_text() -> Result<()> {
        let mut sync = ClipboardSync::new()?;

        // Set text
        let content = ClipboardContent::Text("Hello, World!".to_string());
        sync.set_content(content.clone())?;

        // Get it back
        let retrieved = sync.get_content()?;
        assert_eq!(retrieved, content);

        // Check shouldn't detect change (same content)
        let change = sync.check_for_changes();
        assert!(change.is_none());

        Ok(())
    }

    #[test]
    fn test_clipboard_clear() -> Result<()> {
        let mut sync = ClipboardSync::new()?;

        // Set text
        sync.set_content(ClipboardContent::Text("test".to_string()))?;

        // Clear
        sync.clear()?;

        // Should be empty
        let content = sync.get_content()?;
        assert!(content.is_empty());

        Ok(())
    }
}
