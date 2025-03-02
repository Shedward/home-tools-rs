use egui::*;

use crate::ui::ds::space::Space;

pub struct Cell {
    pub fill: Color32,
}

pub struct Pos {
    pub x: u32,
    pub y: u32,
}

pub struct DotGrid<CellFn: Fn(Pos) -> Cell> {
    pub cols: u32,
    pub rows: u32,

    pub padding: Vec2,
    pub cell_size: Vec2,
    pub inner_padding: Vec2,

    pub item: CellFn,
}

impl<CellFn> DotGrid<CellFn>
where
    CellFn: Fn(Pos) -> Cell,
{
    pub fn new(cell: CellFn) -> Self {
        Self {
            cols: 5,
            rows: 5,
            padding: Space(1).into(),
            cell_size: Space(2).into(),
            inner_padding: vec2(Space::HALF_STEP as f32, Space::HALF_STEP as f32),
            item: cell,
        }
    }

    #[inline]
    pub fn with_size(mut self, cols: usize, rows: usize) -> Self {
        self.cols = cols as u32;
        self.rows = rows as u32;
        self
    }

    #[inline]
    pub fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }

    #[inline]
    pub fn with_inner_padding(mut self, inner_padding: Vec2) -> Self {
        self.inner_padding = inner_padding;
        self
    }

    #[inline]
    pub fn with_cell_size(mut self, cell_size: Vec2) -> Self {
        self.cell_size = cell_size;
        self
    }

    fn size(&self) -> Vec2 {
        Vec2 {
            x: self.cols as f32 * self.cell_size.x
                + safe_paddings_count(self.cols) * self.inner_padding.x
                + 2.0 * self.padding.x,
            y: self.rows as f32 * self.cell_size.y
                + safe_paddings_count(self.rows) * self.inner_padding.y
                + 2.0 * self.padding.y,
        }
    }
}

impl<CellFn> Widget for DotGrid<CellFn>
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
                let pos = Pos { x, y };
                let cell = (self.item)(pos);

                let position = Pos2 {
                    x: self.padding.x
                        + rect.min.x
                        + x as f32 * (self.cell_size.x + self.inner_padding.x),
                    y: self.padding.y
                        + rect.min.y
                        + y as f32 * (self.cell_size.y + self.inner_padding.y),
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
