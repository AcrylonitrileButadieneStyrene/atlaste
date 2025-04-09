use bevy::prelude::*;

#[derive(Component)]
pub struct Marker;

pub fn create(mut commands: Commands) {
    commands
        .spawn((
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(16.0)),
                width: Val::Percent(40.0),
                ..Default::default()
            },
            BackgroundColor(Color::hsl(0.0, 0.0, 0.5)),
            Marker,
        ))
        .with_children(|children| {
            children.spawn((Button, Text::new("Open game"))).observe(
                |_: Trigger<Pointer<Click>>, mut commands: Commands| {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("RPG_RT.ldb", &["ldb"])
                        .pick_file()
                        && path.exists()
                        && let Some(parent) = path.parent()
                    {
                        commands.insert_resource(crate::state::GamePath(parent.to_owned()));
                    }
                },
            );
        });
}
