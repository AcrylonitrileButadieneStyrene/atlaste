use std::path::PathBuf;

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
    .init_resource::<crate::ui::layout::ScreenCrop>()
    .add_systems(Startup, startup);

    // Taking it out of the field so it cannot accidentally be used later
    if let Some(game_dir) = args.game_dir.take() {
        app.insert_resource(GameDir(game_dir));
    }

    app.run();
}

#[derive(Resource)]
pub struct GameDir(pub PathBuf);

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d, crate::ui::layout::CroppedCamera));
}
