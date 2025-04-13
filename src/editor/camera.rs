use bevy::{picking::focus::HoverMap, prelude::*};

// TODO: drop this dependency and use the below system instead
// The step size is nicer. It is logarithmic for zooming in and linear for zooming out.
// I couldn't figure out how to make the screen move the same amount as the mouse, so I used this dependency.
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        bevy_pancam::PanCam {
            move_keys: bevy_pancam::DirectionKeys::NONE,
            grab_buttons: vec![MouseButton::Right],
            ..Default::default()
        },
    ));
}

pub fn check_movement(mut camera: Query<&mut bevy_pancam::PanCam>, hover: Res<HoverMap>) {
    let hitting = hover.iter().any(|hits| !hits.1.is_empty());
    camera.single_mut().enabled = !hitting;
}

// pub fn zoom(
//     mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
//     mut scroll: EventReader<MouseWheel>,
//     mut level: Local<i32>,
// ) {
//     if scroll.is_empty() {
//         return;
//     }

//     for scroll in scroll.read() {
//         if scroll.y.is_sign_positive() {
//             *level -= 1;
//         } else if scroll.y.is_sign_negative() {
//             *level += 1;
//         }
//     }

//     camera.single_mut().scale = std::f32::consts::E.powi(*level).ln_1p() / std::f32::consts::LN_2;
// }
