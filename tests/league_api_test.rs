extern crate zed_script;

use zed_script::api_structs::lol_api_key::LolApiKey;
use zed_script::api_structs::lol_region::Region;
use zed_script::league_api;

use std::env;
use zed_script::api_structs::lol_account::LeagueAccount;
use zed_script::util::http_error::HttpError;

// LeagueOfSausage account id
const LEAGUE_ACCOUNT_ID: &str = "QfMypRv2CyU9Q9w3MXyFfw9rt6UPhlsuOkDc-1VYfhuy1sY";
const LEAGUE_SUMMONER_ID: &str = "n-zcEtpy2E4JUt8AksUMpkEB9SsBw51-6b6rDF27wvZ1YYw";
const MATCH_ID: &str = "3609154837";

const MOCK_PROVIDER_ID: &str = "228";
const MOCK_TOURNAMENT_ID: &str = "1158";

fn get_league_api_token() -> String {
    return String::from(&env::var("LEAGUE_API_KEY").unwrap());
}

fn get_league_api_key() -> LolApiKey {
    LolApiKey {
        api_key: get_league_api_token(),
        region: Region::NA,
    }
}

/**
 * To avoid rate limiting
*/
fn pause_execution() {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

#[test]
fn test_get_status_unauthorized() {
    pause_execution();
    // no api key specified, expect unauthorized response
    let lol_api_key = LolApiKey {
        api_key: "".to_string(),
        region: Region::RU,
    };
    let result = league_api::RiotApi::get_status(&lol_api_key);
    let expected = HttpError {
        error_message: String::from("Unauthorized access to application"),
        http_response_code: Some(401),
    };

    assert_eq!(Err(expected), result);
}

#[test]
fn test_get_status() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_status(&lol_api_key);

    assert!(result.unwrap().contains("active"));
}

#[test]
fn test_get_champion_mastery() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_champion_mastery(&lol_api_key, LEAGUE_SUMMONER_ID);

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.is_empty());
    assert_eq!(LEAGUE_SUMMONER_ID, unwrapped_result[0].summoner_id);
}

#[test]
fn test_get_champion_mastery_by_champion() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    // find LeagueOfSausage/DrMundo
    let result = league_api::RiotApi::get_champion_mastery_by_champion(&lol_api_key, LEAGUE_SUMMONER_ID, "36");

    let unwrapped_result = result.unwrap();
    assert_eq!(LEAGUE_SUMMONER_ID, unwrapped_result.summoner_id);
}

#[test]
fn test_get_champion_mastery_score() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    // find LeagueOfSausage/DrMundo
    let result = league_api::RiotApi::get_champion_mastery_score(&lol_api_key, LEAGUE_SUMMONER_ID);

    let unwrapped_result = result.unwrap();
    assert_eq!("229", unwrapped_result);
}

#[test]
fn test_get_champion_rotation() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_champion_rotation(&lol_api_key);

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.free_champion_ids.is_empty());
}

#[test]
fn test_create_provider_mock() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    // find LeagueOfSausage/DrMundo
    let result = league_api::RiotApi::create_provider_mock(&lol_api_key, "https://www.google.com");

    let unwrapped_result = result.unwrap();
    assert_eq!(MOCK_PROVIDER_ID, unwrapped_result);
}

#[test]
fn test_create_tournament_mock() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    // find LeagueOfSausage/DrMundo
    let result = league_api::RiotApi::create_tournament_mock(&lol_api_key, "https://www.google.com", MOCK_PROVIDER_ID, Option::None);

    let unwrapped_result = result.unwrap();
    assert_eq!(MOCK_TOURNAMENT_ID, unwrapped_result);
}

#[test]
fn test_get_tournament_lobby_events_mock() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_tournament_lobby_events_mock(&lol_api_key, MOCK_TOURNAMENT_ID);
    assert!(result.is_ok());
    assert!(!result.unwrap().events.is_empty());
}

#[test]
fn test_get_summoner_by_account_id() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner_by_account_id(
        LEAGUE_ACCOUNT_ID.to_string(),
        &lol_api_key,
    );

    // No need to assert NotNull, values are non-optional
    let league_account_result: LeagueAccount = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
}

#[test]
fn test_get_summoner_by_puuid_id() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner_by_puuid_id(
        "PovRyqcBB-MYNAchL1945Gt0dGwJ1b0yOSzj7ArsRYQ5kiySs8UX4WN2Lsvjy1s-6ihupmzL1FvnIQ"
            .to_string(),
        &lol_api_key,
    );

    // No need to assert NotNull, values are non-optional
    let league_account_result: LeagueAccount = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
}

#[test]
fn test_get_summoner_by_summoner_id() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner_by_summoner_id(
        LEAGUE_SUMMONER_ID.to_string(),
        &lol_api_key,
    );

    // No need to assert NotNull, values are non-optional
    let league_account_result: LeagueAccount = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
}

#[test]
fn test_get_summoner() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner(String::from("LeagueOfSausage"), &lol_api_key);

    // No need to assert NotNull, values are non-optional
    let league_account_result: LeagueAccount = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
    assert_eq!(106, league_account_result.summoner_level);
    assert_eq!(
        LEAGUE_ACCOUNT_ID,
        league_account_result.account_id
    );
}

#[test]
fn test_get_summoner_not_exist() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner(String::from("LeagueOfSausage22"), &lol_api_key);
    let expected = HttpError {
        error_message: "Error in http request".to_string(),
        http_response_code: Some(404),
    };

    assert_eq!(Err(expected), result);
}

#[test]
fn test_get_third_party_code() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_third_party_code("123", &lol_api_key);
    let expected = HttpError {
        error_message: "Error in http request".to_string(),
        http_response_code: Some(400),
    };

    assert_eq!(Err(expected), result);
}

#[test]
fn test_get_featured_games() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_featured_games(&lol_api_key);
    assert!(result.is_ok());
}

#[test]
fn test_get_active_games() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let active_game_list = league_api::RiotApi::get_featured_games(&lol_api_key);
    assert!(active_game_list.is_ok());
    pause_execution();

    let summoner_name_in_active_game =
        &active_game_list.unwrap().game_list[0].participants[0].summoner_name;

    let summoner =
        league_api::RiotApi::get_summoner(summoner_name_in_active_game.to_string(), &lol_api_key);
    assert!(summoner.is_ok());
    pause_execution();

    let summoner_id_active_in_game = summoner.unwrap();
    let active_game_summoner_id = &summoner_id_active_in_game.summoner_id;

    let result = league_api::RiotApi::get_active_games(&active_game_summoner_id, &lol_api_key);
    assert!(result.is_ok());
}

#[test]
fn test_get_match() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_match(MATCH_ID, &lol_api_key);
    assert!(result.is_ok());
}

#[test]
fn test_get_match_list() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_match_list(LEAGUE_ACCOUNT_ID, &lol_api_key);
    assert!(result.is_ok());
}
