// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ObjectIdType Lookups](https://ddwiki.reso.org/display/DDW17/ObjectIdType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ObjectIdType {
    /// "[ListingId](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245795)": The ObjectID is the MLS listing number
    ListingId,

    /// "[ListingKey](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245796)": The Object is a key field from an MLS system.
    ListingKey,

    /// "[ListingKeyNumeric](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245797)": The Object is a numeric key field from an MLS system.
    ListingKeyNumeric,

    /// "[OpenHouseId](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245798)": The ObjectID is an Open House ID
    OpenHouseId,

    /// "[OpenHouseKey](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245799)": The ObjectID is the key of an Open House record.
    OpenHouseKey,

    /// "[OpenHouseKeyNumeric](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245800)": The ObjectID is the numeric only key of an Open House record.
    OpenHouseKeyNumeric,

    /// "[ParcelNumber](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245801)": When no listing exists or the tracking is property centric, the ObjectIdType of the property's Parcel Number is used.
    ParcelNumber,

    /// "[PUID](https://ddwiki.reso.org/display/DDW17/PUID)": When no listing exists and the tracking property centric, the RESO PUID is being used.
    PUID,

    /// "[SavedSearchKey](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245803)": When the event is the execution of a saved search, the ObjectID will be the SavedSearchKey or SavedSearchKeyNumeric from the system that executed the search. Individual listings from the result set or being viewed would be separate events for each record.
    SavedSearchKey,

    /// "[SavedSearchKeyNumeric](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245804)": When the event is the execution of a saved search, the ObjectID will be the SavedSearchKey or SavedSearchKeyNumeric from the system that executed the search. Individual listings from the result set or being viewed would be separate events for each record.
    SavedSearchKeyNumeric,

    /// "[Unique](https://ddwiki.reso.org/display/DDW17/Unique)": The ObjectID is a unique ID supplied by the source and is not one of the other types…
    Unique,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ObjectIdType {
    fn from_str(s: &str) -> ObjectIdType {
        match s {
            "ListingId" => ObjectIdType::ListingId,

            "ListingKey" => ObjectIdType::ListingKey,

            "ListingKeyNumeric" => ObjectIdType::ListingKeyNumeric,

            "OpenHouseId" => ObjectIdType::OpenHouseId,

            "OpenHouseKey" => ObjectIdType::OpenHouseKey,

            "OpenHouseKeyNumeric" => ObjectIdType::OpenHouseKeyNumeric,

            "ParcelNumber" => ObjectIdType::ParcelNumber,

            "PUID" => ObjectIdType::PUID,

            "SavedSearchKey" => ObjectIdType::SavedSearchKey,

            "SavedSearchKeyNumeric" => ObjectIdType::SavedSearchKeyNumeric,

            "Unique" => ObjectIdType::Unique,

            _ => ObjectIdType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ObjectIdType {
        match s.as_ref() {
            "ListingId" => ObjectIdType::ListingId,

            "ListingKey" => ObjectIdType::ListingKey,

            "ListingKeyNumeric" => ObjectIdType::ListingKeyNumeric,

            "OpenHouseId" => ObjectIdType::OpenHouseId,

            "OpenHouseKey" => ObjectIdType::OpenHouseKey,

            "OpenHouseKeyNumeric" => ObjectIdType::OpenHouseKeyNumeric,

            "ParcelNumber" => ObjectIdType::ParcelNumber,

            "PUID" => ObjectIdType::PUID,

            "SavedSearchKey" => ObjectIdType::SavedSearchKey,

            "SavedSearchKeyNumeric" => ObjectIdType::SavedSearchKeyNumeric,

            "Unique" => ObjectIdType::Unique,

            _ => ObjectIdType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ObjectIdType::ListingId => "ListingId",

            ObjectIdType::ListingKey => "ListingKey",

            ObjectIdType::ListingKeyNumeric => "ListingKeyNumeric",

            ObjectIdType::OpenHouseId => "OpenHouseId",

            ObjectIdType::OpenHouseKey => "OpenHouseKey",

            ObjectIdType::OpenHouseKeyNumeric => "OpenHouseKeyNumeric",

            ObjectIdType::ParcelNumber => "ParcelNumber",

            ObjectIdType::PUID => "PUID",

            ObjectIdType::SavedSearchKey => "SavedSearchKey",

            ObjectIdType::SavedSearchKeyNumeric => "SavedSearchKeyNumeric",

            ObjectIdType::Unique => "Unique",

            ObjectIdType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ObjectIdType::ListingId => "ListingId".into(),

            ObjectIdType::ListingKey => "ListingKey".into(),

            ObjectIdType::ListingKeyNumeric => "ListingKeyNumeric".into(),

            ObjectIdType::OpenHouseId => "OpenHouseId".into(),

            ObjectIdType::OpenHouseKey => "OpenHouseKey".into(),

            ObjectIdType::OpenHouseKeyNumeric => "OpenHouseKeyNumeric".into(),

            ObjectIdType::ParcelNumber => "ParcelNumber".into(),

            ObjectIdType::PUID => "PUID".into(),

            ObjectIdType::SavedSearchKey => "SavedSearchKey".into(),

            ObjectIdType::SavedSearchKeyNumeric => "SavedSearchKeyNumeric".into(),

            ObjectIdType::Unique => "Unique".into(),

            ObjectIdType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ObjectIdType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ObjectIdType {
    fn from(s: String) -> ObjectIdType {
        match s.as_ref() {
            "ListingId" => ObjectIdType::ListingId,

            "ListingKey" => ObjectIdType::ListingKey,

            "ListingKeyNumeric" => ObjectIdType::ListingKeyNumeric,

            "OpenHouseId" => ObjectIdType::OpenHouseId,

            "OpenHouseKey" => ObjectIdType::OpenHouseKey,

            "OpenHouseKeyNumeric" => ObjectIdType::OpenHouseKeyNumeric,

            "ParcelNumber" => ObjectIdType::ParcelNumber,

            "PUID" => ObjectIdType::PUID,

            "SavedSearchKey" => ObjectIdType::SavedSearchKey,

            "SavedSearchKeyNumeric" => ObjectIdType::SavedSearchKeyNumeric,

            "Unique" => ObjectIdType::Unique,

            _ => ObjectIdType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ObjectIdType {
    fn from(s: &str) -> ObjectIdType {
        match s {
            "ListingId" => ObjectIdType::ListingId,

            "ListingKey" => ObjectIdType::ListingKey,

            "ListingKeyNumeric" => ObjectIdType::ListingKeyNumeric,

            "OpenHouseId" => ObjectIdType::OpenHouseId,

            "OpenHouseKey" => ObjectIdType::OpenHouseKey,

            "OpenHouseKeyNumeric" => ObjectIdType::OpenHouseKeyNumeric,

            "ParcelNumber" => ObjectIdType::ParcelNumber,

            "PUID" => ObjectIdType::PUID,

            "SavedSearchKey" => ObjectIdType::SavedSearchKey,

            "SavedSearchKeyNumeric" => ObjectIdType::SavedSearchKeyNumeric,

            "Unique" => ObjectIdType::Unique,

            _ => ObjectIdType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ObjectIdType> for &'a str {
    fn from(s: &'a ObjectIdType) -> &'a str {
        match s {
            ObjectIdType::ListingId => "ListingId",

            ObjectIdType::ListingKey => "ListingKey",

            ObjectIdType::ListingKeyNumeric => "ListingKeyNumeric",

            ObjectIdType::OpenHouseId => "OpenHouseId",

            ObjectIdType::OpenHouseKey => "OpenHouseKey",

            ObjectIdType::OpenHouseKeyNumeric => "OpenHouseKeyNumeric",

            ObjectIdType::ParcelNumber => "ParcelNumber",

            ObjectIdType::PUID => "PUID",

            ObjectIdType::SavedSearchKey => "SavedSearchKey",

            ObjectIdType::SavedSearchKeyNumeric => "SavedSearchKeyNumeric",

            ObjectIdType::Unique => "Unique",

            ObjectIdType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ObjectIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ObjectIdType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
