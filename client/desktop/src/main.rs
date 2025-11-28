use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("ScrDesk PRO Enterprise"),
        ..Default::default()
    };

    eframe::run_native(
        "ScrDesk",
        options,
        Box::new(|_cc| Box::new(ScrDeskApp::default())),
    )
}

struct ScrDeskApp {
    server_url: String,
    email: String,
    password: String,
    device_id: String,
    target_device_id: String,
    status: String,
    connected: bool,
}

impl Default for ScrDeskApp {
    fn default() -> Self {
        Self {
            server_url: "http://72.61.138.218:8000".to_string(),
            email: String::new(),
            password: String::new(),
            device_id: String::new(),
            target_device_id: String::new(),
            status: "Not connected".to_string(),
            connected: false,
        }
    }
}

impl eframe::App for ScrDeskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ScrDesk PRO Enterprise");
            ui.separator();

            if !self.connected {
                // Login panel
                ui.horizontal(|ui| {
                    ui.label("Server:");
                    ui.text_edit_singleline(&mut self.server_url);
                });

                ui.horizontal(|ui| {
                    ui.label("Email:");
                    ui.text_edit_singleline(&mut self.email);
                });

                ui.horizontal(|ui| {
                    ui.label("Password:");
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                });

                if ui.button("Login").clicked() {
                    self.status = "Connecting...".to_string();
                    // TODO: Implement actual login
                    self.connected = true;
                    self.status = "Connected".to_string();
                }
            } else {
                // Connected panel
                ui.label(format!("Status: {}", self.status));
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Your Device ID:");
                    ui.text_edit_singleline(&mut self.device_id);
                });

                ui.horizontal(|ui| {
                    ui.label("Target Device ID:");
                    ui.text_edit_singleline(&mut self.target_device_id);
                });

                if ui.button("Connect to Device").clicked() {
                    self.status = format!("Connecting to {}...", self.target_device_id);
                    // TODO: Implement RustDesk connection
                }

                ui.separator();

                if ui.button("Disconnect").clicked() {
                    self.connected = false;
                    self.status = "Not connected".to_string();
                }
            }

            ui.separator();
            ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
        });
    }
}
