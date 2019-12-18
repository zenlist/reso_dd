// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [NotedBy Lookups](https://ddwiki.reso.org/display/DDW17/NotedBy+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NotedBy {
    /// "[Agent](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246727)": The agent has written the given note about the given listing.
    Agent,

    /// "[Contact](https://ddwiki.reso.org/display/DDW17/Contact)": The contact has written the given note about the given listing.
    Contact,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for NotedBy {
    fn from(s: String) -> NotedBy {
        match s.as_ref() {
            "Agent" => NotedBy::Agent,

            "Contact" => NotedBy::Contact,

            _ => NotedBy::OpenEnumeration(s),
        }
    }
}

impl From<&str> for NotedBy {
    fn from(s: &str) -> NotedBy {
        match s {
            "Agent" => NotedBy::Agent,

            "Contact" => NotedBy::Contact,

            _ => NotedBy::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a NotedBy> for &'a str {
    fn from(s: &'a NotedBy) -> &'a str {
        match s {
            NotedBy::Agent => "Agent",

            NotedBy::Contact => "Contact",

            NotedBy::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for NotedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for NotedBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_noted_by_format {
    use super::NotedBy;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<NotedBy>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<NotedBy>>, D::Error>
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
