use egui::{Context, Ui};

pub trait Tool {
    fn title(&self) -> String;
    fn icon(&self) -> String;
    fn ui(&self, ui: &mut Ui);

    fn show(&self, context: &egui::Context) {
        egui::Window::new(self.title()).show(context, |ui| {
            self.ui(ui);
        });
    }
}
