use crate::constants::*;
use amethyst::core::math::Vector2;

/// Absolute position indicates the position in actual pixels on the screen
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Absolute(pub f32, pub f32);

/// Relative position indicates the position relative to the sprite width
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Relative(pub f32, pub f32);

impl Relative {
    /// Converts a relative position into an absolute position
    pub fn to_absolute(&self) -> Absolute {
        let Relative(rel_x, rel_y) = *self;

        let x = rel_x * SPRITE_WIDTH;
        let y = rel_y * SPRITE_HEIGHT;
        Absolute(x, y)
    }
}

impl Absolute {
    /// Converts an absolute position into a relative position
    pub fn to_relative(&self) -> Relative {
        let Absolute(abs_x, abs_y) = *self;
        let x = (abs_x / SPRITE_WIDTH).floor();
        let y = (abs_y / SPRITE_HEIGHT).floor();
        Relative(x, y)
    }
}

impl From<Relative> for Vector2<f32> {
    fn from(Relative(x, y): Relative) -> Vector2<f32> {
        Vector2::new(x, y)
    }
}

impl From<Absolute> for Vector2<f32> {
    fn from(Absolute(x, y): Absolute) -> Vector2<f32> {
        Vector2::new(x, y)
    }
}
