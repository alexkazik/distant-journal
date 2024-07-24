use crate::data::encounter_type::EncounterType;
use crate::data::location::Location;
use crate::data::quest::{Quest, QuestState};
use crate::data::vis::Vis;
use crate::game::{LocationId, MsgLanguage, QuestId, QuestLocale, QuestType};
use crate::global::app::MsgApp;
use crate::html::text;
use std::collections::{HashMap, VecDeque};
use std::iter::{Copied, FilterMap, Map};
use std::slice::Iter;
use yew::Html;

pub(crate) struct Data {
    // game data
    pub(crate) quest: HashMap<QuestId, Quest>,
    pub(crate) location: HashMap<LocationId, Location>,
    // global settings
    pub(crate) quest_locale: QuestLocale,
    pub(crate) msg: MsgLanguage,
    // messages
    pub(crate) chain_msg: VecDeque<MsgApp>,
}

type QuestAllIter<'a> =
    Map<Copied<Iter<'a, (QuestId, &'static str)>>, fn((QuestId, &'static str)) -> (QuestId, Html)>;

impl Data {
    pub(crate) fn reset(&mut self) {
        self.quest.clear();
        self.location.clear();
        self.built_in();
    }

    pub(crate) fn cleanup(&mut self) {
        self.quest.retain(|quest_id, quest| {
            if quest_id.typ() == QuestType::Keyword {
                // it's a keyword, it can't be completed
                quest.completed = false;
            } else {
                // it's not a keyword, it can only be gained
                quest
                    .encounter
                    .iter_mut()
                    .for_each(|(_, ql)| ql.retain(|et, _| *et == EncounterType::Gain));
            }
            quest
                .encounter
                .retain(|_, quest_location| !quest_location.is_empty());
            !quest.encounter.is_empty() || !quest.note.is_empty() || quest.completed
        });

        self.location
            .retain(|_, location| !location.note.is_empty() || location.removed);
    }

    fn built_in(&mut self) {
        for quest_id in QuestId::STARTING_QUESTS {
            let quest = self.quest.entry(*quest_id).or_default();
            if quest.state != QuestState::Removed {
                quest.state = QuestState::InGame;
            }
            let loc_pro = quest.encounter.entry(LocationId::prologue()).or_default();
            loc_pro.clear();
            loc_pro.insert(EncounterType::Gain, None, Vis::Visible);
        }
    }

    pub(crate) fn quest_all_iter(&self) -> QuestAllIter<'_> {
        self.quest_locale.all_str().map(|(l, n)| (l, text(n)))
    }

    pub(crate) fn quest_iter<'a>(
        &'a self,
    ) -> FilterMap<
        Copied<Iter<'_, (QuestId, &'static str)>>,
        impl FnMut((QuestId, &'static str)) -> Option<(QuestId, &'a Quest, &'static str)>,
    > {
        self.quest_locale
            .all_str()
            .filter_map(move |(quest_id, quest_name)| {
                self.quest
                    .get(&quest_id)
                    .map(move |quest| (quest_id, quest, quest_name))
            })
    }
}
