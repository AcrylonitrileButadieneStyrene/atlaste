#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, strum::VariantArray, strum::EnumProperty)]
pub enum CodePage {
    #[default]
    #[strum(props(Name = "West European"))]
    Ascii,
    #[strum(props(Name = "East European"))]
    Eastern,
    #[strum(props(Name = "Cyrillic"))]
    Cyrillic,
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
}
