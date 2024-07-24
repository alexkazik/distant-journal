use crate::game::generated::{QUESTS_CARDS, QUESTS_COLUMNS, QUEST_TYPES};
use crate::game::quest_type::QuestType;
use yew_bootstrap::icons::BI;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub(crate) struct QuestId(pub(super) u16);

impl QuestId {
    pub(crate) const STARTING_QUESTS: &'static [QuestId] = &[
        QuestId(14 - 1),
        QuestId(15 - 1),
        QuestId(16 - 1),
        QuestId(17 - 1),
        QuestId(128 - 1),
    ];

    #[inline]
    #[must_use]
    pub(crate) fn raw(self) -> usize {
        self.0 as usize
    }

    #[inline]
    #[must_use]
    pub(crate) fn from_raw(raw: usize) -> Option<Self> {
        if raw < QUESTS_COLUMNS {
            #[allow(clippy::cast_possible_truncation)]
            Some(Self(raw as u16))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn typ(self) -> QuestType {
        QUEST_TYPES[self.0 as usize]
    }

    #[inline]
    #[must_use]
    pub(crate) fn icon(self) -> BI {
        self.typ().icon()
    }

    #[inline]
    #[must_use]
    pub(crate) fn icon_completed() -> BI {
        BI::CHECK
    }

    pub(crate) fn cards() -> impl Iterator<Item = (QuestId, &'static str)> {
        #[allow(clippy::cast_possible_truncation)]
        QUESTS_CARDS
            .iter()
            .enumerate()
            .map(|(i, c)| (QuestId(i as u16), *c))
    }

    #[inline]
    #[must_use]
    pub(crate) fn replacement(self) -> Option<QuestId> {
        match self.0 {
            // all numbers are zero offset; 116, 27, 50, 214
            115 => Some(QuestId(26)), // BELL
            49 => Some(QuestId(213)), // LOANED
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn quest_lost() -> QuestId {
        // all numbers are zero offset; 163
        QuestId(162)
    }

    #[inline]
    #[must_use]
    pub(crate) fn quest_confused() -> QuestId {
        // all numbers are zero offset; 217
        QuestId(216)
    }
}
