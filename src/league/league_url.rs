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

    // LEAGUE-EXP-V4
    const GET_LEAGUE_ENTRIES: &'static str = "https://%region%.api.riotgames.com/lol/league-exp/v4/entries/%queue%/%tier%/%division%?api_key=%apikey%";

    // LEAGUE-V4
    const GET_CHALLENGER_LEAGUE: &'static str = "https://%region%.api.riotgames.com/lol/league/v4/challengerleagues/by-queue/%queue%?api_key=%apikey%";
    const GET_LEAGUE_ENTRIES_BY_SUMMONER: &'static str = "https://%region%.api.riotgames.com/lol/league/v4/entries/by-summoner/%summonerid%?api_key=%apikey%";
    const GET_ALL_LEAGUE_ENTRIES: &'static str = "https://%region%.api.riotgames.com/lol/league/v4/entries/%queue%/%tier%/%division%?api_key=%apikey%";
    const GET_GRANDMASTER_LEAGUE: &'static str = "https://%region%.api.riotgames.com/lol/league/v4/grandmasterleagues/by-queue/%queue%?api_key=%apikey%";
    const GET_LEAGUE: &'static str = "https://%region%.api.riotgames.com/lol/league/v4/leagues/%leagueid%?api_key=%apikey%";
    const GET_MASTER_LEAGUE: &'static str = "https://%region%.api.riotgames.com/lol/league/v4/masterleagues/by-queue/%queue%?api_key=%apikey%";

    // LOL-STATUS-V3
    const GET_STATUS_URL: &'static str = "https://%region%.api.riotgames.com/lol/status/v3/shard-data?api_key=%apikey%";

    // MATCH-V4
    const GET_MATCH: &'static str =
        "https://%region%.api.riotgames.com/lol/match/v4/matches/%matchid%?api_key=%apikey%";
    const GET_MATCHLIST: &'static str = "https://%region%.api.riotgames.com/lol/match/v4/matchlists/by-account/%accountid%?api_key=%apikey%";
    const GET_MATCH_TIMELINE: &'static str = "https://%region%.api.riotgames.com/lol/match/v4/timelines/by-match/%matchid%?api_key=%apikey%";
    const GET_MATCHES_BY_TOURNAMENT_CODE: &'static str =  "https://%region%.api.riotgames.com/lol/match/v4/matches/by-tournament-code/%tournamentcode%?api_key=%apikey%";
    const GET_MATCH_BY_ID_AND_TOURNAMENT_CODE: &'static str =  "https://%region%.api.riotgames.com/lol/match/v4/matches/%matchid%/by-tournament-code/%tournamentcode%?api_key=%apikey%";

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

    pub fn get_league_entries(lol_api_key: &LolApiKey, queue: &str, tier: &str, division: &str) -> String {
        return Self::get_url_from_api_key_with_division_queue_tier(Self::GET_LEAGUE_ENTRIES, lol_api_key, queue, tier, division);
    }

    pub fn get_challenger_league(lol_api_key: &LolApiKey, queue: &str) -> String {
        return Self::get_url_from_api_key_with_division_queue_tier(Self::GET_CHALLENGER_LEAGUE, lol_api_key, queue, "", "");
    }

    pub fn get_league_entries_by_summoner(lol_api_key: &LolApiKey, summoner_id: &str) -> String {
        return Self::get_url_from_api_key_with_summoner_id(Self::GET_LEAGUE_ENTRIES_BY_SUMMONER, lol_api_key, summoner_id);
    }

    pub fn get_all_league_entries(lol_api_key: &LolApiKey, queue: &str, tier: &str, division: &str) -> String {
        return Self::get_url_from_api_key_with_division_queue_tier(Self::GET_ALL_LEAGUE_ENTRIES, lol_api_key, queue, tier, division);
    }

    pub fn get_grandmaster_league(lol_api_key: &LolApiKey, queue: &str) -> String {
        return Self::get_url_from_api_key_with_division_queue_tier(Self::GET_GRANDMASTER_LEAGUE, lol_api_key, queue, "", "");
    }

    pub fn get_league(lol_api_key: &LolApiKey, league_id: &str) -> String {
        return Self::get_url_from_api_key(Self::GET_LEAGUE, lol_api_key).replace("%leagueid%", league_id);
    }

    pub fn get_master_league(lol_api_key: &LolApiKey, queue: &str) -> String {
        return Self::get_url_from_api_key_with_division_queue_tier(Self::GET_MASTER_LEAGUE, lol_api_key, queue, "", "");
    }

    pub fn get_status(lol_api_key: &LolApiKey) -> String {
        return Self::get_url_from_api_key(Self::GET_STATUS_URL, lol_api_key);
    }

    pub fn get_match(lol_api_key: &LolApiKey, match_id: &str) -> String {
        return Self::get_url_from_api_key_with_match_tournament(Self::GET_MATCH, lol_api_key, match_id, "");
    }

    pub fn get_match_list(lol_api_key: &LolApiKey, account_id: &str) -> String {
        return Self::get_url_from_api_key(Self::GET_MATCHLIST, lol_api_key).replace("%accountid%", account_id);
    }

    pub fn get_match_timeline(lol_api_key: &LolApiKey, match_id: &str) -> String {
        return Self::get_url_from_api_key_with_match_tournament(Self::GET_MATCH_TIMELINE, lol_api_key, match_id, "");
    }

    pub fn get_matches_by_tournament_code(lol_api_key: &LolApiKey, tournament_code: &str) -> String {
        return Self::get_url_from_api_key_with_match_tournament(Self::GET_MATCHES_BY_TOURNAMENT_CODE, lol_api_key, "", tournament_code);
    }

    pub fn get_matches_by_match_id_and_tournament_code(lol_api_key: &LolApiKey, match_id: &str, tournament_code: &str) -> String {
        return Self::get_url_from_api_key_with_match_tournament(Self::GET_MATCH_BY_ID_AND_TOURNAMENT_CODE, lol_api_key, match_id, tournament_code);
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

    fn get_url_from_api_key_with_division_queue_tier(original_url: &str, lol_api_key: &LolApiKey, queue: &str, tier: &str, division: &str) -> String {
        Self::get_url_from_api_key(original_url, lol_api_key)
            .replace("%queue%", queue)
            .replace("%tier%", tier)
            .replace("%division%", division)
    }

    fn get_url_from_api_key_with_match_tournament(original_url: &str, lol_api_key: &LolApiKey, match_id: &str, tournament_code: &str) -> String {
        Self::get_url_from_api_key(original_url, lol_api_key)
            .replace("%matchid%", match_id)
            .replace("%tournamentcode%", tournament_code)
    }
}