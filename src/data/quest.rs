use crate::data::encounter_type::EncounterType;
use crate::data::note::Note;
use crate::data::quest_location::QuestLocation;
use crate::data::vis::Vis;
use crate::game::{LocationId, QuestId, QuestType};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::pane::map::note_head;
use crate::route::Route;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::BTreeMap;
use yew::virtual_dom::VList;
use yew::{html, Context, Html};
use yew_bootstrap::component::{Button, ButtonSize};
use yew_bootstrap::util::Color;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default, Clone)]
pub(crate) struct Quest {
    pub(crate) state: QuestState,
    pub(crate) encounter: BTreeMap<LocationId, QuestLocation>,
    pub(crate) vis: Vis,
    pub(crate) note: Note,
    pub(crate) completed: bool,
}

impl Quest {
    #[must_use]
    pub(crate) fn view<F, M>(
        &self,
        ctx: &Context<App>,
        quest_id: QuestId,
        name: &str,
        data: &Data,
        view_mode: QuestViewMode,
        view_note: F,
    ) -> Option<Html>
    where
        F: Fn(QuestId) -> M + 'static,
        M: Into<MsgApp>,
    {
        let is_map = matches!(view_mode, QuestViewMode::Location(_));
        if !is_map && self.vis != Vis::Visible {
            // should be hidden
            return None;
        }
        let mut es: Vec<_> = self
            .encounter
            .iter()
            .filter(view_mode.filter())
            .filter_map(|(location_id, quest_location)| {
                let location_id = *location_id;
                let active = quest_location.get_active(self, data, is_map);
                if active.is_empty() {
                    return None;
                }

                let outline = !active.iter().any(|(_, v)| *v);
                let vis = quest_location.iter().map(|(_, qle)| qle.vis).min().unwrap_or(Vis::Visible);
                let pq = quest_location.iter().filter(|(et, _)| active.get(et) != Some(&true)).find_map(|(_, qle)| qle.prerequisite).map(|pq| data.msg.quest_button_for_prerequisite(data.quest_locale.get(pq)));
                let e = active.into_iter().map(|(i, a)| i.icon(a));
                let p = location_id.page().map(|p| html! { <small> {data.msg.page_short_braced(p)} </small> });
                let cb = move |_| if is_map {
                    MsgApp::Go(Route::MapAction(location_id, quest_id))
                } else {
                    MsgApp::Go(Route::TodoAction(quest_id, location_id))
                };
                Some(html! {
                    <>
                        <Button style={vis.to_style_primary()} outline={outline} size={ButtonSize::Small} onclick={ctx.link().callback(cb)} disabled={location_id == LocationId::prologue()}>
                            {data.msg.quest_button(data.quest_locale.location(location_id),  p.unwrap_or_default(),  VList::with_children(e.map(Into::into).collect(), None), pq.unwrap_or_default())}
                        </Button>
                        {" "}
                    </>
                })
            })
            .collect();

        if !is_map && quest_id.typ() != QuestType::Keyword && self.state == QuestState::InGame {
            es.push(html! {
                    <>
                        <Button style={Color::Primary} outline={false} size={ButtonSize::Small} onclick={ctx.link().callback(move |_|MsgApp::Go(Route::TodoComplete(quest_id)))} >
                            {data.msg.todo_complete_quest()}
                        </Button>
                        {" "}
                    </>
                });
        }

        if es.is_empty() && is_map {
            return None;
        }

        let completed = if !is_map && self.completed {
            QuestId::icon_completed().into()
        } else {
            Html::default()
        };

        Some(html! {
            <tr>
              <td>
                {data.msg.quest_header_with_state( name, completed, quest_id.icon(), self.state.text(data))}
              </td>
              <td>
                if es.is_empty() {
                    {data.msg.todo_no_hints_yet()}
                } else {
                    {for es}
                }
                if let Some(quest) = data.quest.get(&quest_id) {
                    if !quest.note.is_empty() {
                        {" "}
                        <Button style={Color::Info} size={ButtonSize::Small} onclick={ctx.link().callback(move |_|view_note(quest_id))}>
                            {data.msg.note_with_icon( Note::icon(),  &note_head(&quest.note))}
                        </Button>
                    }
                }
              </td>
            </tr>
        })
    }

    pub(crate) fn contains_visible_encounter_type(&self, encounter_type: EncounterType) -> bool {
        self.encounter.iter().any(|(_, ql)| {
            ql.iter()
                .any(|(et, qle)| *et == encounter_type && qle.vis == Vis::Visible)
        })
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, Default, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum QuestState {
    #[default]
    NotFound = 0,
    InGame = 1,
    Removed = 2,
}

impl QuestState {
    pub(crate) fn text(self, data: &Data) -> &'static str {
        match self {
            QuestState::NotFound => data.msg.str_state_not_found(),
            QuestState::InGame => data.msg.str_state_in_game(),
            QuestState::Removed => data.msg.str_state_removed(),
        }
    }
    pub(crate) fn to_csv(self) -> &'static str {
        match self {
            QuestState::NotFound => "",
            QuestState::InGame => "in-game",
            QuestState::Removed => "removed",
        }
    }
    pub(crate) fn try_from_csv(input: &str) -> Option<Self> {
        match input {
            "" => Some(QuestState::NotFound),
            "in-game" => Some(QuestState::InGame),
            "removed" => Some(QuestState::Removed),
            _ => None,
        }
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub(crate) enum QuestViewMode {
    Todo,
    Location(LocationId),
}

impl QuestViewMode {
    pub(crate) fn filter(&self) -> impl Fn(&(&LocationId, &QuestLocation)) -> bool + '_ {
        move |(location_id, _)| match self {
            QuestViewMode::Todo => true,
            QuestViewMode::Location(ref_location_id) => *location_id == ref_location_id,
        }
    }
}
