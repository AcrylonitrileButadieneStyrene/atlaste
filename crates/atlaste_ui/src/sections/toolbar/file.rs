use bevy::{
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        rounded_corners::RoundedCorners,
    },
    prelude::*,
    ui_widgets::{Activate, observe},
};

pub fn new() -> impl Bundle {
    crate::components::menu_button::new(
        "File",
        (
            Spawn(button(
                ButtonProps {
                    variant: ButtonVariant::Normal,
                    corners: RoundedCorners::None,
                },
                disabled_on_wasm(),
                Spawn(Text::new("Open local")),
            )),
            Spawn(button(
                ButtonProps {
                    variant: ButtonVariant::Normal,
                    corners: RoundedCorners::None,
                },
                (),
                Spawn(Text::new("Open remote")),
            )),
        ),
    )
}

#[cfg(target_arch = "wasm32")]
fn disabled_on_wasm() -> impl Bundle {
    bevy::ui::InteractionDisabled
}

#[cfg(not(target_arch = "wasm32"))]
fn disabled_on_wasm() -> impl Bundle {
    observe(|_: On<Activate>, mut commands: Commands| {
        commands.spawn(crate::components::modal::new(
            "Open game from local path",
            Spawn((
                Node {
                    flex_grow: 1.,
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(), GridTrack::min_content()],
                    grid_template_rows: vec![
                        GridTrack::min_content(),
                        GridTrack::auto(),
                        GridTrack::min_content(),
                    ],
                    ..Default::default()
                },
                Children::spawn((
                    Spawn(crate::components::text_input::new(
                        "C:/...",
                        Node {
                            grid_column: GridPlacement::start(1),
                            grid_row: GridPlacement::start(1),
                            ..Default::default()
                        },
                    )),
                    Spawn((
                        Node {
                            grid_column: GridPlacement::start(2),
                            grid_row: GridPlacement::start(1),
                            ..Default::default()
                        },
                        Children::spawn_one(button(
                            ButtonProps {
                                variant: ButtonVariant::Normal,
                                corners: RoundedCorners::None,
                            },
                            observe(|_event: On<Activate>| info!("browse")),
                            Spawn(Text::new("Browse")),
                        )),
                    )),
                    Spawn((
                        Node {
                            grid_column: GridPlacement::start(2),
                            grid_row: GridPlacement::start(3),
                            ..Default::default()
                        },
                        Children::spawn_one(button(
                            ButtonProps {
                                variant: ButtonVariant::Primary,
                                corners: RoundedCorners::None,
                            },
                            observe(|_event: On<Activate>| info!("load")),
                            Spawn(Text::new("Load")),
                        )),
                    )),
                )),
            )),
        ));
    })
}
