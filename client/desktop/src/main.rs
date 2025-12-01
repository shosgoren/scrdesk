mod api;
mod connection;

use api::{ApiClient, RegisterDeviceRequest};
use connection::{ConnectionManager, ConnectionState};
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
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

    // Session state
    logged_in: bool,
    user_info: Option<api::UserInfo>,
    device_id: String,
    device_key: Arc<Mutex<Option<String>>>,

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
            logged_in: false,
            user_info: None,
            device_id: String::new(),
            device_key: Arc::new(Mutex::new(None)),
            available_devices: Arc::new(Mutex::new(Vec::new())),
            selected_device: None,
            status_message: "Ready".to_string(),
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
}

impl eframe::App for ScrDeskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🖥️ ScrDesk PRO Enterprise");
                ui.separator();
                if let Some(ref error) = self.error_message {
                    ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
                } else {
                    ui.label(format!("📊 {}", self.status_message));
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
                if self.logged_in {
                    if let Some(ref user) = self.user_info {
                        ui.separator();
                        ui.label(format!("👤 {}", user.email));
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.logged_in {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading("Sign In");
                    ui.add_space(20.0);
                });

                ui.columns(3, |columns| {
                    columns[1].vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Server URL:");
                            ui.text_edit_singleline(&mut self.server_url);
                        });

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.label("Email:");
                            ui.text_edit_singleline(&mut self.email);
                        });

                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.label("Password:");
                            ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                        });

                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.label("2FA Code (optional):");
                            ui.text_edit_singleline(&mut self.totp_code);
                        });

                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            if ui.button("🔐 Login").clicked() {
                                self.handle_login(ctx);
                            }

                            if self.is_logging_in {
                                ui.spinner();
                            }
                        });
                    });
                });
            } else {
                ui.label("✅ Logged in successfully!");
                ui.label("Device management and connection features are being enhanced...");
            }
        });

        ctx.request_repaint();
    }
}
