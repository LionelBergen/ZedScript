use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize)]
pub struct MatchDto {
    #[serde(rename = "gameId")]
    pub match_id: i64,
    #[serde(rename = "participantIdentities")]
    pub participant_identities: Vec<ParticipantIdentityDto>,
    #[serde(rename = "queueId")]
    pub queue_id: i64,
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "gameDuration")]
    pub game_duration: i64,
    pub teams: Vec<TeamStatsDto>,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "gameCreation")]
    pub game_creation: i64,
    #[serde(rename = "seasonId")]
    pub season_id: i64,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "mapId")]
    pub map_id: i64,
    #[serde(rename = "gameMode")]
    pub gameMode: String,
    pub participants: Vec<ParticipantDto>
}

#[derive(Serialize, Deserialize)]
pub struct ParticipantDto {
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    pub runes: Option<Vec<RuneDto>>, // Runes refer to old runes
    pub stats: ParticipantStatsDto,
    #[serde(rename = "teamId")]
    pub team_id: i16,
    pub timeline: ParticipantTimelineDto,
    #[serde(rename = "spell1Id")]
    pub spell_1_id: i32,
    #[serde(rename = "spell2Id")]
    pub spell_2_id: i32,
    #[serde(rename = "highestAchievedSeasonTier")]
    pub highest_achieved_season_tier: Option<String>,
    pub masteries: Option<Vec<MasteryDto>>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MasteryDto {
    pub rank: i16,
    pub masteryId: i16
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParticipantStatsDto {
    #[serde(rename = "item0")]
    pub item_0: i32,
    #[serde(rename = "item2")]
    pub item2: i32,
    #[serde(rename = "totalUnitsHealed")]
    pub total_units_healed: i16,
    #[serde(rename = "item1")]
    pub item_1: i16,
    #[serde(rename = "largestMultiKill")]
    pub largest_multi_kill: i16,
    #[serde(rename = "goldEarned")]
    pub gold_earned: i32,
    #[serde(rename = "firstInhibitorKill")]
    pub first_inhibitor_kill: Option<bool>,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "nodeNeutralizeAssist")]
    pub node_neutralize_assist: Option<i32>,
    #[serde(rename = "totalPlayerScore")]
    pub total_player_score: i32,
    #[serde(rename = "champLevel")]
    pub champ_level: i16,
    #[serde(rename = "damageDealtToObjectives")]
    pub damage_dealt_to_objectives: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "neutralMinionsKilled")]
    pub neutral_minions_killed: i32,
    pub deaths: i32,
    #[serde(rename = "tripleKills")]
    pub triple_kills: i16,
    #[serde(rename = "magicDamageDealtToChampions")]
    pub magic_damage_dealt_to_champions: i64,
    #[serde(rename = "wardsKilled")]
    pub wards_killed: i32,
    #[serde(rename = "pentaKills")]
    pub penta_kills: i16,
    #[serde(rename = "damageSelfMitigated")]
    pub damage_self_mitigated: i64,
    #[serde(rename = "largestCriticalStrike")]
    pub largest_critical_strike: i32,
    #[serde(rename = "nodeNeutralize")]
    pub node_neutralize: Option<i32>,
    #[serde(rename = "totalTimeCrowdControlDealt")]
    pub total_time_crowd_control_dealt: i32,
    #[serde(rename = "firstTowerKill")]
    pub first_tower_kill: bool,
    #[serde(rename = "magicDamageDealt")]
    pub magic_damage_dealt: i64,
    #[serde(rename = "totalScoreRank")]
    pub total_score_rank: i32,
    #[serde(rename = "nodeCapture")]
    pub node_capture: Option<i32>,
    #[serde(rename = "wardsPlaced")]
    pub wards_placed: i32,
    #[serde(rename = "totalDamageDealt")]
    pub total_damage_dealt: i64,
    #[serde(rename = "timeCCingOthers")]
    pub time_CCing_others: i64,
    #[serde(rename = "magicalDamageTaken")]
    pub magical_damage_taken: i64,
    #[serde(rename = "largestKillingSpree")]
    pub largest_killing_spree: i32,
    #[serde(rename = "totalDamageDealtToChampions")]
    pub total_damage_dealt_to_champions: i64,
    #[serde(rename = "physicalDamageDealtToChampions")]
    pub physical_damage_dealt_to_champions: i64,
    #[serde(rename = "neutralMinionsKilledTeamJungle")]
    pub neutral_minions_killed_team_jungle: i32,
    #[serde(rename = "totalMinionsKilled")]
    pub total_minions_killed: i32,
    #[serde(rename = "firstInhibitorAssist")]
    pub first_inhibitor_assist: Option<bool>,
    #[serde(rename = "visionWardsBoughtInGame")]
    pub vision_wards_bought_in_game: i32,
    #[serde(rename = "objectivePlayerScore")]
    pub objective_player_score: i32,
    pub kills: i32,
    #[serde(rename = "firstTowerAssist")]
    pub first_tower_assist: bool,
    #[serde(rename = "combatPlayerScore")]
    pub combat_player_score: i32,
    #[serde(rename = "inhibitorKills")]
    pub inhibitor_kills: i32,
    #[serde(rename = "turretKills")]
    pub turret_kills: i32,
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
    #[serde(rename = "firstBloodAssist")]
    pub first_blood_assist: bool,
    #[serde(rename = "nodeCaptureAssist")]
    pub node_capture_assist: Option<i32>,
    pub assists: i32,
    #[serde(rename = "teamObjective")]
    pub team_objective: Option<i32>,
    #[serde(rename = "altarsNeutralized")]
    pub altars_neutralized: Option<i32>,
    #[serde(rename = "goldSpent")]
    pub gold_spent: i32,
    #[serde(rename = "damageDealtToTurrets")]
    pub damage_dealt_to_turrets: i64,
    #[serde(rename = "altarsCaptured")]
    pub altars_captured: Option<i32>,
    pub win: bool,
    #[serde(rename = "totalHeal")]
    pub total_heal: i64,
    #[serde(rename = "unrealKills")]
    pub unreal_kills: i32,
    #[serde(rename = "visionScore")]
    pub vision_score: i64,
    #[serde(rename = "physicalDamageDealt")]
    pub physical_damage_dealt: i64,
    #[serde(rename = "firstBloodKill")]
    pub first_blood_kill: bool,
    #[serde(rename = "longestTimeSpentLiving")]
    pub longest_time_spent_living: i32,
    #[serde(rename = "killingSprees")]
    pub killing_sprees: i32,
    #[serde(rename = "sightWardsBoughtInGame")]
    pub sight_wards_bought_in_game: i32,
    #[serde(rename = "trueDamageDealtToChampions")]
    pub true_damage_dealt_to_champions: i64,
    #[serde(rename = "neutralMinionsKilledEnemyJungle")]
    pub neutral_minions_killed_enemy_jungle: i32,
    #[serde(rename = "doubleKills")]
    pub double_kills: i32,
    #[serde(rename = "trueDamageDealt")]
    pub true_damage_dealt: i64,
    #[serde(rename = "quadraKills")]
    pub quadra_kills: i32,
    #[serde(rename = "item4")]
    pub item_4: i32,
    #[serde(rename = "item3")]
    pub item_3: i32,
    #[serde(rename = "item6")]
    pub item_6: i32,
    #[serde(rename = "item5")]
    pub item_5: i32,
    #[serde(rename = "playerScore0")]
    pub playerScore_0: i32,
    #[serde(rename = "playerScore1")]
    pub playerScore_1: i32,
    #[serde(rename = "playerScore2")]
    pub playerScore_2: i32,
    #[serde(rename = "playerScore3")]
    pub playerScore_3: i32,
    #[serde(rename = "playerScore4")]
    pub playerScore_4: i32,
    #[serde(rename = "playerScore5")]
    pub playerScore_5: i32,
    #[serde(rename = "playerScore6")]
    pub playerScore_6: i32,
    #[serde(rename = "playerScore7")]
    pub playerScore_7: i32,
    #[serde(rename = "playerScore8")]
    pub playerScore_8: i32,
    #[serde(rename = "playerScore9")]
    pub playerScore_9: i32,
    #[serde(rename = "perk0")]
    pub perk_0: i32,
    #[serde(rename = "perk0Var1")]
    pub perk_0_var_1: i32,
    #[serde(rename = "perk0Var2")]
    pub perk_0_var_2: i32,
    #[serde(rename = "perk0Var3")]
    pub perk_0_var_3: i32,
    #[serde(rename = "perk1")]
    pub perk_1: i32,
    #[serde(rename = "perk1Var1")]
    pub perk_1_var_1: i32,
    #[serde(rename = "perk1Var2")]
    pub perk_1_var_2: i32,
    #[serde(rename = "perk1Var3")]
    pub perk_1_var_3: i32,
    #[serde(rename = "perk2")]
    pub perk_2: i32,
    #[serde(rename = "perk2Var1")]
    pub perk_2_var_1: i32,
    #[serde(rename = "perk2Var2")]
    pub perk_2_var_2: i32,
    #[serde(rename = "perk2Var3")]
    pub perk_2_var_3: i32,
    #[serde(rename = "perk3")]
    pub perk_3: i32,
    #[serde(rename = "perk3Var1")]
    pub perk_3_var_1: i32,
    #[serde(rename = "perk3Var2")]
    pub perk_3_var_2: i32,
    #[serde(rename = "perk3Var3")]
    pub perk_3_var_3: i32,
    #[serde(rename = "perk4")]
    pub perk_4: i32,
    #[serde(rename = "perk4Var1")]
    pub perk_4_var_1: i32,
    #[serde(rename = "perk4Var2")]
    pub perk_4_var_2: i32,
    #[serde(rename = "perk4Var3")]
    pub perk_4_var_3: i32,
    #[serde(rename = "perk5")]
    pub perk_5: i32,
    #[serde(rename = "perk5Var1")]
    pub perk_5_var_1: i32,
    #[serde(rename = "perk5Var2")]
    pub perk_5_var_2: i32,
    #[serde(rename = "perk5Var3")]
    pub perk_5_var_3: i32,
    #[serde(rename = "perkPrimaryStyle")]
    pub perk_primary_style: i32,
    #[serde(rename = "perkSubStyle")]
    pub perk_sub_style: i32,
    #[serde(rename = "statPerk0")]
    pub stat_perk_0: i32,
    #[serde(rename = "statPerk1")]
    pub stat_perk_1: i32,
    #[serde(rename = "statPerk2")]
    pub stat_perk_2: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ParticipantTimelineDto {
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    #[serde(rename = "csDiffPerMinDeltas")]
    pub cs_diff_per_min_deltas: Map<String, f32>,
    #[serde(rename = "damageTakenPerMinDeltas")]
    pub damage_taken_per_min_deltas: Map<String, f32>,
    pub role: String,
    #[serde(rename = "damageTakenDiffPerMinDeltas")]
    pub damage_taken_diff_per_min_deltas: Map<String, f32>,
    #[serde(rename = "xpPerMinDeltas")]
    pub xp_per_min_deltas: Map<String, f32>,
    #[serde(rename = "xpDiffPerMinDeltas")]
    pub xp_diff_per_min_deltas: Map<String, f32>,
    pub lane: String,
    #[serde(rename = "creepsPerMinDeltas")]
    pub creeps_per_min_deltas: Map<String, f32>,
    #[serde(rename = "goldPerMinDeltas")]
    pub gold_per_min_deltas: Map<String, f32>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RuneDto {
    #[serde(rename = "runeId")]
    pub runeId: i32,
    pub rank: i16
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TeamStatsDto {
    #[serde(rename = "towerKills")]
    pub tower_kills: i16,
    #[serde(rename = "riftHeraldKills")]
    pub rift_herald_kills: i16,
    #[serde(rename = "firstBlood")]
    pub first_blood: bool,
    #[serde(rename = "inhibitorKills")]
    pub inhibitor_kills: i16,
    pub bans: Vec<TeamBansDto>,
    #[serde(rename = "firstBaron")]
    pub first_baron: bool,
    #[serde(rename = "firstDragon")]
    pub first_dragon: bool,
    #[serde(rename = "dominionVictoryScore")]
    pub dominion_victory_score: i16,
    #[serde(rename = "dragonKills")]
    pub dragon_kills: i16,
    #[serde(rename = "baronKills")]
    pub baron_kills: i16,
    #[serde(rename = "firstInhibitor")]
    pub first_inhibitor: bool,
    #[serde(rename = "firstTower")]
    pub first_tower: bool,
    #[serde(rename = "vilemawKills")]
    pub vilemaw_kills: i16,
    #[serde(rename = "firstRiftHerald")]
    pub first_rift_herald: bool,
    #[serde(rename = "teamId")]
    pub team_id: i16,
    pub win: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TeamBansDto {
    #[serde(rename = "championId")]
    pub champion_id: i16,
    #[serde(rename = "pickTurn")]
    pub pick_turn: i8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParticipantIdentityDto {
    #[serde(rename = "participantId")]
    pub participant_id: i64,
    pub player: PlayerDto
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerDto {
    #[serde(rename = "profileIcon")]
    pub profile_icon: i64,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "matchHistoryUri")]
    match_history_uri: String,
    #[serde(rename = "currentAccountId")]
    pub currentAccountId: String,
    #[serde(rename = "currentPlatformId")]
    current_platform_id: String,
    #[serde(rename = "summonerName")]
    summoner_name: String,
    #[serde(rename = "summonerId")]
    summoner_id: String,
    #[serde(rename = "platformId")]
    platform_id: String
}