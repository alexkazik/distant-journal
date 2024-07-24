#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(
    Clone,
    Copy,
    Default,
    Eq,
    PartialEq,
    enum_tools::EnumTools,
    serde::Deserialize,
    serde::Serialize,
    typed_i18n::TypedI18N,
)]
#[repr(usize)]
#[enum_tools(as_str, iter, names)]
#[typed_i18n(filename = "game.lrc")]
#[typed_i18n(builder = "mixed_str")]
pub(crate) enum GameLanguage {
    #[serde(rename = "en")]
    #[enum_tools(rename = "English")]
    #[default]
    En,
}