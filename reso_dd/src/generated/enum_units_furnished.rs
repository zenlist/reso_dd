// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [UnitsFurnished Lookups](https://ddwiki.reso.org/display/DDW17/UnitsFurnished+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitsFurnished {
    /// "[All Units](https://ddwiki.reso.org/display/DDW17/All+Units)": All of the units in the listed income property are furnished.
    AllUnits,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246595)": None of the units in the listed income property are furnished.
    None,

    /// "[Varies By Unit](https://ddwiki.reso.org/display/DDW17/Varies+By+Unit)": Some of the units in the listing income property are furnished.
    VariesByUnit,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for UnitsFurnished {
    fn from(s: String) -> UnitsFurnished {
        match s.as_ref() {
            "All Units" => UnitsFurnished::AllUnits,

            "None" => UnitsFurnished::None,

            "Varies By Unit" => UnitsFurnished::VariesByUnit,

            _ => UnitsFurnished::OpenEnumeration(s),
        }
    }
}

impl From<&str> for UnitsFurnished {
    fn from(s: &str) -> UnitsFurnished {
        match s {
            "All Units" => UnitsFurnished::AllUnits,

            "None" => UnitsFurnished::None,

            "Varies By Unit" => UnitsFurnished::VariesByUnit,

            _ => UnitsFurnished::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a UnitsFurnished> for &'a str {
    fn from(s: &'a UnitsFurnished) -> &'a str {
        match s {
            UnitsFurnished::AllUnits => "All Units",

            UnitsFurnished::None => "None",

            UnitsFurnished::VariesByUnit => "Varies By Unit",

            UnitsFurnished::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for UnitsFurnished {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UnitsFurnished {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_units_furnished_format {
    use super::UnitsFurnished;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<UnitsFurnished>>,
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
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Vec<UnitsFurnished>>, D::Error>
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
