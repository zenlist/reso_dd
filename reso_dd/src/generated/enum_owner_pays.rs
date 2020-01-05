// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OwnerPays Lookups](https://ddwiki.reso.org/display/DDW17/OwnerPays+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OwnerPays {
    /// "[All Utilities](https://ddwiki.reso.org/display/DDW17/All+Utilities)": The owner/lessor pays for all utilities.
    AllUtilities,

    /// "[Association Fees](https://ddwiki.reso.org/display/DDW17/Association+Fees)": The owner/lessor pays for association fees.
    AssociationFees,

    /// "[Cable TV](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245709)": The owner/lessor pays for cable television.
    CableTV,

    /// "[Common Area Maintenance](https://ddwiki.reso.org/display/DDW17/Common+Area+Maintenance)": The owner/lessor pays for common area maintenance.
    CommonAreaMaintenance,

    /// "[Electricity](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245711)": The owner/lessor pays for electricity.
    Electricity,

    /// "[Exterior Maintenance](https://ddwiki.reso.org/display/DDW17/Exterior+Maintenance)": The owner/lessor pays for exterior maintenance.
    ExteriorMaintenance,

    /// "[Gas](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245713)": The owner/lessor pays for gas.
    Gas,

    /// "[Grounds Care](https://ddwiki.reso.org/display/DDW17/Grounds+Care)": The owner/lessor pays for grounds care.
    GroundsCare,

    /// "[Hot Water](https://ddwiki.reso.org/display/DDW17/Hot+Water)": The owner/lessor pays for hot water.
    HotWater,

    /// "[HVAC Maintenance](https://ddwiki.reso.org/display/DDW17/HVAC+Maintenance)": The owner/lessor pays for HVAC maintenance.
    HVACMaintenance,

    /// "[Insurance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245717)": The owner/lessor pays for insurance.
    Insurance,

    /// "[Janitorial Service](https://ddwiki.reso.org/display/DDW17/Janitorial+Service)": The owner/lessor pays for janitorial service .
    JanitorialService,

    /// "[Management](https://ddwiki.reso.org/display/DDW17/Management)": The owner/lessor pays for management.
    Management,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245720)": The owner/lessor pays for no utilities, services, etc.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245721)": The owner/lessor pays for items that are not included in this list.
    Other,

    /// "[Other Tax](https://ddwiki.reso.org/display/DDW17/Other+Tax)": The owner/lessor pays for other taxes.
    OtherTax,

    /// "[Parking Fee](https://ddwiki.reso.org/display/DDW17/Parking+Fee)": The owner/lessor pays for parking fees.
    ParkingFee,

    /// "[Pest Control](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245724)": The owner/lessor pays for pest control.
    PestControl,

    /// "[Pool Maintenance](https://ddwiki.reso.org/display/DDW17/Pool+Maintenance)": The owner/lessor pays for pool maintenance.
    PoolMaintenance,

    /// "[Repairs](https://ddwiki.reso.org/display/DDW17/Repairs)": The owner/lessor pays for repairs.
    Repairs,

    /// "[Roof Maintenance](https://ddwiki.reso.org/display/DDW17/Roof+Maintenance)": The owner/lessor pays for roof maintenance.
    RoofMaintenance,

    /// "[Security](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245728)": The owner/lessor pays for security.
    Security,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245729)": See the listing's remarks for details on what the owner/lessor pays for.
    SeeRemarks,

    /// "[Sewer](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245730)": The owner/lessor pays for sewer.
    Sewer,

    /// "[Snow Removal](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245731)": The owner/lessor pays for snow removal.
    SnowRemoval,

    /// "[Taxes](https://ddwiki.reso.org/display/DDW17/Taxes)": The owner/lessor pays for taxes.
    Taxes,

    /// "[Telephone](https://ddwiki.reso.org/display/DDW17/Telephone)": The owner/lessor pays for telephone.
    Telephone,

    /// "[Trash Collection](https://ddwiki.reso.org/display/DDW17/Trash+Collection)": The owner/lessor pays for trash collection.
    TrashCollection,

    /// "[Water](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245735)": The owner/lessor pays for water.
    Water,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OwnerPays {
    fn from_str(s: &str) -> OwnerPays {
        match s {
            "All Utilities" => OwnerPays::AllUtilities,

            "Association Fees" => OwnerPays::AssociationFees,

            "Cable TV" => OwnerPays::CableTV,

            "Common Area Maintenance" => OwnerPays::CommonAreaMaintenance,

            "Electricity" => OwnerPays::Electricity,

            "Exterior Maintenance" => OwnerPays::ExteriorMaintenance,

            "Gas" => OwnerPays::Gas,

            "Grounds Care" => OwnerPays::GroundsCare,

            "Hot Water" => OwnerPays::HotWater,

            "HVAC Maintenance" => OwnerPays::HVACMaintenance,

            "Insurance" => OwnerPays::Insurance,

            "Janitorial Service" => OwnerPays::JanitorialService,

            "Management" => OwnerPays::Management,

            "None" => OwnerPays::None,

            "Other" => OwnerPays::Other,

            "Other Tax" => OwnerPays::OtherTax,

            "Parking Fee" => OwnerPays::ParkingFee,

            "Pest Control" => OwnerPays::PestControl,

            "Pool Maintenance" => OwnerPays::PoolMaintenance,

            "Repairs" => OwnerPays::Repairs,

            "Roof Maintenance" => OwnerPays::RoofMaintenance,

            "Security" => OwnerPays::Security,

            "See Remarks" => OwnerPays::SeeRemarks,

            "Sewer" => OwnerPays::Sewer,

            "Snow Removal" => OwnerPays::SnowRemoval,

            "Taxes" => OwnerPays::Taxes,

            "Telephone" => OwnerPays::Telephone,

            "Trash Collection" => OwnerPays::TrashCollection,

            "Water" => OwnerPays::Water,

            _ => OwnerPays::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OwnerPays {
        match s.as_ref() {
            "All Utilities" => OwnerPays::AllUtilities,

            "Association Fees" => OwnerPays::AssociationFees,

            "Cable TV" => OwnerPays::CableTV,

            "Common Area Maintenance" => OwnerPays::CommonAreaMaintenance,

            "Electricity" => OwnerPays::Electricity,

            "Exterior Maintenance" => OwnerPays::ExteriorMaintenance,

            "Gas" => OwnerPays::Gas,

            "Grounds Care" => OwnerPays::GroundsCare,

            "Hot Water" => OwnerPays::HotWater,

            "HVAC Maintenance" => OwnerPays::HVACMaintenance,

            "Insurance" => OwnerPays::Insurance,

            "Janitorial Service" => OwnerPays::JanitorialService,

            "Management" => OwnerPays::Management,

            "None" => OwnerPays::None,

            "Other" => OwnerPays::Other,

            "Other Tax" => OwnerPays::OtherTax,

            "Parking Fee" => OwnerPays::ParkingFee,

            "Pest Control" => OwnerPays::PestControl,

            "Pool Maintenance" => OwnerPays::PoolMaintenance,

            "Repairs" => OwnerPays::Repairs,

            "Roof Maintenance" => OwnerPays::RoofMaintenance,

            "Security" => OwnerPays::Security,

            "See Remarks" => OwnerPays::SeeRemarks,

            "Sewer" => OwnerPays::Sewer,

            "Snow Removal" => OwnerPays::SnowRemoval,

            "Taxes" => OwnerPays::Taxes,

            "Telephone" => OwnerPays::Telephone,

            "Trash Collection" => OwnerPays::TrashCollection,

            "Water" => OwnerPays::Water,

            _ => OwnerPays::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OwnerPays::AllUtilities => "All Utilities",

            OwnerPays::AssociationFees => "Association Fees",

            OwnerPays::CableTV => "Cable TV",

            OwnerPays::CommonAreaMaintenance => "Common Area Maintenance",

            OwnerPays::Electricity => "Electricity",

            OwnerPays::ExteriorMaintenance => "Exterior Maintenance",

            OwnerPays::Gas => "Gas",

            OwnerPays::GroundsCare => "Grounds Care",

            OwnerPays::HotWater => "Hot Water",

            OwnerPays::HVACMaintenance => "HVAC Maintenance",

            OwnerPays::Insurance => "Insurance",

            OwnerPays::JanitorialService => "Janitorial Service",

            OwnerPays::Management => "Management",

            OwnerPays::None => "None",

            OwnerPays::Other => "Other",

            OwnerPays::OtherTax => "Other Tax",

            OwnerPays::ParkingFee => "Parking Fee",

            OwnerPays::PestControl => "Pest Control",

            OwnerPays::PoolMaintenance => "Pool Maintenance",

            OwnerPays::Repairs => "Repairs",

            OwnerPays::RoofMaintenance => "Roof Maintenance",

            OwnerPays::Security => "Security",

            OwnerPays::SeeRemarks => "See Remarks",

            OwnerPays::Sewer => "Sewer",

            OwnerPays::SnowRemoval => "Snow Removal",

            OwnerPays::Taxes => "Taxes",

            OwnerPays::Telephone => "Telephone",

            OwnerPays::TrashCollection => "Trash Collection",

            OwnerPays::Water => "Water",

            OwnerPays::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OwnerPays::AllUtilities => "All Utilities".into(),

            OwnerPays::AssociationFees => "Association Fees".into(),

            OwnerPays::CableTV => "Cable TV".into(),

            OwnerPays::CommonAreaMaintenance => "Common Area Maintenance".into(),

            OwnerPays::Electricity => "Electricity".into(),

            OwnerPays::ExteriorMaintenance => "Exterior Maintenance".into(),

            OwnerPays::Gas => "Gas".into(),

            OwnerPays::GroundsCare => "Grounds Care".into(),

            OwnerPays::HotWater => "Hot Water".into(),

            OwnerPays::HVACMaintenance => "HVAC Maintenance".into(),

            OwnerPays::Insurance => "Insurance".into(),

            OwnerPays::JanitorialService => "Janitorial Service".into(),

            OwnerPays::Management => "Management".into(),

            OwnerPays::None => "None".into(),

            OwnerPays::Other => "Other".into(),

            OwnerPays::OtherTax => "Other Tax".into(),

            OwnerPays::ParkingFee => "Parking Fee".into(),

            OwnerPays::PestControl => "Pest Control".into(),

            OwnerPays::PoolMaintenance => "Pool Maintenance".into(),

            OwnerPays::Repairs => "Repairs".into(),

            OwnerPays::RoofMaintenance => "Roof Maintenance".into(),

            OwnerPays::Security => "Security".into(),

            OwnerPays::SeeRemarks => "See Remarks".into(),

            OwnerPays::Sewer => "Sewer".into(),

            OwnerPays::SnowRemoval => "Snow Removal".into(),

            OwnerPays::Taxes => "Taxes".into(),

            OwnerPays::Telephone => "Telephone".into(),

            OwnerPays::TrashCollection => "Trash Collection".into(),

            OwnerPays::Water => "Water".into(),

            OwnerPays::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OwnerPays::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OwnerPays {
    fn from(s: String) -> OwnerPays {
        match s.as_ref() {
            "All Utilities" => OwnerPays::AllUtilities,

            "Association Fees" => OwnerPays::AssociationFees,

            "Cable TV" => OwnerPays::CableTV,

            "Common Area Maintenance" => OwnerPays::CommonAreaMaintenance,

            "Electricity" => OwnerPays::Electricity,

            "Exterior Maintenance" => OwnerPays::ExteriorMaintenance,

            "Gas" => OwnerPays::Gas,

            "Grounds Care" => OwnerPays::GroundsCare,

            "Hot Water" => OwnerPays::HotWater,

            "HVAC Maintenance" => OwnerPays::HVACMaintenance,

            "Insurance" => OwnerPays::Insurance,

            "Janitorial Service" => OwnerPays::JanitorialService,

            "Management" => OwnerPays::Management,

            "None" => OwnerPays::None,

            "Other" => OwnerPays::Other,

            "Other Tax" => OwnerPays::OtherTax,

            "Parking Fee" => OwnerPays::ParkingFee,

            "Pest Control" => OwnerPays::PestControl,

            "Pool Maintenance" => OwnerPays::PoolMaintenance,

            "Repairs" => OwnerPays::Repairs,

            "Roof Maintenance" => OwnerPays::RoofMaintenance,

            "Security" => OwnerPays::Security,

            "See Remarks" => OwnerPays::SeeRemarks,

            "Sewer" => OwnerPays::Sewer,

            "Snow Removal" => OwnerPays::SnowRemoval,

            "Taxes" => OwnerPays::Taxes,

            "Telephone" => OwnerPays::Telephone,

            "Trash Collection" => OwnerPays::TrashCollection,

            "Water" => OwnerPays::Water,

            _ => OwnerPays::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OwnerPays {
    fn from(s: &str) -> OwnerPays {
        match s {
            "All Utilities" => OwnerPays::AllUtilities,

            "Association Fees" => OwnerPays::AssociationFees,

            "Cable TV" => OwnerPays::CableTV,

            "Common Area Maintenance" => OwnerPays::CommonAreaMaintenance,

            "Electricity" => OwnerPays::Electricity,

            "Exterior Maintenance" => OwnerPays::ExteriorMaintenance,

            "Gas" => OwnerPays::Gas,

            "Grounds Care" => OwnerPays::GroundsCare,

            "Hot Water" => OwnerPays::HotWater,

            "HVAC Maintenance" => OwnerPays::HVACMaintenance,

            "Insurance" => OwnerPays::Insurance,

            "Janitorial Service" => OwnerPays::JanitorialService,

            "Management" => OwnerPays::Management,

            "None" => OwnerPays::None,

            "Other" => OwnerPays::Other,

            "Other Tax" => OwnerPays::OtherTax,

            "Parking Fee" => OwnerPays::ParkingFee,

            "Pest Control" => OwnerPays::PestControl,

            "Pool Maintenance" => OwnerPays::PoolMaintenance,

            "Repairs" => OwnerPays::Repairs,

            "Roof Maintenance" => OwnerPays::RoofMaintenance,

            "Security" => OwnerPays::Security,

            "See Remarks" => OwnerPays::SeeRemarks,

            "Sewer" => OwnerPays::Sewer,

            "Snow Removal" => OwnerPays::SnowRemoval,

            "Taxes" => OwnerPays::Taxes,

            "Telephone" => OwnerPays::Telephone,

            "Trash Collection" => OwnerPays::TrashCollection,

            "Water" => OwnerPays::Water,

            _ => OwnerPays::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OwnerPays> for &'a str {
    fn from(s: &'a OwnerPays) -> &'a str {
        match s {
            OwnerPays::AllUtilities => "All Utilities",

            OwnerPays::AssociationFees => "Association Fees",

            OwnerPays::CableTV => "Cable TV",

            OwnerPays::CommonAreaMaintenance => "Common Area Maintenance",

            OwnerPays::Electricity => "Electricity",

            OwnerPays::ExteriorMaintenance => "Exterior Maintenance",

            OwnerPays::Gas => "Gas",

            OwnerPays::GroundsCare => "Grounds Care",

            OwnerPays::HotWater => "Hot Water",

            OwnerPays::HVACMaintenance => "HVAC Maintenance",

            OwnerPays::Insurance => "Insurance",

            OwnerPays::JanitorialService => "Janitorial Service",

            OwnerPays::Management => "Management",

            OwnerPays::None => "None",

            OwnerPays::Other => "Other",

            OwnerPays::OtherTax => "Other Tax",

            OwnerPays::ParkingFee => "Parking Fee",

            OwnerPays::PestControl => "Pest Control",

            OwnerPays::PoolMaintenance => "Pool Maintenance",

            OwnerPays::Repairs => "Repairs",

            OwnerPays::RoofMaintenance => "Roof Maintenance",

            OwnerPays::Security => "Security",

            OwnerPays::SeeRemarks => "See Remarks",

            OwnerPays::Sewer => "Sewer",

            OwnerPays::SnowRemoval => "Snow Removal",

            OwnerPays::Taxes => "Taxes",

            OwnerPays::Telephone => "Telephone",

            OwnerPays::TrashCollection => "Trash Collection",

            OwnerPays::Water => "Water",

            OwnerPays::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OwnerPays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OwnerPays {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
