use bitflags::bitflags;
use num::{FromPrimitive, ToPrimitive};
use num_derive::FromPrimitive;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    Char(u8),
    Bool(bool),
    Int(i32),
    BitField(i32),
    Float(f32),
    Double(f64),
    ETCount,
    Array(Vec<IrValue>),
}

impl TryFrom<IrValue> for u8 {
    type Error = crate::error::IrError;

    fn try_from(value: IrValue) -> Result<Self, Self::Error> {
        match value {
            IrValue::Char(value) => Ok(value),
            _ => Err(crate::error::IrError::TypeError),
        }
    }
}

impl TryFrom<IrValue> for i32 {
    type Error = crate::error::IrError;

    fn try_from(value: IrValue) -> Result<Self, Self::Error> {
        match value {
            IrValue::Char(value) => Ok(value as i32),
            IrValue::Int(value) => Ok(value),
            IrValue::Float(value) => {
                if value > i32::MAX as f32 || value < i32::MIN as f32 {
                    Err(crate::error::IrError::ExceedsVariableRange)
                } else {
                    Ok(value.round() as i32)
                }
            }
            IrValue::Double(value) => {
                if value > i32::MAX as f64 || value < i32::MIN as f64 {
                    Err(crate::error::IrError::ExceedsVariableRange)
                } else {
                    Ok(value.round() as i32)
                }
            }
            IrValue::BitField(value) => Ok(value),
            _ => Err(crate::error::IrError::TypeError),
        }
    }
}

impl TryFrom<IrValue> for isize {
    type Error = crate::error::IrError;

    fn try_from(value: IrValue) -> Result<Self, Self::Error> {
        <i32>::try_from(value).map(|v| v as isize)
    }
}

impl TryFrom<IrValue> for i64 {
    type Error = crate::error::IrError;

    fn try_from(value: IrValue) -> Result<Self, Self::Error> {
        <i32>::try_from(value).map(|v| v as i64)
    }
}

impl TryFrom<IrValue> for f32 {
    type Error = crate::error::IrError;

    fn try_from(value: IrValue) -> Result<Self, Self::Error> {
        match value {
            IrValue::Int(value) => value
                .to_f32()
                .ok_or(crate::error::IrError::ExceedsVariableRange),
            IrValue::Float(value) => Ok(value),
            _ => Err(crate::error::IrError::TypeError),
        }
    }
}

impl TryFrom<IrValue> for f64 {
    type Error = crate::error::IrError;

    fn try_from(value: IrValue) -> Result<Self, Self::Error> {
        match value {
            IrValue::Int(value) => value
                .to_f64()
                .ok_or(crate::error::IrError::ExceedsVariableRange),
            IrValue::Float(value) => Ok(value as f64),
            IrValue::Double(value) => Ok(value),
            _ => Err(crate::error::IrError::TypeError),
        }
    }
}

impl TryFrom<IrValue> for bool {
    type Error = crate::error::IrError;

    fn try_from(value: IrValue) -> Result<Self, Self::Error> {
        match value {
            IrValue::Bool(value) => Ok(value),
            _ => Err(crate::error::IrError::TypeError),
        }
    }
}

impl<T> TryFrom<IrValue> for Vec<T>
where
    T: TryFrom<IrValue>,
{
    type Error = crate::error::IrError;

    fn try_from(value: IrValue) -> Result<Self, Self::Error> {
        match value {
            IrValue::Array(value) => Ok(value
                .into_iter()
                .filter_map(|x| T::try_from(x).ok())
                .collect::<Vec<T>>()),
            _ => Err(crate::error::IrError::TypeError),
        }
    }
}

