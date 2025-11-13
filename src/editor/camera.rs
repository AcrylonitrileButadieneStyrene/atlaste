use bevy::{color::palettes::tailwind::STONE_950, picking::hover::HoverMap, prelude::*};

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::from(STONE_950)));
    commands.spawn((
        Camera2d,
        bevy_pancam::PanCam {
            move_keys: bevy_pancam::DirectionKeys::NONE,
            grab_buttons: vec![MouseButton::Right],
            zoom_to_cursor: true,
            ..Default::default()
        },
    ));
}

pub fn disable_when_hovering_over_ui(
    mut camera: Query<&mut bevy_pancam::PanCam>,
    hover: Res<HoverMap>,
    is_ui: Query<Has<Node>>,
) -> Result {
    // actually you can just check if they are hovering over ui nodes
    camera.single_mut()?.enabled = !hover
        .iter()
        .flat_map(|(_, hits)| hits.iter().map(|(entity, _)| *entity))
        .any(|entity| is_ui.get(entity).unwrap_or_default());

    Ok(())
}
