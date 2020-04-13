mod components;
mod constants;
mod states;
mod systems;
mod types;
mod util;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use states::*;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings = config_dir.join("bindings.ron");

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?.with_clear(constants::BLACK),
        )
        .with_plugin(RenderFlat2D::default());

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(bindings)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(rendering_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::SnakeInput, "snake_input", &["input_system"])
        .with(systems::Movement, "movement_system", &["snake_input"]);

    let mut game = Application::new(assets_dir, MainState, game_data)?;
    game.run();

    Ok(())
}
