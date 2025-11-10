use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Display::Grid,
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            grid_template_rows: vec![GridTrack::auto()],
            grid_template_columns: vec![
                GridTrack::px(200.0),
                GridTrack::auto(),
                GridTrack::px(200.0),
            ],
            ..Default::default()
        },
        BackgroundColor(Color::srgb_u8(0, 50, 0)),
        Children::spawn((Spawn(crate::sections::settings::new()),)),
    ));
}
