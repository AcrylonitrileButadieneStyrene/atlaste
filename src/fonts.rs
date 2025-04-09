use bevy::prelude::*;

#[derive(Resource)]
pub struct Fonts {
    pub default: Handle<Font>,
    pub jp: Handle<Font>,
}

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Apparently bevy doesn't support variable fonts yet.
    // Changing the fonts would waste space in git.
    // So the inconsistent font weights will remain.
    commands.insert_resource(Fonts {
        default: asset_server.load("NotoSans-Variable.ttf"),
        jp: asset_server.load("NotoSansJP-Variable.ttf"),
    });
}
