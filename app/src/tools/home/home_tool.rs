use crate::tools::tool::Tool;

pub struct HomeTool;

impl Tool for HomeTool {
    fn id(&self) -> &'static str {
        "home"
    }

    fn title(&self) -> String {
        "Home".to_string()
    }

    fn icon(&self) -> String {
        egui_phosphor::regular::HOUSE.to_owned()
    }

    fn ui(&mut self, _: &mut egui::Ui) {}
}
