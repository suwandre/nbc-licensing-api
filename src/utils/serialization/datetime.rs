use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

/// Custom serialization for DateTime<Utc>
pub fn serialize_datetime<S>(
    date: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = date.to_rfc3339();
    serializer.serialize_str(&formatted)
}

/// Custom deserialization for DateTime<Utc>
pub fn deserialize_datetime<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)
        .map(|dt| dt.with_timezone(&Utc))
}