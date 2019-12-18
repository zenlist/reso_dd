// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [BodyType Lookups](https://ddwiki.reso.org/display/DDW17/BodyType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BodyType {
    /// "[Double Wide](https://ddwiki.reso.org/display/DDW17/Double+Wide)": The body/structure type of the mobile/manufacture home is double wide.
    DoubleWide,

    /// "[Expando](https://ddwiki.reso.org/display/DDW17/Expando)": The body/structure type of the mobile/manufacture home is Expando.
    Expando,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243847)": A body type not included in this list.
    Other,

    /// "[Quad Wide](https://ddwiki.reso.org/display/DDW17/Quad+Wide)": The body/structure type of the mobile/manufacture home is quad wide.
    QuadWide,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243844)": The body/structure type of the mobile/manufacture home is see remarks.
    SeeRemarks,

    /// "[Single Wide](https://ddwiki.reso.org/display/DDW17/Single+Wide)": The body/structure type of the mobile/manufacture home is single wide.
    SingleWide,

    /// "[Triple Wide](https://ddwiki.reso.org/display/DDW17/Triple+Wide)": The body/structure type of the mobile/manufacture home is triple wide.
    TripleWide,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for BodyType {
    fn from(s: String) -> BodyType {
        match s.as_ref() {
            "Double Wide" => BodyType::DoubleWide,

            "Expando" => BodyType::Expando,

            "Other" => BodyType::Other,

            "Quad Wide" => BodyType::QuadWide,

            "See Remarks" => BodyType::SeeRemarks,

            "Single Wide" => BodyType::SingleWide,

            "Triple Wide" => BodyType::TripleWide,

            _ => BodyType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for BodyType {
    fn from(s: &str) -> BodyType {
        match s {
            "Double Wide" => BodyType::DoubleWide,

            "Expando" => BodyType::Expando,

            "Other" => BodyType::Other,

            "Quad Wide" => BodyType::QuadWide,

            "See Remarks" => BodyType::SeeRemarks,

            "Single Wide" => BodyType::SingleWide,

            "Triple Wide" => BodyType::TripleWide,

            _ => BodyType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a BodyType> for &'a str {
    fn from(s: &'a BodyType) -> &'a str {
        match s {
            BodyType::DoubleWide => "Double Wide",

            BodyType::Expando => "Expando",

            BodyType::Other => "Other",

            BodyType::QuadWide => "Quad Wide",

            BodyType::SeeRemarks => "See Remarks",

            BodyType::SingleWide => "Single Wide",

            BodyType::TripleWide => "Triple Wide",

            BodyType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for BodyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for BodyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_body_type_format {
    use super::BodyType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<BodyType>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<BodyType>>, D::Error>
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
