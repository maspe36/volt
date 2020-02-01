use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};

mod components;
mod states;
mod systems;

use crate::states::Overworld;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_root = app_root.join("config");
    let assets_dir = app_root.join("assets");
    let display_config_path = config_root.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(
                config_root.join("bindings.ron"),
            )?,
        )?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::TrainerMovementSystem, "trainer_movement_system", &["input_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(assets_dir, Overworld::default(), game_data)?;

    game.run();

    Ok(())
}
