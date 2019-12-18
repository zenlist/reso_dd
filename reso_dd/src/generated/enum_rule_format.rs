// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [RuleFormat Lookups](https://ddwiki.reso.org/display/DDW17/RuleFormat+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuleFormat {
    /// "[$filter](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246248)": Business rules expressed utilizing the OData $filter syntax.  The underlying structure of the rules are not defined at this time and may vary from source to source.
    Filter,

    /// "[JavaScript](https://ddwiki.reso.org/display/DDW17/JavaScript)": Business rules expressed utilizing the JavaScript language.  The underlying structure of the rules are not defined at this time and may vary from source to source.
    JavaScript,

    /// "[REBR](https://ddwiki.reso.org/display/DDW17/REBR)": Real Estate Business Rule (REBR) notation, based on  RuleSpeak structured notation, uses a predictable syntax to allow humans to clearly and unambiguously specify real estate business rules.  REBR is not machine consumable and design for human production and consumption.
    REBR,

    /// "[RetsValidation](https://ddwiki.reso.org/display/DDW17/RetsValidation)": Business rules expressed using the well defined RETS 1.9 Validation Expressions.  See section 11.4.7 of the RETS 1.9 Specification for additional details. https://www.reso.org/download/rets-1-9-specification/
    RetsValidation,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for RuleFormat {
    fn from(s: String) -> RuleFormat {
        match s.as_ref() {
            "$filter" => RuleFormat::Filter,

            "JavaScript" => RuleFormat::JavaScript,

            "REBR" => RuleFormat::REBR,

            "RetsValidation" => RuleFormat::RetsValidation,

            _ => RuleFormat::OpenEnumeration(s),
        }
    }
}

impl From<&str> for RuleFormat {
    fn from(s: &str) -> RuleFormat {
        match s {
            "$filter" => RuleFormat::Filter,

            "JavaScript" => RuleFormat::JavaScript,

            "REBR" => RuleFormat::REBR,

            "RetsValidation" => RuleFormat::RetsValidation,

            _ => RuleFormat::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a RuleFormat> for &'a str {
    fn from(s: &'a RuleFormat) -> &'a str {
        match s {
            RuleFormat::Filter => "$filter",

            RuleFormat::JavaScript => "JavaScript",

            RuleFormat::REBR => "REBR",

            RuleFormat::RetsValidation => "RetsValidation",

            RuleFormat::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for RuleFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RuleFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_rule_format_format {
    use super::RuleFormat;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<RuleFormat>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<RuleFormat>>, D::Error>
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
