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

mod state;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with(systems::CrabSystem, "crab_system", &["input_system"])
        .with(systems::KrabSystem, "krab_system", &[])
        .with(systems::MoveCrabSystem, "move_crab", &[])
        .with(
            systems::JumpSystem,
            "collision_system",
            &["crab_system", "move_crab"],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(resources, state::MenuState::default(), game_data)?;
    game.run();

    Ok(())
}
