extern crate zed_script;

use zed_script::api_structs::lol_api_key::LolApiKey;
use zed_script::api_structs::lol_region::Region;
use zed_script::league_api;

use std::env;
use zed_script::api_structs::lol_account::LeagueAccount;
use zed_script::util::http_error::HttpError;
use zed_script::api_structs::clash::lol_team_dto::PlayerDto;
use zed_script::api_structs::summoner::lol_summoner_dto::SummonerDTO;

use serial_test::serial;

// LeagueOfSausage account id
const LEAGUE_ACCOUNT_ID: &str = "QfMypRv2CyU9Q9w3MXyFfw9rt6UPhlsuOkDc-1VYfhuy1sY";
const LEAGUE_SUMMONER_ID: &str = "n-zcEtpy2E4JUt8AksUMpkEB9SsBw51-6b6rDF27wvZ1YYw";

// TODO: Find different types of matches, such as dominion, ARAM etc. Also find a very old match to test 'Runes', which are now absolete.
// TODO: Find a few different match timelines to test out....
const MATCH_ID: &str = "3609154837";

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
#[serial]
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
#[serial]
fn test_get_status() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_status(&lol_api_key);

    assert_ne!("", result.unwrap().name);
}

#[test]
#[serial]
fn test_get_champion_mastery() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_champion_mastery(&lol_api_key, LEAGUE_SUMMONER_ID);

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.is_empty());
    assert_eq!(LEAGUE_SUMMONER_ID, unwrapped_result[0].summoner_id);
}

#[test]
#[serial]
fn test_get_champion_mastery_by_champion() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    // find LeagueOfSausage/DrMundo
    let result = league_api::RiotApi::get_champion_mastery_by_champion(&lol_api_key, LEAGUE_SUMMONER_ID, "36");

    let unwrapped_result = result.unwrap();
    assert_eq!(LEAGUE_SUMMONER_ID, unwrapped_result.summoner_id);
}

#[test]
#[serial]
fn test_get_champion_mastery_score() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    // find LeagueOfSausage/DrMundo
    let result = league_api::RiotApi::get_champion_mastery_score(&lol_api_key, LEAGUE_SUMMONER_ID);

    let unwrapped_result = result.unwrap();
    assert_eq!("229", unwrapped_result);
}

#[test]
#[serial]
fn test_get_champion_rotation() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_champion_rotation(&lol_api_key);

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.free_champion_ids.is_empty());
}

#[test]
#[serial]
fn test_get_clash_players() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_clash_players(&lol_api_key, LEAGUE_SUMMONER_ID);

    let unwrapped_result = result.unwrap();
    assert!(unwrapped_result.is_empty());
}

// TODO:
/*#[test]
fn test_get_clash_team() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_clash_team(&lol_api_key, "100");

    let unwrapped_result = result.unwrap();
    assert_eq!("gg", unwrapped_result.name);
}*/

// TODO:
/*#[test]
fn test_get_clash_tournament_by_team_id() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_clash_tournament_by_team_id(&lol_api_key, "100");

    let unwrapped_result = result.unwrap();
    assert_eq!("gg", unwrapped_result.name_key);
}*/

#[test]
#[serial]
fn test_get_clash_tournaments() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_clash_tournaments(&lol_api_key);

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.is_empty());
}

#[test]
#[serial]
fn test_get_clash_tournament() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let tournaments = league_api::RiotApi::get_clash_tournaments(&lol_api_key).unwrap();
    pause_execution();
    let result = league_api::RiotApi::get_clash_tournament(&lol_api_key, &tournaments[0].id.to_string());

    let unwrapped_result = result.unwrap();
    assert_ne!("", unwrapped_result.name_key);
}

#[test]
#[serial]
fn test_get_league_entries() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_league_entries(&lol_api_key, "RANKED_SOLO_5x5", "CHALLENGER", "I");

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.is_empty());
}

#[test]
#[serial]
fn test_get_challenger_league() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_challenger_league(&lol_api_key, "RANKED_SOLO_5x5");

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.entries.is_empty());
}

#[test]
#[serial]
fn test_get_league_entries_by_summoner_id() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_league_entries_by_summoner_id(&lol_api_key, "0pkZjg-IXKqoi6MTFpMyV1tY1bL5zTLiRvjVmkh5lpO8pg8");

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.is_empty());
}

#[test]
#[serial]
fn test_get_all_league_entries() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_all_league_entries(&lol_api_key, "RANKED_SOLO_5x5", "DIAMOND", "I");

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.is_empty());
}

#[test]
#[serial]
fn test_get_grandmaster_league() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_grandmaster_league(&lol_api_key, "RANKED_SOLO_5x5");

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.entries.is_empty());
}

#[test]
#[serial]
fn test_get_league() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_league(&lol_api_key, "c60807e8-6afb-38fd-ab9b-ae8588dc8b27");

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.entries.is_empty());
}

#[test]
#[serial]
fn test_get_master_league() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_master_league(&lol_api_key, "RANKED_SOLO_5x5");

    let unwrapped_result = result.unwrap();
    assert!(!unwrapped_result.entries.is_empty());
}

