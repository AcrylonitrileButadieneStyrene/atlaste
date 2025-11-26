use bevy::{
    feathers::{
        controls::radio,
        theme::{ThemeBackgroundColor, ThemeBorderColor},
    },
    prelude::*,
};
use strum::EnumProperty as _;

mod code_page;
pub use code_page::CodePage;

pub fn new() -> impl Bundle {
    (
        Name::new("UI Settings Panel"),
        Node {
            flex_direction: FlexDirection::Column,
            grid_row: GridPlacement::start(2),
            grid_column: GridPlacement::start(4),
            padding: UiRect::all(px(4)),
            border: UiRect::bottom(px(1)),
            ..Default::default()
        },
        ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
        ThemeBorderColor(crate::theme::tokens::PANEL_BORDER),
        Children::spawn_one(crate::components::collapsable::new(
            crate::components::enum_selector::new::<CodePage, _, _>(|variant| {
                radio(
                    (),
                    Spawn(Text::new(variant.get_str("Name").map_or_else(
                        || format!("{:?}", variant),
                        |str| str.to_owned(),
                    ))),
                )
            }),
            "Codepage",
            false,
        )),
    )
}
