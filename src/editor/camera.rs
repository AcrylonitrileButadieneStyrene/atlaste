use bevy::{picking::hover::HoverMap, prelude::*};

pub fn setup(mut commands: Commands) {
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

pub fn check_movement(mut camera: Query<&mut bevy_pancam::PanCam>, hover: Res<HoverMap>) -> Result {
    camera.single_mut()?.enabled = !hover.iter().any(|hits| hits.1.len() > 1);
    Ok(())
}
