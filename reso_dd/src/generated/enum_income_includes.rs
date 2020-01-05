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

impl crate::ResoEnumeration for IncomeIncludes {
    fn from_str(s: &str) -> IncomeIncludes {
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

    fn from_string(s: String) -> IncomeIncludes {
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

    fn to_str(&self) -> &str {
        match self {
            IncomeIncludes::Laundry => "Laundry",

            IncomeIncludes::Parking => "Parking",

            IncomeIncludes::Recreation => "Recreation",

            IncomeIncludes::RentOnly => "Rent Only",

            IncomeIncludes::RVStorage => "RV Storage",

            IncomeIncludes::Storage => "Storage",

            IncomeIncludes::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            IncomeIncludes::Laundry => "Laundry".into(),

            IncomeIncludes::Parking => "Parking".into(),

            IncomeIncludes::Recreation => "Recreation".into(),

            IncomeIncludes::RentOnly => "Rent Only".into(),

            IncomeIncludes::RVStorage => "RV Storage".into(),

            IncomeIncludes::Storage => "Storage".into(),

            IncomeIncludes::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            IncomeIncludes::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
