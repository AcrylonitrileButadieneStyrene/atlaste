use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn run(mut args: crate::Args) {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(bevy::window::Window {
                title: String::from("Atlaste"),
                ..Default::default()
            }),
            ..Default::default()
        }),
        TilemapPlugin,
        bevy_egui::EguiPlugin,
        crate::ui::Plugin,
    ))
    .init_asset::<crate::lcf_asset_loader::LcfAsset>()
    .init_asset_loader::<crate::lcf_asset_loader::LcfAssetLoader>()
    .init_resource::<crate::ui::layout::ScreenCrop>()
    .add_systems(Startup, startup)
    .add_systems(
        Update,
        crate::lcf_asset_loader::load_game.run_if(resource_added::<crate::app::GameDir>),
    );

    // Taking it out of the field so it cannot accidentally be used later
    if let Some(game_dir) = args.game_dir.take() {
        app.insert_resource(crate::app::GameDir(game_dir));
    }

    app.run();
}

#[derive(Resource)]
pub struct GameDir(pub std::path::PathBuf);

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d, crate::ui::layout::CroppedCamera));
}
