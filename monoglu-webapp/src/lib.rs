mod apps;
mod state;

use apps::Application;
use egui::style::Margin;
use egui::Color32;

struct WebApp {
    user_info: Option<state::UserInfo>,
    login: apps::Login,
    chat: apps::Chat,
}

impl WebApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let me = Self {
            user_info: None,
            login: apps::Login::new(),
            chat: apps::Chat::new(),
        };
        me
    }

    fn navigation_bar(&self, ctx: &egui::Context, ratio: f32) {
        let frame = egui::Frame::none()
            .inner_margin(Margin { left: 15.0, right: 15.0, top: 15.0, bottom: 15.0 })
            .fill(Color32::GRAY);
        egui::TopBottomPanel::top("navigation_bar")
            .frame(frame)
            .min_height(ctx.available_rect().height() * ratio)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Monoglu");
                });
            });
    }

    fn side_bar(&self, ctx: &egui::Context, ratio: f32) {
        egui::SidePanel::left("side_bar")
            .min_width(ctx.available_rect().width() * ratio)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Apps");
                ui.separator();

                // Add apps
                let chat_app = ui.label("chat");
            });
    }
}

impl eframe::App for WebApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.navigation_bar(ctx, 0.02);
        self.side_bar(ctx, 0.1);

        // Application guard distinguishing users from non-users.
        match &self.user_info {
            Some(user_info) => self.chat.view(ctx),
            None => self.login.view(ctx),
        };
    }
}

#[cfg(target_arch = "wasm32")]
pub fn run() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    // Enable logging on the browser console.
    wasm_logger::init(wasm_logger::Config::default());

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "main_canvas", // hardcode it
        web_options,
        Box::new(|cc| Box::new(WebApp::new(cc))),
    )
    .expect("failed to start eframe");
}
