// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ObjectType Lookups](https://ddwiki.reso.org/display/DDW17/ObjectType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ObjectType {
    /// "[Document](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245812)": The Object of the tracking event is a Document.
    Document,

    /// "[Listing](https://ddwiki.reso.org/display/DDW17/Listing)": The object of the tracking event is a real estate listing
    Listing,

    /// "[Open House](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245808)": The object of the tracking event is an Open House event
    OpenHouse,

    /// "[Photo](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245811)": The Object of the tracking event is a Photo.
    Photo,

    /// "[Property](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245809)": When no listing exists or the tracking is property centric, the ObjectType of Property is used.
    Property,

    /// "[Saved Search](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245810)": When the event is the execution of a saved search, the Object will be a Saved Search from the system that executed the search. If tracked, individual listings from the result set or being viewed would be separate events for each record.
    SavedSearch,

    /// "[Virtual Tour](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245813)": The Object of the tracking event is considered a Virtual Tour.
    VirtualTour,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ObjectType {
    fn from_str(s: &str) -> ObjectType {
        match s {
            "Document" => ObjectType::Document,

            "Listing" => ObjectType::Listing,

            "Open House" => ObjectType::OpenHouse,

            "Photo" => ObjectType::Photo,

            "Property" => ObjectType::Property,

            "Saved Search" => ObjectType::SavedSearch,

            "Virtual Tour" => ObjectType::VirtualTour,

            _ => ObjectType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ObjectType {
        match s.as_ref() {
            "Document" => ObjectType::Document,

            "Listing" => ObjectType::Listing,

            "Open House" => ObjectType::OpenHouse,

            "Photo" => ObjectType::Photo,

            "Property" => ObjectType::Property,

            "Saved Search" => ObjectType::SavedSearch,

            "Virtual Tour" => ObjectType::VirtualTour,

            _ => ObjectType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ObjectType::Document => "Document",

            ObjectType::Listing => "Listing",

            ObjectType::OpenHouse => "Open House",

            ObjectType::Photo => "Photo",

            ObjectType::Property => "Property",

            ObjectType::SavedSearch => "Saved Search",

            ObjectType::VirtualTour => "Virtual Tour",

            ObjectType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ObjectType::Document => "Document".into(),

            ObjectType::Listing => "Listing".into(),

            ObjectType::OpenHouse => "Open House".into(),

            ObjectType::Photo => "Photo".into(),

            ObjectType::Property => "Property".into(),

            ObjectType::SavedSearch => "Saved Search".into(),

            ObjectType::VirtualTour => "Virtual Tour".into(),

            ObjectType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ObjectType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ObjectType {
    fn from(s: String) -> ObjectType {
        match s.as_ref() {
            "Document" => ObjectType::Document,

            "Listing" => ObjectType::Listing,

            "Open House" => ObjectType::OpenHouse,

            "Photo" => ObjectType::Photo,

            "Property" => ObjectType::Property,

            "Saved Search" => ObjectType::SavedSearch,

            "Virtual Tour" => ObjectType::VirtualTour,

            _ => ObjectType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ObjectType {
    fn from(s: &str) -> ObjectType {
        match s {
            "Document" => ObjectType::Document,

            "Listing" => ObjectType::Listing,

            "Open House" => ObjectType::OpenHouse,

            "Photo" => ObjectType::Photo,

            "Property" => ObjectType::Property,

            "Saved Search" => ObjectType::SavedSearch,

            "Virtual Tour" => ObjectType::VirtualTour,

            _ => ObjectType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ObjectType> for &'a str {
    fn from(s: &'a ObjectType) -> &'a str {
        match s {
            ObjectType::Document => "Document",

            ObjectType::Listing => "Listing",

            ObjectType::OpenHouse => "Open House",

            ObjectType::Photo => "Photo",

            ObjectType::Property => "Property",

            ObjectType::SavedSearch => "Saved Search",

            ObjectType::VirtualTour => "Virtual Tour",

            ObjectType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ObjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ObjectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
