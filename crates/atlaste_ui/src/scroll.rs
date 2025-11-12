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
    let Some(delta) = wheel
        .read()
        .map(|event| match event.unit {
            MouseScrollUnit::Line => event.y * 24.,
            MouseScrollUnit::Pixel => event.y,
        })
        .reduce(|acc, x| acc + x)
    else {
        return;
    };

    for entity in hover
        .iter()
        .flat_map(|(_, map)| map.iter().map(|(entity, _)| entity))
    {
        if let Ok(mut scroll_position) = position.get_mut(*entity) {
            scroll_position.y -= delta;
        }
    }
}
