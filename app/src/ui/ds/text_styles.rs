use std::collections::BTreeMap;

use super::fonts::FontKind;

#[derive(Clone, Copy)]
pub enum TextStyle {
    Heading,
    Body,
    Caption,
    Monospace,

    BigIcon,
    Icon,
    Button,
}

impl From<TextStyle> for egui::FontId {
    #[inline]
    fn from(value: TextStyle) -> Self {
        match value {
            TextStyle::Heading => egui::FontId::new(18.0, FontKind::Bold.family()),
            TextStyle::Body => egui::FontId::new(12.0, FontKind::Body.family()),
            TextStyle::Caption => egui::FontId::new(10.0, FontKind::Body.family()),
            TextStyle::Monospace => egui::FontId::new(12.0, FontKind::Monospace.family()),
            TextStyle::BigIcon => egui::FontId::new(24.0, FontKind::Body.family()),
            TextStyle::Icon => egui::FontId::new(12.0, FontKind::Body.family()),
            TextStyle::Button => egui::FontId::new(12.0, FontKind::Body.family()),
        }
    }
}

impl From<TextStyle> for egui::TextStyle {
    #[inline]
    fn from(value: TextStyle) -> Self {
        match value {
            TextStyle::Heading => egui::TextStyle::Heading,
            TextStyle::Body => egui::TextStyle::Body,
            TextStyle::Caption => egui::TextStyle::Small,
            TextStyle::Monospace => egui::TextStyle::Name("Monospace".into()),
            TextStyle::BigIcon => egui::TextStyle::Name("BigIcon".into()),
            TextStyle::Icon => egui::TextStyle::Name("Icon".into()),
            TextStyle::Button => egui::TextStyle::Button,
        }
    }
}

impl TextStyle {
    pub const ALL: [Self; 7] = [
        TextStyle::Heading,
        TextStyle::Body,
        TextStyle::Caption,
        TextStyle::Monospace,
        TextStyle::BigIcon,
        TextStyle::Icon,
        TextStyle::Button,
    ];

    pub fn text_styles() -> BTreeMap<egui::TextStyle, egui::FontId> {
        let mut styles = BTreeMap::new();
        for style in Self::ALL {
            styles.insert(style.into(), style.into());
        }
        styles
    }
}
