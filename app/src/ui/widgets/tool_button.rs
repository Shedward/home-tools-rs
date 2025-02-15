use crate::ui::ds;
use egui::*;
use std::rc::Rc;

use crate::ui::ds::geometry::{ContainsEdges, Insetable, Movable};
use crate::{tools::tool::Tool, ui::ds::space::Space};

pub struct ToolButton {
    pub label: String,
    pub icon: String,
    pub is_selected: bool,
}

impl Widget for ToolButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let size: Vec2 = Space(12).into();
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        let content_rect = rect.inset_all(Space(1));

        let mut content_stroke = ui.style().interact(&response).fg_stroke;
        if self.is_selected {
            content_stroke.color = ui.visuals().text_color();
        }

        let text_rect = ui.painter().text(
            content_rect.center_bottom(),
            Align2::CENTER_BOTTOM,
            &self.label,
            ds::TextStyle::Caption.into(),
            content_stroke.color,
        );

        ui.painter().text(
            text_rect.center_top().moved_down(Space::HALF_STEP),
            Align2::CENTER_BOTTOM,
            &self.icon,
            ds::TextStyle::BigIcon.into(),
            content_stroke.color,
        );

        ui.painter()
            .rect_stroke(rect, 1.0, content_stroke, StrokeKind::Inside);

        if self.is_selected {
            let right_center = content_rect.right_edge().center;
            let offset = vec2(10.0, 0.0);

            ui.painter().arrow(right_center, offset, content_stroke);
        }

        response
    }
}

impl ToolButton {
    pub fn new(tool: &Rc<dyn Tool>, is_selected: bool) -> Self {
        Self {
            label: tool.title(),
            icon: tool.icon(),
            is_selected,
        }
    }
}
