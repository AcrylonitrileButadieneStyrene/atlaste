use bevy::prelude::*;

mod code_page;
pub use code_page::CodePage;

pub fn new() -> impl Bundle {
    (
        Node {
            display: Display::Block,
            // flex_direction: FlexDirection::Column,
            grid_column: GridPlacement::start(3),
            ..Default::default()
        },
        BackgroundColor(Color::srgb_u8(50, 0, 0)),
        Children::spawn(Spawn(crate::components::collapsable::new(
            crate::components::enum_selector::new::<CodePage>(),
            "Codepage",
            Visibility::Hidden,
        ))),
    )
}
