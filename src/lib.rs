pub mod riot_api;
pub use riot_api::*;

pub mod api_structs {
    pub mod lol_api_key;
    pub mod lol_region;
}

pub mod util {
    pub mod http_client;
}