use crate::constants::*;
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
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

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
    pub fn new(seg_type: SegmentType) -> Self {
        Self {
            seg_type,
            orientation: Orientation::Right,
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
            .build();
    }
}
