use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use tokio::sync::mpsc;

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB chunks

#[derive(Debug, Clone)]
pub struct TransferInfo {
    pub transfer_id: String,
    pub filename: String,
    pub total_size: u64,
    pub direction: TransferDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferDirection {
    Upload,   // Sending file to remote
    Download, // Receiving file from remote
}

#[derive(Debug)]
pub struct TransferState {
    pub info: TransferInfo,
    pub file_path: PathBuf,
    pub file_handle: File,
    pub bytes_transferred: u64,
    pub chunk_count: u64,
    pub expected_chunks: u64,
    pub checksum: Option<String>,
    pub completed: bool,
    pub error: Option<String>,
}

impl TransferState {
    pub fn progress_percent(&self) -> f32 {
        if self.info.total_size == 0 {
            return 0.0;
        }
        (self.bytes_transferred as f32 / self.info.total_size as f32) * 100.0
    }
}

pub struct FileTransferManager {
    transfers: HashMap<String, TransferState>,
    download_dir: PathBuf,
}

impl FileTransferManager {
    pub fn new(download_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&download_dir)
            .context("Failed to create download directory")?;

        Ok(Self {
            transfers: HashMap::new(),
            download_dir,
        })
    }

    /// Start sending a file to remote
    pub fn start_upload(&mut self, file_path: PathBuf) -> Result<TransferInfo> {
        let file = File::open(&file_path)
            .context("Failed to open file for upload")?;

        let metadata = file.metadata()
            .context("Failed to get file metadata")?;

        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .context("Invalid filename")?
            .to_string();

        let transfer_id = uuid::Uuid::new_v4().to_string();
        let total_size = metadata.len();
        let expected_chunks = (total_size + CHUNK_SIZE as u64 - 1) / CHUNK_SIZE as u64;

        let info = TransferInfo {
            transfer_id: transfer_id.clone(),
            filename,
            total_size,
            direction: TransferDirection::Upload,
        };

        let state = TransferState {
            info: info.clone(),
            file_path,
            file_handle: file,
            bytes_transferred: 0,
            chunk_count: 0,
            expected_chunks,
            checksum: None,
            completed: false,
            error: None,
        };

        self.transfers.insert(transfer_id, state);

        Ok(info)
    }

    /// Read next chunk for upload
    pub fn read_next_chunk(&mut self, transfer_id: &str) -> Result<Option<(u64, Vec<u8>)>> {
        let state = self.transfers
            .get_mut(transfer_id)
            .context("Transfer not found")?;

        if state.completed {
            return Ok(None);
        }

        let mut buffer = vec![0u8; CHUNK_SIZE];
        let bytes_read = state.file_handle.read(&mut buffer)
            .context("Failed to read file chunk")?;

        if bytes_read == 0 {
            // Transfer complete
            state.completed = true;
            state.checksum = Some(self.calculate_checksum(transfer_id)?);
            return Ok(None);
        }

        buffer.truncate(bytes_read);
        let chunk_index = state.chunk_count;
        state.chunk_count += 1;
        state.bytes_transferred += bytes_read as u64;

        tracing::info!(
            "Upload chunk {} of {} ({:.1}%)",
            chunk_index + 1,
            state.expected_chunks,
            state.progress_percent()
        );

        Ok(Some((chunk_index, buffer)))
    }

    /// Start receiving a file from remote
    pub fn start_download(&mut self, transfer_id: String, filename: String, total_size: u64) -> Result<()> {
        let file_path = self.download_dir.join(&filename);

        // Create or truncate file
        let file_handle = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .context("Failed to create download file")?;

        let expected_chunks = (total_size + CHUNK_SIZE as u64 - 1) / CHUNK_SIZE as u64;

        let info = TransferInfo {
            transfer_id: transfer_id.clone(),
            filename,
            total_size,
            direction: TransferDirection::Download,
        };

        let state = TransferState {
            info,
            file_path,
            file_handle,
            bytes_transferred: 0,
            chunk_count: 0,
            expected_chunks,
            checksum: None,
            completed: false,
            error: None,
        };

        self.transfers.insert(transfer_id, state);

        Ok(())
    }

