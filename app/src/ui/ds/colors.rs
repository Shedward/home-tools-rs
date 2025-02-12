use egui::*;

#[derive(Clone, Copy)]
pub struct Colors {
    pub heading: Color32,
    pub primary: Color32,
    pub secondary: Color32,
    pub tertiary: Color32,
    pub selection: Color32,
    pub link: Color32,

    pub background_heading: Color32,
    pub background: Color32,
    pub background_secondary: Color32,

    pub tint: Color32,
    pub positive: Color32,
    pub attention: Color32,
    pub negative: Color32,
}

impl Colors {
    pub fn light() -> Self {
        Colors::dark()
    }

    pub fn dark() -> Self {
        Self {
            heading: Color32::from_rgb(255, 255, 255),
            primary: Color32::from_rgb(251, 241, 198),
            secondary: Color32::from_rgb(111, 107, 94),
            tertiary: Color32::from_rgb(63, 63, 61),
            selection: Color32::from_rgb(63, 70, 67),
            link: Color32::from_rgb(131, 165, 152),

            background_heading: Color32::from_rgb(76, 70, 65),
            background: Color32::from_rgb(57, 55, 53),
            background_secondary: Color32::from_rgb(40, 40, 40),

            tint: Color32::from_rgb(131, 165, 152),
            positive: Color32::from_rgb(185, 187, 37),
            attention: Color32::from_rgb(251, 189, 46),
            negative: Color32::from_rgb(251, 74, 54),
        }
    }

    pub fn accessory(&self) -> Color32 {
        self.tertiary
    }
}
