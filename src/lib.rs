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
    pub mod champion {
        pub mod lol_champion_info;
    }
    pub mod tournament {
        pub mod lol_lobby_event_dto_wrapper;
    }
    pub mod clash {
        pub mod lol_tournament_dto;
        pub mod lol_team_dto;
    }
    pub mod league {
        pub mod lol_league_entry_dto;
        pub mod lol_mini_series_dto;
        pub mod lol_league_item_dto;
        pub mod lol_league_list_dto;
    }
    pub mod status {
        pub mod lol_shard_data;
    }
    pub mod match_v4 {
        pub mod lol_match_dto;
        pub mod lol_match_list_dto;
        pub mod lol_match_timeline_dto;
    }
}

pub mod league {
    pub(crate) mod league_url;
}

pub mod util {
    pub(crate) mod http_client;
    pub mod http_error;
}
