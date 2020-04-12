use amethyst::{core::Transform, prelude::*, renderer::*};

use crate::{components::*, constants::*};

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        init_camera(world);
        //SnakeSegment::init(world, SegmentType::Head)
    }
}

fn init_camera(world: &mut World) {
    let mut trans = Transform::default();
    trans.set_translation_xyz(ARENA_CENTER_X, ARENA_CENTER_Y, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(trans)
        .build();
}
