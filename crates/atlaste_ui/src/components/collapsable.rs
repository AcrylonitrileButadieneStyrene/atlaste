use bevy::{
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        rounded_corners::RoundedCorners,
    },
    prelude::*,
    ui_widgets::{Activate, observe},
};

#[derive(Debug, Default, Component)]
struct Collapsable;

pub fn new<T: Bundle>(bundle: T, label: impl Into<String>, collapsed: bool) -> impl Bundle {
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
            Spawn((
                Collapsable,
                Node {
                    display: match collapsed {
                        true => Display::None,
                        false => Display::DEFAULT,
                    },
                    ..Default::default()
                },
                Children::spawn_one(bundle),
            )),
        )),
    )
}

// this is so ugly
fn on_click(
    activation: On<Activate>,
    parent: Query<&ChildOf>,
    children: Query<&Children>,
    mut collapsable: Query<&mut Node, With<Collapsable>>,
) -> Result {
    let siblings = children.get(parent.get(activation.entity)?.0)?;
    for sibling in siblings {
        let Ok(mut node) = collapsable.get_mut(*sibling) else {
            continue;
        };

        // i cannot believe i made a pr to change the visibility of radio button circles from Visible to Inherited
        // when the correct thing to do the whole time was Display::None (it removes them from the layout entirely instead of just making them invisible)
        // if the settings pane was more like blender (tabs) and not a stack of groups then it would have been necessary though
        // so hopefully that is what it looks like i was trying to do :)
        node.display = match node.display {
            Display::None => Display::DEFAULT,
            _ => Display::None,
        }
    }

    Ok(())
}
