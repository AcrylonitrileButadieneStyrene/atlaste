use bevy::{prelude::*, ui::Checked, ui_widgets::ValueChange};

pub fn on_radio_value_changed(
    event: On<ValueChange<Entity>>,
    group: Query<&Children>,
    mut commands: Commands,
) -> Result {
    commands.entity(event.value).insert(Checked);
    
    for child in group.get(event.source)? {
        if *child != event.value {
            commands.entity(*child).remove::<Checked>();
        }
    }

    Ok(())
}
