use bevy::{ecs::relationship::Relationship as _, prelude::*};

#[derive(Component)]
pub enum Collapsable {
    Collapsed,
    Expanded,
}

#[derive(Component)]
#[require(Button)]
pub struct CollapsableHeader;

pub fn update(
    query: Query<(&Interaction, &ChildOf), (With<CollapsableHeader>, Changed<Interaction>)>,
    mut collapsable: Query<&mut Collapsable>,
) {
    query.iter().for_each(|(interaction, parent)| {
        if *interaction != Interaction::Pressed {
            return;
        }

        if let Ok(mut collapsable) = collapsable.get_mut(parent.get()) {
            *collapsable = match *collapsable {
                Collapsable::Collapsed => Collapsable::Expanded,
                Collapsable::Expanded => Collapsable::Collapsed,
            };
        } else {
            log::error!("Collapsable not direct parent of header");
        }
    });
}

pub fn apply(
    query: Query<(&Collapsable, &Children), Changed<Collapsable>>,
    mut visibility: Query<&mut Visibility, Without<CollapsableHeader>>,
) {
    query.iter().for_each(|(collapsable, children)| {
        for child in children {
            if let Ok(mut visibility) = visibility.get_mut(*child) {
                *visibility = match collapsable {
                    Collapsable::Collapsed => Visibility::Hidden,
                    Collapsable::Expanded => Visibility::Inherited,
                }
            }
        }
    });
}
