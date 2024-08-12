use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct CameraInfo {
    pub groups: Vec<CameraGroup>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct CameraGroup {
    pub group_name: String,
    pub group_num: i32,
    pub is_scenic: Option<bool>,
    pub cameras: Vec<CameraData>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct CameraData {
    pub camera_name: String,
    pub camera_num: i32,
}
