use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioInfo {
    pub selected_radio_num: i32,
    pub radios: Vec<RadioData>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioData {
    pub radio_num: i32,
    pub hop_count: i32,
    pub num_frequencies: i32,
    pub tuned_to_frequency_num: i32,
    pub scanning_is_on: i32,
    pub frequencies: Vec<RadioFrequencyData>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioFrequencyData {
    pub frequency_num: i32,
    pub frequency_name: String,
    pub priority: i32,
    pub entry_idx: i32,
    pub club_id: i32,
    pub can_scan: i32,
    pub can_squawk: i32,
    pub muted: i32,
    pub is_mutable: i32,
    pub is_deletable: i32,
}
