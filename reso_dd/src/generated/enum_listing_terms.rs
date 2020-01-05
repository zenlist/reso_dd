// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ListingTerms Lookups](https://ddwiki.reso.org/display/DDW17/ListingTerms+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ListingTerms {
    /// "[1031 Exchange](https://ddwiki.reso.org/display/DDW17/1031+Exchange)": The seller is may be interested in a 1031 exchange as part of the sale.
    _1031Exchange,

    /// "[All Inclusive Trust Deed](https://ddwiki.reso.org/display/DDW17/All+Inclusive+Trust+Deed)": The property is under an all inclusive trust deed.
    AllInclusiveTrustDeed,

    /// "[Assumable](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245384)": The seller is interested in assumable financing.
    Assumable,

    /// "[Cash](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245385)": The seller would like a cash sale.
    Cash,

    /// "[Contract](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245386)": The seller may be interested in an agreement to perform services, provide product, share of income, or some other agreement as the method of payment for the property.
    Contract,

    /// "[Conventional](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245387)": The seller may accept a buyer using conventional financing to purchase the home.
    Conventional,

    /// "[Existing Bonds](https://ddwiki.reso.org/display/DDW17/Existing+Bonds)": The property for sale has existing bonds.
    ExistingBonds,

    /// "[FHA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245389)": The seller may accept a buyer with a loan from an approved provider that follows the guidelines of, and is insured by, the Federal Housing Administration.
    FHA,

    /// "[Land Use Fee](https://ddwiki.reso.org/display/DDW17/Land+Use+Fee)": The listed property has a land use fee.
    LandUseFee,

    /// "[Lease Back](https://ddwiki.reso.org/display/DDW17/Lease+Back)": The seller may be interested in the simultaneous sale of a property with a lease back to the seller, who then becomes the tenant.
    LeaseBack,

    /// "[Lease Option](https://ddwiki.reso.org/display/DDW17/Lease+Option)": The seller may be interested in selling as a lease option to the buyer.
    LeaseOption,

    /// "[Lease Purchase](https://ddwiki.reso.org/display/DDW17/Lease+Purchase)": The seller may be interested in selling as a lease purchase.
    LeasePurchase,

    /// "[Lien Release](https://ddwiki.reso.org/display/DDW17/Lien+Release)": The property for sale may require a lien release.
    LienRelease,

    /// "[Owner May Carry](https://ddwiki.reso.org/display/DDW17/Owner+May+Carry)": The seller may be interested in carrying the mortgage note.
    OwnerMayCarry,

    /// "[Owner Pay Points](https://ddwiki.reso.org/display/DDW17/Owner+Pay+Points)": The seller may carry points.
    OwnerPayPoints,

    /// "[Owner Will Carry](https://ddwiki.reso.org/display/DDW17/Owner+Will+Carry)": The seller will carry points.
    OwnerWillCarry,

    /// "[Private Financing Available](https://ddwiki.reso.org/display/DDW17/Private+Financing+Available)": Financing is provided by a private party.
    PrivateFinancingAvailable,

    /// "[Relocation Property](https://ddwiki.reso.org/display/DDW17/Relocation+Property)": The property for sale is a relocation property.
    RelocationProperty,

    /// "[Seller Equity Share](https://ddwiki.reso.org/display/DDW17/Seller+Equity+Share)": The seller may be interested in investing in an equity share.
    SellerEquityShare,

    /// "[Special Funding](https://ddwiki.reso.org/display/DDW17/Special+Funding)": The seller may be interested in a special funding arrangement.
    SpecialFunding,

    /// "[Submit](https://ddwiki.reso.org/display/DDW17/Submit)": Contact the listing agent for the listing terms.
    Submit,

    /// "[Trade](https://ddwiki.reso.org/display/DDW17/Trade)": The seller may be interested in a trade arrangement.
    Trade,

    /// "[Trust Conveyance](https://ddwiki.reso.org/display/DDW17/Trust+Conveyance)": A trust conveyance (to another trustee) may be involved in the sale of the property.
    TrustConveyance,

    /// "[Trust Deed](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245405)": The seller may accept financing where title of the property is placed with a trustee who secures payment of the loan for a beneficiary.
    TrustDeed,

    /// "[USDA Loan](https://ddwiki.reso.org/display/DDW17/USDA+Loan)": The seller may accept a loan from an approved provider that follows the guidelines of, and is insured by, the US Department of Agriculture.
    USDALoan,

    /// "[VA Loan](https://ddwiki.reso.org/display/DDW17/VA+Loan)": The seller may accept a loan from an approved provider that follows the guidelines of, and is insured by, the US Department of Veteran's Affairs.
    VALoan,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ListingTerms {
    fn from_str(s: &str) -> ListingTerms {
        match s {
            "1031 Exchange" => ListingTerms::_1031Exchange,

            "All Inclusive Trust Deed" => ListingTerms::AllInclusiveTrustDeed,

            "Assumable" => ListingTerms::Assumable,

            "Cash" => ListingTerms::Cash,

            "Contract" => ListingTerms::Contract,

            "Conventional" => ListingTerms::Conventional,

            "Existing Bonds" => ListingTerms::ExistingBonds,

            "FHA" => ListingTerms::FHA,

            "Land Use Fee" => ListingTerms::LandUseFee,

            "Lease Back" => ListingTerms::LeaseBack,

            "Lease Option" => ListingTerms::LeaseOption,

            "Lease Purchase" => ListingTerms::LeasePurchase,

            "Lien Release" => ListingTerms::LienRelease,

            "Owner May Carry" => ListingTerms::OwnerMayCarry,

            "Owner Pay Points" => ListingTerms::OwnerPayPoints,

            "Owner Will Carry" => ListingTerms::OwnerWillCarry,

            "Private Financing Available" => ListingTerms::PrivateFinancingAvailable,

            "Relocation Property" => ListingTerms::RelocationProperty,

            "Seller Equity Share" => ListingTerms::SellerEquityShare,

            "Special Funding" => ListingTerms::SpecialFunding,

            "Submit" => ListingTerms::Submit,

            "Trade" => ListingTerms::Trade,

            "Trust Conveyance" => ListingTerms::TrustConveyance,

            "Trust Deed" => ListingTerms::TrustDeed,

            "USDA Loan" => ListingTerms::USDALoan,

            "VA Loan" => ListingTerms::VALoan,

            _ => ListingTerms::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ListingTerms {
        match s.as_ref() {
            "1031 Exchange" => ListingTerms::_1031Exchange,

            "All Inclusive Trust Deed" => ListingTerms::AllInclusiveTrustDeed,

            "Assumable" => ListingTerms::Assumable,

            "Cash" => ListingTerms::Cash,

            "Contract" => ListingTerms::Contract,

            "Conventional" => ListingTerms::Conventional,

            "Existing Bonds" => ListingTerms::ExistingBonds,

            "FHA" => ListingTerms::FHA,

            "Land Use Fee" => ListingTerms::LandUseFee,

            "Lease Back" => ListingTerms::LeaseBack,

            "Lease Option" => ListingTerms::LeaseOption,

            "Lease Purchase" => ListingTerms::LeasePurchase,

            "Lien Release" => ListingTerms::LienRelease,

            "Owner May Carry" => ListingTerms::OwnerMayCarry,

            "Owner Pay Points" => ListingTerms::OwnerPayPoints,

            "Owner Will Carry" => ListingTerms::OwnerWillCarry,

            "Private Financing Available" => ListingTerms::PrivateFinancingAvailable,

            "Relocation Property" => ListingTerms::RelocationProperty,

            "Seller Equity Share" => ListingTerms::SellerEquityShare,

            "Special Funding" => ListingTerms::SpecialFunding,

            "Submit" => ListingTerms::Submit,

            "Trade" => ListingTerms::Trade,

            "Trust Conveyance" => ListingTerms::TrustConveyance,

            "Trust Deed" => ListingTerms::TrustDeed,

            "USDA Loan" => ListingTerms::USDALoan,

            "VA Loan" => ListingTerms::VALoan,

            _ => ListingTerms::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ListingTerms::_1031Exchange => "1031 Exchange",

            ListingTerms::AllInclusiveTrustDeed => "All Inclusive Trust Deed",

            ListingTerms::Assumable => "Assumable",

            ListingTerms::Cash => "Cash",

            ListingTerms::Contract => "Contract",

            ListingTerms::Conventional => "Conventional",

            ListingTerms::ExistingBonds => "Existing Bonds",

            ListingTerms::FHA => "FHA",

            ListingTerms::LandUseFee => "Land Use Fee",

            ListingTerms::LeaseBack => "Lease Back",

            ListingTerms::LeaseOption => "Lease Option",

            ListingTerms::LeasePurchase => "Lease Purchase",

            ListingTerms::LienRelease => "Lien Release",

            ListingTerms::OwnerMayCarry => "Owner May Carry",

            ListingTerms::OwnerPayPoints => "Owner Pay Points",

            ListingTerms::OwnerWillCarry => "Owner Will Carry",

            ListingTerms::PrivateFinancingAvailable => "Private Financing Available",

            ListingTerms::RelocationProperty => "Relocation Property",

            ListingTerms::SellerEquityShare => "Seller Equity Share",

            ListingTerms::SpecialFunding => "Special Funding",

            ListingTerms::Submit => "Submit",

            ListingTerms::Trade => "Trade",

            ListingTerms::TrustConveyance => "Trust Conveyance",

            ListingTerms::TrustDeed => "Trust Deed",

            ListingTerms::USDALoan => "USDA Loan",

            ListingTerms::VALoan => "VA Loan",

            ListingTerms::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ListingTerms::_1031Exchange => "1031 Exchange".into(),

            ListingTerms::AllInclusiveTrustDeed => "All Inclusive Trust Deed".into(),

            ListingTerms::Assumable => "Assumable".into(),

            ListingTerms::Cash => "Cash".into(),

            ListingTerms::Contract => "Contract".into(),

            ListingTerms::Conventional => "Conventional".into(),

            ListingTerms::ExistingBonds => "Existing Bonds".into(),

            ListingTerms::FHA => "FHA".into(),

            ListingTerms::LandUseFee => "Land Use Fee".into(),

            ListingTerms::LeaseBack => "Lease Back".into(),

            ListingTerms::LeaseOption => "Lease Option".into(),

            ListingTerms::LeasePurchase => "Lease Purchase".into(),

            ListingTerms::LienRelease => "Lien Release".into(),

            ListingTerms::OwnerMayCarry => "Owner May Carry".into(),

            ListingTerms::OwnerPayPoints => "Owner Pay Points".into(),

            ListingTerms::OwnerWillCarry => "Owner Will Carry".into(),

            ListingTerms::PrivateFinancingAvailable => "Private Financing Available".into(),

            ListingTerms::RelocationProperty => "Relocation Property".into(),

            ListingTerms::SellerEquityShare => "Seller Equity Share".into(),

            ListingTerms::SpecialFunding => "Special Funding".into(),

            ListingTerms::Submit => "Submit".into(),

            ListingTerms::Trade => "Trade".into(),

            ListingTerms::TrustConveyance => "Trust Conveyance".into(),

            ListingTerms::TrustDeed => "Trust Deed".into(),

            ListingTerms::USDALoan => "USDA Loan".into(),

            ListingTerms::VALoan => "VA Loan".into(),

            ListingTerms::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ListingTerms::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ListingTerms {
    fn from(s: String) -> ListingTerms {
        match s.as_ref() {
            "1031 Exchange" => ListingTerms::_1031Exchange,

            "All Inclusive Trust Deed" => ListingTerms::AllInclusiveTrustDeed,

            "Assumable" => ListingTerms::Assumable,

            "Cash" => ListingTerms::Cash,

            "Contract" => ListingTerms::Contract,

            "Conventional" => ListingTerms::Conventional,

            "Existing Bonds" => ListingTerms::ExistingBonds,

            "FHA" => ListingTerms::FHA,

            "Land Use Fee" => ListingTerms::LandUseFee,

            "Lease Back" => ListingTerms::LeaseBack,

            "Lease Option" => ListingTerms::LeaseOption,

            "Lease Purchase" => ListingTerms::LeasePurchase,

            "Lien Release" => ListingTerms::LienRelease,

            "Owner May Carry" => ListingTerms::OwnerMayCarry,

            "Owner Pay Points" => ListingTerms::OwnerPayPoints,

            "Owner Will Carry" => ListingTerms::OwnerWillCarry,

            "Private Financing Available" => ListingTerms::PrivateFinancingAvailable,

            "Relocation Property" => ListingTerms::RelocationProperty,

            "Seller Equity Share" => ListingTerms::SellerEquityShare,

            "Special Funding" => ListingTerms::SpecialFunding,

            "Submit" => ListingTerms::Submit,

            "Trade" => ListingTerms::Trade,

            "Trust Conveyance" => ListingTerms::TrustConveyance,

            "Trust Deed" => ListingTerms::TrustDeed,

            "USDA Loan" => ListingTerms::USDALoan,

            "VA Loan" => ListingTerms::VALoan,

            _ => ListingTerms::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ListingTerms {
    fn from(s: &str) -> ListingTerms {
        match s {
            "1031 Exchange" => ListingTerms::_1031Exchange,

            "All Inclusive Trust Deed" => ListingTerms::AllInclusiveTrustDeed,

            "Assumable" => ListingTerms::Assumable,

            "Cash" => ListingTerms::Cash,

            "Contract" => ListingTerms::Contract,

            "Conventional" => ListingTerms::Conventional,

            "Existing Bonds" => ListingTerms::ExistingBonds,

            "FHA" => ListingTerms::FHA,

            "Land Use Fee" => ListingTerms::LandUseFee,

            "Lease Back" => ListingTerms::LeaseBack,

            "Lease Option" => ListingTerms::LeaseOption,

            "Lease Purchase" => ListingTerms::LeasePurchase,

            "Lien Release" => ListingTerms::LienRelease,

            "Owner May Carry" => ListingTerms::OwnerMayCarry,

            "Owner Pay Points" => ListingTerms::OwnerPayPoints,

            "Owner Will Carry" => ListingTerms::OwnerWillCarry,

            "Private Financing Available" => ListingTerms::PrivateFinancingAvailable,

            "Relocation Property" => ListingTerms::RelocationProperty,

            "Seller Equity Share" => ListingTerms::SellerEquityShare,

            "Special Funding" => ListingTerms::SpecialFunding,

            "Submit" => ListingTerms::Submit,

            "Trade" => ListingTerms::Trade,

            "Trust Conveyance" => ListingTerms::TrustConveyance,

            "Trust Deed" => ListingTerms::TrustDeed,

            "USDA Loan" => ListingTerms::USDALoan,

            "VA Loan" => ListingTerms::VALoan,

            _ => ListingTerms::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ListingTerms> for &'a str {
    fn from(s: &'a ListingTerms) -> &'a str {
        match s {
            ListingTerms::_1031Exchange => "1031 Exchange",

            ListingTerms::AllInclusiveTrustDeed => "All Inclusive Trust Deed",

            ListingTerms::Assumable => "Assumable",

            ListingTerms::Cash => "Cash",

            ListingTerms::Contract => "Contract",

            ListingTerms::Conventional => "Conventional",

            ListingTerms::ExistingBonds => "Existing Bonds",

            ListingTerms::FHA => "FHA",

            ListingTerms::LandUseFee => "Land Use Fee",

            ListingTerms::LeaseBack => "Lease Back",

            ListingTerms::LeaseOption => "Lease Option",

            ListingTerms::LeasePurchase => "Lease Purchase",

            ListingTerms::LienRelease => "Lien Release",

            ListingTerms::OwnerMayCarry => "Owner May Carry",

            ListingTerms::OwnerPayPoints => "Owner Pay Points",

            ListingTerms::OwnerWillCarry => "Owner Will Carry",

            ListingTerms::PrivateFinancingAvailable => "Private Financing Available",

            ListingTerms::RelocationProperty => "Relocation Property",

            ListingTerms::SellerEquityShare => "Seller Equity Share",

            ListingTerms::SpecialFunding => "Special Funding",

            ListingTerms::Submit => "Submit",

            ListingTerms::Trade => "Trade",

            ListingTerms::TrustConveyance => "Trust Conveyance",

            ListingTerms::TrustDeed => "Trust Deed",

            ListingTerms::USDALoan => "USDA Loan",

            ListingTerms::VALoan => "VA Loan",

            ListingTerms::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ListingTerms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ListingTerms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
