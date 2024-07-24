use crate::data::encounter_type::EncounterType;
use crate::data::location::Location;
use crate::data::quest::QuestState;
use crate::data::vis::Vis;
use crate::game::{LocationId, QuestId};
use crate::global::data::Data;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeQuest1<'a>(
    pub(crate) QuestState,
    pub(crate) BTreeMap<usize, SerdeEncounter1>,
    pub(crate) Cow<'a, str>,
    pub(crate) Vis,
    pub(crate) bool,
);

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeLocation1<'a>(pub(crate) Cow<'a, str>, pub(crate) bool);

#[derive(Serialize, Deserialize)]
pub(crate) struct SerdeEncounter1(pub(crate) BTreeMap<EncounterType, (Option<usize>, Vis)>);

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SerdeGameData1<'a> {
    pub(crate) version_1: (),
    pub(crate) quests: BTreeMap<usize, SerdeQuest1<'a>>,
    pub(crate) locations: BTreeMap<usize, SerdeLocation1<'a>>,
}

impl Data {
    pub(crate) fn load_game_data_1(&mut self, game_data: SerdeGameData1) {
        self.reset();

        for (raw_location_id, data) in game_data.locations {
            if let Some(location_id) = LocationId::from_raw(Self::fix_location_id(raw_location_id))
            {
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
                    if let Some(location_id) =
                        LocationId::from_raw(Self::fix_location_id(raw_location_id))
                    {
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

    fn fix_location_id(old: usize) -> usize {
        if old == 21 {
            32
        } else if (22..=32).contains(&old) {
            old - 1
        } else {
            old
        }
    }
}
