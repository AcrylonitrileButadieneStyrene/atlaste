use bevy::{
    ecs::relationship::RelatedSpawner,
    feathers::controls::radio,
    prelude::*,
    ui::Checked,
    ui_widgets::{RadioGroup, ValueChange, observe},
};

#[derive(Clone, Debug, Component, Event)]
pub struct VariantSelected<T>(pub T);

pub fn new<T>() -> impl Bundle
where
    T: Clone + std::fmt::Debug + Default + PartialEq<T> + Send + Sync,
    T: strum::VariantArray + strum::EnumProperty,
{
    (
        Name::new(format!("Enum selector for {}", std::any::type_name::<T>())),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        RadioGroup,
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            for variant in T::VARIANTS {
                let mut ent = parent.spawn(radio(
                    VariantSelected(variant.clone()),
                    Spawn(Text::new(variant.get_str("Name").map_or_else(
                        || format!("{:?}", variant),
                        |str| str.to_owned(),
                    ))),
                ));

                if variant == &T::default() {
                    ent.insert(Checked);
                }
            }
        })),
        observe(
            |selected: On<ValueChange<Entity>>,
             children: Query<&Children>,
             variants: Query<&VariantSelected<T>>,
             checked: Query<Entity, With<Checked>>,
             mut commands: Commands|
             -> Result {
                // remove all existing `Checked` components because i have to do it myself for some reason
                for child in children.get(selected.source)? {
                    let Ok(entity) = checked.get(*child) else {
                        continue;
                    };

                    commands.get_entity(entity)?.remove::<Checked>();
                }

                commands.get_entity(selected.value)?.insert(Checked);
                commands.trigger(variants.get(selected.value)?.clone());
                Ok(())
            },
        ),
    )
}
