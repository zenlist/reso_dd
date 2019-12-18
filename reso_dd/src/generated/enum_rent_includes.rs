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

pub(crate) mod option_vec_rent_includes_format {
    use super::RentIncludes;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<RentIncludes>>,
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
    ) -> Result<Option<Vec<RentIncludes>>, D::Error>
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
