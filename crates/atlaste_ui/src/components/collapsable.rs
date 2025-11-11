use bevy::{
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        rounded_corners::RoundedCorners,
    },
    prelude::*,
    ui_widgets::{Activate, observe},
};

#[derive(Debug, Default, Component)]
struct Collapsable {
    /// Controls visibility when element is unhidden:
    /// - true: [`Visibility::Visible`]
    /// - false: [`Visibility::Inherited`]
    no_inherit: bool,
}

pub fn new<T: Bundle>(bundle: T, label: impl Into<String>, initial: Visibility) -> impl Bundle {
    (
        Name::new("Collapsible"),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        Children::spawn((
            Spawn(button(
                ButtonProps {
                    variant: ButtonVariant::Normal,
                    corners: RoundedCorners::None,
                },
                observe(on_click),
                Spawn(Text::new(label)),
            )),
            Spawn((Collapsable::default(), initial, bundle)),
        )),
    )
}

// this is so ugly
fn on_click(
    activation: On<Activate>,
    parent: Query<&ChildOf>,
    children: Query<&Children>,
    mut collapsable: Query<(&mut Collapsable, &mut Visibility)>,
) -> Result {
    let siblings = children.get(parent.get(activation.entity)?.0)?;
    for sibling in siblings {
        let Ok((mut collapsable, mut visibility)) = collapsable.get_mut(*sibling) else {
            continue;
        };

        *visibility = match *visibility {
            Visibility::Inherited => {
                collapsable.no_inherit = false;
                Visibility::Hidden
            }
            Visibility::Hidden if collapsable.no_inherit => Visibility::Visible,
            Visibility::Hidden => Visibility::Inherited,
            Visibility::Visible => {
                collapsable.no_inherit = true;
                Visibility::Hidden
            }
        }
    }

    Ok(())
}
