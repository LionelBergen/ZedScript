use crate::api_structs::lol_api_key::LolApiKey;
use crate::util::http_client::HttpClient;
use crate::util::http_error::HttpError;
use crate::league::league_url::LeagueUrl;
use crate::api_structs::champion_mastery::lol_champion_mastery_dto::ChampionMasteryDto;
use crate::api_structs::champion::lol_champion_info::ChampionInfo;
use std::collections::HashMap;
use crate::api_structs::tournament::lol_lobby_event_dto_wrapper::LobbyEventDtoWrapper;
use crate::api_structs::clash::lol_tournament_dto::TournamentDto;
use crate::api_structs::clash::lol_team_dto::{PlayerDto, TeamDto};
use crate::api_structs::league::lol_league_entry_dto::LeagueEntryDTO;
use crate::api_structs::league::lol_league_list_dto::LeagueListDTO;
use crate::api_structs::status::lol_shard_data::ShardStatus;
use crate::api_structs::match_v4::lol_match_dto::MatchDto;
use crate::api_structs::match_v4::lol_match_list_dto::MatchListDto;
use crate::api_structs::match_v4::lol_match_timeline_dto::MatchTimelineDto;
use crate::api_structs::spectator::lol_current_game_info::CurrentGameInfo;
use crate::api_structs::spectator::lol_featured_games::FeaturedGames;
use crate::api_structs::summoner::lol_summoner_dto::SummonerDTO;
use crate::api_structs::tournament::lol_tournament_code_dto::TournamentCodeDTO;

pub struct RiotApi {}

