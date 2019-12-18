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

pub(crate) mod option_vec_listing_agreement_format {
    use super::ListingAgreement;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ListingAgreement>>,
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
    ) -> Result<Option<Vec<ListingAgreement>>, D::Error>
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
