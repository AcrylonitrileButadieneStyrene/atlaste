use bevy::{
    ecs::relationship::RelatedSpawner,
    prelude::*,
    ui::Checked,
    ui_widgets::{RadioGroup, ValueChange, observe},
};

#[derive(Clone, Debug, Component, Event)]
pub struct VariantSelected<T>(pub T);

pub fn new<T, S, B>(button_spawner: S) -> impl Bundle
where
    T: Clone + std::fmt::Debug + Default + PartialEq<T> + Send + Sync,
    T: strum::VariantArray,
    S: Fn(&'static T) -> B + Send + Sync + 'static,
    B: Bundle,
{
    (
        Name::new(format!("Enum selector for {}", std::any::type_name::<T>())),
        Node {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        RadioGroup,
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for variant in T::VARIANTS {
                let mut ent =
                    parent.spawn((VariantSelected(variant.clone()), button_spawner(variant)));

                if variant == &T::default() {
                    ent.insert(Checked);
                }
            }
        })),
        observe(
            |selected: On<ValueChange<Entity>>,
             variants: Query<&VariantSelected<T>>,
             mut commands: Commands|
             -> Result {
                commands.trigger(variants.get(selected.value)?.clone());
                Ok(())
            },
        ),
    )
}
