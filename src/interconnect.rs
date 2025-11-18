use bevy::prelude::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_ui_load_game);
    }
}

fn on_ui_load_game(
    event: On<atlaste_ui::sections::toolbar::file::LoadGameFrom>,
    mut commands: Commands,
) {
    commands.trigger(atlaste_lcf::Load(event.0.clone()));
}
