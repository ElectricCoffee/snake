use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    prelude::*,
    renderer::formats::texture::ImageFormat,
    renderer::*,
};

use crate::{components::*, constants::*, util::position};

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let pos = position::Absolute(ARENA_CENTER_X, ARENA_CENTER_Y).to_relative();
        let sprite_sheet = load_sprite_sheet(world);
        world.register::<Snekment>();

        init_camera(world);
        Snekment::init(world, SegmentType::Head, pos, sprite_sheet);
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

pub fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let storage = world.read_resource::<AssetStorage<Texture>>();
        world.read_resource::<Loader>().load(
            "textures/sprites.png",
            ImageFormat::default(),
            (),
            &storage,
        )
    };

    let sprite_sheet = world.read_resource::<AssetStorage<SpriteSheet>>();
    world.read_resource::<Loader>().load(
        "textures/sprites.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet,
    )
}
