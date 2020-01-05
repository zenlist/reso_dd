// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [TenantPays Lookups](https://ddwiki.reso.org/display/DDW17/TenantPays+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TenantPays {
    /// "[All Utilities](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246524)": The tenant pays for all utilities.
    AllUtilities,

    /// "[Association Fees](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246525)": The tenant pays for association fees.
    AssociationFees,

    /// "[Cable TV](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246526)": The tenant pays for cable TV.
    CableTV,

    /// "[Common Area Maintenance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246527)": The tenant pays for common area maintenance.
    CommonAreaMaintenance,

    /// "[Electricity](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246528)": The tenant pays for electricity.
    Electricity,

    /// "[Exterior Maintenance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246529)": The tenant pays for exterior maintenance.
    ExteriorMaintenance,

    /// "[Gas](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246530)": The tenant pays for gas.
    Gas,

    /// "[Grounds Care](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246531)": The tenant pays for grounds care.
    GroundsCare,

    /// "[Hot Water](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246532)": The tenant pays for hot water.
    HotWater,

    /// "[HVAC Maintenance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246533)": The tenant pays for HVAC maintenance.
    HVACMaintenance,

    /// "[Insurance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246534)": The tenant pays for insurance.
    Insurance,

    /// "[Janitorial Service](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246535)": The tenant pays for janitorial service.
    JanitorialService,

    /// "[Management](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246536)": The tenant pays for management.
    Management,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246537)": The tenant pays for no other utilities, services, etc.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246538)": The tenant pays for items other than those in this list.
    Other,

    /// "[Other Tax](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246539)": The tenant pays for other taxes.
    OtherTax,

    /// "[Parking Fee](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246540)": The tenant pays for parking fees.
    ParkingFee,

    /// "[Pest Control](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246541)": The tenant pays for pest control.
    PestControl,

    /// "[Pool Maintenance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246542)": The tenant pays for pool maintenance.
    PoolMaintenance,

    /// "[Repairs](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246543)": The tenant pays for repairs.
    Repairs,

    /// "[Roof](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246544)": The tenant pays for roof maintenance.
    Roof,

    /// "[Security](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246545)": The tenant pays for security.
    Security,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246546)": See the listing's remarks for details on what they tenant pays for.
    SeeRemarks,

    /// "[Sewer](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246547)": The tenant pays for sewer.
    Sewer,

    /// "[Snow Removal](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246548)": The tenant pays for snow removal.
    SnowRemoval,

    /// "[Taxes](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246549)": The tenant pays for taxes.
    Taxes,

    /// "[Telephone](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246550)": The tenant pays for telephone.
    Telephone,

    /// "[Trash Collection](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246551)": The tenant pays for trash collection.
    TrashCollection,

    /// "[Water](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246552)": The tenant pays for water.
    Water,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for TenantPays {
    fn from_str(s: &str) -> TenantPays {
        match s {
            "All Utilities" => TenantPays::AllUtilities,

            "Association Fees" => TenantPays::AssociationFees,

            "Cable TV" => TenantPays::CableTV,

            "Common Area Maintenance" => TenantPays::CommonAreaMaintenance,

            "Electricity" => TenantPays::Electricity,

            "Exterior Maintenance" => TenantPays::ExteriorMaintenance,

            "Gas" => TenantPays::Gas,

            "Grounds Care" => TenantPays::GroundsCare,

            "Hot Water" => TenantPays::HotWater,

            "HVAC Maintenance" => TenantPays::HVACMaintenance,

            "Insurance" => TenantPays::Insurance,

            "Janitorial Service" => TenantPays::JanitorialService,

            "Management" => TenantPays::Management,

            "None" => TenantPays::None,

            "Other" => TenantPays::Other,

            "Other Tax" => TenantPays::OtherTax,

            "Parking Fee" => TenantPays::ParkingFee,

            "Pest Control" => TenantPays::PestControl,

            "Pool Maintenance" => TenantPays::PoolMaintenance,

            "Repairs" => TenantPays::Repairs,

            "Roof" => TenantPays::Roof,

            "Security" => TenantPays::Security,

            "See Remarks" => TenantPays::SeeRemarks,

            "Sewer" => TenantPays::Sewer,

            "Snow Removal" => TenantPays::SnowRemoval,

            "Taxes" => TenantPays::Taxes,

            "Telephone" => TenantPays::Telephone,

            "Trash Collection" => TenantPays::TrashCollection,

            "Water" => TenantPays::Water,

            _ => TenantPays::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> TenantPays {
        match s.as_ref() {
            "All Utilities" => TenantPays::AllUtilities,

            "Association Fees" => TenantPays::AssociationFees,

            "Cable TV" => TenantPays::CableTV,

            "Common Area Maintenance" => TenantPays::CommonAreaMaintenance,

            "Electricity" => TenantPays::Electricity,

            "Exterior Maintenance" => TenantPays::ExteriorMaintenance,

            "Gas" => TenantPays::Gas,

            "Grounds Care" => TenantPays::GroundsCare,

            "Hot Water" => TenantPays::HotWater,

            "HVAC Maintenance" => TenantPays::HVACMaintenance,

            "Insurance" => TenantPays::Insurance,

            "Janitorial Service" => TenantPays::JanitorialService,

            "Management" => TenantPays::Management,

            "None" => TenantPays::None,

            "Other" => TenantPays::Other,

            "Other Tax" => TenantPays::OtherTax,

            "Parking Fee" => TenantPays::ParkingFee,

            "Pest Control" => TenantPays::PestControl,

            "Pool Maintenance" => TenantPays::PoolMaintenance,

            "Repairs" => TenantPays::Repairs,

            "Roof" => TenantPays::Roof,

            "Security" => TenantPays::Security,

            "See Remarks" => TenantPays::SeeRemarks,

            "Sewer" => TenantPays::Sewer,

            "Snow Removal" => TenantPays::SnowRemoval,

            "Taxes" => TenantPays::Taxes,

            "Telephone" => TenantPays::Telephone,

            "Trash Collection" => TenantPays::TrashCollection,

            "Water" => TenantPays::Water,

            _ => TenantPays::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            TenantPays::AllUtilities => "All Utilities",

            TenantPays::AssociationFees => "Association Fees",

            TenantPays::CableTV => "Cable TV",

            TenantPays::CommonAreaMaintenance => "Common Area Maintenance",

            TenantPays::Electricity => "Electricity",

            TenantPays::ExteriorMaintenance => "Exterior Maintenance",

            TenantPays::Gas => "Gas",

            TenantPays::GroundsCare => "Grounds Care",

            TenantPays::HotWater => "Hot Water",

            TenantPays::HVACMaintenance => "HVAC Maintenance",

            TenantPays::Insurance => "Insurance",

            TenantPays::JanitorialService => "Janitorial Service",

            TenantPays::Management => "Management",

            TenantPays::None => "None",

            TenantPays::Other => "Other",

            TenantPays::OtherTax => "Other Tax",

            TenantPays::ParkingFee => "Parking Fee",

            TenantPays::PestControl => "Pest Control",

            TenantPays::PoolMaintenance => "Pool Maintenance",

            TenantPays::Repairs => "Repairs",

            TenantPays::Roof => "Roof",

            TenantPays::Security => "Security",

            TenantPays::SeeRemarks => "See Remarks",

            TenantPays::Sewer => "Sewer",

            TenantPays::SnowRemoval => "Snow Removal",

            TenantPays::Taxes => "Taxes",

            TenantPays::Telephone => "Telephone",

            TenantPays::TrashCollection => "Trash Collection",

            TenantPays::Water => "Water",

            TenantPays::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            TenantPays::AllUtilities => "All Utilities".into(),

            TenantPays::AssociationFees => "Association Fees".into(),

            TenantPays::CableTV => "Cable TV".into(),

            TenantPays::CommonAreaMaintenance => "Common Area Maintenance".into(),

            TenantPays::Electricity => "Electricity".into(),

            TenantPays::ExteriorMaintenance => "Exterior Maintenance".into(),

            TenantPays::Gas => "Gas".into(),

            TenantPays::GroundsCare => "Grounds Care".into(),

            TenantPays::HotWater => "Hot Water".into(),

            TenantPays::HVACMaintenance => "HVAC Maintenance".into(),

            TenantPays::Insurance => "Insurance".into(),

            TenantPays::JanitorialService => "Janitorial Service".into(),

            TenantPays::Management => "Management".into(),

            TenantPays::None => "None".into(),

            TenantPays::Other => "Other".into(),

            TenantPays::OtherTax => "Other Tax".into(),

            TenantPays::ParkingFee => "Parking Fee".into(),

            TenantPays::PestControl => "Pest Control".into(),

            TenantPays::PoolMaintenance => "Pool Maintenance".into(),

            TenantPays::Repairs => "Repairs".into(),

            TenantPays::Roof => "Roof".into(),

            TenantPays::Security => "Security".into(),

            TenantPays::SeeRemarks => "See Remarks".into(),

            TenantPays::Sewer => "Sewer".into(),

            TenantPays::SnowRemoval => "Snow Removal".into(),

            TenantPays::Taxes => "Taxes".into(),

            TenantPays::Telephone => "Telephone".into(),

            TenantPays::TrashCollection => "Trash Collection".into(),

            TenantPays::Water => "Water".into(),

            TenantPays::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            TenantPays::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for TenantPays {
    fn from(s: String) -> TenantPays {
        match s.as_ref() {
            "All Utilities" => TenantPays::AllUtilities,

            "Association Fees" => TenantPays::AssociationFees,

            "Cable TV" => TenantPays::CableTV,

            "Common Area Maintenance" => TenantPays::CommonAreaMaintenance,

            "Electricity" => TenantPays::Electricity,

            "Exterior Maintenance" => TenantPays::ExteriorMaintenance,

            "Gas" => TenantPays::Gas,

            "Grounds Care" => TenantPays::GroundsCare,

            "Hot Water" => TenantPays::HotWater,

            "HVAC Maintenance" => TenantPays::HVACMaintenance,

            "Insurance" => TenantPays::Insurance,

            "Janitorial Service" => TenantPays::JanitorialService,

            "Management" => TenantPays::Management,

            "None" => TenantPays::None,

            "Other" => TenantPays::Other,

            "Other Tax" => TenantPays::OtherTax,

            "Parking Fee" => TenantPays::ParkingFee,

            "Pest Control" => TenantPays::PestControl,

            "Pool Maintenance" => TenantPays::PoolMaintenance,

            "Repairs" => TenantPays::Repairs,

            "Roof" => TenantPays::Roof,

            "Security" => TenantPays::Security,

            "See Remarks" => TenantPays::SeeRemarks,

            "Sewer" => TenantPays::Sewer,

            "Snow Removal" => TenantPays::SnowRemoval,

            "Taxes" => TenantPays::Taxes,

            "Telephone" => TenantPays::Telephone,

            "Trash Collection" => TenantPays::TrashCollection,

            "Water" => TenantPays::Water,

            _ => TenantPays::OpenEnumeration(s),
        }
    }
}

impl From<&str> for TenantPays {
    fn from(s: &str) -> TenantPays {
        match s {
            "All Utilities" => TenantPays::AllUtilities,

            "Association Fees" => TenantPays::AssociationFees,

            "Cable TV" => TenantPays::CableTV,

            "Common Area Maintenance" => TenantPays::CommonAreaMaintenance,

            "Electricity" => TenantPays::Electricity,

            "Exterior Maintenance" => TenantPays::ExteriorMaintenance,

            "Gas" => TenantPays::Gas,

            "Grounds Care" => TenantPays::GroundsCare,

            "Hot Water" => TenantPays::HotWater,

            "HVAC Maintenance" => TenantPays::HVACMaintenance,

            "Insurance" => TenantPays::Insurance,

            "Janitorial Service" => TenantPays::JanitorialService,

            "Management" => TenantPays::Management,

            "None" => TenantPays::None,

            "Other" => TenantPays::Other,

            "Other Tax" => TenantPays::OtherTax,

            "Parking Fee" => TenantPays::ParkingFee,

            "Pest Control" => TenantPays::PestControl,

            "Pool Maintenance" => TenantPays::PoolMaintenance,

            "Repairs" => TenantPays::Repairs,

            "Roof" => TenantPays::Roof,

            "Security" => TenantPays::Security,

            "See Remarks" => TenantPays::SeeRemarks,

            "Sewer" => TenantPays::Sewer,

            "Snow Removal" => TenantPays::SnowRemoval,

            "Taxes" => TenantPays::Taxes,

            "Telephone" => TenantPays::Telephone,

            "Trash Collection" => TenantPays::TrashCollection,

            "Water" => TenantPays::Water,

            _ => TenantPays::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a TenantPays> for &'a str {
    fn from(s: &'a TenantPays) -> &'a str {
        match s {
            TenantPays::AllUtilities => "All Utilities",

            TenantPays::AssociationFees => "Association Fees",

            TenantPays::CableTV => "Cable TV",

            TenantPays::CommonAreaMaintenance => "Common Area Maintenance",

            TenantPays::Electricity => "Electricity",

            TenantPays::ExteriorMaintenance => "Exterior Maintenance",

            TenantPays::Gas => "Gas",

            TenantPays::GroundsCare => "Grounds Care",

            TenantPays::HotWater => "Hot Water",

            TenantPays::HVACMaintenance => "HVAC Maintenance",

            TenantPays::Insurance => "Insurance",

            TenantPays::JanitorialService => "Janitorial Service",

            TenantPays::Management => "Management",

            TenantPays::None => "None",

            TenantPays::Other => "Other",

            TenantPays::OtherTax => "Other Tax",

            TenantPays::ParkingFee => "Parking Fee",

            TenantPays::PestControl => "Pest Control",

            TenantPays::PoolMaintenance => "Pool Maintenance",

            TenantPays::Repairs => "Repairs",

            TenantPays::Roof => "Roof",

            TenantPays::Security => "Security",

            TenantPays::SeeRemarks => "See Remarks",

            TenantPays::Sewer => "Sewer",

            TenantPays::SnowRemoval => "Snow Removal",

            TenantPays::Taxes => "Taxes",

            TenantPays::Telephone => "Telephone",

            TenantPays::TrashCollection => "Trash Collection",

            TenantPays::Water => "Water",

            TenantPays::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for TenantPays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TenantPays {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
