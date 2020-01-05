// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ChangeType Lookups](https://ddwiki.reso.org/display/DDW17/ChangeType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ChangeType {
    /// "[Active](https://ddwiki.reso.org/display/DDW17/Active)": The change to the listing was a change of status to Active provided the measurement of the area.
    Active,

    /// "[Active Under Contract](https://ddwiki.reso.org/display/DDW17/Active+Under+Contract)": The change to the listing was a change of status to Active Under Contract provided the measurement of the area.
    ActiveUnderContract,

    /// "[Back On Market](https://ddwiki.reso.org/display/DDW17/Back+On+Market)": The change to the listing was a change of status to Back On Market provided the measurement of the area.
    BackOnMarket,

    /// "[Canceled](https://ddwiki.reso.org/display/DDW17/Canceled)": The change to the listing was a change of status to Canceled provided the measurement of the area.
    Canceled,

    /// "[Closed](https://ddwiki.reso.org/display/DDW17/Closed)": The change to the listing was a change of status to Closed provided the measurement of the area.
    Closed,

    /// "[Deleted](https://ddwiki.reso.org/display/DDW17/Deleted)": The change to the listing was a change of status to Deleted provided the measurement of the area.
    Deleted,

    /// "[Expired](https://ddwiki.reso.org/display/DDW17/Expired)": The change to the listing was a change of status to Expired provided the measurement of the area.
    Expired,

    /// "[Hold](https://ddwiki.reso.org/display/DDW17/Hold)": The change to the listing was a change of status to Hold provided the measurement of the area.
    Hold,

    /// "[New Listing](https://ddwiki.reso.org/display/DDW17/New+Listing)": The listing is new and hasn't had any status or price changes since its original input.
    NewListing,

    /// "[Pending](https://ddwiki.reso.org/display/DDW17/Pending)": The change to the listing was a change of status to Pending provided the measurement of the area.
    Pending,

    /// "[Price Change](https://ddwiki.reso.org/display/DDW17/Price+Change)": The change to the listing was a change to the ListPrice.
    PriceChange,

    /// "[Withdrawn](https://ddwiki.reso.org/display/DDW17/Withdrawn)": The change to the listing was a change of status to Withdrawn provided the measurement of the area.
    Withdrawn,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ChangeType {
    fn from_str(s: &str) -> ChangeType {
        match s {
            "Active" => ChangeType::Active,

            "Active Under Contract" => ChangeType::ActiveUnderContract,

            "Back On Market" => ChangeType::BackOnMarket,

            "Canceled" => ChangeType::Canceled,

            "Closed" => ChangeType::Closed,

            "Deleted" => ChangeType::Deleted,

            "Expired" => ChangeType::Expired,

            "Hold" => ChangeType::Hold,

            "New Listing" => ChangeType::NewListing,

            "Pending" => ChangeType::Pending,

            "Price Change" => ChangeType::PriceChange,

            "Withdrawn" => ChangeType::Withdrawn,

            _ => ChangeType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ChangeType {
        match s.as_ref() {
            "Active" => ChangeType::Active,

            "Active Under Contract" => ChangeType::ActiveUnderContract,

            "Back On Market" => ChangeType::BackOnMarket,

            "Canceled" => ChangeType::Canceled,

            "Closed" => ChangeType::Closed,

            "Deleted" => ChangeType::Deleted,

            "Expired" => ChangeType::Expired,

            "Hold" => ChangeType::Hold,

            "New Listing" => ChangeType::NewListing,

            "Pending" => ChangeType::Pending,

            "Price Change" => ChangeType::PriceChange,

            "Withdrawn" => ChangeType::Withdrawn,

            _ => ChangeType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ChangeType::Active => "Active",

            ChangeType::ActiveUnderContract => "Active Under Contract",

            ChangeType::BackOnMarket => "Back On Market",

            ChangeType::Canceled => "Canceled",

            ChangeType::Closed => "Closed",

            ChangeType::Deleted => "Deleted",

            ChangeType::Expired => "Expired",

            ChangeType::Hold => "Hold",

            ChangeType::NewListing => "New Listing",

            ChangeType::Pending => "Pending",

            ChangeType::PriceChange => "Price Change",

            ChangeType::Withdrawn => "Withdrawn",

            ChangeType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ChangeType::Active => "Active".into(),

            ChangeType::ActiveUnderContract => "Active Under Contract".into(),

            ChangeType::BackOnMarket => "Back On Market".into(),

            ChangeType::Canceled => "Canceled".into(),

            ChangeType::Closed => "Closed".into(),

            ChangeType::Deleted => "Deleted".into(),

            ChangeType::Expired => "Expired".into(),

            ChangeType::Hold => "Hold".into(),

            ChangeType::NewListing => "New Listing".into(),

            ChangeType::Pending => "Pending".into(),

            ChangeType::PriceChange => "Price Change".into(),

            ChangeType::Withdrawn => "Withdrawn".into(),

            ChangeType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ChangeType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ChangeType {
    fn from(s: String) -> ChangeType {
        match s.as_ref() {
            "Active" => ChangeType::Active,

            "Active Under Contract" => ChangeType::ActiveUnderContract,

            "Back On Market" => ChangeType::BackOnMarket,

            "Canceled" => ChangeType::Canceled,

            "Closed" => ChangeType::Closed,

            "Deleted" => ChangeType::Deleted,

            "Expired" => ChangeType::Expired,

            "Hold" => ChangeType::Hold,

            "New Listing" => ChangeType::NewListing,

            "Pending" => ChangeType::Pending,

            "Price Change" => ChangeType::PriceChange,

            "Withdrawn" => ChangeType::Withdrawn,

            _ => ChangeType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ChangeType {
    fn from(s: &str) -> ChangeType {
        match s {
            "Active" => ChangeType::Active,

            "Active Under Contract" => ChangeType::ActiveUnderContract,

            "Back On Market" => ChangeType::BackOnMarket,

            "Canceled" => ChangeType::Canceled,

            "Closed" => ChangeType::Closed,

            "Deleted" => ChangeType::Deleted,

            "Expired" => ChangeType::Expired,

            "Hold" => ChangeType::Hold,

            "New Listing" => ChangeType::NewListing,

            "Pending" => ChangeType::Pending,

            "Price Change" => ChangeType::PriceChange,

            "Withdrawn" => ChangeType::Withdrawn,

            _ => ChangeType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ChangeType> for &'a str {
    fn from(s: &'a ChangeType) -> &'a str {
        match s {
            ChangeType::Active => "Active",

            ChangeType::ActiveUnderContract => "Active Under Contract",

            ChangeType::BackOnMarket => "Back On Market",

            ChangeType::Canceled => "Canceled",

            ChangeType::Closed => "Closed",

            ChangeType::Deleted => "Deleted",

            ChangeType::Expired => "Expired",

            ChangeType::Hold => "Hold",

            ChangeType::NewListing => "New Listing",

            ChangeType::Pending => "Pending",

            ChangeType::PriceChange => "Price Change",

            ChangeType::Withdrawn => "Withdrawn",

            ChangeType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ChangeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ChangeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
