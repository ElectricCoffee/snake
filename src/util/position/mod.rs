mod absolute;
mod relative;

pub use absolute::*;
pub use relative::*;

use amethyst::core::math::{Vector2, Vector3};

pub trait RichPosition {
    fn to_absolute(&self) -> Absolute;
    fn to_relative(&self) -> Relative;
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
