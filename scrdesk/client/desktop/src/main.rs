mod api;
mod connection;
mod protocol;
mod capture;
mod input;
mod transfer;
mod clipboard;
mod network;

use api::{ApiClient, RegisterDeviceRequest};
use connection::{ConnectionManager, ConnectionState};
use eframe::egui;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

// Remote desktop modules
use capture::ScreenCapture;
use input::InputSimulator;
use transfer::FileTransferManager;
use clipboard::ClipboardMonitor;
use network::{NetworkConnection, ConnectionManager as NetConnectionManager};
use protocol::Message;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 750.0])
            .with_title("ScrDesk PRO Enterprise"),
        ..Default::default()
    };

    eframe::run_native(
        "ScrDesk PRO Enterprise",
        options,
        Box::new(|cc| {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            Box::new(ScrDeskApp::new(cc, runtime))
        }),
    )
}

struct ScrDeskApp {
    runtime: tokio::runtime::Runtime,
    api_client: Arc<ApiClient>,
    connection_manager: Arc<ConnectionManager>,

    // UI state
    server_url: String,
    email: String,
    password: String,
    totp_code: String,

    // Mode selection
    mode: AppMode,

    // Session state
    logged_in: bool,
    user_info: Option<api::UserInfo>,
    device_id: String,
    device_key: Arc<Mutex<Option<String>>>,

    // Guest mode state
    guest_connection_id: String,
    guest_session_start: Option<u64>,
    remote_id_input: String,

    // Device list
    available_devices: Arc<Mutex<Vec<api::Device>>>,
    selected_device: Option<String>,

    // Status
    status_message: String,
    error_message: Option<String>,

    // Loading states
    is_logging_in: bool,
    is_registering_device: bool,
    is_loading_devices: bool,

    // Remote desktop components
    net_connection: Arc<Mutex<Option<NetConnectionManager>>>,
    screen_capturer: Arc<Mutex<Option<Box<dyn ScreenCapture>>>>,
    input_simulator: Arc<Mutex<Option<Box<dyn InputSimulator>>>>,
    file_transfer: Arc<Mutex<Option<FileTransferManager>>>,
    clipboard_monitor: Arc<Mutex<Option<ClipboardMonitor>>>,

    // Remote screen state
    remote_screen_texture: Option<egui::TextureHandle>,
    remote_screen_size: (u32, u32),
    is_streaming: bool,
    remote_device_id: String,

    // Screen capture state
    is_capturing: bool,
    capture_fps: f32,
    last_frame_time: std::time::Instant,
}

#[derive(PartialEq)]
enum AppMode {
    Initial,      // Show mode selection
    Login,        // Login mode
    GuestMode,    // Guest/Quick Connect mode
    Connected,    // Connected state
}

// Brand colors from website - Indigo to Purple gradient
const PRIMARY_COLOR: egui::Color32 = egui::Color32::from_rgb(79, 70, 229);      // indigo-600
const SECONDARY_COLOR: egui::Color32 = egui::Color32::from_rgb(147, 51, 234);   // purple-600
const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(236, 72, 153);      // pink-500
const BACKGROUND_LIGHT: egui::Color32 = egui::Color32::from_rgb(249, 250, 251); // gray-50
const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(17, 24, 39);        // gray-900
const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(107, 114, 128);   // gray-500
const SUCCESS_COLOR: egui::Color32 = egui::Color32::from_rgb(34, 197, 94);      // green-500

