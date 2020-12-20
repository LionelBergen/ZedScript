use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize)]
pub struct MatchTimelineDto {
    pub frames: Vec<MatchFrameDto>,
    #[serde(rename = "frameInterval")]
    frame_interval: i64
}

#[derive(Serialize, Deserialize)]
pub struct MatchFrameDto {
    #[serde(rename = "participantFrames")]
    pub participant_frames: Map<String, MatchParticipantFrameDto>,
    pub events: Vec<MatchEventDto>,
    pub timestamp: i64
}

#[derive(Serialize, Deserialize)]
pub struct MatchParticipantFrameDto {
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: i64,
    #[serde(rename = "teamScore")]
    pub team_score: i32,
    #[serde(rename = "dominionScore")]
    pub dominion_score: i32,
    #[serde(rename = "totalGold")]
    pub total_gold: i32,
    pub level: i16,
    pub xp: i32,
    #[serde(rename = "currentGold")]
    pub current_gold: i32,
    pub position: MatchPositionDto,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: i32
}

#[derive(Serialize, Deserialize)]
pub struct MatchPositionDto {
    pub x: i16,
    pub y: i16
}

#[derive(Serialize, Deserialize)]
pub struct MatchEventDto {
    #[serde(rename = "laneType")]
    pub lane_type: Option<String>,
    #[serde(rename = "skillSlot")]
    pub skill_slot: Option<i32>,
    #[serde(rename = "ascendedType")]
    pub ascended_type: Option<String>,
    #[serde(rename = "creatorId")]
    pub creator_id: Option<i32>,
    #[serde(rename = "afterId")]
    pub after_id: Option<i32>,
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,
    #[serde(rename = "type")]
    pub match_event_dto_type: String, // 'type' is a keyword so we can't use that here
    #[serde(rename = "levelUpType")]
    pub level_up_type: Option<String>,
    #[serde(rename = "wardType")]
    pub ward_type: Option<String>,
    #[serde(rename = "participantId")]
    pub participant_id: Option<i64>,
    #[serde(rename = "towerType")]
    pub tower_type: Option<String>,
    #[serde(rename = "itemId")]
    pub item_id: Option<i32>,
    #[serde(rename = "beforeId")]
    pub before_id: Option<i32>,
    #[serde(rename = "pointCaptured")]
    pub point_captured: Option<String>,
    #[serde(rename = "monsterType")]
    pub monster_tyoe: Option<String>,
    #[serde(rename = "monsterSubType")]
    pub monster_sub_type: Option<String>,
    #[serde(rename = "teamId")]
    pub team_id: Option<i32>,
    pub position: Option<MatchPositionDto>,
    #[serde(rename = "killerId")]
    pub killer_id: Option<i64>,
    pub timestamp: i64,
    #[serde(rename = "assistingParticipantIds")]
    pub assisting_participant_ids: Option<Vec<i64>>,
    #[serde(rename = "buildingType")]
    pub building_type: Option<String>,
    #[serde(rename = "victimId")]
    pub victim_id: Option<i64>
}
