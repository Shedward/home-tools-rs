use egui::*;
use std::rc::Rc;

use crate::ui::ds::geometry::{Insetable, Movable};
use crate::{tools::tool::Tool, ui::ds::space::Space};

pub struct ToolButton {
    pub label: String,
    pub icon: String,
}

impl Widget for ToolButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let size: Vec2 = Space(12).into();
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        let wv = ui.style().interact(&response);

        let content_rect = rect.inset_all(Space(1));

        let text_rect = ui.painter().text(
            content_rect.center_bottom(),
            Align2::CENTER_BOTTOM,
            &self.label,
            FontId::new(10.0, FontFamily::Monospace),
            wv.text_color(),
        );

        ui.painter().text(
            text_rect.center_top().moved_up(Space(1)),
            Align2::CENTER_BOTTOM,
            &self.icon,
            FontId::new(36.0, FontFamily::Monospace),
            wv.text_color(),
        );

        ui.painter()
            .rect_stroke(rect, 1.0, wv.fg_stroke, StrokeKind::Inside);

        response
    }
}

impl ToolButton {
    pub fn new(tool: &Rc<dyn Tool>) -> Self {
        Self {
            label: tool.title(),
            icon: tool.icon(),
        }
    }
}