macro_rules! impl_tryfrom_ir_value_enum {
    ($type:ty) => {
        impl TryFrom<IrValue> for $type {
            type Error = crate::error::IrError;

            fn try_from(value: IrValue) -> Result<Self, Self::Error> {
                match value {
                    IrValue::Int(v) => <$type>::from_i32(v).ok_or(crate::error::IrError::TypeError),
                    IrValue::BitField(v) => {
                        <$type>::from_i32(v).ok_or(crate::error::IrError::TypeError)
                    }
                    _ => Err(crate::error::IrError::TypeError),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize)]
pub enum TrackLocation {
    NotInWorld = -1,
    OffTrack = 0,
    InPitStall,
    AproachingPits,
    OnTrack,
}

impl_tryfrom_ir_value_enum!(TrackLocation);

impl Display for TrackLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = match self {
            TrackLocation::NotInWorld => "NotInWorld",
            TrackLocation::OffTrack => "OffTrack",
            TrackLocation::InPitStall => "InPitStall",
            TrackLocation::AproachingPits => "AproachingPits",
            TrackLocation::OnTrack => "OnTrack",
        };
        write!(f, "{}", location)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize)]
pub enum TrackSurface {
    SurfaceNotInWorld = -1,
    UndefinedMaterial = 0,

    Asphalt1Material,
    Asphalt2Material,
    Asphalt3Material,
    Asphalt4Material,
    Concrete1Material,
    Concrete2Material,
    RacingDirt1Material,
    RacingDirt2Material,
    Paint1Material,
    Paint2Material,
    Rumble1Material,
    Rumble2Material,
    Rumble3Material,
    Rumble4Material,
    Grass1Material,
    Grass2Material,
    Grass3Material,
    Grass4Material,
    Dirt1Material,
    Dirt2Material,
    Dirt3Material,
    Dirt4Material,
    SandMaterial,
    Gravel1Material,
    Gravel2Material,
    GrasscreteMaterial,
    AstroturfMaterial,
}

impl_tryfrom_ir_value_enum!(TrackSurface);

impl Display for TrackSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let surface = match self {
            TrackSurface::SurfaceNotInWorld => "SurfaceNotInWorld",
            TrackSurface::UndefinedMaterial => "UndefinedMaterial",
            TrackSurface::Asphalt1Material => "Asphalt1Material",
            TrackSurface::Asphalt2Material => "Asphalt2Material",
            TrackSurface::Asphalt3Material => "Asphalt3Material",
            TrackSurface::Asphalt4Material => "Asphalt4Material",
            TrackSurface::Concrete1Material => "Concrete1Material",
            TrackSurface::Concrete2Material => "Concrete2Material",
            TrackSurface::RacingDirt1Material => "RacingDirt1Material",
            TrackSurface::RacingDirt2Material => "RacingDirt2Material",
            TrackSurface::Paint1Material => "Paint1Material",
            TrackSurface::Paint2Material => "Paint2Material",
            TrackSurface::Rumble1Material => "Rumble1Material",
            TrackSurface::Rumble2Material => "Rumble2Material",
            TrackSurface::Rumble3Material => "Rumble3Material",
            TrackSurface::Rumble4Material => "Rumble4Material",
            TrackSurface::Grass1Material => "Grass1Material",
            TrackSurface::Grass2Material => "Grass2Material",
            TrackSurface::Grass3Material => "Grass3Material",
            TrackSurface::Grass4Material => "Grass4Material",
            TrackSurface::Dirt1Material => "Dirt1Material",
            TrackSurface::Dirt2Material => "Dirt2Material",
            TrackSurface::Dirt3Material => "Dirt3Material",
            TrackSurface::Dirt4Material => "Dirt4Material",
            TrackSurface::SandMaterial => "SandMaterial",
            TrackSurface::Gravel1Material => "Gravel1Material",
            TrackSurface::Gravel2Material => "Gravel2Material",
            TrackSurface::GrasscreteMaterial => "GrasscreteMaterial",
            TrackSurface::AstroturfMaterial => "AstroturfMaterial",
        };
        write!(f, "{}", surface)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize)]
pub enum SessionState {
    Invalid = 0,
    GetInCar,
    Warmup,
    ParadeLaps,
    Racing,
    Checkered,
    CoolDown,
}

impl_tryfrom_ir_value_enum!(SessionState);

impl Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            SessionState::Invalid => "Invalid",
            SessionState::GetInCar => "GetInCar",
            SessionState::Warmup => "Warmup",
            SessionState::ParadeLaps => "ParadeLaps",
            SessionState::Racing => "Racing",
            SessionState::Checkered => "Checkered",
            SessionState::CoolDown => "CoolDown",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize)]
pub enum CarLeftRight {
    Off = 0,
    Clear,    // no cars around us.
    CarLeft,  // there is a car to our left.
    CarRight, // there is a car to our right.
    #[allow(clippy::enum_variant_names)]
    CarLeftRight, // there are cars on each side.
    TwoCarsLeft, // there are two cars to our left.
    TwoCarsRight, // there are two cars to our right.
}