    /// Write received chunk to file
    pub fn write_chunk(&mut self, transfer_id: &str, chunk_index: u64, data: Vec<u8>) -> Result<()> {
        let state = self.transfers
            .get_mut(transfer_id)
            .context("Transfer not found")?;

        // Seek to correct position
        let offset = chunk_index * CHUNK_SIZE as u64;
        state.file_handle.seek(SeekFrom::Start(offset))
            .context("Failed to seek in file")?;

        // Write chunk
        state.file_handle.write_all(&data)
            .context("Failed to write chunk")?;

        state.file_handle.flush()
            .context("Failed to flush file")?;

        state.bytes_transferred += data.len() as u64;
        state.chunk_count += 1;

        tracing::info!(
            "Download chunk {} of {} ({:.1}%)",
            state.chunk_count,
            state.expected_chunks,
            state.progress_percent()
        );

        // Check if complete
        if state.chunk_count >= state.expected_chunks {
            state.completed = true;
            state.checksum = Some(self.calculate_checksum(transfer_id)?);
            tracing::info!("Download complete: {} (checksum: {})", state.info.filename, state.checksum.as_ref().unwrap());
        }

        Ok(())
    }

    /// Calculate SHA256 checksum of file
    fn calculate_checksum(&mut self, transfer_id: &str) -> Result<String> {
        let state = self.transfers
            .get_mut(transfer_id)
            .context("Transfer not found")?;

        // Seek to start
        state.file_handle.seek(SeekFrom::Start(0))
            .context("Failed to seek to start")?;

        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192];

        loop {
            let bytes_read = state.file_handle.read(&mut buffer)
                .context("Failed to read for checksum")?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// Verify checksum matches
    pub fn verify_checksum(&mut self, transfer_id: &str, expected: &str) -> Result<bool> {
        let calculated = self.calculate_checksum(transfer_id)?;
        Ok(calculated == expected)
    }

    /// Get transfer state
    pub fn get_transfer(&self, transfer_id: &str) -> Option<&TransferState> {
        self.transfers.get(transfer_id)
    }

    /// Mark transfer as failed
    pub fn mark_failed(&mut self, transfer_id: &str, error: String) {
        if let Some(state) = self.transfers.get_mut(transfer_id) {
            state.error = Some(error);
            state.completed = true;
        }
    }

    /// Cancel and remove transfer
    pub fn cancel_transfer(&mut self, transfer_id: &str) -> Result<()> {
        if let Some(state) = self.transfers.remove(transfer_id) {
            // Delete partial file for downloads
            if state.info.direction == TransferDirection::Download && !state.completed {
                let _ = std::fs::remove_file(&state.file_path);
            }
            tracing::info!("Transfer cancelled: {}", transfer_id);
        }
        Ok(())
    }

    /// Get all active transfers
    pub fn get_active_transfers(&self) -> Vec<&TransferState> {
        self.transfers
            .values()
            .filter(|t| !t.completed)
            .collect()
    }

    /// Clear completed transfers
    pub fn clear_completed(&mut self) {
        self.transfers.retain(|_, state| !state.completed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_upload_flow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let mut manager = FileTransferManager::new(temp_dir.path().to_path_buf())?;

        // Create test file
        let test_file = temp_dir.path().join("test.txt");
        let mut file = File::create(&test_file)?;
        file.write_all(b"Hello, World!")?;
        drop(file);

        // Start upload
        let info = manager.start_upload(test_file)?;
        assert_eq!(info.total_size, 13);

        // Read chunk
        let chunk = manager.read_next_chunk(&info.transfer_id)?;
        assert!(chunk.is_some());

        let (index, data) = chunk.unwrap();
        assert_eq!(index, 0);
        assert_eq!(data, b"Hello, World!");

        // Should be complete
        let chunk2 = manager.read_next_chunk(&info.transfer_id)?;
        assert!(chunk2.is_none());

        let state = manager.get_transfer(&info.transfer_id).unwrap();
        assert!(state.completed);

        Ok(())
    }

    #[test]
    fn test_download_flow() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let mut manager = FileTransferManager::new(temp_dir.path().to_path_buf())?;

        // Start download
        manager.start_download(
            "test-123".to_string(),
            "received.txt".to_string(),
            13,
        )?;

        // Write chunk
        manager.write_chunk("test-123", 0, b"Hello, World!".to_vec())?;

        // Check state
        let state = manager.get_transfer("test-123").unwrap();
        assert!(state.completed);
        assert_eq!(state.bytes_transferred, 13);

        // Verify file exists
        let file_path = temp_dir.path().join("received.txt");
        assert!(file_path.exists());

        let contents = std::fs::read_to_string(file_path)?;
        assert_eq!(contents, "Hello, World!");

        Ok(())
    }
}
