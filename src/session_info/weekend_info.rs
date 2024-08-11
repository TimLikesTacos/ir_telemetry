use crate::session_info::values::measurement::{MeasurementF32, MeasurementI32};
use serde::{Deserialize, Serialize};

use super::values::bool::InfoBool;
use super::values::percent::Percent;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct WeekendInfo {
    pub track_name: String,
    #[serde(rename = "TrackID")]
    pub track_id: i32,
    pub track_length: MeasurementF32, // docs say f32, //km
    pub track_display_name: String,
    pub track_config_name: String,
    pub track_city: String,
    pub track_country: String,
    pub track_altitude: MeasurementF32, //f32, //meters
    pub track_latitude: MeasurementF32,
    pub track_longitude: MeasurementF32, //f32,
    pub track_north_offset: MeasurementF32,
    pub track_num_turns: MeasurementI32,
    pub track_pit_speed_limit: MeasurementF32, //kph
    pub track_type: String,
    pub track_weather_type: String,
    pub track_skies: String,
    pub track_surface_temp: MeasurementF32, //celsius
    pub track_air_temp: MeasurementF32,     //celsius
    pub track_air_pressure: MeasurementF32, //Hg
    pub track_wind_vel: MeasurementF32,     //m/s
    pub track_wind_dir: MeasurementF32,     //rad
    pub track_relative_humidity: Percent,   //%
    pub track_fog_level: Percent,           // %
    pub track_cleanup: MeasurementI32,
    pub track_dynamic_track: InfoBool,
    pub series_id: i32,
    pub season_id: i32,
    pub session_id: i32,
    pub sub_session_id: i32,
    pub league_id: i32,
    pub official: InfoBool,
    pub race_week: i32,
    pub event_type: String,
    pub category: String,
    pub sim_mode: String,
    pub team_racing: i32,
    pub min_drivers: i32,
    pub max_drivers: i32,

    #[serde(rename = "DCRuleSet")]
    pub dc_rule_set: String,
    pub qualifier_must_start_race: i32,
    pub num_car_classes: i32,
    pub num_car_types: i32,
    pub weekend_options: WeekendOptions,
    pub telemetry_options: TelemetryOptions,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeekendOptions {
    pub num_starters: i32,
    pub starting_grid: String,
    pub qualifying_scoring: Option<String>,
    pub course_cautions: String,
    pub standing_start: i32,
    pub restarts: String,
    pub weather_type: String,
    pub skies: String,
    pub wind_direction: String,
    pub wind_speed: MeasurementF32, //km/h
    pub relative_humidity: Percent, // %
    pub fog_level: Percent,         //%
    pub unofficial: InfoBool,
    pub commercial_mode: String,
    pub night_mode: String,
    pub is_fixed_setup: InfoBool,
    pub strict_laps_checking: String,
    pub has_open_registration: InfoBool,
    pub hardcore_level: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TelemetryOptions {
    telemetry_disk_file: String,
}
