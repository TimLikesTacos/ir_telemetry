use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct SessionInfo {
    pub num_sessions: i32,
    pub sessions: Vec<SessionData>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct SessionData {
    pub session_num: i32,
    pub session_laps: String,
    pub session_time: String, // sec
    pub session_num_laps_to_avg: i32,
    pub session_type: String,
    pub session_track_rubber_state: String,
    pub results_positions: Vec<ResultsPosition>,
    pub results_fastest_lap: Vec<ResultsFastestLap>,
    pub results_average_lap_time: f32,
    pub results_num_caution_flags: i32,
    pub results_num_caution_laps: i32,
    pub results_num_lead_changes: i32,
    pub results_laps_complete: i32,
    pub results_official: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ResultsPosition {
    pub position: i32,
    pub class_position: i32,
    pub car_idx: i32,
    pub lap: i32,
    pub time: f32,
    pub fastest_lap: f32,
    pub last_time: f32,
    pub laps_led: i32,
    pub laps_complete: i32,
    pub laps_driven: f32,
    pub incidents: i32,
    pub reason_out_id: i32,
    pub reason_out_str: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct ResultsFastestLap {
    pub car_idx: i32,
    pub fastest_lap: i32,
    pub fastest_time: f32,
}
