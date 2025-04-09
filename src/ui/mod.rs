mod elements;
mod layout;
mod popups;
mod start;
mod themes;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
};

use crate::state::{CurrentCodePage, GamePath, GameState};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((themes::Plugin, popups::Plugin))
            .add_systems(
                Startup,
                ((
                    layout::setup,
                    (layout::toolbar::setup, layout::settings::setup),
                )
                    .chain(),),
            )
            .add_systems(
                OnEnter(GameState::None),
                start::create.run_if(not(resource_exists::<GamePath>)),
            )
            .add_systems(OnExit(GameState::None), despawn::<start::Marker>)
            .add_systems(
                Update,
                (
                    scroll,
                    deselect_text_inputs,
                    (elements::collapsable::update, elements::collapsable::apply).chain(),
                    elements::popup::propagate,
                    layout::settings::highlight_codepage
                        .run_if(resource_exists_and_changed::<CurrentCodePage>),
                ),
            );
    }
}

fn despawn<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn scroll(
    mut wheel: EventReader<MouseWheel>,
    hover: Res<HoverMap>,
    mut position: Query<&mut ScrollPosition>,
) {
    for event in wheel.read() {
        let dy = match event.unit {
            MouseScrollUnit::Line => event.y * 24.,
            MouseScrollUnit::Pixel => event.y,
        };

        for (_, pointer_map) in hover.iter() {
            for (entity, _) in pointer_map.iter() {
                if let Ok(mut scroll_position) = position.get_mut(*entity) {
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

fn deselect_text_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut bevy_simple_text_input::TextInputInactive>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        query.par_iter_mut().for_each(|mut inactive| {
            inactive.0 = true;
        });
    }
}
