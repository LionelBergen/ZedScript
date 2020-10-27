use crate::api_structs::lol_api_key::LolApiKey;

pub struct LeagueUrl {}

impl LeagueUrl {
    // CHAMPION-MASTERY-V4
    const CHAMPION_MASTERY_BY_SUMMONER_ID: &'static str = "https://%region%.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-summoner/%summonerid%?api_key=%apikey%";

    pub fn get_champion_mastery_url(lol_api_key: &LolApiKey, summoner_id: &str) -> String {
        return Self::get_url_from_api_key_with_summoner_id(Self::CHAMPION_MASTERY_BY_SUMMONER_ID, lol_api_key, summoner_id);
    }

    fn get_url_from_api_key_with_summoner_id(original_url: &str, lol_api_key: &LolApiKey, summonerId: &str) -> String {
        original_url
            .replace("%region%", lol_api_key.region.get_code())
            .replace("%apikey%", &*lol_api_key.api_key)
            .replace("%summonerid%", &summonerId)
    }

    fn get_url_from_api_key(original_url: &str, lol_api_key: &LolApiKey) -> String {
        Self::get_url_from_api_key_with_summoner_id(original_url, lol_api_key, "")
    }
}