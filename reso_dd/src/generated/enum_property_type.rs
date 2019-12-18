// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PropertyType Lookups](https://ddwiki.reso.org/display/DDW17/PropertyType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PropertyType {
    /// "[Business Opportunity](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245901)": The property type of the listing is Business Opportunity. The property type can be references as a class or a field within a single class structure.
    BusinessOpportunity,

    /// "[Commercial Lease](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245902)": The property type of the listing is Commercial Lease. The property type can be references as a class or a field within a single class structure.
    CommercialLease,

    /// "[Commercial Sale](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245903)": The property type of the listing is Commercial Sale. The property type can be references as a class or a field within a single class structure.
    CommercialSale,

    /// "[Farm](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245904)": The property type of the listing is Farm.  The property type can be references as a class or a field within a single class structure.
    Farm,

    /// "[Land](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245905)": The property type of the listing is Land.  The property type can be references as a class or a field within a single class structure.
    Land,

    /// "[Manufactured In Park](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245906)": The property type of the listing is Manufactured in Park.  The property type can be references as a class or a field within a single class structure.
    ManufacturedInPark,

    /// "[Residential](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245907)": The property type of the listing is Residential.  The property type can be references as a class or a field within a single class structure.
    Residential,

    /// "[Residential Income](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245908)": The property type of the listing is Residential Income.  The property type can be references as a class or a field within a single class structure.
    ResidentialIncome,

    /// "[Residential Lease](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245909)": The property type of the listing is Residential Lease.  The property type can be references as a class or a field within a single class structure.
    ResidentialLease,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for PropertyType {
    fn from(s: String) -> PropertyType {
        match s.as_ref() {
            "Business Opportunity" => PropertyType::BusinessOpportunity,

            "Commercial Lease" => PropertyType::CommercialLease,

            "Commercial Sale" => PropertyType::CommercialSale,

            "Farm" => PropertyType::Farm,

            "Land" => PropertyType::Land,

            "Manufactured In Park" => PropertyType::ManufacturedInPark,

            "Residential" => PropertyType::Residential,

            "Residential Income" => PropertyType::ResidentialIncome,

            "Residential Lease" => PropertyType::ResidentialLease,

            _ => PropertyType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PropertyType {
    fn from(s: &str) -> PropertyType {
        match s {
            "Business Opportunity" => PropertyType::BusinessOpportunity,

            "Commercial Lease" => PropertyType::CommercialLease,

            "Commercial Sale" => PropertyType::CommercialSale,

            "Farm" => PropertyType::Farm,

            "Land" => PropertyType::Land,

            "Manufactured In Park" => PropertyType::ManufacturedInPark,

            "Residential" => PropertyType::Residential,

            "Residential Income" => PropertyType::ResidentialIncome,

            "Residential Lease" => PropertyType::ResidentialLease,

            _ => PropertyType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PropertyType> for &'a str {
    fn from(s: &'a PropertyType) -> &'a str {
        match s {
            PropertyType::BusinessOpportunity => "Business Opportunity",

            PropertyType::CommercialLease => "Commercial Lease",

            PropertyType::CommercialSale => "Commercial Sale",

            PropertyType::Farm => "Farm",

            PropertyType::Land => "Land",

            PropertyType::ManufacturedInPark => "Manufactured In Park",

            PropertyType::Residential => "Residential",

            PropertyType::ResidentialIncome => "Residential Income",

            PropertyType::ResidentialLease => "Residential Lease",

            PropertyType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PropertyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PropertyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_property_type_format {
    use super::PropertyType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<PropertyType>>,
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
    ) -> Result<Option<Vec<PropertyType>>, D::Error>
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
