use crate::api;
use online::OnlineApp;

mod online {
    use crate::ui::{
        dot_grid, ds,
        services::{Services, SharedServices},
    };
    use egui::{vec2, Align, Layout, Widget};

    pub struct OnlineApp {
        label: String,
        value: f32,
        device: String,

        services: SharedServices,
    }

    impl Default for OnlineApp {
        fn default() -> Self {
            Self {
                label: "Online App".to_owned(),
                device: "-".to_owned(),
                value: 0.0,
                services: Services::create_shared(),
            }
        }
    }

    impl eframe::App for OnlineApp {
        fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
            let theme = self.theme();

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Home - Online");

                ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("device")
                            .selected_text(&self.device)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.device, "one".into(), "One");
                                ui.selectable_value(&mut self.device, "two".into(), "Two");
                                ui.selectable_value(&mut self.device, "three".into(), "Three");
                            });
                        egui::Separator::default().ui(ui);
                        egui::Button::new("Reload").ui(ui);
                    });
                    dot_grid::Grid {
                        cols: 14,
                        rows: 24,
                        padding: vec2(1.0, 1.0),
                        cell_size: vec2(16.0, 6.0),
                        item: |dot_grid::Pos(x, y)| dot_grid::Cell {
                            fill: if (x + y) % 7 == 0 {
                                theme.colors.positive
                            } else {
                                theme.colors.background_heading
                            },
                        },
                    }
                    .ui(ui)
                });
            });
        }
    }

    impl OnlineApp {
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            let app = OnlineApp::default();
            let style = app.services.lock().unwrap().theme.style();

            cc.egui_ctx.set_theme(egui::Theme::Dark);
            cc.egui_ctx.set_style(style);
            app
        }

        pub fn theme(&self) -> ds::Theme {
            self.services.lock().unwrap().theme.clone()
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
