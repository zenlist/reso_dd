// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [AreaUnits Lookups](https://ddwiki.reso.org/display/DDW17/AreaUnits+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AreaUnits {
    /// "[Square Feet](https://ddwiki.reso.org/display/DDW17/Square+Feet)": The value reported in the related Area field is in square feet.
    SquareFeet,

    /// "[Square Meters](https://ddwiki.reso.org/display/DDW17/Square+Meters)": The value reported in the related Area field is in square feet.
    SquareMeters,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for AreaUnits {
    fn from(s: String) -> AreaUnits {
        match s.as_ref() {
            "Square Feet" => AreaUnits::SquareFeet,

            "Square Meters" => AreaUnits::SquareMeters,

            _ => AreaUnits::OpenEnumeration(s),
        }
    }
}

impl From<&str> for AreaUnits {
    fn from(s: &str) -> AreaUnits {
        match s {
            "Square Feet" => AreaUnits::SquareFeet,

            "Square Meters" => AreaUnits::SquareMeters,

            _ => AreaUnits::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a AreaUnits> for &'a str {
    fn from(s: &'a AreaUnits) -> &'a str {
        match s {
            AreaUnits::SquareFeet => "Square Feet",

            AreaUnits::SquareMeters => "Square Meters",

            AreaUnits::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for AreaUnits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AreaUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_area_units_format {
    use super::AreaUnits;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<AreaUnits>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match items {
            None => return serializer.serialize_none(),
            Some(ref vec) if vec.len() == 0 => serializer.serialize_str(""),
            Some(ref vec) => {
                let items: Vec<&str> = vec.iter().map(|item| item.into()).collect();
                let joined = items.join(",");
                serializer.serialize_str(&joined)
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<AreaUnits>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "" {
            return Ok(Some(vec![]));
        }

        let items = s.split(",").map(|i| From::<&str>::from(i)).collect();
        Ok(Some(items))
    }
}
