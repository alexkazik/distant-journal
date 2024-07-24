use crate::data::encounter_type::EncounterType;
use crate::data::note::Note;
use crate::game::{QuestId, QuestType};
use crate::global::data::Data;
use yew::Html;

pub(crate) fn info_view(data: &Data) -> Html {
    yew::html! {
        <ul class="list-group">
            <li class="list-group-item">
                <h2 class="h4 mb-0">{data.msg.info_data_head()}</h2>
            </li>
            <li class="list-group-item">
                {data.msg.info_data_body()}
            </li>
            <li class="list-group-item">
                <h2 class="h4 mb-0">{data.msg.info_icon_head()}</h2>
            </li>
            <li class="list-group-item">
                {data.msg.info_icon_keyword(QuestType::Keyword.icon())}<br/>
                {data.msg.info_icon_totem(QuestType::Totem.icon())}<br/>
                {data.msg.info_icon_treasure_map(QuestType::TreasureMap.icon())}<br/>
                {data.msg.info_icon_distant_location(QuestType::Location.icon())}<br/>
                {data.msg.info_icon_other(QuestType::Other.icon())}<br/>
                {data.msg.info_icon_completed_quest(QuestId::icon_completed())}<br/>
                {data.msg.info_icon_gain(EncounterType::Gain.icon_active())}<br/>
                {data.msg.info_icon_when(EncounterType::When.icon_active())}<br/>
                {data.msg.info_icon_complete(EncounterType::Complete.icon_active())}<br/>
                {data.msg.info_icon_lose(EncounterType::Lose.icon_active())}<br/>
                {data.msg.info_icon_unless(EncounterType::Unless.icon_active())}<br/>
                {data.msg.info_icon_note(Note::icon())}<br/>
            </li>
            <li class="list-group-item">
                <h2 class="h4 mb-0">{data.msg.info_link_head()}</h2>
            </li>
            <li class="list-group-item">
                <a href="https://distant-skies.rulepop.com/" target="_blank">{data.msg.info_link_rules()}</a>
            </li>
        </ul>
    }
}
