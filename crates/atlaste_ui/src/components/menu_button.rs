use bevy::{
    ecs::spawn::SpawnableList,
    feathers::controls::{ButtonProps, ButtonVariant, button},
    picking::hover::HoverMap,
    prelude::*,
    ui_widgets::{Activate, observe},
};

#[derive(Component)]
pub struct Marker;

#[derive(Component)]
pub struct Popup {
    pub timer: Timer,
}

impl Default for Popup {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.3, TimerMode::Once),
        }
    }
}

pub fn new<C: SpawnableList<ChildOf> + Send + Sync + 'static>(
    label: impl Into<String>,
    children: C,
) -> impl Bundle {
    (
        Marker,
        Name::new("Menu Button"),
        Node::DEFAULT,
        Children::spawn((
            Spawn(button(
                ButtonProps {
                    variant: ButtonVariant::Normal,
                    corners: bevy::feathers::rounded_corners::RoundedCorners::All,
                },
                observe(
                    |activation: On<Activate>,
                     parent: Query<&ChildOf>,
                     children: Query<&Children>,
                     mut visibility: Query<(&mut Visibility, &mut Popup)>|
                     -> Result {
                        for child in children.get(parent.get(activation.entity)?.parent())? {
                            let Ok((mut visibility, mut popup)) = visibility.get_mut(*child) else {
                                continue;
                            };

                            *visibility = match *visibility {
                                Visibility::Hidden => Visibility::Inherited,
                                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                            };
                            popup.timer.reset();
                        }

                        Ok(())
                    },
                ),
                Spawn(Text::new(label)),
            )),
            Spawn((
                Popup::default(),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(crate::sections::TITLE_BAR_HEIGHT),
                    ..Default::default()
                },
                Visibility::Hidden,
                ZIndex(2),
                BackgroundColor(Color::srgb_u8(0xff, 0, 0)),
                Children::spawn(children),
            )),
        )),
    )
}

// the app only updates when the mouse moves
// so this is incredibly buggy looking
// the popups can stay up forever instead of just 0.3s
// oh well
pub fn check_hover(
    hover: Res<HoverMap>,
    popups: Query<(Entity, &Children), With<Marker>>,
    recurse: Query<&Children>,
    mut content: Query<(&mut Visibility, &mut Popup)>,
    time: Res<Time<Real>>,
) -> Result {
    let mut popups = popups.iter().peekable();
    if popups.peek().is_none() {
        return Ok(());
    }

    let hits = hover
        .iter()
        .flat_map(|(_, hits)| hits.iter().map(|(entity, _)| *entity))
        .collect::<Vec<_>>();

    for (popup, children) in popups {
        let hovered = recurse
            .iter_descendants(popup)
            .any(|child| hits.contains(&child));
        for child in children {
            let Ok((mut visibility, mut popup)) = content.get_mut(*child) else {
                continue;
            };
            if hovered {
                popup.timer.set_elapsed(std::time::Duration::ZERO);
            } else {
                popup.timer.tick(time.delta());
                if popup.timer.is_finished() {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }

    Ok(())
}
