use egui::*;

pub enum Strokes {}

impl Strokes {
    pub fn none() -> Stroke {
        Stroke::NONE
    }

    pub fn thin(color: Color32) -> Stroke {
        Stroke::new(1.0, color)
    }

    pub fn thick(color: Color32) -> Stroke {
        Stroke::new(2.0, color)
    }
}
