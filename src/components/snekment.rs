use crate::components::Orientation;
use crate::constants::*;
use crate::util::position::RichPosition;
use crate::util::*;
use amethyst::{
    assets::Handle,
    core::*,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::*,
};

#[allow(unused)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SegmentType {
    Head,
    Body,
    Tail,
}

impl SegmentType {
    fn sprite_number(&self) -> usize {
        use SegmentType::*;
        match *self {
            Head => sprites::HEAD_IDX,
            Body => sprites::BODY_IDX,
            Tail => sprites::TAIL_IDX,
        }
    }
}

/// A Snake Segment... A Snekment!
#[derive(Debug, Component)]
#[storage(DenseVecStorage)]
pub struct Snekment {
    pub seg_type: SegmentType,
    pub width: f32,
    pub height: f32,
}

impl Snekment {
    pub fn new(seg_type: SegmentType) -> Self {
        Self {
            seg_type,
            width: SPRITE_WIDTH,
            height: SPRITE_HEIGHT,
        }
    }

    pub fn init(
        world: &mut World,
        seg_type: SegmentType,
        pos: position::Relative,
        sprite_sheet: Handle<SpriteSheet>,
    ) {
        let pos = pos.to_absolute();
        let mut transform = Transform::default();
        transform.set_translation_xyz(pos.0, pos.1, 0.);

        let segment = Snekment::new(seg_type);

        let sprite_number = seg_type.sprite_number();

        let renderer = SpriteRender {
            sprite_sheet,
            sprite_number,
        };

        world
            .create_entity()
            .with(segment)
            .with(transform)
            .with(renderer)
            .with(Orientation::Right)
            .build();
    }
}
