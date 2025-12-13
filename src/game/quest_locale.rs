use crate::game::game_language::GameLanguage;
use crate::game::generated::{QUEST_URLS, QUESTS, QUESTS_COLUMNS};
use crate::game::{LocationId, QuestId};
use std::array;
use std::iter::Copied;
use std::slice::Iter;

pub(crate) struct QuestLocale {
    language: GameLanguage,
    translation: [(QuestId, &'static str); QUESTS_COLUMNS],
}

impl QuestLocale {
    #[must_use]
    pub(crate) fn new() -> Self {
        let language = GameLanguage::default();
        #[allow(clippy::cast_possible_truncation)]
        let mut result = Self {
            language,
            translation: array::from_fn(|q| (QuestId(q as u16), "")),
        };
        result.set_language(language);
        result
    }

    #[inline]
    pub(crate) fn all_str(&self) -> Copied<Iter<'_, (QuestId, &'static str)>> {
        self.translation.iter().copied()
    }

    #[inline]
    #[must_use]
    pub(crate) fn language(&self) -> GameLanguage {
        self.language
    }

    pub(crate) fn set_language(&mut self, language: GameLanguage) {
        self.language = language;
        for (q, n) in &mut self.translation {
            *n = QUESTS[language as usize][q.0 as usize];
        }
        self.translation
            .sort_by(|(_, n1), (_, n2)| natord::compare(n1, n2));
    }

    #[inline]
    #[must_use]
    pub(crate) fn get(&self, q: QuestId) -> &'static str {
        QUESTS[self.language as usize][q.0 as usize]
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_url(&self, q: QuestId) -> &'static str {
        QUEST_URLS[self.language as usize][q.0 as usize]
    }

    #[must_use]
    pub(crate) fn try_get_url(&self, name: &str) -> Option<QuestId> {
        #[allow(clippy::cast_possible_truncation)]
        QUEST_URLS[self.language as usize]
            .iter()
            .enumerate()
            .find(|(_, n)| **n == name)
            .map(|(q, _)| QuestId(q as u16))
    }

    #[inline]
    #[must_use]
    pub(crate) fn location(&self, l: LocationId) -> &'static str {
        l.name(self.language)
    }
}
