// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [StandardStatus Lookups](https://ddwiki.reso.org/display/DDW17/StandardStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StandardStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246320)": The listing is on market and an offer has not been accepted.
    Active,

    /// "[Active Under Contract](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246321)": An offer has been accepted but the listing is still on market.
    ActiveUnderContract,

    /// "[Canceled](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246322)": The listing contract has been terminated.
    Canceled,

    /// "[Closed](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246323)": The purchase agreement has been fulfilled or the lease agreement has been executed.
    Closed,

    /// "[Coming Soon](https://ddwiki.reso.org/display/DDW17/Coming+Soon)": This is a listing that has not yet been on market but will be on market soon.  A listing contract has been executed.  Some systems may use Hold or Withdrawn for similar purposes.  When all three are in use, Hold expresses a listing that may have been on market but is off market temporarily and is expected to return to market.  Withdrawn may have been on market but when used in conjunction with Hold, is not expected to return to market.  Coming Soon is different from Hold and Withdrawn as the property, under the current listing contract only, has not been previously on market.
    ComingSoon,

    /// "[Delete](https://ddwiki.reso.org/display/DDW17/Delete)": The listing contract was never valid or other reason for the contract to be nullified.
    Delete,

    /// "[Expired](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246326)": The listing contract has expired.
    Expired,

    /// "[Hold](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246327)": A contract exists between the seller and the listing member.  The listing may be completely off market, not allowing any showings and/or not taking any further offers.  For systems that don't use Hold, Withdrawn is a similar status that may be in use.  When both Hold and Withdrawn are in use, Withdrawn may be used to indicate a greater certainty that the listing will not come back on market.
    Hold,

    /// "[Incomplete](https://ddwiki.reso.org/display/DDW17/Incomplete)": The listing has not yet be completely entered and is not yet published in the MLS.
    Incomplete,

    /// "[Pending](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246329)": An offer has been accepted and the listing is no longer on market.
    Pending,

    /// "[Withdrawn](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246330)": The listing has been withdrawn from the market, but a contract still exists between the seller and the listing member.  For those systems that use both Hold and Withdrawn, Withdrawn may represent an intention not to bring the listing back on the market.  When Hold is not used by the system, Withdrawn does not represent any intention of returning to market or not.
    Withdrawn,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for StandardStatus {
    fn from_str(s: &str) -> StandardStatus {
        match s {
            "Active" => StandardStatus::Active,

            "Active Under Contract" => StandardStatus::ActiveUnderContract,

            "Canceled" => StandardStatus::Canceled,

            "Closed" => StandardStatus::Closed,

            "Coming Soon" => StandardStatus::ComingSoon,

            "Delete" => StandardStatus::Delete,

            "Expired" => StandardStatus::Expired,

            "Hold" => StandardStatus::Hold,

            "Incomplete" => StandardStatus::Incomplete,

            "Pending" => StandardStatus::Pending,

            "Withdrawn" => StandardStatus::Withdrawn,

            _ => StandardStatus::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> StandardStatus {
        match s.as_ref() {
            "Active" => StandardStatus::Active,

            "Active Under Contract" => StandardStatus::ActiveUnderContract,

            "Canceled" => StandardStatus::Canceled,

            "Closed" => StandardStatus::Closed,

            "Coming Soon" => StandardStatus::ComingSoon,

            "Delete" => StandardStatus::Delete,

            "Expired" => StandardStatus::Expired,

            "Hold" => StandardStatus::Hold,

            "Incomplete" => StandardStatus::Incomplete,

            "Pending" => StandardStatus::Pending,

            "Withdrawn" => StandardStatus::Withdrawn,

            _ => StandardStatus::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            StandardStatus::Active => "Active",

            StandardStatus::ActiveUnderContract => "Active Under Contract",

            StandardStatus::Canceled => "Canceled",

            StandardStatus::Closed => "Closed",

            StandardStatus::ComingSoon => "Coming Soon",

            StandardStatus::Delete => "Delete",

            StandardStatus::Expired => "Expired",

            StandardStatus::Hold => "Hold",

            StandardStatus::Incomplete => "Incomplete",

            StandardStatus::Pending => "Pending",

            StandardStatus::Withdrawn => "Withdrawn",

            StandardStatus::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            StandardStatus::Active => "Active".into(),

            StandardStatus::ActiveUnderContract => "Active Under Contract".into(),

            StandardStatus::Canceled => "Canceled".into(),

            StandardStatus::Closed => "Closed".into(),

            StandardStatus::ComingSoon => "Coming Soon".into(),

            StandardStatus::Delete => "Delete".into(),

            StandardStatus::Expired => "Expired".into(),

            StandardStatus::Hold => "Hold".into(),

            StandardStatus::Incomplete => "Incomplete".into(),

            StandardStatus::Pending => "Pending".into(),

            StandardStatus::Withdrawn => "Withdrawn".into(),

            StandardStatus::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            StandardStatus::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for StandardStatus {
    fn from(s: String) -> StandardStatus {
        match s.as_ref() {
            "Active" => StandardStatus::Active,

            "Active Under Contract" => StandardStatus::ActiveUnderContract,

            "Canceled" => StandardStatus::Canceled,

            "Closed" => StandardStatus::Closed,

            "Coming Soon" => StandardStatus::ComingSoon,

            "Delete" => StandardStatus::Delete,

            "Expired" => StandardStatus::Expired,

            "Hold" => StandardStatus::Hold,

            "Incomplete" => StandardStatus::Incomplete,

            "Pending" => StandardStatus::Pending,

            "Withdrawn" => StandardStatus::Withdrawn,

            _ => StandardStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for StandardStatus {
    fn from(s: &str) -> StandardStatus {
        match s {
            "Active" => StandardStatus::Active,

            "Active Under Contract" => StandardStatus::ActiveUnderContract,

            "Canceled" => StandardStatus::Canceled,

            "Closed" => StandardStatus::Closed,

            "Coming Soon" => StandardStatus::ComingSoon,

            "Delete" => StandardStatus::Delete,

            "Expired" => StandardStatus::Expired,

            "Hold" => StandardStatus::Hold,

            "Incomplete" => StandardStatus::Incomplete,

            "Pending" => StandardStatus::Pending,

            "Withdrawn" => StandardStatus::Withdrawn,

            _ => StandardStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a StandardStatus> for &'a str {
    fn from(s: &'a StandardStatus) -> &'a str {
        match s {
            StandardStatus::Active => "Active",

            StandardStatus::ActiveUnderContract => "Active Under Contract",

            StandardStatus::Canceled => "Canceled",

            StandardStatus::Closed => "Closed",

            StandardStatus::ComingSoon => "Coming Soon",

            StandardStatus::Delete => "Delete",

            StandardStatus::Expired => "Expired",

            StandardStatus::Hold => "Hold",

            StandardStatus::Incomplete => "Incomplete",

            StandardStatus::Pending => "Pending",

            StandardStatus::Withdrawn => "Withdrawn",

            StandardStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for StandardStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StandardStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
