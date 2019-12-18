// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Furnished Lookups](https://ddwiki.reso.org/display/DDW17/Furnished+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Furnished {
    /// "[Furnished](https://ddwiki.reso.org/display/DDW17/Furnished)": The dwelling being leased is furnished.
    Furnished,

    /// "[Negotiable](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244748)": The property may be furnished or left unfurnished at the lessor's request.  Contact the listing agent/office to discuss options and cost differences.
    Negotiable,

    /// "[Partially](https://ddwiki.reso.org/display/DDW17/Partially)": The dwelling being leased is partially furnished.
    Partially,

    /// "[Unfurnished](https://ddwiki.reso.org/display/DDW17/Unfurnished)": The dwelling being leased is not furnished.
    Unfurnished,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Furnished {
    fn from(s: String) -> Furnished {
        match s.as_ref() {
            "Furnished" => Furnished::Furnished,

            "Negotiable" => Furnished::Negotiable,

            "Partially" => Furnished::Partially,

            "Unfurnished" => Furnished::Unfurnished,

            _ => Furnished::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Furnished {
    fn from(s: &str) -> Furnished {
        match s {
            "Furnished" => Furnished::Furnished,

            "Negotiable" => Furnished::Negotiable,

            "Partially" => Furnished::Partially,

            "Unfurnished" => Furnished::Unfurnished,

            _ => Furnished::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Furnished> for &'a str {
    fn from(s: &'a Furnished) -> &'a str {
        match s {
            Furnished::Furnished => "Furnished",

            Furnished::Negotiable => "Negotiable",

            Furnished::Partially => "Partially",

            Furnished::Unfurnished => "Unfurnished",

            Furnished::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Furnished {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Furnished {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_furnished_format {
    use super::Furnished;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Furnished>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Furnished>>, D::Error>
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
