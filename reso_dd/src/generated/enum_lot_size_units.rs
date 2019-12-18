// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LotSizeUnits Lookups](https://ddwiki.reso.org/display/DDW17/LotSizeUnits+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LotSizeUnits {
    /// "[Acres](https://ddwiki.reso.org/display/DDW17/Acres)": The value reported in the Lot Size Area field is in acres.
    Acres,

    /// "[Square Feet](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245429)": The value reported in the Lot Size Area field is in square feet.
    SquareFeet,

    /// "[Square Meters](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245430)": The value reported in the Lot Size Area field is in square meters.
    SquareMeters,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for LotSizeUnits {
    fn from(s: String) -> LotSizeUnits {
        match s.as_ref() {
            "Acres" => LotSizeUnits::Acres,

            "Square Feet" => LotSizeUnits::SquareFeet,

            "Square Meters" => LotSizeUnits::SquareMeters,

            _ => LotSizeUnits::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LotSizeUnits {
    fn from(s: &str) -> LotSizeUnits {
        match s {
            "Acres" => LotSizeUnits::Acres,

            "Square Feet" => LotSizeUnits::SquareFeet,

            "Square Meters" => LotSizeUnits::SquareMeters,

            _ => LotSizeUnits::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LotSizeUnits> for &'a str {
    fn from(s: &'a LotSizeUnits) -> &'a str {
        match s {
            LotSizeUnits::Acres => "Acres",

            LotSizeUnits::SquareFeet => "Square Feet",

            LotSizeUnits::SquareMeters => "Square Meters",

            LotSizeUnits::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LotSizeUnits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LotSizeUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_lot_size_units_format {
    use super::LotSizeUnits;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<LotSizeUnits>>,
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
    ) -> Result<Option<Vec<LotSizeUnits>>, D::Error>
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
