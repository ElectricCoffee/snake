use amethyst::ecs::{Component, DenseVecStorage};
#[derive(Debug, Clone, Copy, Eq, PartialEq, Component)]
#[storage(DenseVecStorage)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}
