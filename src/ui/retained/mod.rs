use bevy::prelude::*;

pub mod layout;
pub mod map_tree;
pub mod path_warning;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CurrentTab>()
            .init_resource::<layout::ScreenCrop>()
            .add_observer(map_tree::redraw)
            .add_observer(
                |_: Trigger<crate::state::CodePage>, mut commands: Commands| {
                    commands.trigger(map_tree::Redraw);
                },
            )
            .add_systems(Startup, path_warning::setup)
            .add_systems(OnEnter(CurrentTab::MapTree), map_tree::setup)
            .add_systems(OnExit(CurrentTab::MapTree), map_tree::destroy)
            .add_systems(
                Update,
                (
                    (map_tree::scroll, map_tree::click).run_if(in_state(CurrentTab::MapTree)),
                    path_warning::toggle.run_if(resource_changed_or_removed::<crate::app::GameDir>),
                    layout::update.run_if(
                        resource_changed::<layout::ScreenCrop>
                            .or(on_event::<bevy::window::WindowResized>),
                    ),
                ),
            );
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum CurrentTab {
    #[default]
    None,
    MapTree,
}
