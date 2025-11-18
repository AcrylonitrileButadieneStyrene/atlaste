use bevy::{asset::AssetPath, prelude::*};

mod local;
mod remote;

#[derive(Event)]
pub struct LoadGameFrom(pub AssetPath<'static>);

pub fn new() -> impl Bundle {
    crate::components::menu_button::new("File", (Spawn(local::new()), Spawn(remote::new())))
}
