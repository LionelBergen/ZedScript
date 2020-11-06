use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MiniSeriesDTO {
    pub losses: i16,
    pub progress: String,
    pub target: i16,
    pub wins: i16
}