use crate::constants::*;
use crate::util::*;
use amethyst::{
    core::*,
    ecs::{Component, DenseVecStorage},
    prelude::*,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SegmentType {
    Head,
    Body,
    Tail,
}

/// A Snake Segment... A Snekment!
#[derive(Debug)]
pub struct Snekment {
    orientation: Orientation,
    seg_type: SegmentType,
    width: f32,
    height: f32,
}

impl Component for Snekment {
    type Storage = DenseVecStorage<Self>;
}

impl Snekment {
    fn new(seg_type: SegmentType) -> Self {
        Self {
            seg_type,
            orientation: Orientation::Right,
            width: SPRITE_WIDTH,
            height: SPRITE_HEIGHT,
        }
    }

    fn init(world: &mut World, seg_type: SegmentType, pos: position::Relative) {
        let pos = pos.to_absolute();
        let mut transform = Transform::default();
        transform.set_translation_xyz(pos.0, pos.1, 0.);

        let segment = Snekment::new(seg_type);

        world.create_entity().with(segment).with(transform).build();
    }
}
