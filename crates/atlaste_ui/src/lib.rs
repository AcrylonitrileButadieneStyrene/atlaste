use bevy::prelude::*;

mod scroll;

pub mod components;
pub mod sections;
pub mod theme;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy::feathers::FeathersPlugin,
            bevy::ui_widgets::ButtonPlugin,
            bevy::ui_widgets::SliderPlugin,
            bevy::ui_widgets::CheckboxPlugin,
            bevy::ui_widgets::ScrollbarPlugin,
            bevy::ui_widgets::RadioGroupPlugin,
            bevy_simple_text_input::TextInputPlugin,
        ))
        .insert_resource(theme::default())
        .add_systems(PreStartup, theme::fonts::init)
        .add_systems(Startup, sections::setup)
        .add_systems(
            Update,
            (
                scroll::update,
                components::menu_button::check_hover,
                sections::map_tree::deselect_text_inputs,
                sections::map_tree::search
                    .after(bevy_simple_text_input::TextInputSystem)
                    .run_if(on_message::<bevy_simple_text_input::TextInputSubmitMessage>),
            ),
        )
        .add_observer(sections::map_tree::on_add_entries)
        .add_observer(sections::map_tree::on_code_page_changed)
        .add_observer(sections::map_tree::on_update_text);
    }
}
