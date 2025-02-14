use crate::tools::tool::Tool;
use egui::Ui;

pub struct OnlineCountersTool;

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

    fn ui(&self, ui: &mut Ui) {
        ui.label("Online Counters Tool");
    }
}
