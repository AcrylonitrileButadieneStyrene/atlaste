use bevy::prelude::*;

pub fn run(args: crate::Args) -> AppExit {
    let mut app = App::new();
    app.insert_resource(bevy::winit::WinitSettings::desktop_app())
        .add_plugins((
            default_plugins(),
            bevy_simple_text_input::TextInputPlugin,
            crate::state::Plugin,
            crate::editor::Plugin,
            crate::ui::Plugin,
            atlaste_lcf::Plugin,
        ))
        .init_resource::<crate::state::CurrentCodePage>()
        .add_systems(PreStartup, crate::fonts::init);

    if let Some(game_path) = args.game_dir {
        app.add_systems(Startup, move |mut commands: Commands| {
            commands.trigger(atlaste_lcf::Load(game_path.clone()));
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
