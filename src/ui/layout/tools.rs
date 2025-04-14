use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::state::ToolState;

pub fn setup(mut commands: Commands, slots: Res<super::Slots>, fonts: Res<crate::fonts::Fonts>) {
    for state in ToolState::iter() {
        commands
            .spawn((
                Node {
                    margin: UiRect::all(Val::Px(4.)),
                    ..Default::default()
                },
                Text::new(state.to_name()),
                TextFont::from_font(fonts.default.clone()).with_font_size(16.0),
                Button,
            ))
            .set_parent(slots.left)
            .observe(
                move |_: Trigger<Pointer<Click>>, mut next: ResMut<NextState<ToolState>>| {
                    next.set(state.clone());
                },
            );
    }
}