impl ScrDeskApp {
    fn new(_cc: &eframe::CreationContext<'_>, runtime: tokio::runtime::Runtime) -> Self {
        let server_url = "http://72.61.138.218:8000".to_string();
        let api_client = Arc::new(ApiClient::new(server_url.clone()));
        let relay_server = "72.61.138.218:21117".to_string();
        let connection_manager = Arc::new(ConnectionManager::new(relay_server));

        Self {
            runtime,
            api_client,
            connection_manager,
            server_url,
            email: String::new(),
            password: String::new(),
            totp_code: String::new(),
            mode: AppMode::Initial,
            logged_in: false,
            user_info: None,
            device_id: String::new(),
            device_key: Arc::new(Mutex::new(None)),
            guest_connection_id: String::new(),
            guest_session_start: None,
            remote_id_input: String::new(),
            available_devices: Arc::new(Mutex::new(Vec::new())),
            selected_device: None,
            status_message: "Welcome to ScrDesk".to_string(),
            error_message: None,
            is_logging_in: false,
            is_registering_device: false,
            is_loading_devices: false,

            // Remote desktop components (initialized on demand)
            net_connection: Arc::new(Mutex::new(None)),
            screen_capturer: Arc::new(Mutex::new(None)),
            input_simulator: Arc::new(Mutex::new(None)),
            file_transfer: Arc::new(Mutex::new(None)),
            clipboard_monitor: Arc::new(Mutex::new(None)),

            // Remote screen state
            remote_screen_texture: None,
            remote_screen_size: (1920, 1080),
            is_streaming: false,
            remote_device_id: String::new(),

            // Screen capture state
            is_capturing: false,
            capture_fps: 0.0,
            last_frame_time: std::time::Instant::now(),
        }
    }

