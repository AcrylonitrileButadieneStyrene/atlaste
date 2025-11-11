use bevy::{color::palettes::tailwind, feathers::theme::UiTheme, prelude::*};

pub mod fonts;
pub mod tokens;

pub fn default() -> UiTheme {
    let mut theme = bevy::feathers::dark_theme::create_dark_theme();
    let colors = &mut theme.color;

    {
        use bevy::feathers::tokens;
        colors.insert(tokens::TEXT_MAIN, Color::from(tailwind::ZINC_100));
        colors.insert(tokens::TEXT_DIM, Color::from(tailwind::ZINC_400));
    }

    colors.insert(tokens::PANEL_BACKGROUND, Color::from(tailwind::ZINC_900));
    colors.insert(tokens::PANEL_BORDER, Color::from(tailwind::ZINC_800));

    UiTheme(theme)
}
