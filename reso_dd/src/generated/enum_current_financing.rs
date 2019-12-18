// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [CurrentFinancing Lookups](https://ddwiki.reso.org/display/DDW17/CurrentFinancing+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CurrentFinancing {
    /// "[Assumable](https://ddwiki.reso.org/display/DDW17/Assumable)": The financing currently in place may be assumed.
    Assumable,

    /// "[Contract](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244381)": The purchase of a property involves an agreement to perform services, provide product, share of income, or some other agreement as the method of payment for the property.
    Contract,

    /// "[Conventional](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244382)": The buyer is using conventional financing to purchase the home.
    Conventional,

    /// "[FHA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244383)": A loan from an approved provider that follows the guidelines of, and is insured by, the Federal Housing Administration.
    FHA,

    /// "[FHA 203(b)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244384)": The basic home mortgage loan from an approved provider that follows the guidelines of, and is insured by, the Federal Housing Administration.
    FHA203b,

    /// "[FHA 203(k)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244385)": A loan, for the rehabilitation and repair of single family residence, from an approved provider that follows the guidelines of, and is insured by, the Federal Housing Administration.
    FHA203k,

    /// "[Leased Renewables](https://ddwiki.reso.org/display/DDW17/Leased+Renewables)": Definition:  Renewable system (i.e., solar or wind) belonging to a third-party is installed on a customer’s property at little or no cost to the property owner. Property owner has entered an agreement to pay for the lease of the equipment. Structure:  Fee model is based on equipment to generate power, not power itself.  Transfer: If requirements are met it may be possible to transfer lease from home seller to buyer with the approval of the system owner. If being transferred, see Fannie Mae guidelines for more info: since lease is for equipment it must be included in the buyers DTI calculation.  If a buyer cannot or will not adopt a lease, then the seller is required to pay the remaining contract amount in full and the system would be removed.Real or Personal Property:  Renewable system is typically considered personal property.  All tax credits as well as maintenance responsibilities, etc. belong to the third-party owner. Some leases require the homeowner to pay the personal property tax.  Terms:  Lease should provide items homeowner is responsible for paying. In a solar lease model, a customer will sign a contract with an installer/developer and pay for the solar energy equipment over a period of years or decades. Solar leases can be structured so customers pay no up-front costs, some up-front costs (partially prepaid) or can be fully prepaid (the leasing company is then able to depreciate the equipment over time and offers a lower fee for pre-paid leases).  The homeowner may have the option to purchase the system during or at the end of the lease term (most leases are for 20-year terms) at its fair market value or terms set in the original lease contract.
    LeasedRenewables,

    /// "[None](https://ddwiki.reso.org/display/DDW17/None)": The is no current financing on the property.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244388)": The current owner is using another form of financing that is not included in the options provided in this list.
    Other,

    /// "[Power Purchase Agreement](https://ddwiki.reso.org/display/DDW17/Power+Purchase+Agreement)": Definition:  Renewable system belonging to a third-party is installed on a customer’s property at little or no up-front cost to the property owner. Property owner is in an agreement to buy all the power generated at a fixed rate whether it is actually used or not.  This is typically lower than the local utility rate. Structure:  Fee model is based on power, not equipment to generate it. Transfer: A PPA may be transferred to a homebuyer with approval from the system owner. If a buyer will not adopt a PPA, then the seller is required to pay the remaining contract amount in full and the system would be removed. If being transferred, see Fannie Mae guidelines for more info:  portion of the purchase towards equipment must be included in the buyers DTI calculation.  Real or Personal Property: Renewable system is typically considered personal property.  All tax credits as well as maintenance responsibilities, etc. belong to the third-party owner. Terms: The customer agrees to purchase all energy produced by the system.  At the end of the PPA contract term (usually between 10-25 years), property owners can extend the contract and even buy the solar energy system from the developer based on terms in the contract.Additional info on SEIA.org.
    PowerPurchaseAgreement,

    /// "[Private](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244390)": Financing is provided by a private party.
    Private,

    /// "[Property-Assessed Clean Energy](https://ddwiki.reso.org/display/DDW17/Property-Assessed+Clean+Energy)": Definition: Property-assessed clean energy (PACE) is a financing tool for property owners to fund energy or water efficiency or renewable energy installations. Structure: PACE is a secured loan that attaches to the property as a voluntary assessment on the property taxes.  To be eligible for a PACE loan, the building must be located in a local jurisdiction where the City or County has passed a resolution to participate in a PACE program. PACE financing is often a public-private partnership between a private finance company and a public agency.Transfer? The loan may be transferred to the new owner upon sale of the property with the approval of the system owner. Real or Personal Property? If seller owns and not a third-party, then renewable system is typically considered real property and tax credits and maintenance requirements, etc. belong to the property owner.  Terms:  Loan terms can vary between 5 and 30 years. Some programs also allow PACE to finance solar leases and power purchase agreements (PPAs). Due to the complexities of PACE financing, NAR and states like California have provided statements which are worth further consideration. NAR -  http://www.realtor.org/articles/administration-issues-pace-guidance; CAR - http://www.car.org/newsstand/newsreleases/2015releases/fhapacelien
    PropertyAssessedCleanEnergy,

    /// "[Trust Deed](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244392)": Financing where title of the property is placed with a trustee who secures payment of the loan for a beneficiary.
    TrustDeed,

    /// "[USDA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244393)": A loan from an approved provider that follows the guidelines of, and is insured by, the US Department of Agriculture.
    USDA,

    /// "[VA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244394)": A loan from an approved provider that follows the guidelines of, and is insured by, the US Department of Veteran's Affairs.
    VA,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for CurrentFinancing {
    fn from(s: String) -> CurrentFinancing {
        match s.as_ref() {
            "Assumable" => CurrentFinancing::Assumable,

            "Contract" => CurrentFinancing::Contract,

            "Conventional" => CurrentFinancing::Conventional,

            "FHA" => CurrentFinancing::FHA,

            "FHA 203(b)" => CurrentFinancing::FHA203b,

            "FHA 203(k)" => CurrentFinancing::FHA203k,

            "Leased Renewables" => CurrentFinancing::LeasedRenewables,

            "None" => CurrentFinancing::None,

            "Other" => CurrentFinancing::Other,

            "Power Purchase Agreement" => CurrentFinancing::PowerPurchaseAgreement,

            "Private" => CurrentFinancing::Private,

            "Property-Assessed Clean Energy" => CurrentFinancing::PropertyAssessedCleanEnergy,

            "Trust Deed" => CurrentFinancing::TrustDeed,

            "USDA" => CurrentFinancing::USDA,

            "VA" => CurrentFinancing::VA,

            _ => CurrentFinancing::OpenEnumeration(s),
        }
    }
}

