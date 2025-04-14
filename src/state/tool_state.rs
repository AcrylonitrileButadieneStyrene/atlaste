use bevy::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States, strum::EnumIter)]
pub enum ToolState {
    #[default]
    None,
    Move,
}

impl ToolState {
    // TODO: replace with icons and use this as a hover tooltip
    pub const fn to_name(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Move => "Move",
        }
    }
}
