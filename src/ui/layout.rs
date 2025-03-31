use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};

#[derive(Component)]
pub struct CroppedCamera;

#[derive(Default, Resource)]
pub struct ScreenCrop {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

pub fn update(
    mut camera: Query<&mut Camera, With<CroppedCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
    crop: Res<ScreenCrop>,
) {
    let mut camera = camera.single_mut();
    let window = window.single();
    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(crop.left, crop.top),
        physical_size: UVec2::new(
            window.physical_width() - crop.left - crop.right,
            window.physical_height() - crop.top - crop.bottom,
        ),
        ..Default::default()
    });
}
