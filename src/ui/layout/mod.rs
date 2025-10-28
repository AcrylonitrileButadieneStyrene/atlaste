mod map_tree;
mod settings;

use crate::state::{CurrentCodePage, GameLoadState};
use bevy::prelude::*;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ((setup, settings::setup).chain(),))
            .add_systems(
                Update,
                (
                    settings::highlight_codepage
                        .run_if(resource_exists_and_changed::<CurrentCodePage>),
                    map_tree::search
                        .after(bevy_simple_text_input::TextInputSystem)
                        .run_if(on_message::<bevy_simple_text_input::TextInputSubmitMessage>),
                    (|mut commands: Commands| commands.trigger(map_tree::UpdateText))
                        .run_if(resource_exists_and_changed::<crate::state::CurrentCodePage>),
                ),
            )
            .add_systems(OnEnter(GameLoadState::Loaded), map_tree::setup.after(setup))
            .add_observer(map_tree::update_text);
    }
}

#[derive(Resource)]
pub struct Layout {
    pub root: Entity,
}

pub fn setup(mut commands: Commands) {
    let root = commands
        .spawn((
            Node {
                display: Display::Grid,
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                grid_template_rows: vec![GridTrack::auto()],
                grid_template_columns: vec![
                    GridTrack::px(200.0),
                    GridTrack::auto(),
                    GridTrack::px(200.0),
                ],
                ..Default::default()
            },
            Pickable::IGNORE,
        ))
        .id();

    commands.insert_resource(Layout { root });
}
