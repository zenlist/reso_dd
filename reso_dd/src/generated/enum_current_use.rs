// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [CurrentUse Lookups](https://ddwiki.reso.org/display/DDW17/CurrentUse+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CurrentUse {
    /// "[Agricultural](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244464)": The land is currently used for agriculture.
    Agricultural,

    /// "[Automotive](https://ddwiki.reso.org/display/DDW17/Automotive)": The land is currently used for automotive maintenance or repair.
    Automotive,

    /// "[Cattle](https://ddwiki.reso.org/display/DDW17/Cattle)": The land is currently used for cattle.
    Cattle,

    /// "[Commercial](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244467)": The land is currently used for commercial purposes.
    Commercial,

    /// "[Dairy](https://ddwiki.reso.org/display/DDW17/Dairy)": The land is currently used as a dairy farm.
    Dairy,

    /// "[Farm](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244469)": The land is currently used as a farm.
    Farm,

    /// "[Fishery](https://ddwiki.reso.org/display/DDW17/Fishery)": The land is currently used as a fishery.
    Fishery,

    /// "[Grazing](https://ddwiki.reso.org/display/DDW17/Grazing)": The land is currently used for live stock grazing.
    Grazing,

    /// "[Highway/Tourist Service](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244472)": The land is currently used for a highway/tourist service.
    HighwayTouristService,

    /// "[Horses](https://ddwiki.reso.org/display/DDW17/Horses)": The land is currently used for horses.
    Horses,

    /// "[Hunting](https://ddwiki.reso.org/display/DDW17/Hunting)": The land is currently used for hunting.
    Hunting,

    /// "[Industrial](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244475)": The land is currently used for industrial purposes.
    Industrial,

    /// "[Investment](https://ddwiki.reso.org/display/DDW17/Investment)": The land is currently used as an investment.
    Investment,

    /// "[Livestock](https://ddwiki.reso.org/display/DDW17/Livestock)": The land is currently used for livestock.
    Livestock,

    /// "[Manufactured Home](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244478)": The land is currently used for manufactured home.
    ManufacturedHome,

    /// "[Medical/Dental](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244479)": The land is currently used for medical/dental business.
    MedicalDental,

    /// "[Mini-Storage](https://ddwiki.reso.org/display/DDW17/Mini-Storage)": The land is currently used for mini-storage business.
    MiniStorage,

    /// "[Mixed Use](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244481)": The land is currently used for mixed uses.
    MixedUse,

    /// "[Multi-Family](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244482)": The land is currently used for multi-family dwelling.
    MultiFamily,

    /// "[Nursery](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244483)": The land is currently used as a nursery.
    Nursery,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244484)": The land is currently used as office space.
    Office,

    /// "[Orchard](https://ddwiki.reso.org/display/DDW17/Orchard)": The land is currently used for an orchard.
    Orchard,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244486)": The land is currently used for some use other than those in this list.
    Other,

    /// "[Pasture](https://ddwiki.reso.org/display/DDW17/Pasture)": The land is currently used as a pasture.
    Pasture,

    /// "[Place of Worship](https://ddwiki.reso.org/display/DDW17/Place+of+Worship)": The land is currently used for a place or worship.
    PlaceofWorship,

    /// "[Plantable](https://ddwiki.reso.org/display/DDW17/Plantable)": The land is currently used as a plantabe field.
    Plantable,

    /// "[Poultry](https://ddwiki.reso.org/display/DDW17/Poultry)": The land is currently used as a poultry farm.
    Poultry,

    /// "[Ranch](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244491)": The land is currently used as a ranch.
    Ranch,

    /// "[Recreational](https://ddwiki.reso.org/display/DDW17/Recreational)": The land is currently used for recreational purposes.
    Recreational,

    /// "[Residential](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244493)": The land is currently used for residential purposes.
    Residential,

    /// "[Retail](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244494)": The land is currently used for retail purposes.
    Retail,

    /// "[Row Crops](https://ddwiki.reso.org/display/DDW17/Row+Crops)": The land is currently used for row crops.
    RowCrops,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244496)": See the Public or Private remarks for details on the current use.
    SeeRemarks,

    /// "[Single Family](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244497)": The land is currently used for single family residence.
    SingleFamily,

    /// "[Subdivision](https://ddwiki.reso.org/display/DDW17/Subdivision)": The land is currently used for property subdivisions.
    Subdivision,

    /// "[Timber](https://ddwiki.reso.org/display/DDW17/Timber)": The land is currently used for timber.
    Timber,

    /// "[Tree Farm](https://ddwiki.reso.org/display/DDW17/Tree+Farm)": The land is currently used as a tree farm.
    TreeFarm,

    /// "[Unimproved](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244501)": The land is currently unimproved.
    Unimproved,

    /// "[Vacant](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244502)": The land is currently vacant.
    Vacant,

    /// "[Vineyard](https://ddwiki.reso.org/display/DDW17/Vineyard)": The land is currently used as a vineyard.
    Vineyard,

    /// "[Warehouse](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244504)": The land is currently used for warehousing.
    Warehouse,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for CurrentUse {
    fn from_str(s: &str) -> CurrentUse {
        match s {
            "Agricultural" => CurrentUse::Agricultural,

            "Automotive" => CurrentUse::Automotive,

            "Cattle" => CurrentUse::Cattle,

            "Commercial" => CurrentUse::Commercial,

            "Dairy" => CurrentUse::Dairy,

            "Farm" => CurrentUse::Farm,

            "Fishery" => CurrentUse::Fishery,

            "Grazing" => CurrentUse::Grazing,

            "Highway/Tourist Service" => CurrentUse::HighwayTouristService,

            "Horses" => CurrentUse::Horses,

            "Hunting" => CurrentUse::Hunting,

            "Industrial" => CurrentUse::Industrial,

            "Investment" => CurrentUse::Investment,

            "Livestock" => CurrentUse::Livestock,

            "Manufactured Home" => CurrentUse::ManufacturedHome,

            "Medical/Dental" => CurrentUse::MedicalDental,

            "Mini-Storage" => CurrentUse::MiniStorage,

            "Mixed Use" => CurrentUse::MixedUse,

            "Multi-Family" => CurrentUse::MultiFamily,

            "Nursery" => CurrentUse::Nursery,

            "Office" => CurrentUse::Office,

            "Orchard" => CurrentUse::Orchard,

            "Other" => CurrentUse::Other,

            "Pasture" => CurrentUse::Pasture,

            "Place of Worship" => CurrentUse::PlaceofWorship,

            "Plantable" => CurrentUse::Plantable,

            "Poultry" => CurrentUse::Poultry,

            "Ranch" => CurrentUse::Ranch,

            "Recreational" => CurrentUse::Recreational,

            "Residential" => CurrentUse::Residential,

            "Retail" => CurrentUse::Retail,

            "Row Crops" => CurrentUse::RowCrops,

            "See Remarks" => CurrentUse::SeeRemarks,

            "Single Family" => CurrentUse::SingleFamily,

            "Subdivision" => CurrentUse::Subdivision,

            "Timber" => CurrentUse::Timber,

            "Tree Farm" => CurrentUse::TreeFarm,

            "Unimproved" => CurrentUse::Unimproved,

            "Vacant" => CurrentUse::Vacant,

            "Vineyard" => CurrentUse::Vineyard,

            "Warehouse" => CurrentUse::Warehouse,

            _ => CurrentUse::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> CurrentUse {
        match s.as_ref() {
            "Agricultural" => CurrentUse::Agricultural,

            "Automotive" => CurrentUse::Automotive,

            "Cattle" => CurrentUse::Cattle,

            "Commercial" => CurrentUse::Commercial,

            "Dairy" => CurrentUse::Dairy,

            "Farm" => CurrentUse::Farm,

            "Fishery" => CurrentUse::Fishery,

            "Grazing" => CurrentUse::Grazing,

            "Highway/Tourist Service" => CurrentUse::HighwayTouristService,

            "Horses" => CurrentUse::Horses,

            "Hunting" => CurrentUse::Hunting,

            "Industrial" => CurrentUse::Industrial,

            "Investment" => CurrentUse::Investment,

            "Livestock" => CurrentUse::Livestock,

            "Manufactured Home" => CurrentUse::ManufacturedHome,

            "Medical/Dental" => CurrentUse::MedicalDental,

            "Mini-Storage" => CurrentUse::MiniStorage,

            "Mixed Use" => CurrentUse::MixedUse,

            "Multi-Family" => CurrentUse::MultiFamily,

            "Nursery" => CurrentUse::Nursery,

            "Office" => CurrentUse::Office,

            "Orchard" => CurrentUse::Orchard,

            "Other" => CurrentUse::Other,

            "Pasture" => CurrentUse::Pasture,

            "Place of Worship" => CurrentUse::PlaceofWorship,

            "Plantable" => CurrentUse::Plantable,

            "Poultry" => CurrentUse::Poultry,

            "Ranch" => CurrentUse::Ranch,

            "Recreational" => CurrentUse::Recreational,

            "Residential" => CurrentUse::Residential,

            "Retail" => CurrentUse::Retail,

            "Row Crops" => CurrentUse::RowCrops,

            "See Remarks" => CurrentUse::SeeRemarks,

            "Single Family" => CurrentUse::SingleFamily,

            "Subdivision" => CurrentUse::Subdivision,

            "Timber" => CurrentUse::Timber,

            "Tree Farm" => CurrentUse::TreeFarm,

            "Unimproved" => CurrentUse::Unimproved,

            "Vacant" => CurrentUse::Vacant,

            "Vineyard" => CurrentUse::Vineyard,

            "Warehouse" => CurrentUse::Warehouse,

            _ => CurrentUse::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            CurrentUse::Agricultural => "Agricultural",

            CurrentUse::Automotive => "Automotive",

            CurrentUse::Cattle => "Cattle",

            CurrentUse::Commercial => "Commercial",

            CurrentUse::Dairy => "Dairy",

            CurrentUse::Farm => "Farm",

            CurrentUse::Fishery => "Fishery",

            CurrentUse::Grazing => "Grazing",

            CurrentUse::HighwayTouristService => "Highway/Tourist Service",

            CurrentUse::Horses => "Horses",

            CurrentUse::Hunting => "Hunting",

            CurrentUse::Industrial => "Industrial",

            CurrentUse::Investment => "Investment",

            CurrentUse::Livestock => "Livestock",

            CurrentUse::ManufacturedHome => "Manufactured Home",

            CurrentUse::MedicalDental => "Medical/Dental",

            CurrentUse::MiniStorage => "Mini-Storage",

            CurrentUse::MixedUse => "Mixed Use",

            CurrentUse::MultiFamily => "Multi-Family",

            CurrentUse::Nursery => "Nursery",

            CurrentUse::Office => "Office",

            CurrentUse::Orchard => "Orchard",

            CurrentUse::Other => "Other",

            CurrentUse::Pasture => "Pasture",

            CurrentUse::PlaceofWorship => "Place of Worship",

            CurrentUse::Plantable => "Plantable",

            CurrentUse::Poultry => "Poultry",

            CurrentUse::Ranch => "Ranch",

            CurrentUse::Recreational => "Recreational",

            CurrentUse::Residential => "Residential",

            CurrentUse::Retail => "Retail",

            CurrentUse::RowCrops => "Row Crops",

            CurrentUse::SeeRemarks => "See Remarks",

            CurrentUse::SingleFamily => "Single Family",

            CurrentUse::Subdivision => "Subdivision",

            CurrentUse::Timber => "Timber",

            CurrentUse::TreeFarm => "Tree Farm",

            CurrentUse::Unimproved => "Unimproved",

            CurrentUse::Vacant => "Vacant",

            CurrentUse::Vineyard => "Vineyard",

            CurrentUse::Warehouse => "Warehouse",

            CurrentUse::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            CurrentUse::Agricultural => "Agricultural".into(),

            CurrentUse::Automotive => "Automotive".into(),

            CurrentUse::Cattle => "Cattle".into(),

            CurrentUse::Commercial => "Commercial".into(),

            CurrentUse::Dairy => "Dairy".into(),

            CurrentUse::Farm => "Farm".into(),

            CurrentUse::Fishery => "Fishery".into(),

            CurrentUse::Grazing => "Grazing".into(),

            CurrentUse::HighwayTouristService => "Highway/Tourist Service".into(),

            CurrentUse::Horses => "Horses".into(),

            CurrentUse::Hunting => "Hunting".into(),

            CurrentUse::Industrial => "Industrial".into(),

            CurrentUse::Investment => "Investment".into(),

            CurrentUse::Livestock => "Livestock".into(),

            CurrentUse::ManufacturedHome => "Manufactured Home".into(),

            CurrentUse::MedicalDental => "Medical/Dental".into(),

            CurrentUse::MiniStorage => "Mini-Storage".into(),

            CurrentUse::MixedUse => "Mixed Use".into(),

            CurrentUse::MultiFamily => "Multi-Family".into(),

            CurrentUse::Nursery => "Nursery".into(),

            CurrentUse::Office => "Office".into(),

            CurrentUse::Orchard => "Orchard".into(),

            CurrentUse::Other => "Other".into(),

            CurrentUse::Pasture => "Pasture".into(),

            CurrentUse::PlaceofWorship => "Place of Worship".into(),

            CurrentUse::Plantable => "Plantable".into(),

            CurrentUse::Poultry => "Poultry".into(),

            CurrentUse::Ranch => "Ranch".into(),

            CurrentUse::Recreational => "Recreational".into(),

            CurrentUse::Residential => "Residential".into(),

            CurrentUse::Retail => "Retail".into(),

            CurrentUse::RowCrops => "Row Crops".into(),

            CurrentUse::SeeRemarks => "See Remarks".into(),

            CurrentUse::SingleFamily => "Single Family".into(),

            CurrentUse::Subdivision => "Subdivision".into(),

            CurrentUse::Timber => "Timber".into(),

            CurrentUse::TreeFarm => "Tree Farm".into(),

            CurrentUse::Unimproved => "Unimproved".into(),

            CurrentUse::Vacant => "Vacant".into(),

            CurrentUse::Vineyard => "Vineyard".into(),

            CurrentUse::Warehouse => "Warehouse".into(),

            CurrentUse::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            CurrentUse::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for CurrentUse {
    fn from(s: String) -> CurrentUse {
        match s.as_ref() {
            "Agricultural" => CurrentUse::Agricultural,

            "Automotive" => CurrentUse::Automotive,

            "Cattle" => CurrentUse::Cattle,

            "Commercial" => CurrentUse::Commercial,

            "Dairy" => CurrentUse::Dairy,

            "Farm" => CurrentUse::Farm,

            "Fishery" => CurrentUse::Fishery,

            "Grazing" => CurrentUse::Grazing,

            "Highway/Tourist Service" => CurrentUse::HighwayTouristService,

            "Horses" => CurrentUse::Horses,

            "Hunting" => CurrentUse::Hunting,

            "Industrial" => CurrentUse::Industrial,

            "Investment" => CurrentUse::Investment,

            "Livestock" => CurrentUse::Livestock,

            "Manufactured Home" => CurrentUse::ManufacturedHome,

            "Medical/Dental" => CurrentUse::MedicalDental,

            "Mini-Storage" => CurrentUse::MiniStorage,

            "Mixed Use" => CurrentUse::MixedUse,

            "Multi-Family" => CurrentUse::MultiFamily,

            "Nursery" => CurrentUse::Nursery,

            "Office" => CurrentUse::Office,

            "Orchard" => CurrentUse::Orchard,

            "Other" => CurrentUse::Other,

            "Pasture" => CurrentUse::Pasture,

            "Place of Worship" => CurrentUse::PlaceofWorship,

            "Plantable" => CurrentUse::Plantable,

            "Poultry" => CurrentUse::Poultry,

            "Ranch" => CurrentUse::Ranch,

            "Recreational" => CurrentUse::Recreational,

            "Residential" => CurrentUse::Residential,

            "Retail" => CurrentUse::Retail,

            "Row Crops" => CurrentUse::RowCrops,

            "See Remarks" => CurrentUse::SeeRemarks,

            "Single Family" => CurrentUse::SingleFamily,

            "Subdivision" => CurrentUse::Subdivision,

            "Timber" => CurrentUse::Timber,

            "Tree Farm" => CurrentUse::TreeFarm,

            "Unimproved" => CurrentUse::Unimproved,

            "Vacant" => CurrentUse::Vacant,

            "Vineyard" => CurrentUse::Vineyard,

            "Warehouse" => CurrentUse::Warehouse,

            _ => CurrentUse::OpenEnumeration(s),
        }
    }
}

impl From<&str> for CurrentUse {
    fn from(s: &str) -> CurrentUse {
        match s {
            "Agricultural" => CurrentUse::Agricultural,

            "Automotive" => CurrentUse::Automotive,

            "Cattle" => CurrentUse::Cattle,

            "Commercial" => CurrentUse::Commercial,

            "Dairy" => CurrentUse::Dairy,

            "Farm" => CurrentUse::Farm,

            "Fishery" => CurrentUse::Fishery,

            "Grazing" => CurrentUse::Grazing,

            "Highway/Tourist Service" => CurrentUse::HighwayTouristService,

            "Horses" => CurrentUse::Horses,

            "Hunting" => CurrentUse::Hunting,

            "Industrial" => CurrentUse::Industrial,

            "Investment" => CurrentUse::Investment,

            "Livestock" => CurrentUse::Livestock,

            "Manufactured Home" => CurrentUse::ManufacturedHome,

            "Medical/Dental" => CurrentUse::MedicalDental,

            "Mini-Storage" => CurrentUse::MiniStorage,

            "Mixed Use" => CurrentUse::MixedUse,

            "Multi-Family" => CurrentUse::MultiFamily,

            "Nursery" => CurrentUse::Nursery,

            "Office" => CurrentUse::Office,

            "Orchard" => CurrentUse::Orchard,

            "Other" => CurrentUse::Other,

            "Pasture" => CurrentUse::Pasture,

            "Place of Worship" => CurrentUse::PlaceofWorship,

            "Plantable" => CurrentUse::Plantable,

            "Poultry" => CurrentUse::Poultry,

            "Ranch" => CurrentUse::Ranch,

            "Recreational" => CurrentUse::Recreational,

            "Residential" => CurrentUse::Residential,

            "Retail" => CurrentUse::Retail,

            "Row Crops" => CurrentUse::RowCrops,

            "See Remarks" => CurrentUse::SeeRemarks,

            "Single Family" => CurrentUse::SingleFamily,

            "Subdivision" => CurrentUse::Subdivision,

            "Timber" => CurrentUse::Timber,

            "Tree Farm" => CurrentUse::TreeFarm,

            "Unimproved" => CurrentUse::Unimproved,

            "Vacant" => CurrentUse::Vacant,

            "Vineyard" => CurrentUse::Vineyard,

            "Warehouse" => CurrentUse::Warehouse,

            _ => CurrentUse::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a CurrentUse> for &'a str {
    fn from(s: &'a CurrentUse) -> &'a str {
        match s {
            CurrentUse::Agricultural => "Agricultural",

            CurrentUse::Automotive => "Automotive",

            CurrentUse::Cattle => "Cattle",

            CurrentUse::Commercial => "Commercial",

            CurrentUse::Dairy => "Dairy",

            CurrentUse::Farm => "Farm",

            CurrentUse::Fishery => "Fishery",

            CurrentUse::Grazing => "Grazing",

            CurrentUse::HighwayTouristService => "Highway/Tourist Service",

            CurrentUse::Horses => "Horses",

            CurrentUse::Hunting => "Hunting",

            CurrentUse::Industrial => "Industrial",

            CurrentUse::Investment => "Investment",

            CurrentUse::Livestock => "Livestock",

            CurrentUse::ManufacturedHome => "Manufactured Home",

            CurrentUse::MedicalDental => "Medical/Dental",

            CurrentUse::MiniStorage => "Mini-Storage",

            CurrentUse::MixedUse => "Mixed Use",

            CurrentUse::MultiFamily => "Multi-Family",

            CurrentUse::Nursery => "Nursery",

            CurrentUse::Office => "Office",

            CurrentUse::Orchard => "Orchard",

            CurrentUse::Other => "Other",

            CurrentUse::Pasture => "Pasture",

            CurrentUse::PlaceofWorship => "Place of Worship",

            CurrentUse::Plantable => "Plantable",

            CurrentUse::Poultry => "Poultry",

            CurrentUse::Ranch => "Ranch",

            CurrentUse::Recreational => "Recreational",

            CurrentUse::Residential => "Residential",

            CurrentUse::Retail => "Retail",

            CurrentUse::RowCrops => "Row Crops",

            CurrentUse::SeeRemarks => "See Remarks",

            CurrentUse::SingleFamily => "Single Family",

            CurrentUse::Subdivision => "Subdivision",

            CurrentUse::Timber => "Timber",

            CurrentUse::TreeFarm => "Tree Farm",

            CurrentUse::Unimproved => "Unimproved",

            CurrentUse::Vacant => "Vacant",

            CurrentUse::Vineyard => "Vineyard",

            CurrentUse::Warehouse => "Warehouse",

            CurrentUse::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for CurrentUse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CurrentUse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
