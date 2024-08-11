use crate::session_info::{
    camera_info::CameraInfo, driver_info::DriverInfo, qualify_results::QualifyResults,
    radio_info::RadioInfo, session_info::SessionInfo, split_time_info::SplitTimeInfo,
    weekend_info::WeekendInfo,
};
use serde::Deserialize;
use serde::Serialize;

/// A struct representing the session data.  This serves as a convienient way to access most of the
/// session data in one place.  Since the session data structure can change / be updated, it is
/// recommended to use a custom way to access the data from the yaml string for full functionality.
/// All members are public and the are member functions associated with this struct.  It is purely for
/// deserialization and getting the data.  
/// This data is this struct also implements `serde::Serialize`.  Some of the raw data is numeric in nature
/// but is stores as a string with it's unit of measurement.  Use the feature `value_only_measurement` to
/// get only the value and not the unit of measurement.  This is useful in that numbers will be serialized
/// as numbers and not strings.
#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct Session {
    pub weekend_info: WeekendInfo,
    pub session_info: SessionInfo,
    pub qualify_results_info: Option<QualifyResults>,
    pub camera_info: CameraInfo,
    pub radio_info: RadioInfo,
    pub driver_info: DriverInfo,
    pub split_time_info: SplitTimeInfo,
}
