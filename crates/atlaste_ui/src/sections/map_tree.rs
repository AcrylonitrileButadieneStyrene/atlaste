use bevy::{
    feathers::{theme::ThemeBackgroundColor, tokens},
    prelude::*,
    ui_widgets::observe,
};
use bevy_simple_text_input::{
    TextInput, TextInputCursorPos, TextInputInactive, TextInputPlaceholder, TextInputSettings,
    TextInputSubmitMessage,
};

use crate::sections::settings::CodePage;

#[derive(Component)]
pub struct Marker;

#[derive(Clone, Component)]
pub struct Entry {
    pub id: u32,
    pub name: Vec<u8>,
    pub indentation: u32,
}

#[derive(Event)]
pub struct AddEntries(pub CodePage, pub Vec<Entry>);

#[derive(Event)]
pub struct UpdateText(CodePage);

pub fn new() -> impl Bundle {
    (
        Marker,
        Name::new("UI Map Tree"),
        Node {
            flex_direction: FlexDirection::Column,
            grid_row: GridPlacement::start(2),
            grid_column: GridPlacement::start(1),
            padding: UiRect::all(px(4)),
            ..Default::default()
        },
        ThemeBackgroundColor(crate::theme::tokens::PANEL_BACKGROUND),
        Children::spawn(Spawn((
            Node {
                border: UiRect::bottom(Val::Px(1.)),
                width: Val::Percent(100.),
                ..Default::default()
            },
            ThemeBackgroundColor(tokens::BUTTON_BG),
            TextInput,
            TextInputSettings {
                retain_on_submit: true,
                ..Default::default()
            },
            TextInputCursorPos::default(),
            TextInputInactive(true),
            TextInputPlaceholder {
                value: String::from("Search..."),
                ..Default::default()
            },
            observe(
                |trigger: On<Pointer<Click>>, mut query: Query<&mut TextInputInactive>| {
                    query.get_mut(trigger.entity).unwrap().0 = false;
                },
            ),
        ))),
    )
}

pub fn deselect_text_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut bevy_simple_text_input::TextInputInactive>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        query.par_iter_mut().for_each(|mut inactive| {
            inactive.0 = true;
        });
    }
}

pub fn search(
    mut events: MessageReader<TextInputSubmitMessage>,
    check: Query<Has<Marker>>,
    mut query: Query<(&mut Node, &Text), With<Entry>>,
) {
    let event = events.read().last().unwrap();
    if !check.get(event.entity).unwrap_or_default() {
        return;
    }

    query.par_iter_mut().for_each(|(mut node, text)| {
        node.display = if text.0.contains(&event.value) {
            Display::DEFAULT
        } else {
            Display::None
        };
    });
}

pub fn on_add_entries(
    add: On<AddEntries>,
    map_tree: Query<(Entity, &Children), With<Marker>>,
    is_entry: Query<Has<Entry>>,
    mut commands: Commands,
) -> Result {
    let (map_tree, children) = map_tree.single()?;
    for child in children {
        if is_entry.get(*child)? {
            commands.entity(*child).despawn();
        }
    }

    commands.entity(map_tree).with_children(|parent| {
        for entry in &add.1 {
            parent.spawn((
                Node {
                    margin: UiRect::left(Val::Px(entry.indentation as f32 * 8.0)),
                    ..Default::default()
                },
                TextFont::from_font_size(12.),
                entry.clone(),
            ));
        }
    });
    commands.trigger(UpdateText(add.0));

    Ok(())
}

pub fn on_update_text(
    update: On<UpdateText>,
    fonts: Res<crate::theme::fonts::Fonts>,
    mut entries: Query<(&mut Text, &mut TextFont, &Entry)>,
) {
    let encoding = update.0.to_encoding();
    let font = fonts.font(&update.0.to_theme_token());

    entries
        .par_iter_mut()
        .for_each(|(mut text, mut text_font, entry)| {
            // TODO: if the name doesn't include its map id, prepend it.
            // and also append the parent
            // example: Map0123 map name P0045
            // probably best to do with a regex
            text.0 = encoding.decode(&entry.name).0.to_string();
            text_font.font = font.clone();
        });
}
