use serde::{Deserialize, Serialize};
use yew_bootstrap::icons::BI;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub(crate) enum QuestType {
    #[default]
    Keyword,
    Totem,
    TreasureMap,
    Location,
    Other,
    Unavailable,
}

impl QuestType {
    #[must_use]
    pub(crate) fn icon(self) -> BI {
        match self {
            QuestType::Keyword => BI::KEY,
            QuestType::Totem => BI::FILE,
            QuestType::TreasureMap => BI::MAP,
            QuestType::Location => BI::GEO_ALT,
            QuestType::Other => BI::QUESTION_CIRCLE,
            QuestType::Unavailable => BI::ASTERISK,
        }
    }
}
