// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PossibleUse Lookups](https://ddwiki.reso.org/display/DDW17/PossibleUse+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PossibleUse {
    /// "[Agricultural](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245919)": The land could be used for agriculture.
    Agricultural,

    /// "[Cattle](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245920)": The land could be used for cattle.
    Cattle,

    /// "[Commercial](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245921)": The land could be used for commercial purposes.
    Commercial,

    /// "[Dairy](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245922)": The land could be used for a dairy farm.
    Dairy,

    /// "[Development](https://ddwiki.reso.org/display/DDW17/Development)": The land could be used for new development.
    Development,

    /// "[Farm](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245924)": The land could be used for a farm.
    Farm,

    /// "[Fishery](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245925)": The land could be used for a fishery.
    Fishery,

    /// "[Grazing](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245926)": The land could be used for livestock grazing.
    Grazing,

    /// "[Highway/Tourist Service](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245927)": The land could be used for a highway/tourist service.
    HighwayTouristService,

    /// "[Horses](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245928)": The land could be used for horses.
    Horses,

    /// "[Hunting](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245929)": The land could be used for hunting.
    Hunting,

    /// "[Industrial](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245930)": The land could be used for industrial purposes.
    Industrial,

    /// "[Investment](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245931)": The land could be used as an investment land.
    Investment,

    /// "[Livestock](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245932)": The land could be used for livestock.
    Livestock,

    /// "[Manufactured Home](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245933)": The land could be used for manufactured home(s).
    ManufacturedHome,

    /// "[Mini-Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245934)": The land could be used for mini-storage.
    MiniStorage,

    /// "[Multi-Family](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245935)": The land could be used for multi-family home(s).
    MultiFamily,

    /// "[Orchard](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245936)": The land could be used for an orchard.
    Orchard,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245937)": The land could be used for a purpose other than those in this list.
    Other,

    /// "[Pasture](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245938)": The land could be used as a pasture.
    Pasture,

    /// "[Place of Worship](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245939)": The land could be used for place of worship.
    PlaceofWorship,

    /// "[Poultry](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245940)": The land could be used for poultry.
    Poultry,

    /// "[Ranch](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245941)": The land could be used as a ranch.
    Ranch,

    /// "[Recreational](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245942)": The land could be used for recreational purposes.
    Recreational,

    /// "[Residential](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245943)": The land could be used for residential purposes.
    Residential,

    /// "[Retail](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245944)": The land could be used for retail business.
    Retail,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245945)": See the Public or Private remarks for details on possible uses for the land.
    SeeRemarks,

    /// "[Single Family](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245946)": The land could be used for single family residence(s).
    SingleFamily,

    /// "[Subdevelopment](https://ddwiki.reso.org/display/DDW17/Subdevelopment)": The land could be used for subdevelopment(s).
    Subdevelopment,

    /// "[Timber](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245948)": The land could be used for timber.
    Timber,

    /// "[Unimproved](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245949)": The land could be kept undeveloped.
    Unimproved,

    /// "[Vacant](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245950)": The land could be kept vacant.
    Vacant,

    /// "[Warehouse](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245951)": The land could be used for warehousing.
    Warehouse,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for PossibleUse {
    fn from_str(s: &str) -> PossibleUse {
        match s {
            "Agricultural" => PossibleUse::Agricultural,

            "Cattle" => PossibleUse::Cattle,

            "Commercial" => PossibleUse::Commercial,

            "Dairy" => PossibleUse::Dairy,

            "Development" => PossibleUse::Development,

            "Farm" => PossibleUse::Farm,

            "Fishery" => PossibleUse::Fishery,

            "Grazing" => PossibleUse::Grazing,

            "Highway/Tourist Service" => PossibleUse::HighwayTouristService,

            "Horses" => PossibleUse::Horses,

            "Hunting" => PossibleUse::Hunting,

            "Industrial" => PossibleUse::Industrial,

            "Investment" => PossibleUse::Investment,

            "Livestock" => PossibleUse::Livestock,

            "Manufactured Home" => PossibleUse::ManufacturedHome,

            "Mini-Storage" => PossibleUse::MiniStorage,

            "Multi-Family" => PossibleUse::MultiFamily,

            "Orchard" => PossibleUse::Orchard,

            "Other" => PossibleUse::Other,

            "Pasture" => PossibleUse::Pasture,

            "Place of Worship" => PossibleUse::PlaceofWorship,

            "Poultry" => PossibleUse::Poultry,

            "Ranch" => PossibleUse::Ranch,

            "Recreational" => PossibleUse::Recreational,

            "Residential" => PossibleUse::Residential,

            "Retail" => PossibleUse::Retail,

            "See Remarks" => PossibleUse::SeeRemarks,

            "Single Family" => PossibleUse::SingleFamily,

            "Subdevelopment" => PossibleUse::Subdevelopment,

            "Timber" => PossibleUse::Timber,

            "Unimproved" => PossibleUse::Unimproved,

            "Vacant" => PossibleUse::Vacant,

            "Warehouse" => PossibleUse::Warehouse,

            _ => PossibleUse::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> PossibleUse {
        match s.as_ref() {
            "Agricultural" => PossibleUse::Agricultural,

            "Cattle" => PossibleUse::Cattle,

            "Commercial" => PossibleUse::Commercial,

            "Dairy" => PossibleUse::Dairy,

            "Development" => PossibleUse::Development,

            "Farm" => PossibleUse::Farm,

            "Fishery" => PossibleUse::Fishery,

            "Grazing" => PossibleUse::Grazing,

            "Highway/Tourist Service" => PossibleUse::HighwayTouristService,

            "Horses" => PossibleUse::Horses,

            "Hunting" => PossibleUse::Hunting,

            "Industrial" => PossibleUse::Industrial,

            "Investment" => PossibleUse::Investment,

            "Livestock" => PossibleUse::Livestock,

            "Manufactured Home" => PossibleUse::ManufacturedHome,

            "Mini-Storage" => PossibleUse::MiniStorage,

            "Multi-Family" => PossibleUse::MultiFamily,

            "Orchard" => PossibleUse::Orchard,

            "Other" => PossibleUse::Other,

            "Pasture" => PossibleUse::Pasture,

            "Place of Worship" => PossibleUse::PlaceofWorship,

            "Poultry" => PossibleUse::Poultry,

            "Ranch" => PossibleUse::Ranch,

            "Recreational" => PossibleUse::Recreational,

            "Residential" => PossibleUse::Residential,

            "Retail" => PossibleUse::Retail,

            "See Remarks" => PossibleUse::SeeRemarks,

            "Single Family" => PossibleUse::SingleFamily,

            "Subdevelopment" => PossibleUse::Subdevelopment,

            "Timber" => PossibleUse::Timber,

            "Unimproved" => PossibleUse::Unimproved,

            "Vacant" => PossibleUse::Vacant,

            "Warehouse" => PossibleUse::Warehouse,

            _ => PossibleUse::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            PossibleUse::Agricultural => "Agricultural",

            PossibleUse::Cattle => "Cattle",

            PossibleUse::Commercial => "Commercial",

            PossibleUse::Dairy => "Dairy",

            PossibleUse::Development => "Development",

            PossibleUse::Farm => "Farm",

            PossibleUse::Fishery => "Fishery",

            PossibleUse::Grazing => "Grazing",

            PossibleUse::HighwayTouristService => "Highway/Tourist Service",

            PossibleUse::Horses => "Horses",

            PossibleUse::Hunting => "Hunting",

            PossibleUse::Industrial => "Industrial",

            PossibleUse::Investment => "Investment",

            PossibleUse::Livestock => "Livestock",

            PossibleUse::ManufacturedHome => "Manufactured Home",

            PossibleUse::MiniStorage => "Mini-Storage",

            PossibleUse::MultiFamily => "Multi-Family",

            PossibleUse::Orchard => "Orchard",

            PossibleUse::Other => "Other",

            PossibleUse::Pasture => "Pasture",

            PossibleUse::PlaceofWorship => "Place of Worship",

            PossibleUse::Poultry => "Poultry",

            PossibleUse::Ranch => "Ranch",

            PossibleUse::Recreational => "Recreational",

            PossibleUse::Residential => "Residential",

            PossibleUse::Retail => "Retail",

            PossibleUse::SeeRemarks => "See Remarks",

            PossibleUse::SingleFamily => "Single Family",

            PossibleUse::Subdevelopment => "Subdevelopment",

            PossibleUse::Timber => "Timber",

            PossibleUse::Unimproved => "Unimproved",

            PossibleUse::Vacant => "Vacant",

            PossibleUse::Warehouse => "Warehouse",

            PossibleUse::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            PossibleUse::Agricultural => "Agricultural".into(),

            PossibleUse::Cattle => "Cattle".into(),

            PossibleUse::Commercial => "Commercial".into(),

            PossibleUse::Dairy => "Dairy".into(),

            PossibleUse::Development => "Development".into(),

            PossibleUse::Farm => "Farm".into(),

            PossibleUse::Fishery => "Fishery".into(),

            PossibleUse::Grazing => "Grazing".into(),

            PossibleUse::HighwayTouristService => "Highway/Tourist Service".into(),

            PossibleUse::Horses => "Horses".into(),

            PossibleUse::Hunting => "Hunting".into(),

            PossibleUse::Industrial => "Industrial".into(),

            PossibleUse::Investment => "Investment".into(),

            PossibleUse::Livestock => "Livestock".into(),

            PossibleUse::ManufacturedHome => "Manufactured Home".into(),

            PossibleUse::MiniStorage => "Mini-Storage".into(),

            PossibleUse::MultiFamily => "Multi-Family".into(),

            PossibleUse::Orchard => "Orchard".into(),

            PossibleUse::Other => "Other".into(),

            PossibleUse::Pasture => "Pasture".into(),

            PossibleUse::PlaceofWorship => "Place of Worship".into(),

            PossibleUse::Poultry => "Poultry".into(),

            PossibleUse::Ranch => "Ranch".into(),

            PossibleUse::Recreational => "Recreational".into(),

            PossibleUse::Residential => "Residential".into(),

            PossibleUse::Retail => "Retail".into(),

            PossibleUse::SeeRemarks => "See Remarks".into(),

            PossibleUse::SingleFamily => "Single Family".into(),

            PossibleUse::Subdevelopment => "Subdevelopment".into(),

            PossibleUse::Timber => "Timber".into(),

            PossibleUse::Unimproved => "Unimproved".into(),

            PossibleUse::Vacant => "Vacant".into(),

            PossibleUse::Warehouse => "Warehouse".into(),

            PossibleUse::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            PossibleUse::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for PossibleUse {
    fn from(s: String) -> PossibleUse {
        match s.as_ref() {
            "Agricultural" => PossibleUse::Agricultural,

            "Cattle" => PossibleUse::Cattle,

            "Commercial" => PossibleUse::Commercial,

            "Dairy" => PossibleUse::Dairy,

            "Development" => PossibleUse::Development,

            "Farm" => PossibleUse::Farm,

            "Fishery" => PossibleUse::Fishery,

            "Grazing" => PossibleUse::Grazing,

            "Highway/Tourist Service" => PossibleUse::HighwayTouristService,

            "Horses" => PossibleUse::Horses,

            "Hunting" => PossibleUse::Hunting,

            "Industrial" => PossibleUse::Industrial,

            "Investment" => PossibleUse::Investment,

            "Livestock" => PossibleUse::Livestock,

            "Manufactured Home" => PossibleUse::ManufacturedHome,

            "Mini-Storage" => PossibleUse::MiniStorage,

            "Multi-Family" => PossibleUse::MultiFamily,

            "Orchard" => PossibleUse::Orchard,

            "Other" => PossibleUse::Other,

            "Pasture" => PossibleUse::Pasture,

            "Place of Worship" => PossibleUse::PlaceofWorship,

            "Poultry" => PossibleUse::Poultry,

            "Ranch" => PossibleUse::Ranch,

            "Recreational" => PossibleUse::Recreational,

            "Residential" => PossibleUse::Residential,

            "Retail" => PossibleUse::Retail,

            "See Remarks" => PossibleUse::SeeRemarks,

            "Single Family" => PossibleUse::SingleFamily,

            "Subdevelopment" => PossibleUse::Subdevelopment,

            "Timber" => PossibleUse::Timber,

            "Unimproved" => PossibleUse::Unimproved,

            "Vacant" => PossibleUse::Vacant,

            "Warehouse" => PossibleUse::Warehouse,

            _ => PossibleUse::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PossibleUse {
    fn from(s: &str) -> PossibleUse {
        match s {
            "Agricultural" => PossibleUse::Agricultural,

            "Cattle" => PossibleUse::Cattle,

            "Commercial" => PossibleUse::Commercial,

            "Dairy" => PossibleUse::Dairy,

            "Development" => PossibleUse::Development,

            "Farm" => PossibleUse::Farm,

            "Fishery" => PossibleUse::Fishery,

            "Grazing" => PossibleUse::Grazing,

            "Highway/Tourist Service" => PossibleUse::HighwayTouristService,

            "Horses" => PossibleUse::Horses,

            "Hunting" => PossibleUse::Hunting,

            "Industrial" => PossibleUse::Industrial,

            "Investment" => PossibleUse::Investment,

            "Livestock" => PossibleUse::Livestock,

            "Manufactured Home" => PossibleUse::ManufacturedHome,

            "Mini-Storage" => PossibleUse::MiniStorage,

            "Multi-Family" => PossibleUse::MultiFamily,

            "Orchard" => PossibleUse::Orchard,

            "Other" => PossibleUse::Other,

            "Pasture" => PossibleUse::Pasture,

            "Place of Worship" => PossibleUse::PlaceofWorship,

            "Poultry" => PossibleUse::Poultry,

            "Ranch" => PossibleUse::Ranch,

            "Recreational" => PossibleUse::Recreational,

            "Residential" => PossibleUse::Residential,

            "Retail" => PossibleUse::Retail,

            "See Remarks" => PossibleUse::SeeRemarks,

            "Single Family" => PossibleUse::SingleFamily,

            "Subdevelopment" => PossibleUse::Subdevelopment,

            "Timber" => PossibleUse::Timber,

            "Unimproved" => PossibleUse::Unimproved,

            "Vacant" => PossibleUse::Vacant,

            "Warehouse" => PossibleUse::Warehouse,

            _ => PossibleUse::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PossibleUse> for &'a str {
    fn from(s: &'a PossibleUse) -> &'a str {
        match s {
            PossibleUse::Agricultural => "Agricultural",

            PossibleUse::Cattle => "Cattle",

            PossibleUse::Commercial => "Commercial",

            PossibleUse::Dairy => "Dairy",

            PossibleUse::Development => "Development",

            PossibleUse::Farm => "Farm",

            PossibleUse::Fishery => "Fishery",

            PossibleUse::Grazing => "Grazing",

            PossibleUse::HighwayTouristService => "Highway/Tourist Service",

            PossibleUse::Horses => "Horses",

            PossibleUse::Hunting => "Hunting",

            PossibleUse::Industrial => "Industrial",

            PossibleUse::Investment => "Investment",

            PossibleUse::Livestock => "Livestock",

            PossibleUse::ManufacturedHome => "Manufactured Home",

            PossibleUse::MiniStorage => "Mini-Storage",

            PossibleUse::MultiFamily => "Multi-Family",

            PossibleUse::Orchard => "Orchard",

            PossibleUse::Other => "Other",

            PossibleUse::Pasture => "Pasture",

            PossibleUse::PlaceofWorship => "Place of Worship",

            PossibleUse::Poultry => "Poultry",

            PossibleUse::Ranch => "Ranch",

            PossibleUse::Recreational => "Recreational",

            PossibleUse::Residential => "Residential",

            PossibleUse::Retail => "Retail",

            PossibleUse::SeeRemarks => "See Remarks",

            PossibleUse::SingleFamily => "Single Family",

            PossibleUse::Subdevelopment => "Subdevelopment",

            PossibleUse::Timber => "Timber",

            PossibleUse::Unimproved => "Unimproved",

            PossibleUse::Vacant => "Vacant",

            PossibleUse::Warehouse => "Warehouse",

            PossibleUse::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PossibleUse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PossibleUse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
