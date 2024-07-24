use crate::data::encounter_type::EncounterType;
use crate::data::location::Location;
use crate::data::quest::{Quest, QuestState};
use crate::data::quest_location::QuestLocation;
use crate::data::vis::Vis;
use crate::game::{LocationId, QuestId};
use crate::global::data::Data;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeQuest2<'a>(
    pub(crate) QuestState,
    pub(crate) BTreeMap<usize, SerdeEncounter2>,
    pub(crate) Cow<'a, str>,
    pub(crate) Vis,
    pub(crate) bool,
);

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeLocation2<'a>(pub(crate) Cow<'a, str>, pub(crate) bool);

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeEncounter2(pub(crate) BTreeMap<EncounterType, (Option<usize>, Vis)>);

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SerdeGameData2<'a> {
    pub(crate) version_2: (),
    pub(crate) quests: BTreeMap<usize, SerdeQuest2<'a>>,
    pub(crate) locations: BTreeMap<usize, SerdeLocation2<'a>>,
}

impl Data {
    fn save_encounter(encounters: &QuestLocation) -> SerdeEncounter2 {
        SerdeEncounter2(
            encounters
                .iter()
                .map(|(et, quest_location_encounter)| {
                    (
                        *et,
                        (
                            quest_location_encounter.prerequisite.map(QuestId::raw),
                            quest_location_encounter.vis,
                        ),
                    )
                })
                .collect(),
        )
    }

    fn save_quest(quest: &Quest) -> SerdeQuest2 {
        SerdeQuest2(
            quest.state,
            quest
                .encounter
                .iter()
                .filter(|(location_id, _)| **location_id != LocationId::prologue())
                .map(|(location_id, encounters)| {
                    (location_id.raw(), Self::save_encounter(encounters))
                })
                .collect(),
            Cow::Borrowed(&quest.note),
            quest.vis,
            quest.completed,
        )
    }

    pub(crate) fn save_game_data(&self) -> SerdeGameData2 {
        SerdeGameData2 {
            version_2: (),
            quests: self
                .quest
                .iter()
                .map(|(quest_id, quest)| (quest_id.raw(), Self::save_quest(quest)))
                .collect(),
            locations: self
                .location
                .iter()
                .map(|(location_id, location)| {
                    (
                        location_id.raw(),
                        SerdeLocation2(Cow::Borrowed(&location.note), location.removed),
                    )
                })
                .collect(),
        }
    }

    pub(crate) fn load_game_data_2(&mut self, game_data: SerdeGameData2) {
        self.reset();

        for (raw_location_id, data) in game_data.locations {
            if let Some(location_id) = LocationId::from_raw(raw_location_id) {
                self.location.insert(
                    location_id,
                    Location {
                        note: data.0.to_string().into(),
                        removed: data.1,
                    },
                );
            }
        }

        for (raw_quest_id, data) in game_data.quests {
            if let Some(quest_id) = QuestId::from_raw(raw_quest_id) {
                let quest = self.quest.entry(quest_id).or_default();
                quest.state = data.0;
                for (raw_location_id, e_data) in data.1 {
                    if let Some(location_id) = LocationId::from_raw(raw_location_id) {
                        let encounters = quest.encounter.entry(location_id).or_default();
                        for (encounter_type, (raw_prerequisite, vis)) in e_data.0 {
                            let prerequisite = if encounter_type == EncounterType::Gain {
                                raw_prerequisite.and_then(QuestId::from_raw)
                            } else {
                                None
                            };
                            encounters.insert(encounter_type, prerequisite, vis);
                        }
                    }
                }
                quest.note = data.2.to_string().into();
                quest.vis = data.3;
                quest.completed = data.4;
            }
        }

        self.cleanup();
    }
}