impl_tryfrom_ir_value_enum!(CarLeftRight);

impl Display for CarLeftRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            CarLeftRight::Off => "Off",
            CarLeftRight::Clear => "Clear",
            CarLeftRight::CarLeft => "CarLeft",
            CarLeftRight::CarRight => "CarRight",
            CarLeftRight::CarLeftRight => "CarLeftRight",
            CarLeftRight::TwoCarsLeft => "TwoCarsLeft",
            CarLeftRight::TwoCarsRight => "TwoCarsRight",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize)]
pub enum PitStatus {
    // status
    #[serde(rename = "None")]
    NoStatus = 0, // renamed from None from SDK to avoid confusion with Option
    InProgress,
    Complete,

    // errors
    TooFarLeft = 100,
    TooFarRight,
    TooFarForward,
    TooFarBack,
    BadAngle,
    CantFixThat,
}

impl_tryfrom_ir_value_enum!(PitStatus);

impl Display for PitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            PitStatus::NoStatus => "NoStatus",
            PitStatus::InProgress => "InProgress",
            PitStatus::Complete => "Complete",
            PitStatus::TooFarLeft => "TooFarLeft",
            PitStatus::TooFarRight => "TooFarRight",
            PitStatus::TooFarForward => "TooFarForward",
            PitStatus::TooFarBack => "TooFarBack",
            PitStatus::BadAngle => "BadAngle",
            PitStatus::CantFixThat => "CantFixThat",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize)]
pub enum PaceMode {
    SingleFileStart = 0,
    DoubleFileStart,
    SingleFileRestart,
    DoubleFileRestart,
    NotPacing,
}

impl_tryfrom_ir_value_enum!(PaceMode);

impl Display for PaceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            PaceMode::SingleFileStart => "SingleFileStart",
            PaceMode::DoubleFileStart => "DoubleFileStart",
            PaceMode::SingleFileRestart => "SingleFileRestart",
            PaceMode::DoubleFileRestart => "DoubleFileRestart",
            PaceMode::NotPacing => "NotPacing",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize)]
pub enum TrackWetness {
    Unknown = 0,
    Dry,
    MostlyDry,
    VeryLightlyWet,
    LightlyWet,
    ModeratelyWet,
    VeryWet,
    ExtremelyWet,
}

impl_tryfrom_ir_value_enum!(TrackWetness);

impl Display for TrackWetness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            TrackWetness::Unknown => "Unknown",
            TrackWetness::Dry => "Dry",
            TrackWetness::MostlyDry => "MostlyDry",
            TrackWetness::VeryLightlyWet => "VeryLightlyWet",
            TrackWetness::LightlyWet => "LightlyWet",
            TrackWetness::ModeratelyWet => "ModeratelyWet",
            TrackWetness::VeryWet => "VeryWet",
            TrackWetness::ExtremelyWet => "ExtremelyWet",
        };
        write!(f, "{}", state)
    }
}

macro_rules! impl_tryfrom_ir_value_bitflags {
    ($type:ty) => {
        impl TryFrom<IrValue> for $type {
            type Error = crate::error::IrError;

            fn try_from(value: IrValue) -> Result<Self, Self::Error> {
                match value {
                    IrValue::BitField(v) => Ok(Self::from_bits_truncate(v as u32)),
                    IrValue::Int(v) => Ok(Self::from_bits_truncate(v as u32)),
                    _ => Err(crate::error::IrError::TypeError),
                }
            }
        }
    };
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Serialize)]
    pub struct EngineWarnings: u32 {
        const WATER_TEMP	     = 0x0001;
        const FUEL_PRESSURE	     = 0x0002;
        const OIL_PRESSURE	     = 0x0004;
        const ENGINE_STALLED	 = 0x0008;
        const PIT_SPEED_LIMITER	 = 0x0010;
        const REV_LIMITER_ACTIVE = 0x0020;
        const OIL_TEMPERATURE	 = 0x0040;
    }
}

impl_tryfrom_ir_value_bitflags!(EngineWarnings);

