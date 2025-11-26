use atlaste_ui::{
    components::enum_selector::VariantSelected,
    sections::{toolbar::file::LoadGameFrom, tools::Tool},
};
use bevy::prelude::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Tool>()
            .add_observer(on_ui_load_game)
            .add_observer(on_ui_tool_change);
    }
}

fn on_ui_load_game(event: On<LoadGameFrom>, mut commands: Commands) {
    commands.trigger(atlaste_lcf::Load(event.0.clone()));
}

fn on_ui_tool_change(event: On<VariantSelected<Tool>>, mut next: ResMut<NextState<Tool>>) {
    next.set(event.0.clone());
}
