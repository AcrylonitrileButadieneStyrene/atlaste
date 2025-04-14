use bevy::prelude::*;

use crate::{
    state::{CodePage, CurrentCodePage},
    ui::elements::collapsable::{Collapsable, CollapsableHeader},
};

#[derive(Component)]
pub struct CodePageValue(CodePage);

pub fn setup(mut commands: Commands, slots: Res<super::Slots>, fonts: Res<crate::fonts::Fonts>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            Collapsable::Collapsed,
        ))
        .set_parent(slots.right)
        .with_children(|children| {
            let font = TextFont::from_font(fonts.default.clone()).with_font_size(12.);

            children.spawn((
                Text::new("Code page"),
                font.clone().with_font_size(16.),
                CollapsableHeader,
            ));

            for code_page in [
                CodePage::Ascii,
                CodePage::Eastern,
                CodePage::Cyrillic,
                CodePage::ShiftJIS,
                CodePage::Big5,
            ] {
                children
                    .spawn((
                        Node {
                            margin: UiRect::left(Val::Px(8.)),
                            ..Default::default()
                        },
                        Button,
                        BackgroundColor(Color::NONE),
                        Text::new(code_page.to_str().to_owned()),
                        font.clone(),
                        CodePageValue(code_page),
                    ))
                    .observe(
                        move |_: Trigger<Pointer<Click>>, mut current: ResMut<CurrentCodePage>| {
                            current.0 = code_page;
                        },
                    );
            }
        });
}

pub fn highlight_codepage(
    mut query: Query<(&mut BackgroundColor, &CodePageValue)>,
    current: Res<CurrentCodePage>,
) {
    query
        .iter_mut()
        .for_each(|(mut background_color, code_page_value)| {
            if code_page_value.0 == current.0 {
                background_color.0 = Color::hsl(0., 0., 0.4);
            } else {
                background_color.0 = Color::NONE;
            }
        });
}
