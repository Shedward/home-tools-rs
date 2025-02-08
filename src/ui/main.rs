use online::OnlineApp;

mod online {

    use egui::{Slider, Theme};

    use crate::ui::services::{Services, SharedServices};

    pub struct OnlineApp {
        label: String,
        value: f32,

        services: SharedServices,
    }

    impl Default for OnlineApp {
        fn default() -> Self {
            Self {
                label: "Online App".to_owned(),
                value: 0.0,
                services: Services::create_shared(),
            }
        }
    }

    impl eframe::App for OnlineApp {
        fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
            egui::SidePanel::left("left").show(ctx, |ui| ui.button("Close"));
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Calculations");
                ui.separator();
                ui.label(format!("Label: {}", self.label));
                egui::Grid::new("grid").show(ui, |ui| {
                    ui.label("Label 1");
                    ui.label("Label 2");
                    ui.end_row();

                    ui.label("Label 3");
                    ui.button("Hello").clicked();
                    ui.button("This").clicked();
                    ui.button("Three").clicked();
                    ui.end_row();
                });

                ui.text_edit_singleline(&mut self.label);
                ui.add(Slider::new(&mut self.value, 0.0..=100.0));
                ui.add(Slider::new(&mut self.value, 0.0..=100.0));
            });
        }
    }

    impl OnlineApp {
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            let app = OnlineApp::default();
            let style = app.services.lock().unwrap().theme.style();

            cc.egui_ctx.set_theme(Theme::Dark);
            cc.egui_ctx.set_style(style);
            app
        }
    }
}

pub fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "eframe",
        native_options,
        Box::new(|cc| Ok(Box::new(OnlineApp::new(cc)))),
    )
}
