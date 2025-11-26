use bevy::{
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        rounded_corners::RoundedCorners,
        theme::ThemeBackgroundColor,
        tokens,
    },
    prelude::*,
    ui::Checked,
    ui_widgets::{RadioButton, observe},
};
use strum::EnumProperty;

#[derive(
    Clone, Debug, Default, PartialEq, Eq, Hash, strum::EnumProperty, strum::VariantArray, States,
)]
pub enum Tool {
    #[default]
    #[strum(props(Name = "St"))]
    Select,
    #[strum(props(Name = "Mv"))]
    Move,
}

pub fn new() -> impl Bundle {
    let button_props = || ButtonProps {
        variant: ButtonVariant::Normal,
        corners: RoundedCorners::None,
    };

    (
        Node {
            grid_row: GridPlacement::start_span(2, 2),
            grid_column: GridPlacement::start(2),
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(px(4)),
            row_gap: px(4),
            ..Default::default()
        },
        Children::spawn((
            Spawn((
                crate::components::enum_selector::new::<Tool, _, _>(tool_variant_to_button),
                ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
            )),
            Spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
                Children::spawn((Spawn(button(button_props(), (), Spawn(Text::new("Del")))),)),
            )),
        )),
    )
}

fn tool_variant_to_button(variant: &'static Tool) -> impl Bundle {
    (
        Node {
            padding: UiRect::horizontal(Val::Auto),
            height: px(40),
            ..Default::default()
        },
        RadioButton,
        Text::new(variant.get_str("Name").unwrap_or("Unknown")),
        TextFont::from_font_size(32.),
        observe(|on: On<Add, Checked>, mut commands: Commands| {
            commands
                .entity(on.entity)
                .insert(ThemeBackgroundColor(tokens::RADIO_MARK));
        }),
        observe(|on: On<Remove, Checked>, mut commands: Commands| {
            commands
                .entity(on.entity)
                .remove::<(ThemeBackgroundColor, BackgroundColor)>();
        }),
    )
}
