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
            available_devices: Arc::new(Mutex::new(Vec::new())),
            selected_device: None,
            status_message: "Welcome to ScrDesk".to_string(),
            error_message: None,
            is_logging_in: false,
            is_registering_device: false,
            is_loading_devices: false,
        }
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

    fn start_guest_session(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.guest_session_start = Some(now);

        // Generate a random connection ID for guest mode
        self.guest_connection_id = format!("GUEST-{}", now);
        self.mode = AppMode::GuestMode;
        self.status_message = "Guest mode activated - 1 hour free trial".to_string();
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

    fn render_initial_mode(&mut self, ui: &mut egui::Ui) {
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
                self.start_guest_session();
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

    fn render_guest_mode(&mut self, ui: &mut egui::Ui) {
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
                let mut remote_id = String::new();
                let input = egui::TextEdit::singleline(&mut remote_id)
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

                if ui.add(connect_btn).clicked() {
                    self.status_message = "Connecting...".to_string();
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

    fn render_connected_mode(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(
                egui::RichText::new("✅ Connected Successfully!")
                    .size(28.0)
                    .color(SUCCESS_COLOR)
            );
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new("Remote desktop features and device management are being enhanced...")
                    .size(16.0)
                    .color(TEXT_SECONDARY)
            );
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
                AppMode::Initial => self.render_initial_mode(ui),
                AppMode::Login => self.render_login_mode(ui, ctx),
                AppMode::GuestMode => self.render_guest_mode(ui),
                AppMode::Connected => self.render_connected_mode(ui),
            }
        });

        // Request repaint for animations and timer updates
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}
