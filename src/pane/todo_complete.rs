use crate::data::quest::QuestState;
use crate::data::sub_component::{SubComponent, UpdateResult, UpdateResults};
use crate::game::{QuestId, QuestType};
use crate::global::app::{App, MsgApp};
use crate::global::data::Data;
use crate::ser::settings::EmptySer;
use web_sys::HtmlInputElement;
use yew::{html, Context, Html, NodeRef};
use yew_bootstrap::component::Button;
use yew_bootstrap::util::Color;

#[derive(Clone)]
pub(crate) enum MsgTodoComplete {
    Save,
}

impl From<MsgTodoComplete> for MsgApp {
    #[inline]
    fn from(msg: MsgTodoComplete) -> Self {
        MsgApp::MsgTodoComplete(msg)
    }
}

pub(crate) struct PaneTodoComplete {
    quest_id: QuestId,
    note_input: NodeRef,
}

impl SubComponent for PaneTodoComplete {
    type Message = MsgTodoComplete;
    type Ser = EmptySer;

    fn create(_ctx: &Context<App>) -> Self {
        Self {
            quest_id: QuestId::STARTING_QUESTS[0], // anything will do
            note_input: NodeRef::default(),
        }
    }

    fn update(
        &mut self,
        data: &mut Data,
        _ctx: &Context<App>,
        msg: Self::Message,
    ) -> UpdateResults {
        match msg {
            MsgTodoComplete::Save => {
                #[cfg(feature = "debug")]
                web_sys::console::log_1(&"save".into());
                if let Some(quest) = data.quest.get_mut(&self.quest_id) {
                    quest.completed = true;
                    quest.state = QuestState::Removed;
                    quest.note = self
                        .note_input
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value()
                        .into();
                    #[cfg(feature = "debug")]
                    web_sys::console::log_1(&format!("note {}", quest.note).into());
                }
                data.chain_msg.push_back(MsgApp::Back);
                UpdateResult::SaveGameData.into()
            }
        }
    }

    fn view(&self, data: &Data, ctx: &Context<App>) -> Html {
        let quest_note = data
            .quest
            .get(&self.quest_id)
            .map_or("", |quest| &quest.note)
            .to_string();

        html! {
            <ul class="list-group">
                <li class="list-group-item">
                    <Button style={Color::Primary} onclick={ctx.link().callback(|_|MsgApp::Back)} children={data.msg.back()} />
                </li>
                <li class="list-group-item">
                    <h3>{data.msg.to_co_header()}</h3>
                </li>
                <li class="list-group-item">
                    <h3>{data.msg.quest_header(data.quest_locale.get(self.quest_id), self.quest_id.icon())}</h3>
                </li>
                <li class="list-group-item">
                    {data.msg.note_quest()}<br/>
                    <div class="form-floating">
                      <textarea class="form-control" placeholder={data.msg.str_note_placeholder()} id="floatingTextarea" value={quest_note} style="height: 150px" ref={&self.note_input} />
                      <label for="floatingTextarea">{data.msg.note()}</label>
                    </div>
                </li>
                <li class="list-group-item">
                    <Button style={Color::Success} onclick={ctx.link().callback(|_|MsgTodoComplete::Save)} children={data.msg.to_co_save()} />
                </li>
            </ul>
        }
    }

    fn save(&self) -> Self::Ser {
        EmptySer {}
    }

    fn load(&mut self, _stored: Self::Ser) {}
}

impl PaneTodoComplete {
    pub(crate) fn go(&mut self, data: &Data, quest_id: QuestId) -> bool {
        if quest_id.typ() == QuestType::Keyword {
            // keywords can't be completed
            false
        } else if let Some(quest) = data.quest.get(&quest_id) {
            self.quest_id = quest_id;
            // only able to complete when the quest was gained before
            quest.state == QuestState::InGame
        } else {
            // quest is not known
            false
        }
    }
}
