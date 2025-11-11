use bevy::{feathers::theme::ThemeToken, prelude::*};

#[derive(Default, Resource)]
pub struct Fonts(pub std::collections::HashMap<ThemeToken, Handle<Font>>);

impl Fonts {
    pub fn font(&self, token: &ThemeToken) -> Handle<Font> {
        match self.0.get(token) {
            Some(c) => c.clone(),
            None => {
                warn_once!("Font {} not found.", token);
                self.0.get(&super::tokens::FONT_NORMAL).unwrap().clone()
            }
        }
    }
}

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts = Fonts::default();
    fonts.0.insert(
        super::tokens::FONT_NORMAL,
        asset_server.load("NotoSans-Variable.ttf"),
    );
    fonts.0.insert(
        super::tokens::FONT_JAPANESE,
        asset_server.load("NotoSansJP-Variable.ttf"),
    );
    commands.insert_resource(fonts);
}
