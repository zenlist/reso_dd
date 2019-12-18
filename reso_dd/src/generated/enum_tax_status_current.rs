// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [TaxStatusCurrent Lookups](https://ddwiki.reso.org/display/DDW17/TaxStatusCurrent+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TaxStatusCurrent {
    /// "[Personal](https://ddwiki.reso.org/display/DDW17/Personal)": The tax is based on personal property.
    Personal,

    /// "[Personal And Real](https://ddwiki.reso.org/display/DDW17/Personal+And+Real)": The tax is based on both personal and real property.
    PersonalAndReal,

    /// "[Real](https://ddwiki.reso.org/display/DDW17/Real)": The tax is based on real property.
    Real,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for TaxStatusCurrent {
    fn from(s: String) -> TaxStatusCurrent {
        match s.as_ref() {
            "Personal" => TaxStatusCurrent::Personal,

            "Personal And Real" => TaxStatusCurrent::PersonalAndReal,

            "Real" => TaxStatusCurrent::Real,

            _ => TaxStatusCurrent::OpenEnumeration(s),
        }
    }
}

impl From<&str> for TaxStatusCurrent {
    fn from(s: &str) -> TaxStatusCurrent {
        match s {
            "Personal" => TaxStatusCurrent::Personal,

            "Personal And Real" => TaxStatusCurrent::PersonalAndReal,

            "Real" => TaxStatusCurrent::Real,

            _ => TaxStatusCurrent::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a TaxStatusCurrent> for &'a str {
    fn from(s: &'a TaxStatusCurrent) -> &'a str {
        match s {
            TaxStatusCurrent::Personal => "Personal",

            TaxStatusCurrent::PersonalAndReal => "Personal And Real",

            TaxStatusCurrent::Real => "Real",

            TaxStatusCurrent::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for TaxStatusCurrent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TaxStatusCurrent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_tax_status_current_format {
    use super::TaxStatusCurrent;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<TaxStatusCurrent>>,
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
    ) -> Result<Option<Vec<TaxStatusCurrent>>, D::Error>
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
