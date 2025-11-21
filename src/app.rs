use bevy::prelude::*;

use crate::settings::Settings;

pub fn run(args: crate::Args) -> AppExit {
    let mut app = App::new();
    app.insert_resource(bevy::winit::WinitSettings::mobile())
        .add_plugins((
            default_plugins(),
            MeshPickingPlugin,
            atlaste_asset::Plugin,
            atlaste_lcf::Plugin,
            atlaste_ui::Plugin,
            crate::editor::Plugin,
            crate::state::Plugin,
            crate::utils::Plugin,
            crate::interconnect::Plugin,
        ))
        .init_resource::<crate::state::CurrentCodePage>()
        .insert_resource(
            bevy_persistent::Persistent::<crate::settings::Settings>::builder()
                .name("settings")
                .format(bevy_persistent::StorageFormat::Toml)
                .path(dirs::config_dir().map_or_else(
                    || std::path::Path::new("local").join("settings"),
                    |dir| dir.join("atlaste.toml"),
                ))
                .default(Settings::default())
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap(),
        );

    if let Some(game_path) = args.game_dir {
        app.add_systems(Startup, move |mut commands: Commands| {
            commands.trigger(atlaste_lcf::Load(game_path.clone().into()));
        });
    }

    app.run()
}

fn default_plugins() -> bevy::app::PluginGroupBuilder {
    DefaultPlugins
        .set(AssetPlugin {
            unapproved_path_mode: bevy::asset::UnapprovedPathMode::Allow,
            ..Default::default()
        })
        .set(WindowPlugin {
            primary_window: Some(bevy::window::Window {
                title: String::from("Atlaste"),
                fit_canvas_to_parent: true,
                resize_constraints: WindowResizeConstraints {
                    min_width: 400.0,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(bevy::log::LogPlugin {
            #[cfg(debug_assertions)]
            level: bevy::log::Level::INFO,
            #[cfg(not(debug_assertions))]
            level: bevy::log::Level::WARN,
            ..Default::default()
        })
}
