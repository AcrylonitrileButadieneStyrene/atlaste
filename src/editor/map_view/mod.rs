/*!
   Controls the view of maps inside of the 2d editor.

   Loads begin in [`map_unit`], with an observer for when the map tree emits an [`atlaste_ui::sections::map_tree::EntryClicked`] event.

   When the map finishes loading:
   - [`events`] begins loading all of the different charsets for every event.
   - [`panorama`] begins loading the panorama image for the dynamic background.
   - [`tiles`] begins loading the chipset for the upper and lower tiles.
   - [`background`] creates a clickable region for controlling the map itself.
*/

use bevy::prelude::*;

pub mod background;
pub mod events;
pub mod map_unit;
pub mod panorama;
pub mod tiles;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((events::Plugin, panorama::Plugin, tiles::Plugin))
            .add_observer(background::on_add_map_unit)
            .add_observer(map_unit::on_map_tree_entry_clicked);
    }
}
