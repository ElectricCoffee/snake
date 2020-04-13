use super::*;
use crate::constants::*;
use amethyst::core::math::Vector2;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Absolute position indicates the position in actual pixels on the screen
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Absolute(pub f32, pub f32);
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

impl From<Absolute> for Vector2<f32> {
    fn from(Absolute(x, y): Absolute) -> Vector2<f32> {
        Vector2::new(x, y)
    }
}

impl Add for Absolute {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Absolute(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Absolute {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Absolute {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Absolute(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Absolute {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
