// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [CommonInterest Lookups](https://ddwiki.reso.org/display/DDW17/CommonInterest+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CommonInterest {
    /// "[Community Apartment](https://ddwiki.reso.org/display/DDW17/Community+Apartment)": Ownership interest where purchaser receives a partial/fractional interest in the land coupled with the right of exclusive occupancy of an apartment located thereon. The owners elect a governing board which operates and maintains the project.
    CommunityApartment,

    /// "[Condominium](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244396)": Ownership of  an individual unit where each homeowner only owns their individual unit space, and an undivided share in the ownership of common areas or in a common homeowner’s association (HOA). Generally, the ownership of the individual unit is described in a Condominium Plan and usually consists of ownership of the surface of the walls and the space within.  The CC&Rs will detail what building components and other complex amenities are considered part of the common area and will describe the common area or HOA ownership percentages, and maintenance responsibilities.
    Condominium,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244401)": Ownership of an entire parcel or lot that is not in a CID, or not held subject to  any other Common Interest rights.
    None,

    /// "[Planned Development](https://ddwiki.reso.org/display/DDW17/Planned+Development)": Ownership consisting of an individual lot or parcel, generally including the ownership of the land and any structures on the individual lot or parcel.  Owners also receive use right in common areas that are generally owned by a HOA. Some common areas may be reserved for the use of some or all of the individual lot owners. Generally, the CC&Rs will detail the method of management, maintenance, use and control of the common areas and may provide for some control and maintenance of the individual lots.
    PlannedDevelopment,

    /// "[Stock Cooperative](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244399)": Ownership of an interest in a corporation which is formed primarily for the purpose of holding title to improved real property, either in fee simple or for a term of years. All or substantially all of the shareholders receive a right of exclusive occupancy of a portion of the real property, which right is transferable only concurrently with the transfer of the share(s) of stock in the corporation.
    StockCooperative,

    /// "[Timeshare](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244400)": Ownership in a time period or a point system granting possession rights to a unit or occupancy rights at a  property.  The property may be owned either by  a number of individuals on a fractional basis, or may be an interest in a corporation each with the right of possession for a specified time interval. Time-sharing is commonly applied to resort and vacation properties.
    Timeshare,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for CommonInterest {
    fn from_str(s: &str) -> CommonInterest {
        match s {
            "Community Apartment" => CommonInterest::CommunityApartment,

            "Condominium" => CommonInterest::Condominium,

            "None" => CommonInterest::None,

            "Planned Development" => CommonInterest::PlannedDevelopment,

            "Stock Cooperative" => CommonInterest::StockCooperative,

            "Timeshare" => CommonInterest::Timeshare,

            _ => CommonInterest::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> CommonInterest {
        match s.as_ref() {
            "Community Apartment" => CommonInterest::CommunityApartment,

            "Condominium" => CommonInterest::Condominium,

            "None" => CommonInterest::None,

            "Planned Development" => CommonInterest::PlannedDevelopment,

            "Stock Cooperative" => CommonInterest::StockCooperative,

            "Timeshare" => CommonInterest::Timeshare,

            _ => CommonInterest::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            CommonInterest::CommunityApartment => "Community Apartment",

            CommonInterest::Condominium => "Condominium",

            CommonInterest::None => "None",

            CommonInterest::PlannedDevelopment => "Planned Development",

            CommonInterest::StockCooperative => "Stock Cooperative",

            CommonInterest::Timeshare => "Timeshare",

            CommonInterest::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            CommonInterest::CommunityApartment => "Community Apartment".into(),

            CommonInterest::Condominium => "Condominium".into(),

            CommonInterest::None => "None".into(),

            CommonInterest::PlannedDevelopment => "Planned Development".into(),

            CommonInterest::StockCooperative => "Stock Cooperative".into(),

            CommonInterest::Timeshare => "Timeshare".into(),

            CommonInterest::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            CommonInterest::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for CommonInterest {
    fn from(s: String) -> CommonInterest {
        match s.as_ref() {
            "Community Apartment" => CommonInterest::CommunityApartment,

            "Condominium" => CommonInterest::Condominium,

            "None" => CommonInterest::None,

            "Planned Development" => CommonInterest::PlannedDevelopment,

            "Stock Cooperative" => CommonInterest::StockCooperative,

            "Timeshare" => CommonInterest::Timeshare,

            _ => CommonInterest::OpenEnumeration(s),
        }
    }
}

impl From<&str> for CommonInterest {
    fn from(s: &str) -> CommonInterest {
        match s {
            "Community Apartment" => CommonInterest::CommunityApartment,

            "Condominium" => CommonInterest::Condominium,

            "None" => CommonInterest::None,

            "Planned Development" => CommonInterest::PlannedDevelopment,

            "Stock Cooperative" => CommonInterest::StockCooperative,

            "Timeshare" => CommonInterest::Timeshare,

            _ => CommonInterest::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a CommonInterest> for &'a str {
    fn from(s: &'a CommonInterest) -> &'a str {
        match s {
            CommonInterest::CommunityApartment => "Community Apartment",

            CommonInterest::Condominium => "Condominium",

            CommonInterest::None => "None",

            CommonInterest::PlannedDevelopment => "Planned Development",

            CommonInterest::StockCooperative => "Stock Cooperative",

            CommonInterest::Timeshare => "Timeshare",

            CommonInterest::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for CommonInterest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CommonInterest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
