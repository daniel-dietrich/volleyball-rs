mod audio;
mod components;
mod systems;
mod volleyball;

use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

const RGBA: [i32; 4] = [0, 0, 0, 1];

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("bindings.ron");

    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(display_config_path)?.with_clear(RGBA))
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(input_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with(systems::PlayerSystem, "player_system", &[])
        .with(systems::BallSystem, "ball_system", &[])
        .with(systems::WinnerSystem, "winner_system", &[])
        .with(systems::BounceSystem, "bounce_system", &[]);

    let mut game = Application::new(assets_dir, volleyball::Volleyball, game_data)?;
    game.run();

    Ok(())
}
