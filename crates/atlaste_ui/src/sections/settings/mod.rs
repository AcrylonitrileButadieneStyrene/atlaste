use bevy::{feathers::theme::ThemeBackgroundColor, prelude::*};

mod code_page;
pub use code_page::CodePage;

pub fn new() -> impl Bundle {
    (
        Name::new("UI Settings Panel"),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            grid_row: GridPlacement::start(2),
            grid_column: GridPlacement::start(3),
            padding: UiRect::all(px(4)),
            ..Default::default()
        },
        ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
        Children::spawn_one(crate::components::collapsable::new(
            crate::components::enum_selector::new::<CodePage>(),
            "Codepage",
            false,
        )),
    )
}
