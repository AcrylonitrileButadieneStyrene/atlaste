use bevy::feathers::theme::ThemeToken;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, strum::VariantArray, strum::EnumProperty)]
pub enum CodePage {
    #[strum(props(Name = "West European"))]
    Ascii,
    #[strum(props(Name = "East European"))]
    Eastern,
    #[strum(props(Name = "Cyrillic"))]
    Cyrillic,
    #[default]
    #[strum(props(Name = "Japanese"))]
    ShiftJIS,
    #[strum(props(Name = "Chinese"))]
    Big5,
}

impl CodePage {
    pub const fn to_encoding(self) -> &'static encoding_rs::Encoding {
        match self {
            Self::Ascii => encoding_rs::WINDOWS_1252,
            Self::Eastern => encoding_rs::WINDOWS_1250,
            Self::Cyrillic => encoding_rs::WINDOWS_1251,
            Self::ShiftJIS => encoding_rs::SHIFT_JIS,
            Self::Big5 => encoding_rs::BIG5,
        }
    }

    pub const fn to_theme_token(self) -> ThemeToken {
        match self {
            CodePage::Ascii | CodePage::Eastern | CodePage::Cyrillic | CodePage::Big5 => {
                crate::theme::tokens::FONT_NORMAL
            }
            CodePage::ShiftJIS => crate::theme::tokens::FONT_JAPANESE,
        }
    }
}
