pub trait Tool {
    fn id(&self) -> &'static str;
    fn title(&self) -> String;
    fn icon(&self) -> String;
    fn ui(&self, ui: &mut egui::Ui);

    fn show(&self, context: &egui::Context) {
        egui::Window::new(self.title()).show(context, |ui| {
            self.ui(ui);
        });
    }
}
