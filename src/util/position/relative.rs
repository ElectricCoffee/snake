use super::*;
use crate::constants::*;
use amethyst::core::math::Vector2;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Relative position indicates the position relative to the sprite width
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Relative(pub i32, pub i32);

impl Relative {
    pub fn up() -> Relative {
        Relative(0, 1)
    }

    pub fn down() -> Relative {
        Relative(0, -1)
    }

    pub fn left() -> Relative {
        Relative(-1, 0)
    }

    pub fn right() -> Relative {
        Relative(1, 0)
    }
}

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

impl From<Relative> for Vector2<f32> {
    fn from(Relative(x, y): Relative) -> Vector2<f32> {
        Vector2::new(x as f32, y as f32)
    }
}

impl Add for Relative {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Relative(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Relative {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Relative {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Relative(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Relative {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
