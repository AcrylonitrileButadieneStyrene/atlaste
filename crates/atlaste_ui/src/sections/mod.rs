use bevy::prelude::*;

pub mod layers;
pub mod map_tree;
pub mod settings;
pub mod toolbar;

pub const TITLE_BAR_HEIGHT: f32 = 28.;

pub(crate) fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Root"),
        Node {
            display: Display::Grid,
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            grid_template_rows: vec![
                GridTrack::px(TITLE_BAR_HEIGHT),
                GridTrack::flex(1.),
                GridTrack::flex(1.),
            ],
            grid_template_columns: vec![
                GridTrack::px(224.0),
                GridTrack::auto(),
                GridTrack::px(224.0),
            ],
            ..Default::default()
        },
        Pickable::IGNORE,
        Children::spawn((
            Spawn(crate::sections::toolbar::new()),
            Spawn(crate::sections::map_tree::new()),
            Spawn(crate::sections::settings::new()),
            Spawn(crate::sections::layers::new()),
        )),
    ));
}
