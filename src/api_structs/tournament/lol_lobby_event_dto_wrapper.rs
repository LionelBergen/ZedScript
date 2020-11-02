use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LobbyEvent {
    #[serde(rename = "summonerId")]
    pub summoner_id: Option<String>,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "timestamp")]
    pub time_stamp: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LobbyEventDtoWrapper {
    #[serde(rename = "eventList")]
    pub events: Vec<LobbyEvent>
}
