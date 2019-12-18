// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [SpecialListingConditions Lookups](https://ddwiki.reso.org/display/DDW17/SpecialListingConditions+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpecialListingConditions {
    /// "[Auction](https://ddwiki.reso.org/display/DDW17/Auction)": The listing is an auction.
    Auction,

    /// "[Bankruptcy Property](https://ddwiki.reso.org/display/DDW17/Bankruptcy+Property)": The listed property is currently involved in a bankruptcy.
    BankruptcyProperty,

    /// "[HUD Owned](https://ddwiki.reso.org/display/DDW17/HUD+Owned)": The listed property is owned, and being sold, by the US Department of Housing and Urban Development.
    HUDOwned,

    /// "[In Foreclosure](https://ddwiki.reso.org/display/DDW17/In+Foreclosure)": The listed property is currently in the process of foreclosure.
    InForeclosure,

    /// "[Notice Of Default](https://ddwiki.reso.org/display/DDW17/Notice+Of+Default)": There is a notice of default on the listed property.
    NoticeOfDefault,

    /// "[Probate Listing](https://ddwiki.reso.org/display/DDW17/Probate+Listing)": The listed property is a probate sale.
    ProbateListing,

    /// "[Real Estate Owned](https://ddwiki.reso.org/display/DDW17/Real+Estate+Owned)": The listed property is currently bank/lender owned.
    RealEstateOwned,

    /// "[Short Sale](https://ddwiki.reso.org/display/DDW17/Short+Sale)": The listing is a short sale (short pay) and may require bank approval.
    ShortSale,

    /// "[Standard](https://ddwiki.reso.org/display/DDW17/Standard)": The listing has none of the other conditions in the Special Listing Conditions field.
    Standard,

    /// "[Third Party Approval](https://ddwiki.reso.org/display/DDW17/Third+Party+Approval)": A court or other third party approval is required for the sale to finalize.
    ThirdPartyApproval,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for SpecialListingConditions {
    fn from(s: String) -> SpecialListingConditions {
        match s.as_ref() {
            "Auction" => SpecialListingConditions::Auction,

            "Bankruptcy Property" => SpecialListingConditions::BankruptcyProperty,

            "HUD Owned" => SpecialListingConditions::HUDOwned,

            "In Foreclosure" => SpecialListingConditions::InForeclosure,

            "Notice Of Default" => SpecialListingConditions::NoticeOfDefault,

            "Probate Listing" => SpecialListingConditions::ProbateListing,

            "Real Estate Owned" => SpecialListingConditions::RealEstateOwned,

            "Short Sale" => SpecialListingConditions::ShortSale,

            "Standard" => SpecialListingConditions::Standard,

            "Third Party Approval" => SpecialListingConditions::ThirdPartyApproval,

            _ => SpecialListingConditions::OpenEnumeration(s),
        }
    }
}

impl From<&str> for SpecialListingConditions {
    fn from(s: &str) -> SpecialListingConditions {
        match s {
            "Auction" => SpecialListingConditions::Auction,

            "Bankruptcy Property" => SpecialListingConditions::BankruptcyProperty,

            "HUD Owned" => SpecialListingConditions::HUDOwned,

            "In Foreclosure" => SpecialListingConditions::InForeclosure,

            "Notice Of Default" => SpecialListingConditions::NoticeOfDefault,

            "Probate Listing" => SpecialListingConditions::ProbateListing,

            "Real Estate Owned" => SpecialListingConditions::RealEstateOwned,

            "Short Sale" => SpecialListingConditions::ShortSale,

            "Standard" => SpecialListingConditions::Standard,

            "Third Party Approval" => SpecialListingConditions::ThirdPartyApproval,

            _ => SpecialListingConditions::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a SpecialListingConditions> for &'a str {
    fn from(s: &'a SpecialListingConditions) -> &'a str {
        match s {
            SpecialListingConditions::Auction => "Auction",

            SpecialListingConditions::BankruptcyProperty => "Bankruptcy Property",

            SpecialListingConditions::HUDOwned => "HUD Owned",

            SpecialListingConditions::InForeclosure => "In Foreclosure",

            SpecialListingConditions::NoticeOfDefault => "Notice Of Default",

            SpecialListingConditions::ProbateListing => "Probate Listing",

            SpecialListingConditions::RealEstateOwned => "Real Estate Owned",

            SpecialListingConditions::ShortSale => "Short Sale",

            SpecialListingConditions::Standard => "Standard",

            SpecialListingConditions::ThirdPartyApproval => "Third Party Approval",

            SpecialListingConditions::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for SpecialListingConditions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SpecialListingConditions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_special_listing_conditions_format {
    use super::SpecialListingConditions;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<SpecialListingConditions>>,
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
    ) -> Result<Option<Vec<SpecialListingConditions>>, D::Error>
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
