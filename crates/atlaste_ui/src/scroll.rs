use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

pub fn update(
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
