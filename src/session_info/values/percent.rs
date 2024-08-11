use serde::{Deserialize, Deserializer, Serialize};

/// Used when session data is a string with a percent value.
/// Use the feature `value_only_measurement` to only serialize the value.  This is useful
/// in that numbers will be serialized as numbers and not strings.
#[derive(Debug, Clone, Default)]
pub struct Percent(pub f32);

impl<'de> Deserialize<'de> for Percent {
    fn deserialize<D>(deserializer: D) -> Result<Percent, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        let value = str
            .split_once('%')
            .and_then(|(v, _)| {
                let v = v.trim();
                v.parse::<f32>()
                    .map_err(|_| (v.to_owned() + ".").parse::<f32>())
                    .ok()
            })
            .ok_or_else(|| {
                serde::de::Error::custom(format!("Failed to parse value from '{}'", str))
            })?;
        Ok(Percent(value))
    }
}

impl Serialize for Percent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        #[cfg(not(feature = "value_only_measurement"))]
        {
            serializer.serialize_str(&format!("{} %", self.0))
        }
        #[cfg(feature = "value_only_measurement")]
        self.0.serialize(serializer)
    }
}

impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} %", self.0)
    }
}

impl std::ops::Deref for Percent {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Percent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
