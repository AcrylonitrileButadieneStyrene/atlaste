use bevy::{
    feathers::{
        theme::{ThemeBackgroundColor, ThemeBorderColor, ThemeFontColor},
        tokens,
    },
    prelude::*,
};

pub mod file;

pub fn new() -> impl Bundle {
    (
        Name::new("UI Toolbar"),
        Node {
            grid_row: GridPlacement::start(1),
            grid_column: GridPlacement::start_span(1, 4),
            align_items: AlignItems::Center,
            border: UiRect::bottom(px(1)),
            padding: UiRect::axes(px(8.), Val::ZERO),
            column_gap: px(4),
            ..Default::default()
        },
        ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
        ThemeBorderColor(crate::theme::tokens::PANEL_BORDER),
        BoxShadow::new(Color::BLACK, Val::ZERO, Val::ZERO, px(8.), px(2.)),
        ZIndex(1),
        Children::spawn((
            Spawn((
                Text::new("Atlaste"),
                TextFont::from_font_size(20.),
                ThemeFontColor(tokens::TEXT_MAIN),
            )),
            Spawn(file::new()),
        )),
    )
}
