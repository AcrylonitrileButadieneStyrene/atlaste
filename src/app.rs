use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn run(mut args: crate::Args) {
    let mut app = App::new();
    app.insert_resource(bevy::winit::WinitSettings::desktop_app())
        .add_plugins((
            default_plugins(),
            TilemapPlugin,
            bevy_simple_text_input::TextInputPlugin,
            crate::state::Plugin,
            crate::editor::Plugin,
            crate::ui::Plugin,
            crate::lcf_asset_loader::Plugin,
        ))
        .init_resource::<crate::state::CurrentCodePage>()
        .add_systems(PreStartup, crate::fonts::init)
        .add_systems(Startup, startup);

    // Taking it out of the field so it cannot accidentally be used later
    if let Some(game_dir) = args.game_dir.take() {
        app.insert_resource(crate::state::GamePath(game_dir));
    }

    app.run();
}

#[autodefault::autodefault]
fn default_plugins() -> bevy::app::PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(bevy::window::Window {
                title: String::from("Atlaste"),
                fit_canvas_to_parent: true,
                resize_constraints: WindowResizeConstraints { min_width: 400.0 },
            }),
        })
        .set(bevy::log::LogPlugin {
            #[cfg(debug_assertions)]
            level: bevy::log::Level::INFO,
            #[cfg(not(debug_assertions))]
            level: bevy::log::Level::WARN,
        })
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
