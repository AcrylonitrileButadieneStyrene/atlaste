use bevy::{prelude::*, ui_widgets::RadioGroup};

pub mod components;
mod fonts;
mod layout;
pub mod sections;

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
        .insert_resource(bevy::feathers::theme::UiTheme(
            bevy::feathers::dark_theme::create_dark_theme(),
        ))
        .add_systems(Startup, layout::setup)
        .add_systems(PreStartup, fonts::init);
    }
}
