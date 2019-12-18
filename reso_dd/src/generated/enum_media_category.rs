// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [MediaCategory Lookups](https://ddwiki.reso.org/display/DDW17/MediaCategory+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MediaCategory {
    /// "[Agent Photo](https://ddwiki.reso.org/display/DDW17/Agent+Photo)": The media is an agent photo.
    AgentPhoto,

    /// "[Branded Virtual Tour](https://ddwiki.reso.org/display/DDW17/Branded+Virtual+Tour)": The media is a branded virtual tour.
    BrandedVirtualTour,

    /// "[Document](https://ddwiki.reso.org/display/DDW17/Document)": The media is a document.
    Document,

    /// "[Floor Plan](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245560)": The media is a floor plan.
    FloorPlan,

    /// "[Office Logo](https://ddwiki.reso.org/display/DDW17/Office+Logo)": The media is an office logo.
    OfficeLogo,

    /// "[Office Photo](https://ddwiki.reso.org/display/DDW17/Office+Photo)": The media is an office photo.
    OfficePhoto,

    /// "[Photo](https://ddwiki.reso.org/display/DDW17/Photo)": The media is a photo.
    Photo,

    /// "[Unbranded Virtual Tour](https://ddwiki.reso.org/display/DDW17/Unbranded+Virtual+Tour)": The media is an unbranded virtual tour.
    UnbrandedVirtualTour,

    /// "[Video](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245565)": The media is a video.
    Video,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for MediaCategory {
    fn from(s: String) -> MediaCategory {
        match s.as_ref() {
            "Agent Photo" => MediaCategory::AgentPhoto,

            "Branded Virtual Tour" => MediaCategory::BrandedVirtualTour,

            "Document" => MediaCategory::Document,

            "Floor Plan" => MediaCategory::FloorPlan,

            "Office Logo" => MediaCategory::OfficeLogo,

            "Office Photo" => MediaCategory::OfficePhoto,

            "Photo" => MediaCategory::Photo,

            "Unbranded Virtual Tour" => MediaCategory::UnbrandedVirtualTour,

            "Video" => MediaCategory::Video,

            _ => MediaCategory::OpenEnumeration(s),
        }
    }
}

impl From<&str> for MediaCategory {
    fn from(s: &str) -> MediaCategory {
        match s {
            "Agent Photo" => MediaCategory::AgentPhoto,

            "Branded Virtual Tour" => MediaCategory::BrandedVirtualTour,

            "Document" => MediaCategory::Document,

            "Floor Plan" => MediaCategory::FloorPlan,

            "Office Logo" => MediaCategory::OfficeLogo,

            "Office Photo" => MediaCategory::OfficePhoto,

            "Photo" => MediaCategory::Photo,

            "Unbranded Virtual Tour" => MediaCategory::UnbrandedVirtualTour,

            "Video" => MediaCategory::Video,

            _ => MediaCategory::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a MediaCategory> for &'a str {
    fn from(s: &'a MediaCategory) -> &'a str {
        match s {
            MediaCategory::AgentPhoto => "Agent Photo",

            MediaCategory::BrandedVirtualTour => "Branded Virtual Tour",

            MediaCategory::Document => "Document",

            MediaCategory::FloorPlan => "Floor Plan",

            MediaCategory::OfficeLogo => "Office Logo",

            MediaCategory::OfficePhoto => "Office Photo",

            MediaCategory::Photo => "Photo",

            MediaCategory::UnbrandedVirtualTour => "Unbranded Virtual Tour",

            MediaCategory::Video => "Video",

            MediaCategory::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for MediaCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MediaCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_media_category_format {
    use super::MediaCategory;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<MediaCategory>>,
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
    ) -> Result<Option<Vec<MediaCategory>>, D::Error>
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
