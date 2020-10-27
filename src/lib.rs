pub mod league_api;
pub use league_api::*;

pub mod api_structs {
    pub mod lol_account;
    pub mod lol_api_key;
    pub mod lol_game;
    pub mod lol_match_list;
    pub mod lol_participant;
    pub mod lol_region;
    pub mod champion_mastery {
        pub mod lol_champion_mastery_dto;
    }
}

pub mod league {
    pub(crate) mod league_url;
}

pub mod util {
    pub(crate) mod http_client;
    pub mod http_error;
}
