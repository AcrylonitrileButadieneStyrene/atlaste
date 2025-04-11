use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, bevy_pancam::PanCam::default()));
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

//     if let Ok(mut projection) = camera.get_single_mut() {
//         projection.scale = std::f32::consts::E.powi(*level).ln_1p() / std::f32::consts::LN_2;
//     }
// }
