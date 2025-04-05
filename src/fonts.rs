use bevy::prelude::*;

#[derive(Resource)]
pub struct Fonts {
    pub default: Handle<Font>,
    pub jp: Handle<Font>,
}

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Fonts {
        default: asset_server.load("NotoSans-Variable.ttf"),
        jp: asset_server.load("NotoSansJP-Variable.ttf"),
    });
}
