use bevy::prelude::*;

use crate::ui::popups::Popups;

pub fn setup(mut commands: Commands, slots: Res<super::Slots>, fonts: Res<crate::fonts::Fonts>) {
    commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Px(4.)),
                ..Default::default()
            },
            Text::new("Map Tree"),
            TextFont::from_font(fonts.default.clone()).with_font_size(16.0),
            Button,
        ))
        .set_parent(slots.top)
        .observe(
            |_: Trigger<Pointer<Click>>, mut query: Query<&mut Visibility>, popups: Res<Popups>| {
                if let Ok(mut visibility) = query.get_mut(popups.map_tree.0) {
                    *visibility = Visibility::Inherited;
                } else {
                    log::error!("Map tree entity or visibility missing");
                }
            },
        );
}
