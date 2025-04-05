use bevy::{prelude::*, winit::WinitSettings};
use bevy_ecs_tilemap::prelude::*;

pub fn run(mut args: crate::Args) {
    let mut app = App::new();
    app.insert_resource(WinitSettings::desktop_app())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(bevy::window::Window {
                    title: String::from("Atlaste"),
                    fit_canvas_to_parent: true,
                    enabled_buttons: bevy::window::EnabledButtons {
                        // Minimizing causes crashes with the viewport layout
                        minimize: false,
                        ..Default::default()
                    },
                    resize_constraints: WindowResizeConstraints {
                        min_width: 200.0,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            TilemapPlugin,
            bevy_egui::EguiPlugin,
            crate::ui::Plugin,
            crate::lcf_asset_loader::Plugin,
        ))
        .init_resource::<crate::state::CodePage>()
        .add_systems(Startup, (startup, crate::fonts::init));

    // Taking it out of the field so it cannot accidentally be used later
    if let Some(game_dir) = args.game_dir.take() {
        app.insert_resource(crate::app::GameDir(game_dir));
    }

    app.run();
}

#[derive(Clone, Event, Resource)]
pub struct GameDir(pub std::path::PathBuf);

fn startup(mut commands: Commands) {
    commands.spawn((Camera2d, crate::ui::retained::layout::CroppedCamera));
}
