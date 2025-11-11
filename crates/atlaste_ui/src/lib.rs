use bevy::prelude::*;

pub mod components;
mod fonts;
pub mod sections;
pub mod theme;

pub use fonts::Fonts;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
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
        .add_systems(PreStartup, fonts::init)
        .add_systems(Startup, sections::setup)
        .add_systems(Update, components::menu_button::check_hover);
    }
}