impl From<&str> for CurrentFinancing {
    fn from(s: &str) -> CurrentFinancing {
        match s {
            "Assumable" => CurrentFinancing::Assumable,

            "Contract" => CurrentFinancing::Contract,

            "Conventional" => CurrentFinancing::Conventional,

            "FHA" => CurrentFinancing::FHA,

            "FHA 203(b)" => CurrentFinancing::FHA203b,

            "FHA 203(k)" => CurrentFinancing::FHA203k,

            "Leased Renewables" => CurrentFinancing::LeasedRenewables,

            "None" => CurrentFinancing::None,

            "Other" => CurrentFinancing::Other,

            "Power Purchase Agreement" => CurrentFinancing::PowerPurchaseAgreement,

            "Private" => CurrentFinancing::Private,

            "Property-Assessed Clean Energy" => CurrentFinancing::PropertyAssessedCleanEnergy,

            "Trust Deed" => CurrentFinancing::TrustDeed,

            "USDA" => CurrentFinancing::USDA,

            "VA" => CurrentFinancing::VA,

            _ => CurrentFinancing::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a CurrentFinancing> for &'a str {
    fn from(s: &'a CurrentFinancing) -> &'a str {
        match s {
            CurrentFinancing::Assumable => "Assumable",

            CurrentFinancing::Contract => "Contract",

            CurrentFinancing::Conventional => "Conventional",

            CurrentFinancing::FHA => "FHA",

            CurrentFinancing::FHA203b => "FHA 203(b)",

            CurrentFinancing::FHA203k => "FHA 203(k)",

            CurrentFinancing::LeasedRenewables => "Leased Renewables",

            CurrentFinancing::None => "None",

            CurrentFinancing::Other => "Other",

            CurrentFinancing::PowerPurchaseAgreement => "Power Purchase Agreement",

            CurrentFinancing::Private => "Private",

            CurrentFinancing::PropertyAssessedCleanEnergy => "Property-Assessed Clean Energy",

            CurrentFinancing::TrustDeed => "Trust Deed",

            CurrentFinancing::USDA => "USDA",

            CurrentFinancing::VA => "VA",

            CurrentFinancing::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for CurrentFinancing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CurrentFinancing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_current_financing_format {
    use super::CurrentFinancing;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<CurrentFinancing>>,
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
    ) -> Result<Option<Vec<CurrentFinancing>>, D::Error>
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
