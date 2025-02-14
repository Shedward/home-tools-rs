use std::collections::BTreeMap;
use std::sync::Arc;

use egui::FontFamily;

const JET_BRAINS_MONO_REGULAR: &[u8] =
    include_bytes!("../../../resources/fonts/JetBrainsMono-Regular.ttf");

const JET_BRAINS_MONO_BOLD: &[u8] =
    include_bytes!("../../../resources/fonts/JetBrainsMono-Bold.ttf");

pub enum Fonts {
    JetBrainsRegular,
    JetBrainsBold,
    PhosphorThin,
    PhosphorBold,
}

impl Fonts {
    const ALL: [Self; 4] = [
        Fonts::JetBrainsRegular,
        Fonts::JetBrainsBold,
        Fonts::PhosphorThin,
        Fonts::PhosphorBold,
    ];

    fn name(&self) -> &'static str {
        match self {
            Fonts::JetBrainsRegular => "JetBrainsRegular",
            Fonts::JetBrainsBold => "JetBrainsBold",
            Fonts::PhosphorThin => "PhosphorThin",
            Fonts::PhosphorBold => "PhosphorBold",
        }
    }

    fn font_data(&self) -> Arc<egui::FontData> {
        match self {
            Fonts::JetBrainsRegular => {
                Arc::new(egui::FontData::from_static(JET_BRAINS_MONO_REGULAR))
            }
            Fonts::JetBrainsBold => Arc::new(egui::FontData::from_static(JET_BRAINS_MONO_BOLD)),
            Fonts::PhosphorThin => Arc::new(egui_phosphor::Variant::Thin.font_data()),
            Fonts::PhosphorBold => Arc::new(egui_phosphor::Variant::Bold.font_data()),
        }
    }

    pub fn font_definitions() -> egui::FontDefinitions {
        let mut font_data: BTreeMap<String, Arc<egui::FontData>> = BTreeMap::new();
        for font in Self::ALL {
            font_data.insert(font.name().into(), font.font_data());
        }

        let mut families: BTreeMap<FontFamily, Vec<String>> = BTreeMap::new();
        for kind in FontKind::ALL {
            families.insert(kind.family(), kind.font_names());
        }

        let defs = egui::FontDefinitions {
            font_data,
            families,
        };

        defs
    }
}

pub enum FontKind {
    Body,
    Bold,
    Monospace,
}

impl FontKind {
    const ALL: [FontKind; 3] = [FontKind::Body, FontKind::Bold, FontKind::Monospace];

    #[inline]
    pub fn family(&self) -> FontFamily {
        match self {
            FontKind::Body => FontFamily::Proportional,
            FontKind::Monospace => FontFamily::Monospace,
            FontKind::Bold => FontFamily::Name("bold".into()),
        }
    }

    fn font_names(&self) -> Vec<String> {
        match self {
            FontKind::Body => vec![
                Fonts::JetBrainsRegular.name().into(),
                Fonts::PhosphorThin.name().into(),
            ],
            FontKind::Monospace => vec![
                Fonts::JetBrainsRegular.name().into(),
                Fonts::PhosphorThin.name().into(),
            ],
            FontKind::Bold => vec![
                Fonts::JetBrainsBold.name().into(),
                Fonts::PhosphorBold.name().into(),
            ],
        }
    }
}
