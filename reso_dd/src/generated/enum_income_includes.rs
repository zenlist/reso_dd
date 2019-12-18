// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [IncomeIncludes Lookups](https://ddwiki.reso.org/display/DDW17/IncomeIncludes+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IncomeIncludes {
    /// "[Laundry](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245093)": The income amount includes income from laundry facilities.
    Laundry,

    /// "[Parking](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245094)": The income amount includes income from parking.
    Parking,

    /// "[Recreation](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245095)": The income amount includes income from charging for recreation facilities.
    Recreation,

    /// "[Rent Only](https://ddwiki.reso.org/display/DDW17/Rent+Only)": The income amount includes income from only the rent charged to the tenants.
    RentOnly,

    /// "[RV Storage](https://ddwiki.reso.org/display/DDW17/RV+Storage)": The income amount includes income from charging for RV storage.
    RVStorage,

    /// "[Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245098)": The income amount includes income from charging for general storage.
    Storage,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for IncomeIncludes {
    fn from(s: String) -> IncomeIncludes {
        match s.as_ref() {
            "Laundry" => IncomeIncludes::Laundry,

            "Parking" => IncomeIncludes::Parking,

            "Recreation" => IncomeIncludes::Recreation,

            "Rent Only" => IncomeIncludes::RentOnly,

            "RV Storage" => IncomeIncludes::RVStorage,

            "Storage" => IncomeIncludes::Storage,

            _ => IncomeIncludes::OpenEnumeration(s),
        }
    }
}

impl From<&str> for IncomeIncludes {
    fn from(s: &str) -> IncomeIncludes {
        match s {
            "Laundry" => IncomeIncludes::Laundry,

            "Parking" => IncomeIncludes::Parking,

            "Recreation" => IncomeIncludes::Recreation,

            "Rent Only" => IncomeIncludes::RentOnly,

            "RV Storage" => IncomeIncludes::RVStorage,

            "Storage" => IncomeIncludes::Storage,

            _ => IncomeIncludes::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a IncomeIncludes> for &'a str {
    fn from(s: &'a IncomeIncludes) -> &'a str {
        match s {
            IncomeIncludes::Laundry => "Laundry",

            IncomeIncludes::Parking => "Parking",

            IncomeIncludes::Recreation => "Recreation",

            IncomeIncludes::RentOnly => "Rent Only",

            IncomeIncludes::RVStorage => "RV Storage",

            IncomeIncludes::Storage => "Storage",

            IncomeIncludes::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for IncomeIncludes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for IncomeIncludes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_income_includes_format {
    use super::IncomeIncludes;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<IncomeIncludes>>,
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
    ) -> Result<Option<Vec<IncomeIncludes>>, D::Error>
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
