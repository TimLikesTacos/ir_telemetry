use crate::mapped_file::full_value::VarType;
use crate::mapped_file::values::DataVarType;
use crate::utils::byte_array_to_rust_string;

use super::full_value::SingleVarType;

const MAX_STRING: usize = 32;
const MAX_DESC: usize = 64;

pub type VarHeaderData = VarHeaderGeneric<[u8; MAX_STRING], [u8; MAX_DESC]>;
pub type VarHeader = VarHeaderGeneric<String, String>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VarHeaderGeneric<S, D> {
    pub(crate) _type: DataVarType, // irsdk_VarType
    pub(crate) offset: i32,        // offset fron start of buffer row
    pub(crate) count: i32,         // number of entrys (array)
    // so length in bytes would be irsdk_VarTypeBytes[type] * count
    pub(crate) count_as_time: u8,
    pad: [u8; 3], // (16 byte align)

    pub(crate) name: S,
    pub(crate) desc: D,
    pub(crate) unit: S, // something like "kg/m^2"
}

impl VarHeader {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.desc.as_str()
    }

    pub fn units(&self) -> &str {
        self.unit.as_str()
    }

    pub fn var_type(&self) -> VarType {
        let is_array = self.count > 1;
        let single = match self._type {
            DataVarType::Char => SingleVarType::Char,
            DataVarType::Bool => SingleVarType::Bool,
            DataVarType::Int => {
                match self.name.as_str() {
                    // TODO These seem backwards, but it's whats in in the pdf
                    "CarIdxTrackSurface" => SingleVarType::TrackLocation,
                    "SessionState" => SingleVarType::SessionState,
                    "CarLeftRight" => SingleVarType::CarLeftRight,
                    // TODO : verify these? Can't find it in the pdf
                    "PitSvStatus" => SingleVarType::PitStatus,
                    "PaceMode" => SingleVarType::PaceMode,
                    "TrackWetness" => SingleVarType::TrackWetness,
                    _ => SingleVarType::Int,
                }
            }
            DataVarType::Float => SingleVarType::Float,
            DataVarType::Double => SingleVarType::Double,
            DataVarType::BitField => match self.name.as_str() {
                "EngineWarnings" => SingleVarType::EngineWarnings,
                "PitSvFlags" => SingleVarType::PitServiceFlags,
                "CarIdxSessionFlags" | "SessionFlags" => SingleVarType::Flags,
                "CamCameraState" => SingleVarType::CameraState,
                "CarIdxPaceFlags" => SingleVarType::PaceFlags,
                _ => {
                    log::error!("Unknown bitfield: {}, using an integer", self.name);
                    SingleVarType::Int
                }
            },
            DataVarType::ETCount => {
                log::error!("Not used type (ETCount): {}, using a float", self.name);
                SingleVarType::Float
            }
        };
        if is_array {
            VarType::Array(single)
        } else {
            VarType::Single(single)
        }
    }
}

impl From<VarHeaderData> for VarHeader {
    fn from(header: VarHeaderData) -> Self {
        Self {
            _type: header._type,
            offset: header.offset,
            count: header.count,
            count_as_time: header.count_as_time,
            pad: header.pad,
            name: byte_array_to_rust_string(&header.name),
            desc: byte_array_to_rust_string(&header.desc),
            unit: byte_array_to_rust_string(&header.unit),
        }
    }
}

impl From<&VarHeaderData> for VarHeader {
    fn from(header: &VarHeaderData) -> Self {
        Self {
            _type: header._type,
            offset: header.offset,
            count: header.count,
            count_as_time: header.count_as_time,
            pad: header.pad,
            name: byte_array_to_rust_string(&header.name),
            desc: byte_array_to_rust_string(&header.desc),
            unit: byte_array_to_rust_string(&header.unit),
        }
    }
}

impl From<&VarHeaderData> for VarHeaderData {
    fn from(header: &VarHeaderData) -> Self {
        Self {
            _type: header._type,
            offset: header.offset,
            count: header.count,
            count_as_time: header.count_as_time,
            pad: header.pad,
            name: header.name,
            desc: header.desc,
            unit: header.unit,
        }
    }
}
