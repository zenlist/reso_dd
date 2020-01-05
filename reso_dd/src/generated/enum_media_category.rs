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

impl crate::ResoEnumeration for MediaCategory {
    fn from_str(s: &str) -> MediaCategory {
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

    fn from_string(s: String) -> MediaCategory {
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

    fn to_str(&self) -> &str {
        match self {
            MediaCategory::AgentPhoto => "Agent Photo",

            MediaCategory::BrandedVirtualTour => "Branded Virtual Tour",

            MediaCategory::Document => "Document",

            MediaCategory::FloorPlan => "Floor Plan",

            MediaCategory::OfficeLogo => "Office Logo",

            MediaCategory::OfficePhoto => "Office Photo",

            MediaCategory::Photo => "Photo",

            MediaCategory::UnbrandedVirtualTour => "Unbranded Virtual Tour",

            MediaCategory::Video => "Video",

            MediaCategory::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            MediaCategory::AgentPhoto => "Agent Photo".into(),

            MediaCategory::BrandedVirtualTour => "Branded Virtual Tour".into(),

            MediaCategory::Document => "Document".into(),

            MediaCategory::FloorPlan => "Floor Plan".into(),

            MediaCategory::OfficeLogo => "Office Logo".into(),

            MediaCategory::OfficePhoto => "Office Photo".into(),

            MediaCategory::Photo => "Photo".into(),

            MediaCategory::UnbrandedVirtualTour => "Unbranded Virtual Tour".into(),

            MediaCategory::Video => "Video".into(),

            MediaCategory::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            MediaCategory::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
