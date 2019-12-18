// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [BuyerFinancing Lookups](https://ddwiki.reso.org/display/DDW17/BuyerFinancing+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BuyerFinancing {
    /// "[Assumed](https://ddwiki.reso.org/display/DDW17/Assumed)": The buyer assumed a current form of financing.
    Assumed,

    /// "[Cash](https://ddwiki.reso.org/display/DDW17/Cash)": The buyer paid cash for the property.
    Cash,

    /// "[Contract](https://ddwiki.reso.org/display/DDW17/Contract)": The purchase of a property involves an agreement to perform services, provide product, share of income, or some other agreement as the method of payment for the property.
    Contract,

    /// "[Conventional](https://ddwiki.reso.org/display/DDW17/Conventional)": The buyer is using conventional financing to purchase the home.
    Conventional,

    /// "[FHA](https://ddwiki.reso.org/display/DDW17/FHA)": A loan from an approved provider that follows the guidelines of, and is insured by, the Federal Housing Administration.
    FHA,

    /// "[FHA 203(b)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243965)": The basic home mortgage loan from an approved provider that follows the guidelines of, and is insured by, the Federal Housing Administration.
    FHA203b,

    /// "[FHA 203(k)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243966)": A loan, for the rehabilitation and repair of single family residence, from an approved provider that follows the guidelines of, and is insured by, the Federal Housing Administration.
    FHA203k,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243967)": The buyer is using another form of financing that is not included in the options provided in this list.
    Other,

    /// "[Private](https://ddwiki.reso.org/display/DDW17/Private)": Financing is provided by a private party.
    Private,

    /// "[Seller Financing](https://ddwiki.reso.org/display/DDW17/Seller+Financing)": The seller is providing financing to the buyer.
    SellerFinancing,

    /// "[Trust Deed](https://ddwiki.reso.org/display/DDW17/Trust+Deed)": Financing where title of the property is placed with a trustee who secures payment of the loan for a beneficiary.
    TrustDeed,

    /// "[USDA](https://ddwiki.reso.org/display/DDW17/USDA)": A loan from an approved provider that follows the guidelines of, and is insured by, the US Department of Agriculture.
    USDA,

    /// "[VA](https://ddwiki.reso.org/display/DDW17/VA)": A loan from an approved provider that follows the guidelines of, and is insured by, the US Department of Veteran's Affairs.
    VA,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for BuyerFinancing {
    fn from(s: String) -> BuyerFinancing {
        match s.as_ref() {
            "Assumed" => BuyerFinancing::Assumed,

            "Cash" => BuyerFinancing::Cash,

            "Contract" => BuyerFinancing::Contract,

            "Conventional" => BuyerFinancing::Conventional,

            "FHA" => BuyerFinancing::FHA,

            "FHA 203(b)" => BuyerFinancing::FHA203b,

            "FHA 203(k)" => BuyerFinancing::FHA203k,

            "Other" => BuyerFinancing::Other,

            "Private" => BuyerFinancing::Private,

            "Seller Financing" => BuyerFinancing::SellerFinancing,

            "Trust Deed" => BuyerFinancing::TrustDeed,

            "USDA" => BuyerFinancing::USDA,

            "VA" => BuyerFinancing::VA,

            _ => BuyerFinancing::OpenEnumeration(s),
        }
    }
}

impl From<&str> for BuyerFinancing {
    fn from(s: &str) -> BuyerFinancing {
        match s {
            "Assumed" => BuyerFinancing::Assumed,

            "Cash" => BuyerFinancing::Cash,

            "Contract" => BuyerFinancing::Contract,

            "Conventional" => BuyerFinancing::Conventional,

            "FHA" => BuyerFinancing::FHA,

            "FHA 203(b)" => BuyerFinancing::FHA203b,

            "FHA 203(k)" => BuyerFinancing::FHA203k,

            "Other" => BuyerFinancing::Other,

            "Private" => BuyerFinancing::Private,

            "Seller Financing" => BuyerFinancing::SellerFinancing,

            "Trust Deed" => BuyerFinancing::TrustDeed,

            "USDA" => BuyerFinancing::USDA,

            "VA" => BuyerFinancing::VA,

            _ => BuyerFinancing::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a BuyerFinancing> for &'a str {
    fn from(s: &'a BuyerFinancing) -> &'a str {
        match s {
            BuyerFinancing::Assumed => "Assumed",

            BuyerFinancing::Cash => "Cash",

            BuyerFinancing::Contract => "Contract",

            BuyerFinancing::Conventional => "Conventional",

            BuyerFinancing::FHA => "FHA",

            BuyerFinancing::FHA203b => "FHA 203(b)",

            BuyerFinancing::FHA203k => "FHA 203(k)",

            BuyerFinancing::Other => "Other",

            BuyerFinancing::Private => "Private",

            BuyerFinancing::SellerFinancing => "Seller Financing",

            BuyerFinancing::TrustDeed => "Trust Deed",

            BuyerFinancing::USDA => "USDA",

            BuyerFinancing::VA => "VA",

            BuyerFinancing::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for BuyerFinancing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for BuyerFinancing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_buyer_financing_format {
    use super::BuyerFinancing;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<BuyerFinancing>>,
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
    ) -> Result<Option<Vec<BuyerFinancing>>, D::Error>
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
