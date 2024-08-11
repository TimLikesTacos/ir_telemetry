use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CameraInfo {
    pub groups: Vec<CameraGroup>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CameraGroup {
    pub group_name: String,
    pub group_num: i32,
    pub is_scenic: bool,
    pub cameras: Vec<CameraData>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CameraData {
    pub camera_name: String,
    pub camera_num: i32,
}
