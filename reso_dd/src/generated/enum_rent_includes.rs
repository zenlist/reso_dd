// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [RentIncludes Lookups](https://ddwiki.reso.org/display/DDW17/RentIncludes+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RentIncludes {
    /// "[All Utilities](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246093)": Rent for the dwelling includes all utilities.
    AllUtilities,

    /// "[Cable TV](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246094)": Rent for the dwelling includes cable TV.
    CableTV,

    /// "[Electricity](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246095)": Rent for the dwelling includes electricity.
    Electricity,

    /// "[Gardener](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246096)": Rent for the dwelling includes gardener.
    Gardener,

    /// "[Gas](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246097)": Rent for the dwelling includes gas.
    Gas,

    /// "[Internet](https://ddwiki.reso.org/display/DDW17/Internet)": Rent for the dwelling includes internet.
    Internet,

    /// "[Management](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246099)": Rent for the dwelling includes management.
    Management,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246100)": Rent for the dwelling does not include other potential costs such as utilities, management, services, etc.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246105)": An item of what rent includes not in this list.
    Other,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246101)": See the listing's remarks for details about things included in the rent.
    SeeRemarks,

    /// "[Sewer](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246102)": Rent for the dwelling includes sewer.
    Sewer,

    /// "[Trash Collection](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246103)": Rent for the dwelling includes trash collection.
    TrashCollection,

    /// "[Water](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246104)": Rent for the dwelling includes water.
    Water,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for RentIncludes {
    fn from_str(s: &str) -> RentIncludes {
        match s {
            "All Utilities" => RentIncludes::AllUtilities,

            "Cable TV" => RentIncludes::CableTV,

            "Electricity" => RentIncludes::Electricity,

            "Gardener" => RentIncludes::Gardener,

            "Gas" => RentIncludes::Gas,

            "Internet" => RentIncludes::Internet,

            "Management" => RentIncludes::Management,

            "None" => RentIncludes::None,

            "Other" => RentIncludes::Other,

            "See Remarks" => RentIncludes::SeeRemarks,

            "Sewer" => RentIncludes::Sewer,

            "Trash Collection" => RentIncludes::TrashCollection,

            "Water" => RentIncludes::Water,

            _ => RentIncludes::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> RentIncludes {
        match s.as_ref() {
            "All Utilities" => RentIncludes::AllUtilities,

            "Cable TV" => RentIncludes::CableTV,

            "Electricity" => RentIncludes::Electricity,

            "Gardener" => RentIncludes::Gardener,

            "Gas" => RentIncludes::Gas,

            "Internet" => RentIncludes::Internet,

            "Management" => RentIncludes::Management,

            "None" => RentIncludes::None,

            "Other" => RentIncludes::Other,

            "See Remarks" => RentIncludes::SeeRemarks,

            "Sewer" => RentIncludes::Sewer,

            "Trash Collection" => RentIncludes::TrashCollection,

            "Water" => RentIncludes::Water,

            _ => RentIncludes::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            RentIncludes::AllUtilities => "All Utilities",

            RentIncludes::CableTV => "Cable TV",

            RentIncludes::Electricity => "Electricity",

            RentIncludes::Gardener => "Gardener",

            RentIncludes::Gas => "Gas",

            RentIncludes::Internet => "Internet",

            RentIncludes::Management => "Management",

            RentIncludes::None => "None",

            RentIncludes::Other => "Other",

            RentIncludes::SeeRemarks => "See Remarks",

            RentIncludes::Sewer => "Sewer",

            RentIncludes::TrashCollection => "Trash Collection",

            RentIncludes::Water => "Water",

            RentIncludes::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            RentIncludes::AllUtilities => "All Utilities".into(),

            RentIncludes::CableTV => "Cable TV".into(),

            RentIncludes::Electricity => "Electricity".into(),

            RentIncludes::Gardener => "Gardener".into(),

            RentIncludes::Gas => "Gas".into(),

            RentIncludes::Internet => "Internet".into(),

            RentIncludes::Management => "Management".into(),

            RentIncludes::None => "None".into(),

            RentIncludes::Other => "Other".into(),

            RentIncludes::SeeRemarks => "See Remarks".into(),

            RentIncludes::Sewer => "Sewer".into(),

            RentIncludes::TrashCollection => "Trash Collection".into(),

            RentIncludes::Water => "Water".into(),

            RentIncludes::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            RentIncludes::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for RentIncludes {
    fn from(s: String) -> RentIncludes {
        match s.as_ref() {
            "All Utilities" => RentIncludes::AllUtilities,

            "Cable TV" => RentIncludes::CableTV,

            "Electricity" => RentIncludes::Electricity,

            "Gardener" => RentIncludes::Gardener,

            "Gas" => RentIncludes::Gas,

            "Internet" => RentIncludes::Internet,

            "Management" => RentIncludes::Management,

            "None" => RentIncludes::None,

            "Other" => RentIncludes::Other,

            "See Remarks" => RentIncludes::SeeRemarks,

            "Sewer" => RentIncludes::Sewer,

            "Trash Collection" => RentIncludes::TrashCollection,

            "Water" => RentIncludes::Water,

            _ => RentIncludes::OpenEnumeration(s),
        }
    }
}

impl From<&str> for RentIncludes {
    fn from(s: &str) -> RentIncludes {
        match s {
            "All Utilities" => RentIncludes::AllUtilities,

            "Cable TV" => RentIncludes::CableTV,

            "Electricity" => RentIncludes::Electricity,

            "Gardener" => RentIncludes::Gardener,

            "Gas" => RentIncludes::Gas,

            "Internet" => RentIncludes::Internet,

            "Management" => RentIncludes::Management,

            "None" => RentIncludes::None,

            "Other" => RentIncludes::Other,

            "See Remarks" => RentIncludes::SeeRemarks,

            "Sewer" => RentIncludes::Sewer,

            "Trash Collection" => RentIncludes::TrashCollection,

            "Water" => RentIncludes::Water,

            _ => RentIncludes::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a RentIncludes> for &'a str {
    fn from(s: &'a RentIncludes) -> &'a str {
        match s {
            RentIncludes::AllUtilities => "All Utilities",

            RentIncludes::CableTV => "Cable TV",

            RentIncludes::Electricity => "Electricity",

            RentIncludes::Gardener => "Gardener",

            RentIncludes::Gas => "Gas",

            RentIncludes::Internet => "Internet",

            RentIncludes::Management => "Management",

            RentIncludes::None => "None",

            RentIncludes::Other => "Other",

            RentIncludes::SeeRemarks => "See Remarks",

            RentIncludes::Sewer => "Sewer",

            RentIncludes::TrashCollection => "Trash Collection",

            RentIncludes::Water => "Water",

            RentIncludes::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for RentIncludes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RentIncludes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
