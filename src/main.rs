mod play_state;

use amethyst::{
    core::{
        TransformBundle,
    },
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        types::DefaultBackend,
        RenderDebugLines, RenderFlat2D, RenderToWindow, RenderingBundle, 
    },
    tiles::{MortonEncoder, RenderTiles2D},
    utils::application_root_dir,
};

use play_state::{PlayState, MapTile};
use play_state::player_system::PlayerSystem;
use play_state::monster_ai_system::MonsterAISystem;


fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("roguelike", log::LevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;
    let assets_directory = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file("config/input.ron")?,
        )?
        .with(PlayerSystem::new(), "player_system", &["input_system"])
        .with(MonsterAISystem::new(), "monster_ai_system", &["player_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<MapTile, MortonEncoder>::default()),
        )?;

    let mut game = Application::build(assets_directory, PlayState)?.build(game_data)?;
    game.run();
    Ok(())
}