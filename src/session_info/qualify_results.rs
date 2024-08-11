use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QualifyResults {
    results: Vec<QualData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QualData {
    position: i32,
    class_position: i32,
    car_idx: i32,
    fastest_lap: i32,
    fastest_time: f32,
}
