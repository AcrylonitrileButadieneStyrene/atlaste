mod elements;
mod layout;
mod themes;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((themes::Plugin, layout::Plugin))
            .add_systems(
                Update,
                (
                    scroll,
                    deselect_text_inputs,
                    (elements::collapsable::update, elements::collapsable::apply).chain(),
                ),
            );
    }
}

fn despawn<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn scroll(
    mut wheel: MessageReader<MouseWheel>,
    hover: Res<HoverMap>,
    mut position: Query<&mut ScrollPosition>,
) {
    for event in wheel.read() {
        let dy = match event.unit {
            MouseScrollUnit::Line => event.y * 24.,
            MouseScrollUnit::Pixel => event.y,
        };

        for (_, pointer_map) in hover.iter() {
            for (entity, _) in pointer_map {
                if let Ok(mut scroll_position) = position.get_mut(*entity) {
                    scroll_position.0.y -= dy;
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
