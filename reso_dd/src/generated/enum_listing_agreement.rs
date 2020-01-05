// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ListingAgreement Lookups](https://ddwiki.reso.org/display/DDW17/ListingAgreement+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ListingAgreement {
    /// "[Exclusive Agency](https://ddwiki.reso.org/display/DDW17/Exclusive+Agency)": A contract giving one Brokerage Firm, for a specified time, the right to sell/lease the property and also allowing the owner, acting alone, to sell/lease the property without paying commission.
    ExclusiveAgency,

    /// "[Exclusive Right To Lease](https://ddwiki.reso.org/display/DDW17/Exclusive+Right+To+Lease)": A contract giving the Broker the right to collect commission if the property is leased by anyone, including the owning, during the term of the agreement.
    ExclusiveRightToLease,

    /// "[Exclusive Right To Sell](https://ddwiki.reso.org/display/DDW17/Exclusive+Right+To+Sell)": A contract giving the Broker the right to collect commission if the property is sold by anyone, including the owning, during the term of the agreement.
    ExclusiveRightToSell,

    /// "[Exclusive Right With Exception](https://ddwiki.reso.org/display/DDW17/Exclusive+Right+With+Exception)": A contract giving the Broker the right to collect commission if the property is sold by anyone, including the owner, during the term of the agreement unless some specified exceptions to the agreement occur.
    ExclusiveRightWithException,

    /// "[Net](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245374)": A listing in which the broker's commission is the excess of the sale price over an agreed-upon (net0 price to the seller; illegal in some states because it can create a conflict of interest for the broker.
    Net,

    /// "[Open](https://ddwiki.reso.org/display/DDW17/Open)": Often used for commercial property, a listing given to any number of Brokers without liability to compensate any except the one who first secures a buyer who is ready, willing and able to meet the terms of the listing and secures the seller's acceptance.  The seller may, acting alone, sell the property without paying commission.
    Open,

    /// "[Probate](https://ddwiki.reso.org/display/DDW17/Probate)": An Exclusive Right To Sell listing agreement that also resides under authority of the local probate code.
    Probate,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ListingAgreement {
    fn from_str(s: &str) -> ListingAgreement {
        match s {
            "Exclusive Agency" => ListingAgreement::ExclusiveAgency,

            "Exclusive Right To Lease" => ListingAgreement::ExclusiveRightToLease,

            "Exclusive Right To Sell" => ListingAgreement::ExclusiveRightToSell,

            "Exclusive Right With Exception" => ListingAgreement::ExclusiveRightWithException,

            "Net" => ListingAgreement::Net,

            "Open" => ListingAgreement::Open,

            "Probate" => ListingAgreement::Probate,

            _ => ListingAgreement::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ListingAgreement {
        match s.as_ref() {
            "Exclusive Agency" => ListingAgreement::ExclusiveAgency,

            "Exclusive Right To Lease" => ListingAgreement::ExclusiveRightToLease,

            "Exclusive Right To Sell" => ListingAgreement::ExclusiveRightToSell,

            "Exclusive Right With Exception" => ListingAgreement::ExclusiveRightWithException,

            "Net" => ListingAgreement::Net,

            "Open" => ListingAgreement::Open,

            "Probate" => ListingAgreement::Probate,

            _ => ListingAgreement::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ListingAgreement::ExclusiveAgency => "Exclusive Agency",

            ListingAgreement::ExclusiveRightToLease => "Exclusive Right To Lease",

            ListingAgreement::ExclusiveRightToSell => "Exclusive Right To Sell",

            ListingAgreement::ExclusiveRightWithException => "Exclusive Right With Exception",

            ListingAgreement::Net => "Net",

            ListingAgreement::Open => "Open",

            ListingAgreement::Probate => "Probate",

            ListingAgreement::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ListingAgreement::ExclusiveAgency => "Exclusive Agency".into(),

            ListingAgreement::ExclusiveRightToLease => "Exclusive Right To Lease".into(),

            ListingAgreement::ExclusiveRightToSell => "Exclusive Right To Sell".into(),

            ListingAgreement::ExclusiveRightWithException => {
                "Exclusive Right With Exception".into()
            }

            ListingAgreement::Net => "Net".into(),

            ListingAgreement::Open => "Open".into(),

            ListingAgreement::Probate => "Probate".into(),

            ListingAgreement::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ListingAgreement::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ListingAgreement {
    fn from(s: String) -> ListingAgreement {
        match s.as_ref() {
            "Exclusive Agency" => ListingAgreement::ExclusiveAgency,

            "Exclusive Right To Lease" => ListingAgreement::ExclusiveRightToLease,

            "Exclusive Right To Sell" => ListingAgreement::ExclusiveRightToSell,

            "Exclusive Right With Exception" => ListingAgreement::ExclusiveRightWithException,

            "Net" => ListingAgreement::Net,

            "Open" => ListingAgreement::Open,

            "Probate" => ListingAgreement::Probate,

            _ => ListingAgreement::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ListingAgreement {
    fn from(s: &str) -> ListingAgreement {
        match s {
            "Exclusive Agency" => ListingAgreement::ExclusiveAgency,

            "Exclusive Right To Lease" => ListingAgreement::ExclusiveRightToLease,

            "Exclusive Right To Sell" => ListingAgreement::ExclusiveRightToSell,

            "Exclusive Right With Exception" => ListingAgreement::ExclusiveRightWithException,

            "Net" => ListingAgreement::Net,

            "Open" => ListingAgreement::Open,

            "Probate" => ListingAgreement::Probate,

            _ => ListingAgreement::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ListingAgreement> for &'a str {
    fn from(s: &'a ListingAgreement) -> &'a str {
        match s {
            ListingAgreement::ExclusiveAgency => "Exclusive Agency",

            ListingAgreement::ExclusiveRightToLease => "Exclusive Right To Lease",

            ListingAgreement::ExclusiveRightToSell => "Exclusive Right To Sell",

            ListingAgreement::ExclusiveRightWithException => "Exclusive Right With Exception",

            ListingAgreement::Net => "Net",

            ListingAgreement::Open => "Open",

            ListingAgreement::Probate => "Probate",

            ListingAgreement::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ListingAgreement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ListingAgreement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
