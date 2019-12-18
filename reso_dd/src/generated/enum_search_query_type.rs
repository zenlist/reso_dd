// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [SearchQueryType Lookups](https://ddwiki.reso.org/display/DDW17/SearchQueryType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SearchQueryType {
    /// "[$filter](https://ddwiki.reso.org/display/DDW17/%24filter)": The query is in the form of Odata's $filter.
    Filter,

    /// "[DMQL2](https://ddwiki.reso.org/display/DDW17/DMQL2)": The query is in the form of DMQL2.
    DMQL2,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for SearchQueryType {
    fn from(s: String) -> SearchQueryType {
        match s.as_ref() {
            "$filter" => SearchQueryType::Filter,

            "DMQL2" => SearchQueryType::DMQL2,

            _ => SearchQueryType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for SearchQueryType {
    fn from(s: &str) -> SearchQueryType {
        match s {
            "$filter" => SearchQueryType::Filter,

            "DMQL2" => SearchQueryType::DMQL2,

            _ => SearchQueryType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a SearchQueryType> for &'a str {
    fn from(s: &'a SearchQueryType) -> &'a str {
        match s {
            SearchQueryType::Filter => "$filter",

            SearchQueryType::DMQL2 => "DMQL2",

            SearchQueryType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for SearchQueryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SearchQueryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_search_query_type_format {
    use super::SearchQueryType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<SearchQueryType>>,
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
    ) -> Result<Option<Vec<SearchQueryType>>, D::Error>
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
