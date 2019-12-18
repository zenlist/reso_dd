// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OfficeBranchType Lookups](https://ddwiki.reso.org/display/DDW17/OfficeBranchType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OfficeBranchType {
    /// "[Branch](https://ddwiki.reso.org/display/DDW17/Branch)": This office is a branch office.
    Branch,

    /// "[Main](https://ddwiki.reso.org/display/DDW17/Main)": This office is the broker's main office.
    Main,

    /// "[Stand Alone](https://ddwiki.reso.org/display/DDW17/Stand+Alone)": This office is a stand alone, or single office brokerage.
    StandAlone,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for OfficeBranchType {
    fn from(s: String) -> OfficeBranchType {
        match s.as_ref() {
            "Branch" => OfficeBranchType::Branch,

            "Main" => OfficeBranchType::Main,

            "Stand Alone" => OfficeBranchType::StandAlone,

            _ => OfficeBranchType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OfficeBranchType {
    fn from(s: &str) -> OfficeBranchType {
        match s {
            "Branch" => OfficeBranchType::Branch,

            "Main" => OfficeBranchType::Main,

            "Stand Alone" => OfficeBranchType::StandAlone,

            _ => OfficeBranchType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OfficeBranchType> for &'a str {
    fn from(s: &'a OfficeBranchType) -> &'a str {
        match s {
            OfficeBranchType::Branch => "Branch",

            OfficeBranchType::Main => "Main",

            OfficeBranchType::StandAlone => "Stand Alone",

            OfficeBranchType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OfficeBranchType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OfficeBranchType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_office_branch_type_format {
    use super::OfficeBranchType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OfficeBranchType>>,
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
    ) -> Result<Option<Vec<OfficeBranchType>>, D::Error>
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
