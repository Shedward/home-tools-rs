use std::collections::BTreeMap;

use egui::{style::*, *};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Light,
    Dark,
}

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

    fn accessory(&self) -> Color32 {
        self.tertiary
    }
}

pub enum Strokes {}

pub struct Space(pub i8);

impl Space {
    pub const ZERO: Space = Space(0);
    pub const MIN: Space = Space(1);
    pub const MAX: Space = Space(5);
    pub const COLUMN: Space = Space(36);

    pub fn value(&self) -> f32 {
        4.0 * self.0 as f32
    }

    pub fn add(&self, step: i8) -> Self {
        Self(self.0 + step)
    }

    pub fn down(&self) -> Self {
        self.add(-1)
    }

    pub fn up(&self) -> Self {
        self.add(1)
    }

    pub fn mult(&self, factor: i8) -> Self {
        Self(self.0 * factor)
    }
}

pub struct SpaceSize {
    width: Space,
    height: Space,
}

impl SpaceSize {
    const CONTROL_MIN: SpaceSize = SpaceSize {
        width: Space(4),
        height: Space(4),
    };

    const CONTROL: SpaceSize = SpaceSize {
        width: Space(6),
        height: Space(4),
    };

    const CONTROL_WIDE: SpaceSize = SpaceSize {
        width: Space(12),
        height: Space(4),
    };

    const CONTROL_COLUMN: SpaceSize = SpaceSize {
        width: Space::COLUMN,
        height: Space(4),
    };

    fn new(w: i8, h: i8) -> Self {
        Self {
            width: Space(w),
            height: Space(h),
        }
    }
}

impl From<Space> for Margin {
    fn from(value: Space) -> Self {
        Margin {
            left: value.value(),
            right: value.value(),
            top: value.value(),
            bottom: value.value(),
        }
    }
}

impl From<Space> for Vec2 {
    fn from(value: Space) -> Self {
        Vec2::new(value.value(), value.value())
    }
}

impl From<SpaceSize> for Vec2 {
    fn from(value: SpaceSize) -> Self {
        Vec2 {
            x: value.width.value(),
            y: value.height.value(),
        }
    }
}

impl From<Space> for f32 {
    fn from(value: Space) -> Self {
        value.value()
    }
}

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

pub struct Shadows {}

impl Shadows {
    pub fn window() -> egui::epaint::Shadow {
        epaint::Shadow {
            offset: (0.0, 4.0).into(),
            blur: Space(4).value(),
            spread: 0.0,
            color: Color32::from_black_alpha(128),
        }
    }

    pub fn panel() -> egui::epaint::Shadow {
        epaint::Shadow {
            offset: (0.0, 4.0).into(),
            blur: Space(2).value(),
            spread: 0.0,
            color: Color32::from_black_alpha(128),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub mode: Mode,
    pub colors: Colors,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: Mode::Light,
            colors: Colors::light(),
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: Mode::Dark,
            colors: Colors::dark(),
        }
    }

    pub fn style(&self) -> egui::Style {
        use egui::TextStyle::*;

        let text_styles: BTreeMap<_, _> = [
            (Heading, FontId::new(18.0, FontFamily::Monospace)),
            (Body, FontId::new(12.0, FontFamily::Monospace)),
            (
                TextStyle::Monospace,
                FontId::new(12.0, FontFamily::Monospace),
            ),
            (Button, FontId::new(12.0, FontFamily::Monospace)),
            (Small, FontId::new(10.0, FontFamily::Monospace)),
        ]
        .into();

        let spacing = Spacing {
            item_spacing: Space::MIN.into(),
            window_margin: Space::MAX.into(),
            button_padding: SpaceSize::new(1, 0).into(),
            menu_margin: Space::MIN.into(),
            indent: Space::MIN.into(),
            interact_size: SpaceSize::CONTROL_MIN.into(),
            slider_width: SpaceSize::CONTROL_WIDE.width.into(),
            slider_rail_height: SpaceSize::CONTROL.height.into(),
            combo_width: SpaceSize::CONTROL_WIDE.width.into(),
            text_edit_width: SpaceSize::CONTROL_COLUMN.width.into(),
            icon_width: Space(2).into(),
            icon_width_inner: Space::MIN.into(),
            icon_spacing: Space::MIN.into(),
            default_area_size: SpaceSize::CONTROL_COLUMN.into(),
            tooltip_width: SpaceSize::CONTROL_WIDE.width.into(),
            menu_width: Space::COLUMN.into(),
            menu_spacing: Space::ZERO.into(),
            indent_ends_with_horizontal_line: false,
            combo_height: SpaceSize::CONTROL.height.into(),
            scroll: ScrollStyle::thin(),
        };

        let widget_visuals = WidgetVisuals {
            bg_fill: self.colors.background,
            weak_bg_fill: self.colors.background_heading,
            bg_stroke: Strokes::thin(self.colors.heading),
            rounding: Rounding::ZERO,
            fg_stroke: Strokes::thin(self.colors.heading),
            expansion: 0.0,
        };

        let widgets = Widgets {
            noninteractive: WidgetVisuals {
                bg_stroke: Strokes::thin(self.colors.secondary),
                ..widget_visuals
            },
            inactive: WidgetVisuals {
                bg_stroke: Strokes::thin(self.colors.secondary),
                fg_stroke: Strokes::thin(self.colors.secondary),
                ..widget_visuals
            },
            hovered: WidgetVisuals {
                bg_stroke: Strokes::thin(self.colors.primary),
                ..widget_visuals
            },
            active: WidgetVisuals {
                bg_stroke: Strokes::thick(self.colors.primary),
                ..widget_visuals
            },
            open: widget_visuals,
        };

        let selection = Selection {
            bg_fill: self.colors.selection,
            stroke: Strokes::none(),
        };

        let visuals = Visuals {
            dark_mode: self.mode == Mode::Dark,
            override_text_color: Option::Some(self.colors.primary),

            widgets,
            selection,

            hyperlink_color: self.colors.link,
            faint_bg_color: self.colors.background,
            extreme_bg_color: self.colors.background_secondary,
            code_bg_color: self.colors.background_secondary,
            warn_fg_color: self.colors.attention,
            error_fg_color: self.colors.negative,
            window_fill: self.colors.background,
            panel_fill: self.colors.background,

            window_shadow: Shadows::window(),
            popup_shadow: Shadows::panel(),

            window_rounding: Rounding::ZERO,
            window_stroke: Strokes::thin(self.colors.accessory()),
            window_highlight_topmost: false,
            menu_rounding: Rounding::ZERO,
            resize_corner_size: 0.0,
            text_cursor: Default::default(),
            clip_rect_margin: Space::MIN.into(),
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: false,
            striped: true,
            slider_trailing_fill: true,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.25 },
            interact_cursor: Option::None,
            image_loading_spinners: false,
            numeric_color_space: NumericColorSpace::GammaByte,

            ..Default::default()
        };

        let egui_style = Style {
            override_text_style: Option::None,
            override_text_valign: Option::None,
            override_font_id: Option::None,

            text_styles,
            drag_value_text_style: Body,
            spacing,
            visuals,

            animation_time: 0.25,

            ..Default::default()
        };

        egui_style
    }
}
