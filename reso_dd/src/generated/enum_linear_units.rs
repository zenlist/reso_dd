// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LinearUnits Lookups](https://ddwiki.reso.org/display/DDW17/LinearUnits+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LinearUnits {
    /// "[Feet](https://ddwiki.reso.org/display/DDW17/Feet)": The elevation of the property is measured in feet.
    Feet,

    /// "[Meters](https://ddwiki.reso.org/display/DDW17/Meters)": The elevation of the property is measured in meters.
    Meters,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for LinearUnits {
    fn from(s: String) -> LinearUnits {
        match s.as_ref() {
            "Feet" => LinearUnits::Feet,

            "Meters" => LinearUnits::Meters,

            _ => LinearUnits::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LinearUnits {
    fn from(s: &str) -> LinearUnits {
        match s {
            "Feet" => LinearUnits::Feet,

            "Meters" => LinearUnits::Meters,

            _ => LinearUnits::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LinearUnits> for &'a str {
    fn from(s: &'a LinearUnits) -> &'a str {
        match s {
            LinearUnits::Feet => "Feet",

            LinearUnits::Meters => "Meters",

            LinearUnits::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LinearUnits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LinearUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_linear_units_format {
    use super::LinearUnits;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<LinearUnits>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<LinearUnits>>, D::Error>
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
