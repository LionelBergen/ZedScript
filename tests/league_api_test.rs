//use riot_api;
extern crate zed_script;

use zed_script::riot_api;
use zed_script::api_structs::lol_api_key::LolApiKey;
use zed_script::api_structs::lol_region::Region;
use std::error::Error;

#[test]
fn get_status_unauthorized() {
    let lol_api_key = LolApiKey {api_key: String::from(""), region: Region::NA, fully_load_classes: true};
    let result = riot_api::RiotApi::get_status(&lol_api_key);

    println!("{:?}", result);
    panic!("Just writing this so test fails...");
}
/*
#[test]
fn get_status() {
    let lol_api_key = LolApiKey {api_key: String::from(""), region: Region::NA, fully_load_classes: true};
    let result = riot_api::RiotApi::get_status(&lol_api_key);

    match result {
        Ok(result) =>  {
            println!("{:?}", result)
        },
        Err(e) => {
                println!("Error: {:?}", e);
                panic!("Erorr occured, http call not okay: {}", e);
        }
    }
    panic!("Just writing this so test fails...");
}*/