impl Display for EngineWarnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut warnings = Vec::new();
        if self.contains(Self::WATER_TEMP) {
            warnings.push("Water Temp");
        }
        if self.contains(Self::FUEL_PRESSURE) {
            warnings.push("Fuel Pressure");
        }
        if self.contains(Self::OIL_PRESSURE) {
            warnings.push("Oil Pressure");
        }
        if self.contains(Self::ENGINE_STALLED) {
            warnings.push("Engine Stalled");
        }
        if self.contains(Self::PIT_SPEED_LIMITER) {
            warnings.push("Pit Speed Limiter");
        }
        if self.contains(Self::REV_LIMITER_ACTIVE) {
            warnings.push("Rev Limiter Active");
        }
        if self.contains(Self::OIL_TEMPERATURE) {
            warnings.push("Oil Temperature");
        }
        write!(f, "{}", warnings.join(", "))
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Serialize)]
    pub struct Flags: u32 {
        const CHECKERED_FLAG	 = 0x0001;
        const WHITE_FLAG		 = 0x0002;
        const GREEN_FLAG		 = 0x0004;
        const YELLOW_FLAG		 = 0x0008;
        const RED_FLAG			 = 0x0010;
        const BLUE_FLAG			 = 0x0020;
        const DEBRIS			 = 0x0040;
        const CROSSED			 = 0x0080;
        const YELLOW_WAVING	     = 0x0100;
        const ONE_LAP_TO_GREEN   = 0x0200;
        const GREEN_HELD         = 0x0400;
        const TEN_TO_GO          = 0x0800;
        const FIVE_TO_GO         = 0x1000;
        const RANDOM_WAVING      = 0x2000;
        const CAUTION            = 0x4000;
        const CAUTION_WAVING     = 0x8000;

        // Driver's black flags
        const BLACK			     = 0x10000;
        const DISQUALIFY	     = 0x20000;
        const SERVICIBLE    	 = 0x40000;
        const FURLED		     = 0x80000;
        const REPAIR		     = 0x100000;

        //Start lights
        const START_HIDDEN      = 0x10000000;
        const START_READY       = 0x20000000;
        const START_SET         = 0x40000000;
        const START_GO          = 0x80000000;
    }
}

impl_tryfrom_ir_value_bitflags!(Flags);

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut flags = Vec::new();
        if self.contains(Self::CHECKERED_FLAG) {
            flags.push("Checkered Flag");
        }
        if self.contains(Self::WHITE_FLAG) {
            flags.push("White Flag");
        }
        if self.contains(Self::GREEN_FLAG) {
            flags.push("Green Flag");
        }
        if self.contains(Self::YELLOW_FLAG) {
            flags.push("Yellow Flag");
        }
        if self.contains(Self::RED_FLAG) {
            flags.push("Red Flag");
        }
        if self.contains(Self::BLUE_FLAG) {
            flags.push("Blue Flag");
        }
        if self.contains(Self::DEBRIS) {
            flags.push("Debris");
        }
        if self.contains(Self::CROSSED) {
            flags.push("Crossed");
        }
        if self.contains(Self::YELLOW_WAVING) {
            flags.push("Yellow Waving");
        }
        if self.contains(Self::ONE_LAP_TO_GREEN) {
            flags.push("One Lap To Green");
        }
        if self.contains(Self::GREEN_HELD) {
            flags.push("Green Held");
        }
        if self.contains(Self::TEN_TO_GO) {
            flags.push("Ten To Go");
        }
        if self.contains(Self::FIVE_TO_GO) {
            flags.push("Five To Go");
        }
        if self.contains(Self::RANDOM_WAVING) {
            flags.push("Random Waving");
        }
        if self.contains(Self::CAUTION) {
            flags.push("Caution");
        }
        if self.contains(Self::CAUTION_WAVING) {
            flags.push("Caution Waving");
        }
        if self.contains(Self::BLACK) {
            flags.push("Black");
        }
        if self.contains(Self::DISQUALIFY) {
            flags.push("Disqualify");
        }
        if self.contains(Self::SERVICIBLE) {
            flags.push("Servicible");
        }
        if self.contains(Self::FURLED) {
            flags.push("Furled");
        }
        if self.contains(Self::REPAIR) {
            flags.push("Repair");
        }
        if self.contains(Self::START_HIDDEN) {
            flags.push("Start Hidden");
        }
        if self.contains(Self::START_READY) {
            flags.push("Start Ready");
        }
        if self.contains(Self::START_SET) {
            flags.push("Start Set");
        }
        if self.contains(Self::START_GO) {
            flags.push("Start Go");
        }
        write!(f, "{}", flags.join(", "))
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Serialize)]
    pub struct CameraState: u32 {
        const IS_SESSION_SCREEN	 = 0x0001;
        const IS_SCENIC_ACTIVE     = 0x0002;

        // Below can be changed with a broadcast message
        const CAM_TOOL_ACTIVE	         = 0x0004;
        const UI_HIDDEN		             = 0x0008;
        const USE_AUTO_SHOT_SELECT	     = 0x0010;
        const USE_TEMPORARY_EDITS         = 0x0020;
        const USE_KEY_ACCELERATION	     = 0x0040;
        const USE_KEY_10X_ACCELERATION	 = 0x0080;
        const USE_MOUSE_AIM_MODE	     = 0x0100;
    }
}

