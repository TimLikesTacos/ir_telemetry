use crate::error::*;
use crate::ir_data_inner::IrDataInner;
use crate::mapped_file::full_value::{SingleVarType, VarType};
use crate::mapped_file::var_header::VarHeader;
use crate::types::IrValue;
/// Contains the latest telemetry data and a convienent hashmap of all the variables.
#[derive(Debug, Clone)]
pub struct IrData {
    data: IrDataInner,
}

impl IrData {
    pub(crate) fn new(data: IrDataInner) -> Self {
        Self { data }
    }

    /// Returns the tick count of the data.
    pub fn tick(&self) -> i32 {
        self.data.tick()
    }

    /// Returns the raw data.
    pub fn data(&self) -> &[u8] {
        self.data.data()
    }

    /// Returns the value.  This requires defining the type of the variable based on the expected value.  
    /// For example, if you want the speed of the car, you would use `get::<f32>(var_header.get("Speed"))`.
    pub fn get<T>(&self, variable: Option<&VarHeader>) -> Option<T>
    where
        T: TryFrom<IrValue>,
        <T as TryFrom<IrValue>>::Error: Into<IrError>,
    {
        if let Some(variable) = variable {
            self.pull_or_none::<T>(variable)
        } else {
            None
        }
    }

    fn pull_or_none<T>(&self, variable: &VarHeader) -> Option<T>
    where
        T: TryFrom<IrValue>,
    {
        match T::try_from(IrValue::from((variable, self.data.data()))) {
            Ok(value) => Some(value),
            Err(_) => {
                log::error!("Error converting value: {:?}", variable.name());
                None
            }
        }
    }

    /// Useful for getting data without specifing a primitive type.  
    /// The return type must be able to handle
    /// the basic primitives, like for example  `serde_json::Value`
    pub fn get_into<T>(&self, variable: Option<&VarHeader>) -> Option<T>
    where
        T: From<i32>
            + From<f32>
            + From<f64>
            + From<bool>
            + From<String>
            + From<Vec<i32>>
            + From<Vec<f32>>
            + From<Vec<bool>>
            + From<Vec<String>>
            + From<Vec<f64>>,
    {
        if let Some(variable) = variable {
            match variable.var_type() {
                VarType::Single(single) => self.get_into_single(variable, single, false),
                VarType::Array(single) => self.get_into_single(variable, single, true),
            }
        } else {
            None
        }
    }

    fn get_into_single<T>(
        &self,
        variable: &VarHeader,
        single_type: SingleVarType,
        is_array: bool,
    ) -> Option<T>
    where
        T: From<i32>
            + From<f32>
            + From<f64>
            + From<bool>
            + From<String>
            + From<Vec<i32>>
            + From<Vec<f32>>
            + From<Vec<bool>>
            + From<Vec<String>>
            + From<Vec<f64>>,
    {
        use crate::types::*;

        match single_type {
            SingleVarType::Char | SingleVarType::Int => {
                if is_array {
                    self.pull_or_none::<Vec<i32>>(variable).map(|v| v.into())
                } else {
                    self.pull_or_none::<i32>(variable).map(|v| v.into())
                }
            }
            SingleVarType::Bool => {
                if is_array {
                    self.pull_or_none::<Vec<bool>>(variable).map(|v| v.into())
                } else {
                    self.pull_or_none::<bool>(variable).map(|v| v.into())
                }
            }
            SingleVarType::Float => {
                if is_array {
                    self.pull_or_none::<Vec<f32>>(variable).map(|v| v.into())
                } else {
                    self.pull_or_none::<f32>(variable).map(|v| v.into())
                }
            }
            SingleVarType::Double => {
                if is_array {
                    self.pull_or_none::<Vec<f64>>(variable).map(|v| v.into())
                } else {
                    self.pull_or_none::<f64>(variable).map(|v| v.into())
                }
            }
            SingleVarType::TrackLocation => {
                if is_array {
                    self.pull_or_none::<Vec<TrackLocation>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<TrackLocation>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::TrackSurface => {
                if is_array {
                    self.pull_or_none::<Vec<TrackSurface>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<TrackSurface>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::SessionState => {
                if is_array {
                    self.pull_or_none::<Vec<SessionState>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<SessionState>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::CarLeftRight => {
                if is_array {
                    self.pull_or_none::<Vec<CarLeftRight>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<CarLeftRight>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::PitStatus => {
                if is_array {
                    self.pull_or_none::<Vec<PitStatus>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<PitStatus>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::PaceMode => {
                if is_array {
                    self.pull_or_none::<Vec<PaceMode>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<PaceMode>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::TrackWetness => {
                if is_array {
                    self.pull_or_none::<Vec<TrackWetness>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<TrackWetness>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::EngineWarnings => {
                if is_array {
                    self.pull_or_none::<Vec<EngineWarnings>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<EngineWarnings>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::Flags => {
                if is_array {
                    self.pull_or_none::<Vec<Flags>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<Flags>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::CameraState => {
                if is_array {
                    self.pull_or_none::<Vec<CameraState>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<CameraState>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::PitServiceFlags => {
                if is_array {
                    self.pull_or_none::<Vec<PitServiceFlags>>(variable)
                        .map(|v| {
                            v.into_iter()
                                .map(|l| l.to_string())
                                .collect::<Vec<_>>()
                                .into()
                        })
                } else {
                    self.pull_or_none::<PitServiceFlags>(variable)
                        .map(|v| v.to_string().into())
                }
            }
            SingleVarType::PaceFlags => {
                if is_array {
                    self.pull_or_none::<Vec<PaceFlags>>(variable).map(|v| {
                        v.into_iter()
                            .map(|l| l.to_string())
                            .collect::<Vec<_>>()
                            .into()
                    })
                } else {
                    self.pull_or_none::<PaceFlags>(variable)
                        .map(|v| v.to_string().into())
                }
            }
        }
    }
}
