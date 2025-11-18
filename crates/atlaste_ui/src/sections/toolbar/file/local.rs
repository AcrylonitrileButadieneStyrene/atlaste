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
        disabled_on_wasm(),
        Spawn(Text::new("Load local")),
    )
}

#[cfg(target_arch = "wasm32")]
fn disabled_on_wasm() -> impl Bundle {
    bevy::ui::InteractionDisabled
}

#[cfg(not(target_arch = "wasm32"))]
fn disabled_on_wasm() -> impl Bundle {
    use bevy::ui_widgets::{Activate, observe};

    observe(|_: On<Activate>, mut commands: Commands| {
        use bevy_simple_text_input::TextInputValue;

        commands.spawn(crate::components::modal::new(
            "Load game from local path",
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
                            observe(
                                |event: On<Activate>,
                                 parent: Query<&ChildOf>,
                                 children: Query<&Children>,
                                 text: Query<&TextInputValue>,
                                 mut commands: Commands|
                                 -> Result {
                                    let siblings =
                                        children.get(parent.get(event.entity)?.parent())?;
                                    for sibling in siblings {
                                        let Ok(text) = text.get(*sibling) else {
                                            continue;
                                        };

                                        commands
                                            .trigger(super::LoadGameFrom(text.0.clone().into()));
                                    }
                                    Ok(())
                                },
                            ),
                            Spawn(Text::new("Load")),
                        )),
                    )),
                )),
            )),
        ));
    })
}
