// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [WaterSource Lookups](https://ddwiki.reso.org/display/DDW17/WaterSource+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WaterSource {
    /// "[Cistern](https://ddwiki.reso.org/display/DDW17/Cistern)": The property's source of water has/includes a cistern.
    Cistern,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246615)": The property has no current source of water.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246616)": The property has a source of water other than those listed.
    Other,

    /// "[Private](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246617)": The property's source of water is private.
    Private,

    /// "[Public](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246618)": The property's source of water is public.
    Public,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246619)": See the listing's remarks for details on the property's water source.
    SeeRemarks,

    /// "[Shared Well](https://ddwiki.reso.org/display/DDW17/Shared+Well)": The property's source of water has/includes a shared well.
    SharedWell,

    /// "[Spring](https://ddwiki.reso.org/display/DDW17/Spring)": The property's source of water has/includes a spring.
    Spring,

    /// "[Well](https://ddwiki.reso.org/display/DDW17/Well)": The property's source of water has/includes a well.
    Well,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for WaterSource {
    fn from(s: String) -> WaterSource {
        match s.as_ref() {
            "Cistern" => WaterSource::Cistern,

            "None" => WaterSource::None,

            "Other" => WaterSource::Other,

            "Private" => WaterSource::Private,

            "Public" => WaterSource::Public,

            "See Remarks" => WaterSource::SeeRemarks,

            "Shared Well" => WaterSource::SharedWell,

            "Spring" => WaterSource::Spring,

            "Well" => WaterSource::Well,

            _ => WaterSource::OpenEnumeration(s),
        }
    }
}

impl From<&str> for WaterSource {
    fn from(s: &str) -> WaterSource {
        match s {
            "Cistern" => WaterSource::Cistern,

            "None" => WaterSource::None,

            "Other" => WaterSource::Other,

            "Private" => WaterSource::Private,

            "Public" => WaterSource::Public,

            "See Remarks" => WaterSource::SeeRemarks,

            "Shared Well" => WaterSource::SharedWell,

            "Spring" => WaterSource::Spring,

            "Well" => WaterSource::Well,

            _ => WaterSource::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a WaterSource> for &'a str {
    fn from(s: &'a WaterSource) -> &'a str {
        match s {
            WaterSource::Cistern => "Cistern",

            WaterSource::None => "None",

            WaterSource::Other => "Other",

            WaterSource::Private => "Private",

            WaterSource::Public => "Public",

            WaterSource::SeeRemarks => "See Remarks",

            WaterSource::SharedWell => "Shared Well",

            WaterSource::Spring => "Spring",

            WaterSource::Well => "Well",

            WaterSource::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for WaterSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for WaterSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_water_source_format {
    use super::WaterSource;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<WaterSource>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<WaterSource>>, D::Error>
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
