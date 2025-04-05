pub mod immediate;
pub mod retained;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowTheme, WindowThemeChanged},
};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((retained::Plugin, immediate::Plugin))
            .add_systems(Startup, force_update_theme)
            .add_systems(Update, update_theme);
    }
}

fn force_update_theme(
    window: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut events: EventWriter<WindowThemeChanged>,
) {
    let (entity, window) = window.single();
    if let Some(theme) = window.window_theme {
        events.send(WindowThemeChanged {
            window: entity,
            theme,
        });
    }
}

fn update_theme(
    mut commands: Commands,
    mut contexts: bevy_egui::EguiContexts,
    mut events: EventReader<WindowThemeChanged>,
) {
    for event in events.read() {
        let (egui, color) = match event.theme {
            WindowTheme::Dark => (bevy_egui::egui::Visuals::dark(), Color::hsl(0., 0., 0.05)),
            WindowTheme::Light => (bevy_egui::egui::Visuals::light(), Color::hsl(0., 0., 0.75)),
        };

        contexts.ctx_mut().set_visuals(egui);
        commands.insert_resource(ClearColor(color));
    }
}
