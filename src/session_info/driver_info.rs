use serde::{Deserialize, Serialize};

use super::values::percent::Percent;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct DriverInfo {
    pub driver_car_idx: i32,
    pub driver_head_pos_x: f32,
    pub driver_head_pos_y: f32,
    pub driver_head_pos_z: f32,

    #[serde(rename = "DriverCarSLFirstRPM")]
    pub driver_car_idle_rpm: f32,
    pub driver_car_red_line: f32,
    pub driver_car_fuel_kg_per_ltr: f32,
    pub driver_car_fuel_max_ltr: f32,
    pub driver_car_max_fuel_pct: f32,

    #[serde(rename = "DriverCarSLFirstRPM")]
    pub driver_car_sl_first_rpm: f32,

    #[serde(rename = "DriverCarSLLastRPM")]
    pub driver_car_sl_last_rpm: f32,

    #[serde(rename = "DriverCarSLShiftRPM")]
    pub driver_car_sl_shift_rpm: f32,

    #[serde(rename = "DriverCarSLBlinkRPM")]
    pub driver_car_sl_blink_rpm: f32,

    pub driver_pit_trk_pct: f32,
    pub driver_car_est_lap_time: f32,
    pub driver_setup_name: String,
    pub driver_setup_is_modified: i32,
    pub driver_setup_load_type_name: String,
    pub driver_setup_passed_tech: i32,
    #[serde(deserialize_with = "driver_data_vec")]
    pub drivers: Vec<DriverData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct DriverData {
    pub car_idx: i32,
    pub user_name: String,
    pub abbrev_name: String,
    pub initials: String,

    #[serde(rename = "UserID")]
    pub user_id: i32,

    #[serde(rename = "TeamID")]
    pub team_id: i32,
    pub car_number: String,
    pub car_number_raw: i32,
    pub car_path: String,
    pub car_class_id: i32,

    #[serde(rename = "CarID")]
    pub car_id: i32,
    pub car_screen_name: String,
    pub car_screen_name_short: String,
    pub car_class_short_name: String,
    pub car_class_rel_speed: i32,
    pub car_class_license_level: i32,
    pub car_class_max_fuel_pct: Percent,
    pub car_class_weight_pentalty: f32,
    pub car_class_color: String,

    #[serde(rename = "IRating")]
    pub irating: i32,
    pub lic_level: i32,
    pub lic_sub_level: i32,
    pub lic_string: String,
    pub lic_color: String,
    pub is_spectator: i32,
    // TODO: deser these into hex color values
    pub car_design_str: String,
    pub helmet_design_str: String,
    pub suit_design_str: String,
    pub car_number_design_str: String,
    pub car_sponsor_1: i32,
    pub car_sponsor_2: i32,
    pub club_name: String,
    pub division_name: String,
}

fn driver_data_vec<'de, D>(deserializer: D) -> Result<Vec<DriverData>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Not sure if the data coming accross is sorted by this at first, but we'll do it and then you can access the vec by car idx.
    let mut v = Vec::<DriverData>::deserialize(deserializer)?;
    v.sort_by_key(|d| d.car_idx);
    Ok(v)
}