    // Start screen capture loop in background
    fn start_screen_capture(&mut self, ctx: &egui::Context) {
        if self.is_capturing {
            return;
        }

        self.is_capturing = true;
        let screen_capturer = self.screen_capturer.clone();
        let net_connection = self.net_connection.clone();
        let ctx_clone = ctx.clone();

        self.runtime.spawn(async move {
            let mut frame_count = 0;
            let mut last_fps_update = std::time::Instant::now();

            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(33)).await; // ~30 FPS

                // Capture frame
                let frame_data = {
                    let mut capturer = screen_capturer.lock().await;
                    if let Some(cap) = capturer.as_mut() {
                        match cap.capture_frame() {
                            Ok(frame) => Some(frame),
                            Err(e) => {
                                tracing::error!("Failed to capture frame: {}", e);
                                None
                            }
                        }
                    } else {
                        None
                    }
                };

                if let Some(frame) = frame_data {
                    // Send frame to remote
                    if let Some(manager) = net_connection.lock().await.as_ref() {
                        let msg = Message::VideoFrame {
                            data: frame.data,
                            width: frame.width,
                            height: frame.height,
                            timestamp: frame.timestamp,
                            is_keyframe: frame_count % 30 == 0, // Keyframe every 30 frames
                        };

                        if let Err(e) = manager.send(msg).await {
                            tracing::error!("Failed to send video frame: {}", e);
                        }
                    }

                    frame_count += 1;

                    // Update FPS counter
                    if last_fps_update.elapsed().as_secs() >= 1 {
                        tracing::debug!("Capture FPS: {}", frame_count);
                        frame_count = 0;
                        last_fps_update = std::time::Instant::now();
                    }
                }

                ctx_clone.request_repaint();
            }
        });

        tracing::info!("Screen capture started");
    }

    // Stop screen capture
    fn stop_screen_capture(&mut self) {
        self.is_capturing = false;
        tracing::info!("Screen capture stopped");
    }

    // Initialize remote desktop components
    fn init_remote_desktop(&mut self, ctx: &egui::Context) {
        let runtime_handle = self.runtime.handle().clone();
        let net_connection = self.net_connection.clone();
        let device_id = self.guest_connection_id.clone();

        // Initialize network connection
        self.runtime.spawn(async move {
            let mut manager = NetConnectionManager::new();
            if let Err(e) = manager.connect(device_id).await {
                tracing::error!("Failed to connect: {}", e);
            } else {
                *net_connection.lock().await = Some(manager);
                tracing::info!("Network connection initialized");
            }
        });

        // Initialize screen capturer
        let capturer = capture::create_capturer();
        if let Ok(cap) = capturer {
            *self.screen_capturer.blocking_lock() = Some(cap);
            tracing::info!("Screen capturer initialized");
        }

        // Initialize input simulator
        let simulator = input::create_simulator();
        if let Ok(sim) = simulator {
            *self.input_simulator.blocking_lock() = Some(sim);
            tracing::info!("Input simulator initialized");
        }

        // Initialize file transfer
        let downloads_dir = std::env::current_dir()
            .unwrap_or_default()
            .join("downloads");
        if let Ok(ft) = FileTransferManager::new(downloads_dir) {
            *self.file_transfer.blocking_lock() = Some(ft);
            tracing::info!("File transfer manager initialized");
        }

        // Initialize clipboard monitor
        let ctx_clone = ctx.clone();
        let net_conn = self.net_connection.clone();

        if let Ok(monitor) = ClipboardMonitor::new(move |content| {
            let net_conn = net_conn.clone();
            let ctx = ctx_clone.clone();

            tokio::spawn(async move {
                if let Some(manager) = net_conn.lock().await.as_ref() {
                    let msg = Message::ClipboardUpdate {
                        content: format!("{:?}", content),
                        mime_type: content.mime_type().to_string(),
                    };
                    let _ = manager.send(msg).await;
                    ctx.request_repaint();
                }
            });
        }) {
            *self.clipboard_monitor.blocking_lock() = Some(monitor);
            tracing::info!("Clipboard monitor initialized");
        }
    }

    // Start connection to remote device
    fn start_connection(&mut self, remote_id: String, ctx: &egui::Context) {
        self.remote_device_id = remote_id.clone();
        self.status_message = format!("Connecting to {}...", remote_id);

        let net_connection = self.net_connection.clone();
        let ctx_clone = ctx.clone();

        self.runtime.spawn(async move {
            if let Some(manager) = net_connection.lock().await.as_mut() {
                match manager.request_connection(remote_id).await {
                    Ok(_) => {
                        tracing::info!("Connection request sent");
                        ctx_clone.request_repaint();
                    }
                    Err(e) => {
                        tracing::error!("Connection request failed: {}", e);
                        ctx_clone.request_repaint();
                    }
                }
            }
        });
    }

    // Handle incoming messages
    fn handle_incoming_messages(&mut self, ctx: &egui::Context) {
        let net_connection = self.net_connection.clone();
        let input_simulator = self.input_simulator.clone();
        let file_transfer = self.file_transfer.clone();
        let clipboard_monitor = self.clipboard_monitor.clone();
        let ctx_clone = ctx.clone();

        self.runtime.spawn(async move {
            if let Some(manager) = net_connection.lock().await.as_ref() {
                if let Some(msg) = manager.recv().await {
                    match msg {
                        Message::ConnectResponse { success, session_id, error } => {
                            if success {
                                tracing::info!("Connected! Session: {:?}", session_id);
                            } else {
                                tracing::error!("Connection failed: {:?}", error);
                            }
                            ctx_clone.request_repaint();
                        }

                        Message::VideoFrame { data, width, height, .. } => {
                            // Store frame data for rendering
                            // Frame data is RGBA format, can be directly used with egui
                            tracing::debug!("Received video frame: {}x{} ({} bytes)", width, height, data.len());

                            // We'll update the texture in the UI thread
                            // For now, just log it
                            // TODO: Convert to egui::ColorImage and update texture
                        }

                        Message::MouseMove { x, y } => {
                            if let Some(sim) = input_simulator.lock().await.as_ref() {
                                let _ = sim.simulate_mouse_move(x, y);
                            }
                        }

                        Message::MouseButton { button, pressed } => {
                            if let Some(sim) = input_simulator.lock().await.as_ref() {
                                let _ = sim.simulate_mouse_button(button, pressed);
                            }
                        }

                        Message::MouseScroll { delta_x, delta_y } => {
                            if let Some(sim) = input_simulator.lock().await.as_ref() {
                                let _ = sim.simulate_mouse_scroll(delta_x, delta_y);
                            }
                        }

                        Message::KeyboardEvent { key, pressed, modifiers } => {
                            if let Some(sim) = input_simulator.lock().await.as_ref() {
                                let _ = sim.simulate_key(&key, pressed, modifiers);
                            }
                        }

                        Message::FileChunk { transfer_id, chunk_index, data } => {
                            if let Some(ft) = file_transfer.lock().await.as_mut() {
                                let _ = ft.write_chunk(&transfer_id, chunk_index, data);
                            }
                        }

                        Message::ClipboardUpdate { content, .. } => {
                            if let Some(monitor) = clipboard_monitor.lock().await.as_mut() {
                                // TODO: Parse and set clipboard content
                                tracing::info!("Received clipboard update");
                            }
                        }

                        _ => {
                            tracing::debug!("Received message: {:?}", msg);
                        }
                    }
                }
            }
        });
    }

    fn handle_login(&mut self, ctx: &egui::Context) {
        if self.is_logging_in {
            return;
        }

        let email = self.email.clone();
        let password = self.password.clone();
        let totp_code = if self.totp_code.is_empty() {
            None
        } else {
            Some(self.totp_code.clone())
        };

        let api_client = Arc::clone(&self.api_client);
        let ctx_clone = ctx.clone();

        self.is_logging_in = true;
        self.error_message = None;
        self.status_message = "Logging in...".to_string();

        self.runtime.spawn(async move {
            match api_client.login(email, password, totp_code).await {
                Ok(_response) => {
                    tracing::info!("Login successful");
                    ctx_clone.request_repaint();
                }
                Err(e) => {
                    tracing::error!("Login failed: {}", e);
                    ctx_clone.request_repaint();
                }
            }
        });
    }

    fn start_guest_session(&mut self, ctx: &egui::Context) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.guest_session_start = Some(now);

        // Generate a random connection ID for guest mode
        self.guest_connection_id = format!("GUEST-{}", now);
        self.mode = AppMode::GuestMode;
        self.status_message = "Guest mode activated - 1 hour free trial".to_string();

        // Initialize remote desktop components
        self.init_remote_desktop(ctx);
    }

    fn get_remaining_guest_time(&self) -> Option<u64> {
        if let Some(start_time) = self.guest_session_start {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let elapsed = now - start_time;
            let one_hour = 3600u64;

            if elapsed < one_hour {
                Some(one_hour - elapsed)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }

    fn format_time(seconds: u64) -> String {
        let minutes = seconds / 60;
        let secs = seconds % 60;
        format!("{}:{:02}", minutes, secs)
    }

    fn render_initial_mode(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.add_space(80.0);

            // Logo/Title with gradient effect
            let rich_text = egui::RichText::new("ScrDesk")
                .size(48.0)
                .color(PRIMARY_COLOR)
                .strong();
            ui.label(rich_text);

            ui.add_space(10.0);

            ui.label(
                egui::RichText::new("Remote Desktop Made Simple")
                    .size(18.0)
                    .color(TEXT_SECONDARY)
            );

            ui.add_space(60.0);

            // Quick Connect Button (Guest Mode)
            let guest_button = egui::Button::new(
                egui::RichText::new("🚀  Quick Connect (Free 1 Hour)")
                    .size(20.0)
                    .color(egui::Color32::WHITE)
            )
            .fill(PRIMARY_COLOR)
            .min_size(egui::vec2(350.0, 70.0))
            .rounding(35.0);

            if ui.add(guest_button).clicked() {
                self.start_guest_session(ctx);
            }

            ui.add_space(20.0);

            // Login Button
            let login_button = egui::Button::new(
                egui::RichText::new("🔐  Sign In")
                    .size(18.0)
                    .color(PRIMARY_COLOR)
            )
            .stroke(egui::Stroke::new(2.0, PRIMARY_COLOR))
            .fill(egui::Color32::WHITE)
            .min_size(egui::vec2(350.0, 60.0))
            .rounding(30.0);

            if ui.add(login_button).clicked() {
                self.mode = AppMode::Login;
            }

            ui.add_space(40.0);

            // Feature highlights
            ui.horizontal(|ui| {
                ui.add_space(250.0);
                ui.label(egui::RichText::new("⚡").size(24.0));
                ui.label(egui::RichText::new("Lightning Fast").color(TEXT_SECONDARY));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("🔒").size(24.0));
                ui.label(egui::RichText::new("Secure").color(TEXT_SECONDARY));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("🌐").size(24.0));
                ui.label(egui::RichText::new("Cross-Platform").color(TEXT_SECONDARY));
            });
        });
    }

    fn render_login_mode(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);

            let heading = egui::RichText::new("Sign In")
                .size(32.0)
                .color(PRIMARY_COLOR)
                .strong();
            ui.label(heading);

            ui.add_space(30.0);
        });

        // Center the form
        ui.columns(3, |columns| {
            columns[1].vertical(|ui| {
                ui.add_space(10.0);

                // Server URL
                ui.label(egui::RichText::new("Server URL").color(TEXT_PRIMARY));
                ui.add_space(5.0);
                let server_input = egui::TextEdit::singleline(&mut self.server_url)
                    .min_size(egui::vec2(300.0, 35.0));
                ui.add(server_input);

                ui.add_space(15.0);

                // Email
                ui.label(egui::RichText::new("Email").color(TEXT_PRIMARY));
                ui.add_space(5.0);
                let email_input = egui::TextEdit::singleline(&mut self.email)
                    .min_size(egui::vec2(300.0, 35.0));
                ui.add(email_input);

                ui.add_space(15.0);

                // Password
                ui.label(egui::RichText::new("Password").color(TEXT_PRIMARY));
                ui.add_space(5.0);
                let password_input = egui::TextEdit::singleline(&mut self.password)
                    .password(true)
                    .min_size(egui::vec2(300.0, 35.0));
                ui.add(password_input);

                ui.add_space(15.0);

                // 2FA Code
                ui.label(egui::RichText::new("2FA Code (optional)").color(TEXT_PRIMARY));
                ui.add_space(5.0);
                let totp_input = egui::TextEdit::singleline(&mut self.totp_code)
                    .min_size(egui::vec2(300.0, 35.0));
                ui.add(totp_input);

                ui.add_space(30.0);

                // Login Button
                ui.horizontal(|ui| {
                    let login_btn = egui::Button::new(
                        egui::RichText::new("🔐  Login")
                            .size(16.0)
                            .color(egui::Color32::WHITE)
                    )
                    .fill(PRIMARY_COLOR)
                    .min_size(egui::vec2(300.0, 45.0))
                    .rounding(22.5);

                    if ui.add(login_btn).clicked() {
                        self.handle_login(ctx);
                    }
                });

                if self.is_logging_in {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(egui::RichText::new("Logging in...").color(TEXT_SECONDARY));
                    });
                }

                ui.add_space(20.0);

                // Back button
                let back_btn = egui::Button::new(
                    egui::RichText::new("← Back")
                        .color(TEXT_SECONDARY)
                )
                .fill(egui::Color32::TRANSPARENT);

                if ui.add(back_btn).clicked() {
                    self.mode = AppMode::Initial;
                }
            });
        });
    }

    fn render_guest_mode(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);

            // Guest Mode Header
            ui.label(
                egui::RichText::new("🎉 Guest Mode Active")
                    .size(32.0)
                    .color(SUCCESS_COLOR)
                    .strong()
            );

            ui.add_space(20.0);

            // Timer
            if let Some(remaining) = self.get_remaining_guest_time() {
                if remaining > 0 {
                    ui.label(
                        egui::RichText::new(format!("⏱️  Time remaining: {}", Self::format_time(remaining)))
                            .size(20.0)
                            .color(PRIMARY_COLOR)
                    );
                } else {
                    ui.label(
                        egui::RichText::new("⏰ Trial expired - Please sign in to continue")
                            .size(20.0)
                            .color(egui::Color32::RED)
                    );
                }
            }

            ui.add_space(30.0);

            // Connection ID
            ui.horizontal(|ui| {
                ui.add_space(250.0);
                ui.label(egui::RichText::new("Your Connection ID:").color(TEXT_SECONDARY));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(200.0);
                let id_text = egui::RichText::new(&self.guest_connection_id)
                    .size(28.0)
                    .color(PRIMARY_COLOR)
                    .strong()
                    .monospace();
                ui.label(id_text);
            });

            ui.add_space(40.0);

            // Remote Connection ID Input
            ui.horizontal(|ui| {
                ui.add_space(200.0);
                ui.label(egui::RichText::new("Connect to ID:").color(TEXT_PRIMARY));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(200.0);
                let input = egui::TextEdit::singleline(&mut self.remote_id_input)
                    .hint_text("Enter remote connection ID")
                    .min_size(egui::vec2(400.0, 40.0));
                ui.add(input);
            });

            ui.add_space(20.0);

            // Connect Button
            ui.horizontal(|ui| {
                ui.add_space(280.0);
                let connect_btn = egui::Button::new(
                    egui::RichText::new("🔗  Connect")
                        .size(16.0)
                        .color(egui::Color32::WHITE)
                )
                .fill(PRIMARY_COLOR)
                .min_size(egui::vec2(200.0, 50.0))
                .rounding(25.0);

                if ui.add(connect_btn).clicked() && !self.remote_id_input.is_empty() {
                    let remote_id = self.remote_id_input.clone();
                    self.start_connection(remote_id, ctx);
                    self.mode = AppMode::Connected;
                    self.is_streaming = true;
                }
            });

            ui.add_space(40.0);

            // Upgrade message
            ui.label(
                egui::RichText::new("💎 Want unlimited connections? Sign in for full access!")
                    .size(14.0)
                    .color(TEXT_SECONDARY)
            );

            ui.add_space(10.0);

            // Sign in button
            let signin_btn = egui::Button::new(
                egui::RichText::new("Sign In")
                    .color(PRIMARY_COLOR)
            )
            .stroke(egui::Stroke::new(1.5, PRIMARY_COLOR))
            .fill(egui::Color32::WHITE)
            .min_size(egui::vec2(120.0, 35.0))
            .rounding(17.5);

            if ui.add(signin_btn).clicked() {
                self.mode = AppMode::Login;
            }
        });
    }

    fn render_connected_mode(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);

            // Connection info
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!("🔗 Connected to: {}", self.remote_device_id))
                        .size(18.0)
                        .color(SUCCESS_COLOR)
                );

                ui.add_space(20.0);

                // Disconnect button
                let disconnect_btn = egui::Button::new(
                    egui::RichText::new("❌ Disconnect")
                        .color(egui::Color32::WHITE)
                )
                .fill(egui::Color32::from_rgb(239, 68, 68)); // red-500

                if ui.add(disconnect_btn).clicked() {
                    self.stop_screen_capture();
                    self.is_streaming = false;
                    self.mode = AppMode::GuestMode;
                    self.remote_device_id.clear();
                    self.status_message = "Disconnected".to_string();
                }

                ui.add_space(20.0);

                // Start/Stop screen sharing button
                let share_btn_text = if self.is_capturing {
                    "⏸️ Stop Sharing"
                } else {
                    "▶️ Start Sharing"
                };

                let share_btn = egui::Button::new(
                    egui::RichText::new(share_btn_text)
                        .color(egui::Color32::WHITE)
                )
                .fill(PRIMARY_COLOR);

                if ui.add(share_btn).clicked() {
                    if self.is_capturing {
                        self.stop_screen_capture();
                    } else {
                        self.start_screen_capture(ctx);
                    }
                }
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // Remote screen display area
            ui.heading("Remote Screen");
            ui.add_space(10.0);

            // Placeholder for remote screen
            // TODO: Display actual remote screen texture
            let available_size = ui.available_size();
            let screen_rect = egui::Rect::from_min_size(
                ui.cursor().min,
                egui::vec2(available_size.x - 40.0, available_size.y - 60.0),
            );

            ui.allocate_ui_at_rect(screen_rect, |ui| {
                let painter = ui.painter();

                // Draw black background for remote screen
                painter.rect_filled(
                    screen_rect,
                    5.0,
                    egui::Color32::from_rgb(30, 30, 30),
                );

                // Draw border
                painter.rect_stroke(
                    screen_rect,
                    5.0,
                    egui::Stroke::new(2.0, PRIMARY_COLOR),
                );

                // Show "Waiting for remote screen..." text in center
                let text = if self.remote_screen_texture.is_some() {
                    "Remote Screen"
                } else {
                    "Waiting for remote screen..."
                };

                painter.text(
                    screen_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(20.0),
                    TEXT_SECONDARY,
                );

                // TODO: Render actual remote screen texture here
                // if let Some(texture) = &self.remote_screen_texture {
                //     ui.image(texture, screen_rect.size());
                // }
            });

            ui.add_space(10.0);

            // Status info
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!(
                        "📊 Streaming: {} | Capture FPS: {:.1}",
                        if self.is_streaming { "Active" } else { "Inactive" },
                        self.capture_fps
                    ))
                    .color(TEXT_SECONDARY)
                );
            });
        });
    }
}

