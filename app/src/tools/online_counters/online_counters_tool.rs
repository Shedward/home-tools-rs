use crate::app::services::SharedServices;
use crate::tools::tool::Tool;
use crate::ui::widgets::dot_grid;
use egui::{Ui, Widget};

pub struct OnlineCountersTool {
    pub shared_services: SharedServices,
    pub rows: u32,
}

impl OnlineCountersTool {
    pub fn new(shared_services: SharedServices) -> Self {
        Self {
            shared_services,
            rows: 10,
        }
    }
}

impl Tool for OnlineCountersTool {
    fn id(&self) -> &'static str {
        "online_counters"
    }

    fn title(&self) -> String {
        "Online".to_owned()
    }

    fn icon(&self) -> String {
        egui_phosphor::regular::NETWORK.to_owned()
    }

    fn ui(&mut self, ui: &mut Ui) {
        let theme = self.shared_services.lock().unwrap().theme;
        egui::Slider::new(&mut self.rows, 1..=20).ui(ui);
        dot_grid::DotGrid::new(|pos| dot_grid::Cell {
            fill: theme.colors.positive.lerp_to_gamma(
                theme.colors.negative,
                (pos.x + pos.y % self.rows) as f32 / 20.0,
            ),
        })
        .with_size(14, 24)
        .ui(ui);
    }
}
