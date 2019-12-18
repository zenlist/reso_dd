// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LaborInformation Lookups](https://ddwiki.reso.org/display/DDW17/LaborInformation+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LaborInformation {
    /// "[Employee License Required](https://ddwiki.reso.org/display/DDW17/Employee+License+Required)": Special licensing is required for employees.
    EmployeeLicenseRequired,

    /// "[Non-Union](https://ddwiki.reso.org/display/DDW17/Non-Union)": A labor union(s) are not currently established with the given business.
    NonUnion,

    /// "[Union](https://ddwiki.reso.org/display/DDW17/Union)": A labor union(s) are established with the given business.
    Union,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for LaborInformation {
    fn from(s: String) -> LaborInformation {
        match s.as_ref() {
            "Employee License Required" => LaborInformation::EmployeeLicenseRequired,

            "Non-Union" => LaborInformation::NonUnion,

            "Union" => LaborInformation::Union,

            _ => LaborInformation::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LaborInformation {
    fn from(s: &str) -> LaborInformation {
        match s {
            "Employee License Required" => LaborInformation::EmployeeLicenseRequired,

            "Non-Union" => LaborInformation::NonUnion,

            "Union" => LaborInformation::Union,

            _ => LaborInformation::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LaborInformation> for &'a str {
    fn from(s: &'a LaborInformation) -> &'a str {
        match s {
            LaborInformation::EmployeeLicenseRequired => "Employee License Required",

            LaborInformation::NonUnion => "Non-Union",

            LaborInformation::Union => "Union",

            LaborInformation::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LaborInformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LaborInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_labor_information_format {
    use super::LaborInformation;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<LaborInformation>>,
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
    ) -> Result<Option<Vec<LaborInformation>>, D::Error>
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
