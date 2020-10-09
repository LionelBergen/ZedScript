extern crate zed_script;

use zed_script::riot_api;
use zed_script::api_structs::lol_api_key::LolApiKey;
use zed_script::api_structs::lol_region::Region;

use zed_script::util::http_error::HttpError;
use std::env;
use zed_script::api_structs::lol_account::LeagueAccount;

fn get_league_api_token() -> String {
    return String::from(&env::var("LEAGUE_API_KEY").unwrap());
}

/**
 * To avoid rate limiting
*/
fn pause_execution() {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

#[test]
fn test_get_status_unauthorized() {
    // no api key specified, expect unauthorized response
    let lol_api_key = LolApiKey {api_key: String::from(""), region: Region::NA, fully_load_classes: true};
    let result = riot_api::RiotApi::get_status(&lol_api_key);
    let expected = HttpError{ error_message: String::from("Unauthorized access to application"), http_response_code: Some(401) };

    assert_eq!(Err(expected), result);
}

#[test]
fn test_get_status() {
    let lol_api_key = LolApiKey {api_key: get_league_api_token(), region: Region::NA, fully_load_classes: true};
    let result = riot_api::RiotApi::get_status(&lol_api_key);

    assert!(result.unwrap().contains("active"));
    pause_execution();
}

#[test]
fn test_get_summoner() {
    let lol_api_key = LolApiKey {api_key: get_league_api_token(), region: Region::NA, fully_load_classes: true};
    let result = riot_api::RiotApi::get_summoner(String::from("LeagueOfSausage"), &lol_api_key);

    // No need to assert NotNull, values are non-optional
    let league_account_result: LeagueAccount = result.unwrap();
    assert_eq!("LeagueOfSausage", league_account_result.name);
    assert_eq!(106, league_account_result.summoner_level);
    pause_execution();
}

#[test]
fn test_get_summoner_not_exist() {
    let lol_api_key = LolApiKey {api_key: get_league_api_token(), region: Region::NA, fully_load_classes: true};
    let result = riot_api::RiotApi::get_summoner(String::from("LeagueOfSausage22"), &lol_api_key);
    let expected = HttpError{ error_message: "Error in http request".to_string(), http_response_code: Some(404) };

    println!("{:#?}", result);
    assert_eq!(Err(expected), result);
    pause_execution();
}