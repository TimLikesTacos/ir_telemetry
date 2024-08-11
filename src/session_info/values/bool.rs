use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize};

/// Converts 1s or 0s to boolean values
#[derive(Debug, Clone, Default)]
pub struct InfoBool(bool);

impl Deref for InfoBool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for InfoBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de> Deserialize<'de> for InfoBool {
    fn deserialize<D>(deserializer: D) -> Result<InfoBool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(InfoBool(false)),
            1 => Ok(InfoBool(true)),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid value for bool: {}",
                value
            ))),
        }
    }
}

impl Serialize for InfoBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_u8(if self.0 { 1 } else { 0 })
    }
}

impl std::fmt::Display for InfoBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
