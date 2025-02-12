use super::space::*;
use egui::*;

pub struct Shadows {}

impl Shadows {
    pub fn window() -> egui::epaint::Shadow {
        epaint::Shadow {
            offset: (0, 4).into(),
            blur: Space(4).value_u8(),
            spread: 0,
            color: Color32::from_black_alpha(128),
        }
    }

    pub fn panel() -> egui::epaint::Shadow {
        epaint::Shadow {
            offset: (0, 4).into(),
            blur: Space(2).value_u8(),
            spread: 0,
            color: Color32::from_black_alpha(128),
        }
    }
}
