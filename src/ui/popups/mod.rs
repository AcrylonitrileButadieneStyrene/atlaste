use bevy::prelude::*;

mod map_tree;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    map_tree::search
                        .after(bevy_simple_text_input::TextInputSystem)
                        .run_if(on_event::<bevy_simple_text_input::TextInputSubmitEvent>),
                    map_tree::trigger
                        .run_if(resource_exists_and_changed::<crate::state::CurrentCodePage>),
                ),
            )
            .add_systems(OnEnter(crate::state::GameState::Loaded), map_tree::setup)
            .add_systems(OnExit(crate::state::GameState::Loaded), map_tree::destroy)
            .add_observer(map_tree::redraw);
    }
}

#[derive(Resource)]
pub struct Popups {
    pub map_tree: (Entity, Entity),
}

pub fn setup(mut commands: Commands, fonts: Res<crate::fonts::Fonts>) {
    let map_tree =
        crate::ui::elements::popup::spawn(&mut commands, "Map Tree", fonts.default.clone(), 4., 4.);
    commands.insert_resource(Popups { map_tree });
}
