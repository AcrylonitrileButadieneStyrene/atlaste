use bevy::{
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        rounded_corners::RoundedCorners,
    },
    prelude::*,
};

pub fn new() -> impl Bundle {
    button(
        ButtonProps {
            variant: ButtonVariant::Normal,
            corners: RoundedCorners::None,
        },
        (),
        Spawn(Text::new("Load remote")),
    )
}
