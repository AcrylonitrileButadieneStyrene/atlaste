use bevy::{feathers::theme::ThemeBackgroundColor, prelude::*};

pub fn new() -> impl Bundle {
    (
        Name::new("UI Map Tree"),
        Node {
            grid_row: GridPlacement::start(2),
            grid_column: GridPlacement::start(1),
            ..Default::default()
        },
        ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
    )
}
