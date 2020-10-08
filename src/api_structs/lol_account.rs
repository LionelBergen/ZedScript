use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueAccount {
    pub id: String,
    #[serde(rename="accountId")]
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    #[serde(rename="profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename="revisionDate")]
    pub revision_date: i64,
    #[serde(rename="summonerLevel")]
    pub summoner_level: i32
}