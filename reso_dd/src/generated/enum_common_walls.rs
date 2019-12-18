// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [CommonWalls Lookups](https://ddwiki.reso.org/display/DDW17/CommonWalls+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CommonWalls {
    /// "[1 Common Wall](https://ddwiki.reso.org/display/DDW17/1+Common+Wall)": The dwelling being sold has one common wall with another property that is not part of the sale.  Also known as an attached structure.
    _1CommonWall,

    /// "[2+ Common Walls](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244057)": The dwelling being sold has two or more common walls with another property that is not part of the sale.  Also known as an attached structure.
    _2PlusCommonWalls,

    /// "[End Unit](https://ddwiki.reso.org/display/DDW17/End+Unit)": The dwelling being sold has one or more common walls with another property that is not part of the sale and is at the end of a row of units.  Also known as an attached structure.
    EndUnit,

    /// "[No Common Walls](https://ddwiki.reso.org/display/DDW17/No+Common+Walls)": The dwelling being sold has no attached structures that are not part of the sale.  Also know as a detached structure.
    NoCommonWalls,

    /// "[No One Above](https://ddwiki.reso.org/display/DDW17/No+One+Above)": The property is attached to another dwelling that is not part of the sale, but there is no unit above the one being sold.
    NoOneAbove,

    /// "[No One Below](https://ddwiki.reso.org/display/DDW17/No+One+Below)": The property is attached to another dwelling that is not part of the sale, but there is no unit below the one being sold.
    NoOneBelow,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for CommonWalls {
    fn from(s: String) -> CommonWalls {
        match s.as_ref() {
            "1 Common Wall" => CommonWalls::_1CommonWall,

            "2+ Common Walls" => CommonWalls::_2PlusCommonWalls,

            "End Unit" => CommonWalls::EndUnit,

            "No Common Walls" => CommonWalls::NoCommonWalls,

            "No One Above" => CommonWalls::NoOneAbove,

            "No One Below" => CommonWalls::NoOneBelow,

            _ => CommonWalls::OpenEnumeration(s),
        }
    }
}

impl From<&str> for CommonWalls {
    fn from(s: &str) -> CommonWalls {
        match s {
            "1 Common Wall" => CommonWalls::_1CommonWall,

            "2+ Common Walls" => CommonWalls::_2PlusCommonWalls,

            "End Unit" => CommonWalls::EndUnit,

            "No Common Walls" => CommonWalls::NoCommonWalls,

            "No One Above" => CommonWalls::NoOneAbove,

            "No One Below" => CommonWalls::NoOneBelow,

            _ => CommonWalls::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a CommonWalls> for &'a str {
    fn from(s: &'a CommonWalls) -> &'a str {
        match s {
            CommonWalls::_1CommonWall => "1 Common Wall",

            CommonWalls::_2PlusCommonWalls => "2+ Common Walls",

            CommonWalls::EndUnit => "End Unit",

            CommonWalls::NoCommonWalls => "No Common Walls",

            CommonWalls::NoOneAbove => "No One Above",

            CommonWalls::NoOneBelow => "No One Below",

            CommonWalls::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for CommonWalls {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CommonWalls {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_common_walls_format {
    use super::CommonWalls;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<CommonWalls>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<CommonWalls>>, D::Error>
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