impl_tryfrom_ir_value_bitflags!(CameraState);

impl Display for CameraState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut states = Vec::new();
        if self.contains(Self::IS_SESSION_SCREEN) {
            states.push("Session Screen");
        }
        if self.contains(Self::IS_SCENIC_ACTIVE) {
            states.push("Scenic Active");
        }
        if self.contains(Self::CAM_TOOL_ACTIVE) {
            states.push("Cam Tool Active");
        }
        if self.contains(Self::UI_HIDDEN) {
            states.push("UI Hidden");
        }
        if self.contains(Self::USE_AUTO_SHOT_SELECT) {
            states.push("Use Auto Shot Select");
        }
        if self.contains(Self::USE_TEMPORARY_EDITS) {
            states.push("Use Temporary Edits");
        }
        if self.contains(Self::USE_KEY_ACCELERATION) {
            states.push("Use Key Acceleration");
        }
        if self.contains(Self::USE_KEY_10X_ACCELERATION) {
            states.push("Use Key 10X Acceleration");
        }
        if self.contains(Self::USE_MOUSE_AIM_MODE) {
            states.push("Use Mouse Aim Mode");
        }
        write!(f, "{}", states.join(", "))
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Serialize)]
    pub struct PitServiceFlags: u32 {
        const LF_TIRE_CHANGE     = 0x0001;
        const RF_TIRE_CHANGE     = 0x0002;
        const LR_TIRE_CHANGE     = 0x0004;
        const RR_TIRE_CHANGE     = 0x0008;
        const FUEL_FILL          = 0x0010;
        const WINDSHIELD_TEAROFF = 0x0020;
        const FAST_REPAIR        = 0x0040;
    }
}

impl_tryfrom_ir_value_bitflags!(PitServiceFlags);

impl Display for PitServiceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut services = Vec::new();
        if self.contains(Self::LF_TIRE_CHANGE) {
            services.push("LF Tire Change");
        }
        if self.contains(Self::RF_TIRE_CHANGE) {
            services.push("RF Tire Change");
        }
        if self.contains(Self::LR_TIRE_CHANGE) {
            services.push("LR Tire Change");
        }
        if self.contains(Self::RR_TIRE_CHANGE) {
            services.push("RR Tire Change");
        }
        if self.contains(Self::FUEL_FILL) {
            services.push("Fuel Fill");
        }
        if self.contains(Self::WINDSHIELD_TEAROFF) {
            services.push("Windshield Tearoff");
        }
        if self.contains(Self::FAST_REPAIR) {
            services.push("Fast Repair");
        }
        write!(f, "{}", services.join(", "))
    }
}
bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Serialize)]
    pub struct PaceFlags: u32 {
        const END_OF_LINE = 0x0001;
        const FREE_PASS   = 0x0002;
        const WAVE_AROUND = 0x0004;
         }
}

impl_tryfrom_ir_value_bitflags!(PaceFlags);

impl Display for PaceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut flags = Vec::new();
        if self.contains(Self::END_OF_LINE) {
            flags.push("End Of Line");
        }
        if self.contains(Self::FREE_PASS) {
            flags.push("Free Pass");
        }
        if self.contains(Self::WAVE_AROUND) {
            flags.push("Wave Around");
        }
        write!(f, "{}", flags.join(", "))
    }
}
