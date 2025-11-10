use atlaste_ui::{components::enum_selector::VariantSelected, sections::settings::CodePage};
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct CurrentCodePage(pub atlaste_ui::sections::settings::CodePage);

pub fn on_changed(selected: On<VariantSelected<CodePage>>, mut commands: Commands) {
    commands.insert_resource(CurrentCodePage(selected.0));
}
