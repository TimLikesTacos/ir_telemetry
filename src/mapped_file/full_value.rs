use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize)]
pub enum SingleVarType {
    Char,
    Bool,
    Int,
    Float,
    Double,
    TrackLocation,
    TrackSurface,
    SessionState,
    CarLeftRight,
    PitStatus,
    PaceMode,
    TrackWetness,
    EngineWarnings,
    Flags,
    CameraState,
    PitServiceFlags,
    PaceFlags,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum VarType {
    Single(SingleVarType),
    Array(SingleVarType),
}
