use bevy::prelude::*;

mod scroll;

pub mod components;
pub mod sections;
pub mod theme;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy::feathers::FeathersPlugins,
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
                sections::layers::handle_escape,
            ),
        )
        .add_observer(sections::map_tree::on_add_entries)
        .add_observer(sections::map_tree::on_code_page_changed)
        .add_observer(sections::map_tree::on_update_text)
        .add_observer(sections::layers::on_create_add)
        .add_observer(sections::layers::on_deselect_all);
    }
}
