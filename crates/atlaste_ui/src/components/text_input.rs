use bevy::{
    feathers::{theme::ThemeBackgroundColor, tokens},
    prelude::*,
    ui_widgets::observe,
};
use bevy_simple_text_input::{
    TextInput, TextInputCursorPos, TextInputInactive, TextInputPlaceholder, TextInputSettings,
};

pub fn new(placeholder: impl Into<String>, overrides: impl Bundle) -> impl Bundle {
    (
        ThemeBackgroundColor(tokens::BUTTON_BG),
        TextInput,
        TextInputSettings {
            retain_on_submit: true,
            ..Default::default()
        },
        TextInputCursorPos::default(),
        TextInputInactive(true),
        TextInputPlaceholder {
            value: placeholder.into(),
            ..Default::default()
        },
        observe(
            |trigger: On<Pointer<Click>>, mut query: Query<&mut TextInputInactive>| {
                query.get_mut(trigger.entity).unwrap().0 = false;
            },
        ),
        overrides,
    )
}
