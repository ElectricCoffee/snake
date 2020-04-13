use crate::constants::*;
use amethyst::core::math::{Vector2, Vector3};

pub trait RichPosition {
    fn to_absolute(&self) -> Absolute;
    fn to_relative(&self) -> Relative;
}

/// Absolute position indicates the position in actual pixels on the screen
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Absolute(pub f32, pub f32);

/// Relative position indicates the position relative to the sprite width
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Relative(pub i32, pub i32);

impl RichPosition for Relative {
    /// Converts a relative position into an absolute position
    fn to_absolute(&self) -> Absolute {
        let Relative(rel_x, rel_y) = *self;

        let x = rel_x as f32 * SPRITE_WIDTH;
        let y = rel_y as f32 * SPRITE_HEIGHT;
        Absolute(x, y)
    }

    fn to_relative(&self) -> Relative {
        self.clone()
    }
}

impl RichPosition for Absolute {
    /// Converts an absolute position into a relative position
    fn to_relative(&self) -> Relative {
        let Absolute(abs_x, abs_y) = *self;
        let x = (abs_x / SPRITE_WIDTH).floor() as i32;
        let y = (abs_y / SPRITE_HEIGHT).floor() as i32;
        Relative(x, y)
    }

    fn to_absolute(&self) -> Absolute {
        self.clone()
    }
}

impl From<Relative> for Vector2<f32> {
    fn from(Relative(x, y): Relative) -> Vector2<f32> {
        Vector2::new(x as f32, y as f32)
    }
}

impl From<Absolute> for Vector2<f32> {
    fn from(Absolute(x, y): Absolute) -> Vector2<f32> {
        Vector2::new(x, y)
    }
}

impl RichPosition for Vector2<f32> {
    fn to_relative(&self) -> Relative {
        Absolute(self.x, self.y).to_relative()
    }
    fn to_absolute(&self) -> Absolute {
        Absolute(self.x, self.y)
    }
}

impl RichPosition for Vector3<f32> {
    fn to_relative(&self) -> Relative {
        Absolute(self.x, self.y).to_relative()
    }

    fn to_absolute(&self) -> Absolute {
        Absolute(self.x, self.y)
    }
}
