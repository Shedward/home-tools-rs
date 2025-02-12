use egui::*;

use crate::ui::ds::space::Space;

pub struct Cell {
    pub fill: Color32,
}

pub struct Pos(pub u32, pub u32);

pub struct Grid<CellFn: Fn(Pos) -> Cell> {
    pub cols: u32,
    pub rows: u32,

    pub padding: Vec2,
    pub cell_size: Vec2,

    pub item: CellFn,
}

impl<CellFn> Grid<CellFn>
where
    CellFn: Fn(Pos) -> Cell,
{
    const INNER_INSETS: Space = Space(1);

    fn size(&self) -> Vec2 {
        Vec2 {
            x: self.cols as f32 * self.cell_size.x
                + safe_paddings_count(self.cols)
                + 2.0 * Self::INNER_INSETS.value(),
            y: self.rows as f32 * self.cell_size.y
                + safe_paddings_count(self.rows)
                + 2.0 * Self::INNER_INSETS.value(),
        }
    }
}

impl<CellFn> Widget for Grid<CellFn>
where
    CellFn: Fn(Pos) -> Cell,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size(), Sense::hover());

        ui.painter().rect_stroke(
            rect,
            0.0,
            ui.style().interact(&response).bg_stroke,
            StrokeKind::Inside,
        );

        for x in 0..self.cols {
            for y in 0..self.rows {
                let pos = Pos(x, y);
                let cell = (self.item)(pos);

                let position = Pos2 {
                    x: Self::INNER_INSETS.value()
                        + rect.min.x
                        + x as f32 * (self.cell_size.x + self.padding.x),
                    y: Self::INNER_INSETS.value()
                        + rect.min.y
                        + y as f32 * (self.cell_size.y + self.padding.y),
                };

                let cell_rect = Rect {
                    min: position,
                    max: position + self.cell_size,
                };

                ui.painter().rect_filled(cell_rect, 0.0, cell.fill);
            }
        }

        response
    }
}

fn safe_paddings_count(x: u32) -> f32 {
    if x > 0 {
        x as f32 - 1.0
    } else {
        0.0
    }
}
