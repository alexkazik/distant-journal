use crate::data::encounter_type::EncounterType;
use crate::data::quest::{QuestState, QuestViewMode};
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::game::{QuestId, QuestType};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::html::Modal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use yew::{html, Context, Html};

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
pub(crate) enum MsgTodo {
    SettingsShowKeywords(bool),
    SettingsTyp(Typ),
    ShowNote(QuestId),
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum Typ {
    #[default]
    Active = 0,
    GainWithComplete = 1,
    GainWoComplete = 2,
    Unless = 3,
}

impl From<MsgTodo> for MsgApp {
    #[inline]
    fn from(msg: MsgTodo) -> Self {
        MsgApp::MsgTodo(msg)
    }
}

pub(crate) struct PaneTodo {
    show_keywords: bool,
    typ: Typ,
    modal: Modal,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PaneTodoSer {
    show_keywords: bool,
    typ: Typ,
}

impl SubComponent for PaneTodo {
    type Message = MsgTodo;
    type Ser = PaneTodoSer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {
            show_keywords: true,
            typ: Typ::Active,
            modal: Modal::default(),
        }
    }

    fn reset_to_new(&mut self) {
        self.show_keywords = true;
        self.typ = Typ::Active;
    }

    fn update(
        &mut self,
        data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {
            MsgTodo::SettingsShowKeywords(show_keywords) => {
                self.show_keywords = show_keywords;
                if !self.show_keywords && self.typ == Typ::Unless {
                    self.typ = Typ::Active;
                }
                UpdateResult::SaveSettings | UpdateResult::Render
            }
            MsgTodo::SettingsTyp(typ) => {
                if self.show_keywords || self.typ != Typ::Unless {
                    self.typ = typ;
                }
                UpdateResult::SaveSettings | UpdateResult::Render
            }
            MsgTodo::ShowNote(quest_id) => {
                if let Some(quest) = data.quest.get(&quest_id) {
                    self.modal.open(
                        &data
                            .msg
                            .str_note_modal_head(data.quest_locale.get(quest_id)),
                        &quest.note,
                    );
                }
                UpdateResult::empty()
            }
        }
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        let select_g_w_c = data.quest.iter().any(|(qid, q)| {
            self.show_keywords == (qid.typ() == QuestType::Keyword)
                && q.state == QuestState::NotFound
                && q.encounter
                    .iter()
                    .any(|(_, ql)| ql.contains_key(EncounterType::Gain))
                && (q.completed
                    || q.encounter
                        .iter()
                        .any(|(_, ql)| ql.contains_key(EncounterType::Complete)))
        });
        let select_g_wo_c = data.quest.iter().any(|(qid, q)| {
            self.show_keywords == (qid.typ() == QuestType::Keyword)
                && q.state == QuestState::NotFound
                && q.encounter
                    .iter()
                    .any(|(_, ql)| ql.contains_key(EncounterType::Gain))
                && !(q.completed
                    || q.encounter
                        .iter()
                        .any(|(_, ql)| ql.contains_key(EncounterType::Complete)))
        });

        let select_u = data.quest.iter().any(|(qid, q)| {
            self.show_keywords == (qid.typ() == QuestType::Keyword)
                && q.state == QuestState::NotFound
                && q.encounter
                    .iter()
                    .any(|(_, ql)| ql.contains_key(EncounterType::Unless))
        });
        let typ = match self.typ {
            Typ::GainWithComplete if select_g_w_c => Typ::GainWithComplete,
            Typ::GainWoComplete if select_g_wo_c => Typ::GainWoComplete,
            Typ::Unless if select_u => Typ::Unless,
            _ => Typ::Active,
        };
        let list = data.quest_iter().filter_map(|(qid, q, qname)| {
            if self.show_keywords != (qid.typ() == QuestType::Keyword) {
                return None;
            }
            if q.state == QuestState::Removed {
                // nothing to do
                return None;
            }
            if !match typ {
                Typ::Active => q.state == QuestState::InGame,
                Typ::GainWithComplete => {
                    if qid.typ() == QuestType::Keyword {
                        q.state == QuestState::NotFound
                            && q.contains_visible_encounter_type(EncounterType::Gain)
                            && q.contains_visible_encounter_type(EncounterType::Complete)
                    } else {
                        q.state == QuestState::NotFound && q.completed
                    }
                }
                Typ::GainWoComplete => {
                    if qid.typ() == QuestType::Keyword {
                        q.state == QuestState::NotFound
                            && q.contains_visible_encounter_type(EncounterType::Gain)
                            && !q.contains_visible_encounter_type(EncounterType::Complete)
                    } else {
                        q.state == QuestState::NotFound && !q.completed
                    }
                }
                Typ::Unless => {
                    q.state == QuestState::NotFound
                        && q.contains_visible_encounter_type(EncounterType::Unless)
                }
            } {
                return None;
            }
            q.view(
                ctx,
                qid,
                qname,
                data,
                QuestViewMode::Todo,
                MsgTodo::ShowNote,
            )
        });
        html! {
            <>
            <div>
                <div class="btn-group" role="group">
                    <input
                        type="radio"
                        class="btn-check"
                        name="questTypeX"
                        id="questTypeX0"
                        autocomplete="off"
                        checked={self.show_keywords}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsShowKeywords(true))}
                    />
                    <label class="btn btn-outline-primary" for="questTypeX0">{QuestType::Keyword.icon()}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="questTypeX"
                        id="questTypeX1"
                        autocomplete="off"
                        checked={!self.show_keywords}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsShowKeywords(false))}
                    />
                    <label class="btn btn-outline-primary" for="questTypeX1">{QuestType::Totem.icon()}{QuestType::Location.icon()}{QuestType::Other.icon()}{QuestType::TreasureMap.icon()}</label>
                </div>
                {" "}
                <div class="btn-group" role="group">
                    <input
                        type="radio"
                        class="btn-check"
                        name="typX"
                        id="typX0"
                        autocomplete="off"
                        checked={typ == Typ::Active}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsTyp(Typ::Active))}
                    />
                    <label class="btn btn-outline-primary" for="typX0">{data.msg.todo_typ_in_game()}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="typX"
                        id="typX1"
                        autocomplete="off"
                        checked={typ == Typ::GainWithComplete}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsTyp(Typ::GainWithComplete))}
                        disabled={!select_g_w_c}
                    />
                    <label class="btn btn-outline-primary" for="typX1">{data.msg.todo_typ_gain_with_complete(EncounterType::Gain.icon_active(), EncounterType::Complete.icon_active())}</label>

                    <input
                        type="radio"
                        class="btn-check"
                        name="typX"
                        id="typX2"
                        autocomplete="off"
                        checked={typ == Typ::GainWoComplete}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsTyp(Typ::GainWoComplete))}
                        disabled={!select_g_wo_c}
                    />
                    <label class="btn btn-outline-primary" for="typX2">{data.msg.todo_typ_gain_wo_complete(EncounterType::Gain.icon_active(), EncounterType::Complete.icon_active())}</label>
                    <input
                        type="radio"
                        class="btn-check"
                        name="typX"
                        id="typX3"
                        autocomplete="off"
                        checked={typ == Typ::Unless}
                        onchange={ctx.link().callback(|_|MsgTodo::SettingsTyp(Typ::Unless))}
                        disabled={!select_u}
                    />
                    <label class="btn btn-outline-primary" for="typX3">{EncounterType::Unless.icon_active()}</label>
                </div>
            </div>
            <table class="table table-hover align-middle mt-4">
              <thead>
                <tr>
                  <th>{data.msg.todo_header_quest()}</th>
                  <th>{data.msg.todo_header_hints()}</th>
                </tr>
              </thead>
              <tbody>
                {for list}
              </tbody>
            </table>

            {self.modal.html(data)}
            </>
        }
    }

    fn save(&self) -> Self::Ser {
        PaneTodoSer {
            show_keywords: self.show_keywords,
            typ: self.typ,
        }
    }

    fn load(&mut self, stored: Self::Ser) {
        self.show_keywords = stored.show_keywords;
        self.typ = stored.typ;
        if !self.show_keywords && self.typ == Typ::Unless {
            self.typ = Typ::Active;
        }
    }
}
