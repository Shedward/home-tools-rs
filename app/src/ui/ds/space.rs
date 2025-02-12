use egui::*;

pub struct Space(pub i8);

impl Space {
    pub const ZERO: Space = Space(0);
    pub const MIN: Space = Space(1);
    pub const MAX: Space = Space(5);
    pub const COLUMN: Space = Space(36);

    pub fn value(&self) -> f32 {
        4.0 * self.0 as f32
    }

    pub fn value_i8(&self) -> i8 {
        self.0 * 4
    }

    pub fn value_u8(&self) -> u8 {
        self.0 as u8 * 4
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
    pub width: Space,
    pub height: Space,
}

impl SpaceSize {
    pub const CONTROL_MIN: SpaceSize = SpaceSize {
        width: Space(4),
        height: Space(4),
    };

    pub const CONTROL: SpaceSize = SpaceSize {
        width: Space(6),
        height: Space(4),
    };

    pub const CONTROL_WIDE: SpaceSize = SpaceSize {
        width: Space(12),
        height: Space(4),
    };

    pub const CONTROL_COLUMN: SpaceSize = SpaceSize {
        width: Space::COLUMN,
        height: Space(4),
    };

    pub fn new(w: i8, h: i8) -> Self {
        Self {
            width: Space(w),
            height: Space(h),
        }
    }
}

impl From<Space> for Margin {
    fn from(value: Space) -> Self {
        Margin {
            left: value.value_i8(),
            right: value.value_i8(),
            top: value.value_i8(),
            bottom: value.value_i8(),
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

impl From<Space> for i8 {
    fn from(value: Space) -> Self {
        value.value_i8()
    }
}

impl From<Space> for u8 {
    fn from(value: Space) -> Self {
        value.value_i8() as u8
    }
}