impl RiotApi {
    pub fn get_status(lol_api_key: &LolApiKey) -> Result<ShardStatus, HttpError> {
        let url: String = LeagueUrl::get_status(lol_api_key);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let result: ShardStatus = serde_json::from_str(&result).unwrap();

                Ok(result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_champion_mastery(lol_api_key: &LolApiKey, summoner_id: &str) -> Result<Vec<ChampionMasteryDto>, HttpError> {
        let url: String = LeagueUrl::get_champion_mastery(lol_api_key, summoner_id);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let result: Vec<ChampionMasteryDto> = serde_json::from_str(&result).unwrap();

                Ok(result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_champion_mastery_by_champion(lol_api_key: &LolApiKey, summoner_id: &str, champion_id: &str) -> Result<ChampionMasteryDto, HttpError> {
        let url: String = LeagueUrl::get_champion_mastery_url_with_champion(lol_api_key, summoner_id, champion_id);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let result: ChampionMasteryDto = serde_json::from_str(&result).unwrap();

                Ok(result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_champion_mastery_score(lol_api_key: &LolApiKey, summoner_id: &str) -> Result<String, HttpError> {
        let url: String = LeagueUrl::get_champion_mastery_score(lol_api_key, summoner_id);

        HttpClient::get(&url)
    }

    pub fn get_champion_rotation(lol_api_key: &LolApiKey) -> Result<ChampionInfo, HttpError> {
        let url: String = LeagueUrl::get_champion_rotation(lol_api_key);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let result: ChampionInfo = serde_json::from_str(&result).unwrap();

                Ok(result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn create_provider_mock(lol_api_key: &LolApiKey, callback_url: &str) -> Result<String, HttpError> {
        let url: String = LeagueUrl::create_provider_mock(lol_api_key);

        let mut request_paramaters = HashMap::new();
        request_paramaters.insert("region", lol_api_key.region.to_string());
        request_paramaters.insert("url", callback_url);

        HttpClient::post(url, Option::from(request_paramaters))
    }

    pub fn create_provider(lol_api_key: &LolApiKey, callback_url: &str) -> Result<String, HttpError> {
        let url: String = LeagueUrl::create_provider(lol_api_key);

        let mut request_paramaters = HashMap::new();
        request_paramaters.insert("region", lol_api_key.region.to_string());
        request_paramaters.insert("url", callback_url);

        HttpClient::post(url, Option::from(request_paramaters))
    }

    pub fn get_clash_players(lol_api_key: &LolApiKey, summoner_id: &str) -> Result<Vec<PlayerDto>, HttpError> {
        let url: String = LeagueUrl::get_clash_players(lol_api_key, summoner_id);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let team_result: Vec<PlayerDto> = serde_json::from_str(&result).unwrap();

                Ok(team_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_clash_team(lol_api_key: &LolApiKey, team_id: &str) -> Result<TeamDto, HttpError> {
        let url: String = LeagueUrl::get_clash_team(lol_api_key, team_id);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let team_result: TeamDto = serde_json::from_str(&result).unwrap();

                Ok(team_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_clash_tournament_by_team_id(lol_api_key: &LolApiKey, team_id: &str) -> Result<TournamentDto, HttpError> {
        let url: String = LeagueUrl::get_clash_tournament_by_team_id(lol_api_key, team_id);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let tournament_result: TournamentDto = serde_json::from_str(&result).unwrap();

                Ok(tournament_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_clash_tournaments(lol_api_key: &LolApiKey) -> Result<Vec<TournamentDto>, HttpError> {
        let url: String = LeagueUrl::get_clash_tournaments(lol_api_key);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let tournaments_result: Vec<TournamentDto> = serde_json::from_str(&result).unwrap();

                Ok(tournaments_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_clash_tournament(lol_api_key: &LolApiKey, tounament_id: &str) -> Result<TournamentDto, HttpError> {
        let url: String = LeagueUrl::get_clash_tournament(lol_api_key, tounament_id);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: TournamentDto = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_league_entries(lol_api_key: &LolApiKey, queue: &str, tier: &str, division: &str) -> Result<Vec<LeagueEntryDTO>, HttpError> {
        let url: String = LeagueUrl::get_league_entries(lol_api_key, queue, tier, division);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: Vec<LeagueEntryDTO> = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_challenger_league(lol_api_key: &LolApiKey, queue: &str) -> Result<LeagueListDTO, HttpError> {
        let url: String = LeagueUrl::get_challenger_league(lol_api_key, queue);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: LeagueListDTO = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_league_entries_by_summoner_id(lol_api_key: &LolApiKey, summoner_id: &str) -> Result<Vec<LeagueEntryDTO>, HttpError> {
        let url: String = LeagueUrl::get_league_entries_by_summoner(lol_api_key, summoner_id);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: Vec<LeagueEntryDTO> = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_all_league_entries(lol_api_key: &LolApiKey, queue: &str, tier: &str, division: &str) -> Result<Vec<LeagueEntryDTO>, HttpError> {
        let url: String = LeagueUrl::get_all_league_entries(lol_api_key, queue, tier, division);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: Vec<LeagueEntryDTO> = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_grandmaster_league(lol_api_key: &LolApiKey, queue: &str) -> Result<LeagueListDTO, HttpError> {
        let url: String = LeagueUrl::get_grandmaster_league(lol_api_key, queue);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: LeagueListDTO = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_league(lol_api_key: &LolApiKey, league_id: &str) -> Result<LeagueListDTO, HttpError> {
        let url: String = LeagueUrl::get_league(lol_api_key, league_id);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: LeagueListDTO = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_master_league(lol_api_key: &LolApiKey, queue: &str) -> Result<LeagueListDTO, HttpError> {
        let url: String = LeagueUrl::get_master_league(lol_api_key, queue);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: LeagueListDTO = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_tournament_lobby_events_mock(lol_api_key: &LolApiKey, tournament_code: &str) -> Result<LobbyEventDtoWrapper, HttpError> {
        let url: String = LeagueUrl::get_tournament_lobby_events_mock(lol_api_key, tournament_code);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: LobbyEventDtoWrapper = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn create_tournament_mock(lol_api_key: &LolApiKey, provider_id: &str, name_of_tournament: Option<&str>) -> Result<String, HttpError> {
        let url: String = LeagueUrl::create_tournament_mock(lol_api_key);

        let mut request_paramaters = HashMap::new();
        request_paramaters.insert("providerId", provider_id);

        if name_of_tournament.is_some() {
            request_paramaters.insert("name", name_of_tournament.unwrap());
        }

        HttpClient::post(url, Option::from(request_paramaters))
    }

    pub fn create_tournament_code_mock(lol_api_key: &LolApiKey, provider_id: &str, name_of_tournament: Option<&str>) -> Result<String, HttpError> {
        let url: String = LeagueUrl::create_tournament_code_mock(lol_api_key);

        let mut request_paramaters = HashMap::new();
        request_paramaters.insert("providerId", provider_id);

        if name_of_tournament.is_some() {
            request_paramaters.insert("name", name_of_tournament.unwrap());
        }

        HttpClient::post(url, Option::from(request_paramaters))
    }

    pub fn create_tournament_code(lol_api_key: &LolApiKey, tournament_id: &str, count: Option<&str>, allowed_summoner_ids: Option<&str>, map_type: Option<&str>, meta_deta: Option<&str>,
            pick_type: Option<&str>, spectator_type: Option<&str>, team_size: Option<&str>) -> Result<Vec<String>, HttpError> {
        let url: String = LeagueUrl::create_tournament_code(lol_api_key);

        let mut request_paramaters = HashMap::new();
        request_paramaters.insert("tournamentId", tournament_id);

        if count.is_some() {
            request_paramaters.insert("count", count.unwrap());
        }

        if allowed_summoner_ids.is_some() {
            request_paramaters.insert("allowedSummonerIds", allowed_summoner_ids.unwrap());
        }

        if map_type.is_some() {
            request_paramaters.insert("mapType", map_type.unwrap());
        }

        if meta_deta.is_some() {
            request_paramaters.insert("metaData", meta_deta.unwrap());
        }

        if pick_type.is_some() {
            request_paramaters.insert("pickType", pick_type.unwrap());
        }

        if spectator_type.is_some() {
            request_paramaters.insert("spectatorType", spectator_type.unwrap());
        }

        if team_size.is_some() {
            request_paramaters.insert("teamSize", team_size.unwrap());
        }

        let http_result = HttpClient::post(url, Option::from(request_paramaters));

        match http_result {
            Ok(result) => {
                let league_game_result: Vec<String> = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn create_tournament(lol_api_key: &LolApiKey, provider_id: &str, name_of_tournament: Option<&str>) -> Result<String, HttpError> {
        let url: String = LeagueUrl::create_tournament(lol_api_key);

        let mut request_paramaters = HashMap::new();
        request_paramaters.insert("providerId", provider_id);

        if name_of_tournament.is_some() {
            request_paramaters.insert("name", name_of_tournament.unwrap());
        }

        HttpClient::post(url, Option::from(request_paramaters))
    }

    pub fn get_tournament(lol_api_key: &LolApiKey, tournament_code: &str) -> Result<TournamentCodeDTO, HttpError> {
        let url: String = LeagueUrl::get_tournament(lol_api_key, tournament_code);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let result: TournamentCodeDTO = serde_json::from_str(&result).unwrap();

                Ok(result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn update_tournament(lol_api_key: &LolApiKey, tournament_code: &str, allowed_summoner_ids: Option<&str>, map_type: Option<&str>, pick_type: Option<&str>, spectator_type: Option<&str>)
            -> Result<String, HttpError> {
        let url: String = LeagueUrl::update_tournament(lol_api_key, tournament_code);

        let mut request_paramaters = HashMap::new();

        if allowed_summoner_ids.is_some() {
            request_paramaters.insert("allowedSummonerIds", allowed_summoner_ids.unwrap());
        }

        if map_type.is_some() {
            request_paramaters.insert("mapType", map_type.unwrap());
        }

        if pick_type.is_some() {
            request_paramaters.insert("pickType", pick_type.unwrap());
        }

        if spectator_type.is_some() {
            request_paramaters.insert("spectatorType", spectator_type.unwrap());
        }

        HttpClient::put(url, Option::from(request_paramaters))
    }

    pub fn get_tournament_lobby_events(lol_api_key: &LolApiKey, tournament_code: &str) -> Result<LobbyEventDtoWrapper, HttpError> {
        let url: String = LeagueUrl::get_tournament_lobby_events(lol_api_key, tournament_code);

        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: LobbyEventDtoWrapper = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_third_party_code(lol_api_key: &LolApiKey, encrypted_summoner_id: &str) -> Result<String, HttpError> {
        let url: String = LeagueUrl::get_third_party_code(lol_api_key, encrypted_summoner_id);

        HttpClient::get(&url)
    }

    pub fn get_featured_games(lol_api_key: &LolApiKey) -> Result<FeaturedGames, HttpError> {
        let url: String = LeagueUrl::get_featured_games(lol_api_key);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_game_result: FeaturedGames = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_match(lol_api_key: &LolApiKey, match_id: &str) -> Result<MatchDto, HttpError> {
        let url: String = LeagueUrl::get_match(lol_api_key, match_id);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                println!("{}", result);
                let league_game_result: MatchDto = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_match_list(lol_api_key: &LolApiKey, account_id: &str) -> Result<MatchListDto, HttpError> {
        let url: String = LeagueUrl::get_match_list(lol_api_key, account_id);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                println!("{}", result);
                let league_game_result: MatchListDto = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_match_timeline(lol_api_key: &LolApiKey, match_id: &str) -> Result<MatchTimelineDto, HttpError> {
        let url: String = LeagueUrl::get_match_timeline(lol_api_key, match_id);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                println!("{}", result);
                let league_game_result: MatchTimelineDto = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_matches_by_tournament(lol_api_key: &LolApiKey, tournament_code: &str) -> Result<Vec<i64>, HttpError> {
        let url: String = LeagueUrl::get_matches_by_tournament_code(lol_api_key, tournament_code);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                println!("{}", result);
                let league_game_result: Vec<i64> = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_match_by_match_id_and_tournament(lol_api_key: &LolApiKey, match_id: &str, tournament_code: &str) -> Result<MatchDto, HttpError> {
        let url: String = LeagueUrl::get_matches_by_match_id_and_tournament_code(lol_api_key, match_id, tournament_code);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                println!("{}", result);
                let league_game_result: MatchDto = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_active_game(summoner_id: &str, lol_api_key: &LolApiKey) -> Result<CurrentGameInfo, HttpError> {
        let url: String = LeagueUrl::get_active_games(lol_api_key, summoner_id);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                println!("{}", result);
                let league_game_result: CurrentGameInfo = serde_json::from_str(&result).unwrap();

                Ok(league_game_result)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_summoner_by_account_id(lol_api_key: &LolApiKey, account_id: String) -> Result<SummonerDTO, HttpError> {
        let url: String = LeagueUrl::get_summoner_by_account_id(lol_api_key, &account_id);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_account: SummonerDTO = serde_json::from_str(&result).unwrap();

                Ok(league_account)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_summoner_by_puuid_id(lol_api_key: &LolApiKey, puuid: String) -> Result<SummonerDTO, HttpError> {
        let url: String = LeagueUrl::get_summoner_by_puuid(lol_api_key, &puuid);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_account: SummonerDTO = serde_json::from_str(&result).unwrap();

                Ok(league_account)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_summoner_by_summoner_id(lol_api_key: &LolApiKey, summoner_id: String) -> Result<SummonerDTO, HttpError> {
        let url: String = LeagueUrl::get_summoner_by_summoner_id(lol_api_key, &summoner_id);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_account: SummonerDTO = serde_json::from_str(&result).unwrap();

                Ok(league_account)
            }
            Err(error) => Err(error),
        }
    }

    pub fn get_summoner(lol_api_key: &LolApiKey, summoner_name: String) -> Result<SummonerDTO, HttpError> {
        let url: String = LeagueUrl::get_summoner_by_name(lol_api_key, &summoner_name);
        let http_result = HttpClient::get(&url);

        match http_result {
            Ok(result) => {
                let league_account: SummonerDTO = serde_json::from_str(&result).unwrap();

                Ok(league_account)
            }
            Err(error) => Err(error),
        }
    }
}
