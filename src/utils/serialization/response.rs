use serde::{self, Deserialize, Deserializer, Serializer};
use axum::http::StatusCode;

/// Custom serialization for StatusCode
pub fn serialize_status_code<S>(
    status: &StatusCode,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = status.as_u16();
    serializer.serialize_u16(formatted)
}

/// Custom deserialization for StatusCode
pub fn deserialize_status_code<'de, D>(
    deserializer: D,
) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u16 = u16::deserialize(deserializer)?;
    StatusCode::from_u16(s).map_err(serde::de::Error::custom)
}