impl eframe::App for ScrDeskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Custom background color
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = BACKGROUND_LIGHT;
        ctx.set_style(style);

        // Top panel with status
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);

                // Logo
                let logo = egui::RichText::new("S")
                    .size(24.0)
                    .color(egui::Color32::WHITE)
                    .strong();

                ui.visuals_mut().widgets.noninteractive.bg_fill = PRIMARY_COLOR;
                ui.visuals_mut().widgets.noninteractive.rounding = egui::Rounding::same(8.0);

                ui.label(logo);

                ui.add_space(10.0);

                let title = egui::RichText::new("ScrDesk")
                    .size(20.0)
                    .color(PRIMARY_COLOR)
                    .strong();
                ui.label(title);

                ui.separator();

                // Status message
                if let Some(ref error) = self.error_message {
                    ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
                } else {
                    ui.label(egui::RichText::new(format!("📊 {}", self.status_message)).color(TEXT_SECONDARY));
                }
            });
            ui.add_space(5.0);
        });

        // Bottom panel with version info
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).color(TEXT_SECONDARY));

                if self.logged_in {
                    if let Some(ref user) = self.user_info {
                        ui.separator();
                        ui.label(egui::RichText::new(format!("👤 {}", user.email)).color(TEXT_SECONDARY));
                    }
                } else if self.mode == AppMode::GuestMode {
                    ui.separator();
                    ui.label(egui::RichText::new("👥 Guest Mode").color(SUCCESS_COLOR));
                }
            });
            ui.add_space(5.0);
        });

        // Central panel - different views based on mode
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                AppMode::Initial => self.render_initial_mode(ui, ctx),
                AppMode::Login => self.render_login_mode(ui, ctx),
                AppMode::GuestMode => self.render_guest_mode(ui, ctx),
                AppMode::Connected => self.render_connected_mode(ui, ctx),
            }
        });

        // Handle incoming messages when connected
        if self.is_streaming {
            self.handle_incoming_messages(ctx);
        }

        // Update clipboard monitor
        if let Some(monitor) = self.clipboard_monitor.blocking_lock().as_mut() {
            monitor.tick();
        }

        // Request repaint for animations and timer updates
        ctx.request_repaint_after(std::time::Duration::from_millis(16)); // ~60 FPS
    }
}
