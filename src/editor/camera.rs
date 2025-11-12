use bevy::{
    color::palettes::tailwind::STONE_950, picking::hover::HoverMap, prelude::*,
    window::PrimaryWindow,
};

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

pub fn check_movement(
    mut camera: Query<&mut bevy_pancam::PanCam>,
    hover: Res<HoverMap>,
    window: Query<Entity, With<PrimaryWindow>>,
) -> Result {
    let window = window.single()?;

    // if all of the pointer devices are hovering over the window (no ui elements)
    // then the camera is enabled and the map can be moved around
    camera.single_mut()?.enabled = hover.iter().all(|(_, hits)| hits.contains_key(&window));

    Ok(())
}
