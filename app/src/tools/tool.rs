pub trait Tool {
    fn id(&self) -> &'static str;
    fn title(&self) -> String;
    fn icon(&self) -> String;
    fn ui(&mut self, ui: &mut egui::Ui);

    fn on_appear(&mut self) {}

    fn show(&mut self, context: &egui::Context) {
        egui::Window::new(self.title()).show(context, |ui| {
            self.ui(ui);
        });
    }
}
