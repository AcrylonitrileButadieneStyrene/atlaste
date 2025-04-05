use bevy::prelude::*;

#[derive(Component)]
pub struct CroppedCamera;

#[derive(Default, Resource)]
pub struct ScreenCrop {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
}

pub fn update(
    mut camera: Query<&mut Camera, With<CroppedCamera>>,
    window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    crop: Res<ScreenCrop>,
) {
    let window = window.single();
    let window_width = window.physical_width();
    let window_height = window.physical_height();

    if window_width == 0 || window_height == 0 {
        return;
    }

    let scale = window.scale_factor();
    let convert = |x| (x / scale) as u32;

    let top = convert(crop.top);
    let left = convert(crop.left);
    let width = window_width.saturating_sub(convert(crop.right) + left);
    let height = window_height.saturating_sub(convert(crop.bottom) + top);

    let clamped_width = width.min(window_width).max(1);
    let clamped_height = height.min(window_height).max(1);
    let clamped_left = left.min(window_width.saturating_sub(clamped_width));
    let clamped_top = top.min(window_width.saturating_sub(clamped_height));

    camera.single_mut().viewport = Some(bevy::render::camera::Viewport {
        physical_position: UVec2::new(clamped_left, clamped_top),
        physical_size: UVec2::new(clamped_width, clamped_height),
        ..Default::default()
    });
}
