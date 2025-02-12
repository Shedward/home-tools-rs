use super::services::{Services, SharedServices};

pub struct App {
    pub services: SharedServices,
}

impl Default for App {
    fn default() -> Self {
        App {
            services: Services::create_shared(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("actions")
            .exact_width(64.0)
            .show(ctx, |_| {});
        egui::CentralPanel::default().show(ctx, |_| {});
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app = App::default();

        cc.egui_ctx.set_theme(egui::Theme::Dark);
        cc.egui_ctx.set_style(app.theme().style());

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

        app
    }

    pub fn theme(&self) -> crate::ui::ds::theme::Theme {
        self.services.lock().unwrap().theme
    }
}
