use super::colors::*;
use super::shadows::*;
use super::space::*;
use super::strokes::*;
use egui::{style::*, *};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Light,
    Dark,
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
            corner_radius: CornerRadius::ZERO,
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

            window_corner_radius: CornerRadius::ZERO,
            window_stroke: Strokes::thin(self.colors.accessory()),
            window_highlight_topmost: false,
            menu_corner_radius: CornerRadius::ZERO,
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
