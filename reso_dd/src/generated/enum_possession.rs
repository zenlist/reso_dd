// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Possession Lookups](https://ddwiki.reso.org/display/DDW17/Possession+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Possession {
    /// "[Close Of Escrow](https://ddwiki.reso.org/display/DDW17/Close+Of+Escrow)": Possession is passed to the buyer at the close of escrow.
    CloseOfEscrow,

    /// "[Close Plus 1 Day](https://ddwiki.reso.org/display/DDW17/Close+Plus+1+Day)": Possession is passed to the buyer one day after the close of escrow.
    ClosePlus1Day,

    /// "[Close Plus 2 Days](https://ddwiki.reso.org/display/DDW17/Close+Plus+2+Days)": Possession is passed to the buyer two days after the close of escrow.
    ClosePlus2Days,

    /// "[Close Plus 3 Days](https://ddwiki.reso.org/display/DDW17/Close+Plus+3+Days)": Possession is passed to the buyer 3 days after the close of escrow.
    ClosePlus3Days,

    /// "[Close Plus 3 to 5 Days](https://ddwiki.reso.org/display/DDW17/Close+Plus+3+to+5+Days)": Possession is passed to the buyer 3 to 5 days after the close of escrow.
    ClosePlus3to5Days,

    /// "[Close Plus 30 Days](https://ddwiki.reso.org/display/DDW17/Close+Plus+30+Days)": Possession is passed to the buyer 30 days after the close of escrow.
    ClosePlus30Days,

    /// "[Negotiable](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245853)": Timing of the passing of possession to the buyer is negotiable.
    Negotiable,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245858)": A type of possession not included in this list.
    Other,

    /// "[Rental Agreement](https://ddwiki.reso.org/display/DDW17/Rental+Agreement)": Possession is stipulated in the rental agreement.
    RentalAgreement,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245855)": See the listing/agent remarks for more information on possession.
    SeeRemarks,

    /// "[Seller Rent Back](https://ddwiki.reso.org/display/DDW17/Seller+Rent+Back)": Possession is determined by the details of the seller rent back agreement, which is in most cases the seller will remain resident.
    SellerRentBack,

    /// "[Subject To Tenant Rights](https://ddwiki.reso.org/display/DDW17/Subject+To+Tenant+Rights)": The terms of the transfer of possession are subject to the rights of the current tenant.
    SubjectToTenantRights,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Possession {
    fn from_str(s: &str) -> Possession {
        match s {
            "Close Of Escrow" => Possession::CloseOfEscrow,

            "Close Plus 1 Day" => Possession::ClosePlus1Day,

            "Close Plus 2 Days" => Possession::ClosePlus2Days,

            "Close Plus 3 Days" => Possession::ClosePlus3Days,

            "Close Plus 3 to 5 Days" => Possession::ClosePlus3to5Days,

            "Close Plus 30 Days" => Possession::ClosePlus30Days,

            "Negotiable" => Possession::Negotiable,

            "Other" => Possession::Other,

            "Rental Agreement" => Possession::RentalAgreement,

            "See Remarks" => Possession::SeeRemarks,

            "Seller Rent Back" => Possession::SellerRentBack,

            "Subject To Tenant Rights" => Possession::SubjectToTenantRights,

            _ => Possession::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Possession {
        match s.as_ref() {
            "Close Of Escrow" => Possession::CloseOfEscrow,

            "Close Plus 1 Day" => Possession::ClosePlus1Day,

            "Close Plus 2 Days" => Possession::ClosePlus2Days,

            "Close Plus 3 Days" => Possession::ClosePlus3Days,

            "Close Plus 3 to 5 Days" => Possession::ClosePlus3to5Days,

            "Close Plus 30 Days" => Possession::ClosePlus30Days,

            "Negotiable" => Possession::Negotiable,

            "Other" => Possession::Other,

            "Rental Agreement" => Possession::RentalAgreement,

            "See Remarks" => Possession::SeeRemarks,

            "Seller Rent Back" => Possession::SellerRentBack,

            "Subject To Tenant Rights" => Possession::SubjectToTenantRights,

            _ => Possession::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Possession::CloseOfEscrow => "Close Of Escrow",

            Possession::ClosePlus1Day => "Close Plus 1 Day",

            Possession::ClosePlus2Days => "Close Plus 2 Days",

            Possession::ClosePlus3Days => "Close Plus 3 Days",

            Possession::ClosePlus3to5Days => "Close Plus 3 to 5 Days",

            Possession::ClosePlus30Days => "Close Plus 30 Days",

            Possession::Negotiable => "Negotiable",

            Possession::Other => "Other",

            Possession::RentalAgreement => "Rental Agreement",

            Possession::SeeRemarks => "See Remarks",

            Possession::SellerRentBack => "Seller Rent Back",

            Possession::SubjectToTenantRights => "Subject To Tenant Rights",

            Possession::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Possession::CloseOfEscrow => "Close Of Escrow".into(),

            Possession::ClosePlus1Day => "Close Plus 1 Day".into(),

            Possession::ClosePlus2Days => "Close Plus 2 Days".into(),

            Possession::ClosePlus3Days => "Close Plus 3 Days".into(),

            Possession::ClosePlus3to5Days => "Close Plus 3 to 5 Days".into(),

            Possession::ClosePlus30Days => "Close Plus 30 Days".into(),

            Possession::Negotiable => "Negotiable".into(),

            Possession::Other => "Other".into(),

            Possession::RentalAgreement => "Rental Agreement".into(),

            Possession::SeeRemarks => "See Remarks".into(),

            Possession::SellerRentBack => "Seller Rent Back".into(),

            Possession::SubjectToTenantRights => "Subject To Tenant Rights".into(),

            Possession::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Possession::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Possession {
    fn from(s: String) -> Possession {
        match s.as_ref() {
            "Close Of Escrow" => Possession::CloseOfEscrow,

            "Close Plus 1 Day" => Possession::ClosePlus1Day,

            "Close Plus 2 Days" => Possession::ClosePlus2Days,

            "Close Plus 3 Days" => Possession::ClosePlus3Days,

            "Close Plus 3 to 5 Days" => Possession::ClosePlus3to5Days,

            "Close Plus 30 Days" => Possession::ClosePlus30Days,

            "Negotiable" => Possession::Negotiable,

            "Other" => Possession::Other,

            "Rental Agreement" => Possession::RentalAgreement,

            "See Remarks" => Possession::SeeRemarks,

            "Seller Rent Back" => Possession::SellerRentBack,

            "Subject To Tenant Rights" => Possession::SubjectToTenantRights,

            _ => Possession::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Possession {
    fn from(s: &str) -> Possession {
        match s {
            "Close Of Escrow" => Possession::CloseOfEscrow,

            "Close Plus 1 Day" => Possession::ClosePlus1Day,

            "Close Plus 2 Days" => Possession::ClosePlus2Days,

            "Close Plus 3 Days" => Possession::ClosePlus3Days,

            "Close Plus 3 to 5 Days" => Possession::ClosePlus3to5Days,

            "Close Plus 30 Days" => Possession::ClosePlus30Days,

            "Negotiable" => Possession::Negotiable,

            "Other" => Possession::Other,

            "Rental Agreement" => Possession::RentalAgreement,

            "See Remarks" => Possession::SeeRemarks,

            "Seller Rent Back" => Possession::SellerRentBack,

            "Subject To Tenant Rights" => Possession::SubjectToTenantRights,

            _ => Possession::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Possession> for &'a str {
    fn from(s: &'a Possession) -> &'a str {
        match s {
            Possession::CloseOfEscrow => "Close Of Escrow",

            Possession::ClosePlus1Day => "Close Plus 1 Day",

            Possession::ClosePlus2Days => "Close Plus 2 Days",

            Possession::ClosePlus3Days => "Close Plus 3 Days",

            Possession::ClosePlus3to5Days => "Close Plus 3 to 5 Days",

            Possession::ClosePlus30Days => "Close Plus 30 Days",

            Possession::Negotiable => "Negotiable",

            Possession::Other => "Other",

            Possession::RentalAgreement => "Rental Agreement",

            Possession::SeeRemarks => "See Remarks",

            Possession::SellerRentBack => "Seller Rent Back",

            Possession::SubjectToTenantRights => "Subject To Tenant Rights",

            Possession::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Possession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Possession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
