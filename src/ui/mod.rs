pub mod layout;
pub mod path_warning;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowTheme, WindowThemeChanged},
};
use bevy_egui::{EguiContexts, egui};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EguiState>()
            .add_systems(Startup, (force_update_theme, path_warning::setup))
            .add_systems(
                Update,
                (
                    update_theme,
                    draw,
                    path_warning::toggle.run_if(resource_changed_or_removed::<crate::app::GameDir>),
                    layout::update.run_if(resource_changed::<layout::ScreenCrop>),
                ),
            );
    }
}

#[derive(Default, Resource)]
struct EguiState {
    game_dir: std::path::PathBuf,
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
    mut contexts: EguiContexts,
    mut events: EventReader<WindowThemeChanged>,
) {
    for event in events.read() {
        let (egui, color) = match event.theme {
            WindowTheme::Dark => (egui::Visuals::dark(), Color::hsl(0., 0., 0.05)),
            WindowTheme::Light => (egui::Visuals::light(), Color::hsl(0., 0., 0.75)),
        };

        contexts.ctx_mut().set_visuals(egui);
        commands.insert_resource(ClearColor(color));
    }
}

fn draw(
    mut commands: Commands,
    mut contexts: EguiContexts,
    window: Query<&Window, With<PrimaryWindow>>,
    mut crop: ResMut<crate::ui::layout::ScreenCrop>,
    mut state: ResMut<EguiState>,
) {
    let window = window.single();
    let convert = |x| (x / window.scale_factor()) as u32;

    let left =
        egui::SidePanel::new(egui::panel::Side::Left, "aspl").show(contexts.ctx_mut(), |ui| {
            ui.collapsing("Game Installation Path", |ui| {
                let opt_str = state.game_dir.to_str().map(ToOwned::to_owned);
                ui.add_enabled_ui(opt_str.is_some(), |ui| {
                    let mut str = opt_str.unwrap_or_else(|| "INVALID!".to_owned());
                    if ui.text_edit_singleline(&mut str).changed() {
                        state.game_dir = std::path::PathBuf::from(str);
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("Open file picker").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("RGP_RT.ldb", &["ldb"])
                            .pick_file()
                            .and_then(|file| file.parent().map(|file| file.to_owned()))
                        {
                            state.game_dir = path;
                        }
                    }

                    if ui.button("Set").clicked() && state.game_dir.join("RPG_RT.ldb").exists() {
                        commands.insert_resource(crate::app::GameDir(state.game_dir.clone()));
                    }
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Label::new("Atlaste, dynamic atlases at last.")
                        .wrap_mode(egui::TextWrapMode::Extend),
                );
                ui.separator();
            })
        });
    let left = convert(left.response.rect.width());

    if crop.left != left {
        crop.left = left;
    }
}
