//! A serde serialization/deserialization module (for use with `#[serde(with="comma_delimited")]`)
//! that will convert a comma-delimeted list and turn it into a list of strongly-typed values.

use crate::ResoEnumeration;
use serde::{Deserialize, Deserializer, Serializer};

/// Serialize an array of items into a comma-delimited list.
///
/// Note that `None` and `Some(vec![])` are distinct: the former gets serialized into `null`,
/// and the latter gets serialized into `""`.
pub fn serialize<S, T: ResoEnumeration>(
    items: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match items {
        None => return serializer.serialize_none(),
        Some(ref vec) if vec.len() == 0 => serializer.serialize_str(""),
        Some(ref vec) => {
            let items: Vec<&str> = vec
                .iter()
                .map(|item| ResoEnumeration::to_str(item))
                .collect();
            let joined = items.join(",");
            serializer.serialize_str(&joined)
        }
    }
}

/// Deserialize an array of items into a comma-delimited list.
///
/// Note that `null` and `""` are distinct: the former gets serialized into `None`, and the
/// latter gets serialized into `Some(vec![])`.
pub fn deserialize<'de, D, T: ResoEnumeration>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "" {
        return Ok(Some(vec![]));
    }

    let items = s
        .split(",")
        .map(|i| ResoEnumeration::from_str(i.trim()))
        .collect();
    Ok(Some(items))
}
