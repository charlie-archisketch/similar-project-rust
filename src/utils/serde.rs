use serde::{Deserialize, Deserializer};

/// Deserialize an `f64`, treating `null` values as the default `0.0`.
pub fn deserialize_f64_or_default<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;
    Ok(value.unwrap_or_default())
}
