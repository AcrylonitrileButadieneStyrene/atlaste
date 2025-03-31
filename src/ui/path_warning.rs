use bevy::prelude::*;

#[derive(Component)]
pub struct Marker;

pub fn setup(mut commands: Commands) {
    let container = (
        Node {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        Marker,
    );

    let label1 = (
        Text::new("Game installation not set"),
        TextFont::from_font_size(36.),
    );
    let label2 = (
        Text::new("Select the game on the left sidebar"),
        TextFont::from_font_size(26.),
    );
    let label3 = (
        Text::new("Tip: launch with the --game-dir argument to skip this step"),
        TextFont::from_font_size(16.),
    );

    commands.spawn(container).with_children(|children| {
        children.spawn(label1);
        children.spawn(label2);
        children.spawn(label3);
    });
}

pub fn toggle(
    mut text: Query<&mut Visibility, With<Marker>>,
    game_dir: Option<Res<crate::app::GameDir>>,
) {
    *text.single_mut() = if game_dir.is_none() {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}
