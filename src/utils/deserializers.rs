use serde::{Deserialize, Deserializer};

pub fn de_f64_from_string<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>
{
    let v = serde_json::Value::deserialize(d)?;
    match v {
        serde_json::Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
        serde_json::Value::Null => Ok(0.0),
        other => Err(serde::de::Error::custom(format!("expected string, got {}", other))),
    }
}
