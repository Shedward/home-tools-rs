use egui::*;

pub trait Movable
where
    Self: Sized,
{
    fn moved_by(&self, offset: (f32, f32)) -> Self;

    fn moved_up<D: Into<f32>>(&self, distance: D) -> Self {
        self.moved_by((0.0, -distance.into()))
    }

    fn moved_down<D: Into<f32>>(&self, distance: D) -> Self {
        self.moved_by((0.0, distance.into()))
    }

    fn moved_left<D: Into<f32>>(&self, distance: D) -> Self {
        self.moved_by((-distance.into(), 0.0))
    }

    fn moved_right<D: Into<f32>>(&self, distance: D) -> Self {
        self.moved_by((distance.into(), 0.0))
    }
}

impl Movable for Pos2 {
    #[inline]
    fn moved_by(&self, offset: (f32, f32)) -> Self {
        Pos2::new(self.x + offset.0, self.y + offset.1)
    }
}

impl Movable for Vec2 {
    #[inline]
    fn moved_by(&self, offset: (f32, f32)) -> Self {
        Vec2::new(self.x + offset.0, self.y + offset.1)
    }
}

impl Movable for Rect {
    #[inline]
    fn moved_by(&self, offset: (f32, f32)) -> Self {
        Rect {
            min: self.min.moved_by(offset),
            max: self.max.moved_by(offset),
        }
    }
}

pub trait Insetable
where
    Self: Sized,
{
    fn inset_by(&self, top: f32, left: f32, bottom: f32, right: f32) -> Self;

    fn inset_top<D: Into<f32>>(&self, distance: D) -> Self {
        self.inset_by(distance.into(), 0.0, 0.0, 0.0)
    }

    fn inset_left<D: Into<f32>>(&self, distance: D) -> Self {
        self.inset_by(0.0, distance.into(), 0.0, 0.0)
    }

    fn inset_bottom<D: Into<f32>>(&self, distance: D) -> Self {
        self.inset_by(0.0, 0.0, distance.into(), 0.0)
    }

    fn inset_right<D: Into<f32>>(&self, distance: D) -> Self {
        self.inset_by(0.0, 0.0, 0.0, distance.into())
    }

    fn inset_all<D: Into<f32>>(&self, distance: D) -> Self {
        let f = distance.into() as f32;
        self.inset_by(f, f, f, f)
    }

    fn inset_vertical<D: Into<f32>>(&self, distance: D) -> Self {
        let f = distance.into() as f32;
        self.inset_by(0.0, f, 0.0, f)
    }

    fn inset_horizontal<D: Into<f32>>(&self, distance: D) -> Self {
        let f = distance.into() as f32;
        self.inset_by(0.0, f, 0.0, f)
    }
}

impl Insetable for Rect {
    #[inline]
    fn inset_by(&self, top: f32, left: f32, bottom: f32, right: f32) -> Self {
        Rect {
            min: self.min.moved_by((left, top)),
            max: self.max.moved_by((-right, -bottom)),
        }
    }
}

pub struct RectEdge {
    pub center: Pos2,
    pub width: f32,
    pub height: f32,
}

impl RectEdge {
    pub fn with_width(self, width: f32) -> Self {
        RectEdge { width, ..self }
    }

    pub fn with_height(self, height: f32) -> Self {
        RectEdge { height, ..self }
    }

    pub fn with_rel_height(self, rel_height: f32) -> Self {
        RectEdge {
            height: self.height * rel_height,
            ..self
        }
    }

    pub fn with_rel_width(self, rel_width: f32) -> Self {
        RectEdge {
            width: self.width * rel_width,
            ..self
        }
    }

    pub fn as_rect(self) -> Rect {
        self.into()
    }
}

impl From<RectEdge> for Rect {
    fn from(edge: RectEdge) -> Self {
        let half_size = vec2(edge.width / 2.0, edge.height / 2.0);
        Rect {
            min: edge.center - half_size,
            max: edge.center + half_size,
        }
    }
}

pub trait ContainsEdges {
    fn rect(&self) -> Rect;

    fn right_edge(&self) -> RectEdge {
        let rect = self.rect();
        RectEdge {
            center: pos2(rect.max.x, rect.center().y),
            width: 0.0,
            height: rect.height(),
        }
    }

    fn left_edge(&self) -> RectEdge {
        let rect = self.rect();
        RectEdge {
            center: pos2(rect.min.x, rect.center().y),
            width: 0.0,
            height: rect.height(),
        }
    }

    fn top_edge(&self) -> RectEdge {
        let rect = self.rect();
        RectEdge {
            center: pos2(rect.center().x, rect.min.y),
            width: rect.width(),
            height: 0.0,
        }
    }

    fn bottom_edge(&self) -> RectEdge {
        let rect = self.rect();
        RectEdge {
            center: pos2(rect.center().x, rect.max.y),
            width: rect.width(),
            height: 0.0,
        }
    }
}

impl ContainsEdges for Rect {
    fn rect(&self) -> Rect {
        *self
    }
}
