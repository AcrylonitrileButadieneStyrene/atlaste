use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Widget},
};

use crate::ui::retained::{CurrentTab, layout::ScreenCrop};

pub fn draw(
    mut contexts: EguiContexts,
    mut crop: ResMut<ScreenCrop>,
    current_tab: Res<State<CurrentTab>>,
    mut next_tab: ResMut<NextState<CurrentTab>>,
    map_tree: Option<Res<crate::lcf_asset_loader::MapTreeHandle>>,
) {
    let top = egui::TopBottomPanel::top("aspt")
        .show(contexts.ctx_mut(), |ui| {
            if egui::Button::new("Map Tree")
                .selected(matches!(*current_tab.get(), CurrentTab::MapTree))
                .ui(ui)
                .clicked()
                && map_tree.is_some()
            {
                next_tab.set(CurrentTab::MapTree);
            }
        })
        .response
        .rect
        .height();

    if crop.top.ne(&top) {
        crop.top = top;
    }
}
