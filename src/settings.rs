#[derive(bevy::prelude::Resource, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub recent_games: Vec<std::path::PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            recent_games: Vec::new(),
        }
    }
}
