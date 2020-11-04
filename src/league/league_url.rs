use crate::api_structs::lol_api_key::LolApiKey;

pub struct LeagueUrl {}

impl LeagueUrl {
    // CHAMPION-MASTERY-V4
    const CHAMPION_MASTERY_BY_SUMMONER_ID: &'static str = "https://%region%.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-summoner/%summonerid%?api_key=%apikey%";
    const CHAMPION_MASTERY_BY_CHAMPION_ID: &'static str = "https://%region%.api.riotgames.com/lol/champion-mastery/v4/champion-masteries/by-summoner/%summonerid%/by-champion/%championid%?api_key=%apikey%";
    const CHAMPION_MASTERY_SCORE: &'static str = "https://%region%.api.riotgames.com/lol/champion-mastery/v4/scores/by-summoner/%summonerid%?api_key=%apikey%";

    // CHAMPION-V3
    const CHAMPION_ROTATION: &'static str = "https://%region%.api.riotgames.com/lol/platform/v3/champion-rotations?api_key=%apikey%";

    // CLASH-V1
    const GET_CLASH_PLAYERS_BY_SUMMONER_ID: &'static str = "https://%region%.api.riotgames.com/lol/clash/v1/players/by-summoner/%summonerid%?api_key=%apikey%";
    const GET_CLASH_TEAM: &'static str = "https://%region%.api.riotgames.com/lol/clash/v1/teams/%teamid%?api_key=%apikey%";
    const GET_CLASH_TOURNAMENTS: &'static str = "https://%region%.api.riotgames.com/lol/clash/v1/tournaments?api_key=%apikey%";
    const GET_CLASH_TOURNAMENT_BY_TEAM_ID: &'static str = "https://%region%.api.riotgames.com/lol/clash/v1/tournaments/by-team/%teamid%?api_key=%apikey%";
    const GET_CLASH_TOURNAMENT: &'static str = "https://%region%.api.riotgames.com/lol/clash/v1/tournaments/%tournamentid%?api_key=%apikey%";

    // TOURNAMENT-STUB-V4
    const CREATE_TOURNAMENT_CODE_MOCK: &'static str = "https://americas.api.riotgames.com/lol/tournament-stub/v4/codes?api_key=%apikey%";
    const TOURNAMENT_EVENTS_MOCK: &'static str = "https://americas.api.riotgames.com/lol/tournament-stub/v4/lobby-events/by-code/%tournamentcode%?api_key=%apikey%";
    const CREATE_PROVIDER_MOCK: &'static str = "https://americas.api.riotgames.com/lol/tournament-stub/v4/providers?api_key=%apikey%";
    const CREATE_TOURNAMENT_MOCK: &'static str = "https://americas.api.riotgames.com/lol/tournament-stub/v4/tournaments?api_key=%apikey%";

    pub fn get_champion_mastery(lol_api_key: &LolApiKey, summoner_id: &str) -> String {
        return Self::get_url_from_api_key_with_summoner_id(Self::CHAMPION_MASTERY_BY_SUMMONER_ID, lol_api_key, summoner_id);
    }

    pub fn get_champion_mastery_url_with_champion(lol_api_key: &LolApiKey, summoner_id: &str, champion_id: &str) -> String {
        return Self::get_url_from_api_key_with_summoner_champion_id(Self::CHAMPION_MASTERY_BY_CHAMPION_ID, lol_api_key, summoner_id, champion_id);
    }

    pub fn get_champion_mastery_score(lol_api_key: &LolApiKey, summoner_id: &str) -> String {
        return Self::get_url_from_api_key_with_summoner_id(Self::CHAMPION_MASTERY_SCORE, lol_api_key, summoner_id);
    }

    pub fn get_champion_rotation(lol_api_key: &LolApiKey) -> String {
        return Self::get_url_from_api_key(Self::CHAMPION_ROTATION, lol_api_key);
    }

    pub fn create_provider_mock(lol_api_key: &LolApiKey) -> String {
        return Self::get_url_from_api_key(Self::CREATE_PROVIDER_MOCK, lol_api_key);
    }

    pub fn get_clash_players(lol_api_key: &LolApiKey, summoner_id: &str) -> String {
        return Self::get_url_from_api_key_with_summoner_id(Self::GET_CLASH_PLAYERS_BY_SUMMONER_ID, lol_api_key, summoner_id);
    }

    pub fn get_clash_team(lol_api_key: &LolApiKey, team_id: &str) -> String {
        return Self::get_url_from_api_key(Self::GET_CLASH_TEAM, lol_api_key).replace("%teamid%", team_id);
    }

    pub fn get_clash_tournament_by_team_id(lol_api_key: &LolApiKey, team_id: &str) -> String {
        return Self::get_url_from_api_key(Self::GET_CLASH_TOURNAMENT_BY_TEAM_ID, lol_api_key).replace("%teamid%", team_id);
    }

    pub fn get_clash_tournaments(lol_api_key: &LolApiKey) -> String {
        return Self::get_url_from_api_key(Self::GET_CLASH_TOURNAMENTS, lol_api_key);
    }

    pub fn get_clash_tournament(lol_api_key: &LolApiKey, tournament_id: &str) -> String {
        return Self::get_url_from_api_key(Self::GET_CLASH_TOURNAMENT, lol_api_key).replace("%tournamentid%", tournament_id);
    }

    pub fn create_tournament_mock(lol_api_key: &LolApiKey) -> String {
        return Self::get_url_from_api_key(Self::CREATE_TOURNAMENT_MOCK, lol_api_key);
    }

    pub fn get_tournament_lobby_events_mock(lol_api_key: &LolApiKey, tournament_code: &str) -> String {
        return Self::get_url_from_api_key(Self::TOURNAMENT_EVENTS_MOCK, lol_api_key).replace("%tournamentcode%", tournament_code);
    }

    pub fn create_tournament_code_mock(lol_api_key: &LolApiKey, tournament_code: &str) -> String {
        return Self::get_url_from_api_key(Self::CREATE_TOURNAMENT_CODE_MOCK, lol_api_key);
    }

    fn get_url_from_api_key_with_summoner_champion_id(original_url: &str, lol_api_key: &LolApiKey, summoner_id: &str, champion_id: &str) -> String {
        original_url
            .replace("%region%", lol_api_key.region.get_code())
            .replace("%apikey%", &*lol_api_key.api_key)
            .replace("%summonerid%", &summoner_id)
            .replace("%championid%", &champion_id)
    }

    fn get_url_from_api_key_with_summoner_id(original_url: &str, lol_api_key: &LolApiKey, summoner_id: &str) -> String {
        Self::get_url_from_api_key_with_summoner_champion_id(original_url, lol_api_key, summoner_id, "")
    }

    fn get_url_from_api_key(original_url: &str, lol_api_key: &LolApiKey) -> String {
        Self::get_url_from_api_key_with_summoner_id(original_url, lol_api_key, "")
    }
}