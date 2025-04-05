use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::state::Encoding;

pub fn draw(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut crop: ResMut<crate::ui::retained::layout::ScreenCrop>,
    mut state: ResMut<super::EguiState>,
) {
    let left = egui::SidePanel::left("aspl")
        .show(contexts.ctx_mut(), |ui| {
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
                            .and_then(|file| file.parent().map(ToOwned::to_owned))
                        {
                            state.game_dir = path;
                        }
                    }

                    if ui.button("Set").clicked() && state.game_dir.join("RPG_RT.ldb").exists() {
                        commands.insert_resource(crate::app::GameDir(state.game_dir.clone()));
                        commands.trigger(crate::app::GameDir(state.game_dir.clone()));
                    }
                });
            });

            ui.collapsing("Code page", |ui| {
                egui::ComboBox::from_label("")
                    .selected_text(state.code_page.map_or("None", |x| x.to_str()))
                    .show_ui(ui, |ui| {
                        let mut select = |encoding: crate::state::Encoding| {
                            ui.selectable_value(
                                &mut state.code_page,
                                Some(encoding),
                                encoding.to_str(),
                            )
                        };

                        let changed = [
                            select(Encoding::Ascii),
                            select(Encoding::Eastern),
                            select(Encoding::Cyrillic),
                            select(Encoding::ShiftJIS),
                            select(Encoding::Big5),
                        ]
                        .iter()
                        .any(egui::Response::changed);

                        if changed {
                            if let Some(encoding) = state.code_page {
                                commands.insert_resource(crate::state::CodePage(encoding));
                                commands.trigger(crate::state::CodePage(encoding));
                            }
                        }
                    })
            });

            // Resizing breaks without this.
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Label::new("Atlaste, dynamic atlases at last.")
                        .wrap_mode(egui::TextWrapMode::Extend),
                );
                ui.separator();
            })
        })
        .response
        .rect
        .width();

    if crop.left.ne(&left) {
        crop.left = left;
    }
}
