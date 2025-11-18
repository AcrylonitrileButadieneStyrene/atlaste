use bevy::{
    ecs::spawn::SpawnableList,
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        rounded_corners::RoundedCorners,
        theme::{ThemeBackgroundColor, ThemeBorderColor},
    },
    prelude::*,
    ui_widgets::{Activate, observe},
};

#[derive(Component)]
struct Marker;

pub fn new<C: SpawnableList<ChildOf> + Send + Sync + 'static>(
    name: impl Into<String>,
    contents: C,
) -> impl Bundle {
    (
        Marker,
        Node {
            flex_direction: FlexDirection::Column,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            min_width: px(600),
            min_height: px(450),
            ..Default::default()
        },
        ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
        ThemeBorderColor(crate::theme::tokens::PANEL_BORDER),
        Children::spawn((
            Spawn((
                Node {
                    column_gap: px(16),
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(px(4)),
                    border: UiRect::bottom(px(1)),
                    ..Default::default()
                },
                ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
                ThemeBorderColor(crate::theme::tokens::PANEL_BORDER),
                BoxShadow::new(Color::BLACK, Val::ZERO, Val::ZERO, px(8.), px(2.)),
                Children::spawn((
                    Spawn(Text::new(name)),
                    Spawn((
                        Node::DEFAULT,
                        Children::spawn_one(button(
                            ButtonProps {
                                variant: ButtonVariant::Primary,
                                corners: RoundedCorners::None,
                            },
                            observe(
                                |event: On<Activate>,
                                 mut commands: Commands,
                                 query: Query<&ChildOf>,
                                 filter: Query<Has<Marker>>| {
                                    for parent in query.iter_ancestors(event.entity) {
                                        if filter.get(parent).unwrap_or_default() {
                                            commands.entity(parent).despawn();
                                            break;
                                        }
                                    }
                                },
                            ),
                            Spawn(Text::new("X")),
                        )),
                    )),
                )),
            )),
            Spawn((
                Node {
                    flex_grow: 1.,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(px(4)),
                    ..Default::default()
                },
                Children::spawn(contents),
            )),
        )),
    )
}
