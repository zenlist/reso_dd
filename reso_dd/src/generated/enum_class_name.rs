// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ClassName Lookups](https://ddwiki.reso.org/display/DDW17/ClassName+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClassName {
    /// "[Business Opportunity](https://ddwiki.reso.org/display/DDW17/Business+Opportunity)": The class, sometimes known as property type, is a business for sale.
    BusinessOpportunity,

    /// "[Commercial Lease](https://ddwiki.reso.org/display/DDW17/Commercial+Lease)": The class, sometimes known as property type, is a commercial property for lease.
    CommercialLease,

    /// "[Commercial Sale](https://ddwiki.reso.org/display/DDW17/Commercial+Sale)": The class, sometimes known as property type, is a commercial property for sale.
    CommercialSale,

    /// "[Contacts](https://ddwiki.reso.org/display/DDW17/Contacts)": The class is the collection of the member's contacts/clients.
    Contacts,

    /// "[Cross Property](https://ddwiki.reso.org/display/DDW17/Cross+Property)": The class, sometimes known as property type, is a collection of all listing property types.
    CrossProperty,

    /// "[Farm](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244043)": The class, sometimes known as property type, is a farm.
    Farm,

    /// "[History Transactional](https://ddwiki.reso.org/display/DDW17/History+Transactional)": The class is the transactional history of another class.
    HistoryTransactional,

    /// "[Land](https://ddwiki.reso.org/display/DDW17/Land)": The class, sometimes known as property type, is land for sale or lease.
    Land,

    /// "[Manufactured In Park](https://ddwiki.reso.org/display/DDW17/Manufactured+In+Park)": The class, sometimes known as property type, is a manufactured or mobile home in a mobile park.
    ManufacturedInPark,

    /// "[Media](https://ddwiki.reso.org/display/DDW17/Media)": The class is one that contains records referencing media files.
    Media,

    /// "[Member](https://ddwiki.reso.org/display/DDW17/Member)": The class containing member records.
    Member,

    /// "[Office](https://ddwiki.reso.org/display/DDW17/Office)": The class containing office records.
    Office,

    /// "[Open House](https://ddwiki.reso.org/display/DDW17/Open+House)": The class containing Open House records.
    OpenHouse,

    /// "[Residential](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244051)": The class, sometimes known as property type, is residential property for sale.
    Residential,

    /// "[Residential Income](https://ddwiki.reso.org/display/DDW17/Residential+Income)": The class, sometimes known as property type, is income or multi-family property for sale.
    ResidentialIncome,

    /// "[Residential Lease](https://ddwiki.reso.org/display/DDW17/Residential+Lease)": The class, sometimes known as property type, is residential property for lease.
    ResidentialLease,

    /// "[Saved Search](https://ddwiki.reso.org/display/DDW17/Saved+Search)": The class containing saved search data.
    SavedSearch,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for ClassName {
    fn from(s: String) -> ClassName {
        match s.as_ref() {
            "Business Opportunity" => ClassName::BusinessOpportunity,

            "Commercial Lease" => ClassName::CommercialLease,

            "Commercial Sale" => ClassName::CommercialSale,

            "Contacts" => ClassName::Contacts,

            "Cross Property" => ClassName::CrossProperty,

            "Farm" => ClassName::Farm,

            "History Transactional" => ClassName::HistoryTransactional,

            "Land" => ClassName::Land,

            "Manufactured In Park" => ClassName::ManufacturedInPark,

            "Media" => ClassName::Media,

            "Member" => ClassName::Member,

            "Office" => ClassName::Office,

            "Open House" => ClassName::OpenHouse,

            "Residential" => ClassName::Residential,

            "Residential Income" => ClassName::ResidentialIncome,

            "Residential Lease" => ClassName::ResidentialLease,

            "Saved Search" => ClassName::SavedSearch,

            _ => ClassName::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ClassName {
    fn from(s: &str) -> ClassName {
        match s {
            "Business Opportunity" => ClassName::BusinessOpportunity,

            "Commercial Lease" => ClassName::CommercialLease,

            "Commercial Sale" => ClassName::CommercialSale,

            "Contacts" => ClassName::Contacts,

            "Cross Property" => ClassName::CrossProperty,

            "Farm" => ClassName::Farm,

            "History Transactional" => ClassName::HistoryTransactional,

            "Land" => ClassName::Land,

            "Manufactured In Park" => ClassName::ManufacturedInPark,

            "Media" => ClassName::Media,

            "Member" => ClassName::Member,

            "Office" => ClassName::Office,

            "Open House" => ClassName::OpenHouse,

            "Residential" => ClassName::Residential,

            "Residential Income" => ClassName::ResidentialIncome,

            "Residential Lease" => ClassName::ResidentialLease,

            "Saved Search" => ClassName::SavedSearch,

            _ => ClassName::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ClassName> for &'a str {
    fn from(s: &'a ClassName) -> &'a str {
        match s {
            ClassName::BusinessOpportunity => "Business Opportunity",

            ClassName::CommercialLease => "Commercial Lease",

            ClassName::CommercialSale => "Commercial Sale",

            ClassName::Contacts => "Contacts",

            ClassName::CrossProperty => "Cross Property",

            ClassName::Farm => "Farm",

            ClassName::HistoryTransactional => "History Transactional",

            ClassName::Land => "Land",

            ClassName::ManufacturedInPark => "Manufactured In Park",

            ClassName::Media => "Media",

            ClassName::Member => "Member",

            ClassName::Office => "Office",

            ClassName::OpenHouse => "Open House",

            ClassName::Residential => "Residential",

            ClassName::ResidentialIncome => "Residential Income",

            ClassName::ResidentialLease => "Residential Lease",

            ClassName::SavedSearch => "Saved Search",

            ClassName::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ClassName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ClassName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_class_name_format {
    use super::ClassName;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ClassName>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<ClassName>>, D::Error>
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
