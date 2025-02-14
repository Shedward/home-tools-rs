use crate::tools::tool::Tool;

pub struct AboutTool;

impl Tool for AboutTool {
    fn title(&self) -> String {
        "About".to_string()
    }

    fn icon(&self) -> String {
        egui_phosphor::regular::INFO.to_owned()
    }

    fn ui(&self, ui: &mut egui::Ui) {
        ui.heading("About");
        ui.label("v0.0.0");
    }
}