#[test]
#[serial]
fn test_create_provider_mock() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    // find LeagueOfSausage/DrMundo
    let result = league_api::RiotApi::create_provider_mock(&lol_api_key, "https://www.google.com");

    let unwrapped_result : i32 = result.unwrap().parse().unwrap();
    assert!(unwrapped_result > 0);
}

#[test]
#[serial]
fn test_create_tournament_mock() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    // find LeagueOfSausage/DrMundo
    let result = league_api::RiotApi::create_tournament_mock(&lol_api_key, &get_test_provider(), Option::Some("name"));

    let unwrapped_result : i32 = result.unwrap().parse().unwrap();
    assert!(unwrapped_result > 0);
}

#[test]
#[serial]
fn test_get_tournament_lobby_events_mock() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_tournament_lobby_events_mock(&lol_api_key, &get_test_tournament());
    assert!(result.is_ok());
    assert!(!result.unwrap().events.is_empty());
}

// TODO: add tests for TROUNAMENT-V4. Use a Mock for the http calls..

#[test]
#[serial]
fn test_get_summoner_by_account_id() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner_by_account_id(&lol_api_key,LEAGUE_ACCOUNT_ID.to_string());

    // No need to assert NotNull, values are non-optional
    let league_account_result: SummonerDTO = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
}

#[test]
#[serial]
fn test_get_summoner_by_puuid_id() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner_by_puuid_id(&lol_api_key, "PovRyqcBB-MYNAchL1945Gt0dGwJ1b0yOSzj7ArsRYQ5kiySs8UX4WN2Lsvjy1s-6ihupmzL1FvnIQ".to_string());

    // No need to assert NotNull, values are non-optional
    let league_account_result: SummonerDTO = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
}

#[test]
#[serial]
fn test_get_summoner_by_summoner_id() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner_by_summoner_id(
        &lol_api_key,
        LEAGUE_SUMMONER_ID.to_string()
    );

    // No need to assert NotNull, values are non-optional
    let league_account_result: SummonerDTO = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
}

#[test]
#[serial]
fn test_get_summoner() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner(&lol_api_key, String::from("LeagueOfSausage"));

    // No need to assert NotNull, values are non-optional
    let league_account_result: SummonerDTO = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
    assert_eq!(106, league_account_result.summoner_level);
    assert_eq!(
        LEAGUE_ACCOUNT_ID,
        league_account_result.account_id
    );
}

#[test]
#[serial]
fn test_get_summoner_not_exist() {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::get_summoner(&lol_api_key, String::from("LeagueOfSausage22"));
    let expected = HttpError {
        error_message: "Error in http request".to_string(),
        http_response_code: Some(404),
    };

    assert_eq!(Err(expected), result);
}

// TODO: proper test somehow
#[test]
#[serial]
fn test_get_third_party_code() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_third_party_code(&lol_api_key, "123");
    let expected = HttpError {
        error_message: "Error in http request".to_string(),
        http_response_code: Some(400),
    };

    assert_eq!(Err(expected), result);
}

#[test]
#[serial]
fn test_get_featured_games() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_featured_games(&lol_api_key);
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_get_active_game() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let active_game_list = league_api::RiotApi::get_featured_games(&lol_api_key);
    assert!(active_game_list.is_ok());
    pause_execution();

    let summoner_name_in_active_game =
        &active_game_list.unwrap().game_list[0].participants[0].summoner_name;

    let summoner =
        league_api::RiotApi::get_summoner(&lol_api_key, summoner_name_in_active_game.to_string());
    assert!(summoner.is_ok());
    pause_execution();

    let summoner_id_active_in_game = summoner.unwrap();
    let active_game_summoner_id = &summoner_id_active_in_game.summoner_id;

    let result = league_api::RiotApi::get_active_game(&active_game_summoner_id, &lol_api_key);
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_get_match() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_match(&lol_api_key, MATCH_ID);
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_get_match_list() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_match_list(&lol_api_key, LEAGUE_ACCOUNT_ID);
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_get_match_timeline() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let result = league_api::RiotApi::get_match_timeline(&lol_api_key, MATCH_ID);
    assert!(result.is_ok());
}

/*
// TODO: I don't have API access for tournament stuff
#[test]
fn test_get_matches_by_tournament_code() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    league_api::RiotApi::get_matches_by_tournament(&lol_api_key, &get_test_tournament()).unwrap();
}
 */

// TODO: need tournamentCode and matchid
/*#[test]
fn test_get_match_by_match_id_and_tournament() {
    pause_execution();
    let lol_api_key = get_league_api_key();

    let tournamneMatchId = &get_test_tournament();

    let result = league_api::RiotApi::get_match_by_match_id_and_tournament(&lol_api_key, tournamneMatchId, tournamneMatchId);
    assert!(result.is_ok());
}*/

fn get_test_tournament() -> String {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::create_tournament_mock(&lol_api_key, &get_test_provider(), Option::Some("name"));

    result.unwrap()
}

fn get_test_provider() -> String {
    pause_execution();
    let lol_api_key = get_league_api_key();
    let result = league_api::RiotApi::create_provider_mock(&lol_api_key, "https://www.google.com");

    result.unwrap()
}