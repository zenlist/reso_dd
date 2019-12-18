// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Vegetation Lookups](https://ddwiki.reso.org/display/DDW17/Vegetation+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Vegetation {
    /// "[Brush](https://ddwiki.reso.org/display/DDW17/Brush)": The lot has brush.
    Brush,

    /// "[Cleared](https://ddwiki.reso.org/display/DDW17/Cleared)": The lot has been cleared.
    Cleared,

    /// "[Crop(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246678)": There are crops on the lot.
    Crops,

    /// "[Grassed](https://ddwiki.reso.org/display/DDW17/Grassed)": The lot is grassed.
    Grassed,

    /// "[Heavily Wooded](https://ddwiki.reso.org/display/DDW17/Heavily+Wooded)": The lot is heavily wooded.
    HeavilyWooded,

    /// "[Natural State](https://ddwiki.reso.org/display/DDW17/Natural+State)": The lot is in its natural state.
    NaturalState,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246682)": There are other types of vegetation on the lot than those in this list.
    Other,

    /// "[Partially Wooded](https://ddwiki.reso.org/display/DDW17/Partially+Wooded)": The lot is partially wooded.
    PartiallyWooded,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246684)": See the Public or Private Remarks for details about the vegetation found on the lot.
    SeeRemarks,

    /// "[Wooded](https://ddwiki.reso.org/display/DDW17/Wooded)": The lot is wooded.
    Wooded,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Vegetation {
    fn from(s: String) -> Vegetation {
        match s.as_ref() {
            "Brush" => Vegetation::Brush,

            "Cleared" => Vegetation::Cleared,

            "Crop(s)" => Vegetation::Crops,

            "Grassed" => Vegetation::Grassed,

            "Heavily Wooded" => Vegetation::HeavilyWooded,

            "Natural State" => Vegetation::NaturalState,

            "Other" => Vegetation::Other,

            "Partially Wooded" => Vegetation::PartiallyWooded,

            "See Remarks" => Vegetation::SeeRemarks,

            "Wooded" => Vegetation::Wooded,

            _ => Vegetation::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Vegetation {
    fn from(s: &str) -> Vegetation {
        match s {
            "Brush" => Vegetation::Brush,

            "Cleared" => Vegetation::Cleared,

            "Crop(s)" => Vegetation::Crops,

            "Grassed" => Vegetation::Grassed,

            "Heavily Wooded" => Vegetation::HeavilyWooded,

            "Natural State" => Vegetation::NaturalState,

            "Other" => Vegetation::Other,

            "Partially Wooded" => Vegetation::PartiallyWooded,

            "See Remarks" => Vegetation::SeeRemarks,

            "Wooded" => Vegetation::Wooded,

            _ => Vegetation::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Vegetation> for &'a str {
    fn from(s: &'a Vegetation) -> &'a str {
        match s {
            Vegetation::Brush => "Brush",

            Vegetation::Cleared => "Cleared",

            Vegetation::Crops => "Crop(s)",

            Vegetation::Grassed => "Grassed",

            Vegetation::HeavilyWooded => "Heavily Wooded",

            Vegetation::NaturalState => "Natural State",

            Vegetation::Other => "Other",

            Vegetation::PartiallyWooded => "Partially Wooded",

            Vegetation::SeeRemarks => "See Remarks",

            Vegetation::Wooded => "Wooded",

            Vegetation::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Vegetation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Vegetation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_vegetation_format {
    use super::Vegetation;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Vegetation>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Vegetation>>, D::Error>
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
