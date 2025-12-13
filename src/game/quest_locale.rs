use crate::game::data::{NUM_QUESTS, quest_name};
use crate::game::game_language::GameLanguage;
use crate::game::{LocationId, QuestId};
use deunicode::deunicode_with_tofu;
use std::array;
use std::iter::Copied;
use std::slice::Iter;

pub(crate) struct QuestLocale {
    language: GameLanguage,
    translation: [(QuestId, &'static str); NUM_QUESTS],
    urls: [String; NUM_QUESTS],
}

impl QuestLocale {
    #[must_use]
    pub(crate) fn new() -> Self {
        let language = GameLanguage::default();
        #[allow(clippy::cast_possible_truncation)]
        let mut result = Self {
            language,
            translation: array::from_fn(|q| (QuestId(q as u16), "")),
            urls: array::from_fn(|_| String::new()),
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
            *n = quest_name(language, *q);
        }
        self.translation
            .sort_by(|(_, n1), (_, n2)| natord::compare(n1, n2));

        for (q, u) in self.urls.iter_mut().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            Self::mk_url(u, quest_name(self.language, QuestId(q as u16)));
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn get(&self, q: QuestId) -> &'static str {
        quest_name(self.language, q)
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_url(&self, q: QuestId) -> &str {
        &self.urls[q.0 as usize]
    }

    #[must_use]
    pub(crate) fn try_get_url(&self, name: &str) -> Option<QuestId> {
        #[allow(clippy::cast_possible_truncation)]
        self.urls
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

    fn mk_url(dst: &mut String, name: &'static str) {
        dst.clear();
        if name != "*" {
            let z = deunicode_with_tofu(name, "~");
            dst.reserve(z.len());
            let mut last = '\0';
            for mut c in z.chars() {
                if c.is_ascii_alphanumeric() {
                    dst.push(c.to_ascii_lowercase());
                } else if c != '\'' {
                    if !"-_.~".contains(c) {
                        c = '-';
                    }
                    if c != last {
                        dst.push(c);
                    }
                }
                last = c;
            }
        }
    }
}
