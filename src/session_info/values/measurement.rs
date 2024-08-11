use serde::{Deserialize, Deserializer, Serialize};

pub type MeasurementF32 = Measurement<f32>;
pub type MeasurementI32 = Measurement<i32>;

/// Used when session data is a string with a value and unit of measurement.
/// Use the feature `value_only_measurement` to only serialize the value.  This is useful
/// in that numbers will be serialized as numbers and not strings.
#[derive(Debug, Clone, Default)]
pub struct Measurement<V> {
    pub value: V,
    pub unit: Option<String>,
}

impl<'de, T> Deserialize<'de> for Measurement<T>
where
    T: std::str::FromStr + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Measurement<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        let mut iter = str.split_whitespace();
        let value = iter
            .next()
            .and_then(|v| v.parse::<T>().ok())
            .ok_or_else(|| {
                serde::de::Error::custom(format!("Failed to parse value from '{}'", str))
            })?;
        Ok(Measurement {
            value,
            unit: iter.next().map(|s| s.to_string()),
        })
    }
}

impl<T> Serialize for Measurement<T>
where
    T: Serialize + std::fmt::Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match &self.unit {
            #[cfg(not(feature = "value_only_measurement"))]
            Some(unit) => serializer.serialize_str(&format!("{} {}", self.value, unit)),
            #[cfg(feature = "value_only_measurement")]
            Some(unit) => self.value.serialize(serializer),
            None => serializer.serialize_str(&format!("{}", self.value)),
        }
    }
}

impl<T> std::fmt::Display for Measurement<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.unit.is_none() {
            write!(f, "{}", self.value)
        } else {
            // Unwrap is safe as we just checked it is some
            write!(f, "{} {}", &self.value, self.unit.as_ref().unwrap())
        }
    }
}
