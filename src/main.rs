mod config;
mod gameplay;
mod systems;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use gameplay::*;
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    //let app_root = std::path::Path::new("/home/james/Code/blocks");
    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");
    let display_config_path = config_dir.join("display.ron");
    let bindings_path = config_dir.join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path).with_clear([
                        0.0,
                        10.0 / 255.0,
                        116.0 / 255.0,
                        1.0,
                    ]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::PaddleSystem, "paddle_system", &[])
        .with(
            systems::BounceBallSystem,
            "bounce_ball_system",
            &["paddle_system"],
        )
        .with(
            systems::MoveBallSystem,
            "move_ball_system",
            &["bounce_ball_system"],
        )
        .with(
            systems::BlockSystem,
            "block_system",
            &["bounce_ball_system"],
        );
    let mut game = Application::new(assets_dir, BlocksGame, game_data)?;
    game.run();

    Ok(())
}
