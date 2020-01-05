// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PropertySubType Lookups](https://ddwiki.reso.org/display/DDW17/PropertySubType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PropertySubType {
    /// "[Agriculture](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245890)": The property is for farming and agricultural activities.
    Agriculture,

    /// "[Apartment](https://ddwiki.reso.org/display/DDW17/Apartment)": A unit within a wholly owned structure of 5 or more units. This may not be used for Residential or Residential Income. For Residential use Condo. For Income used Residential Income.
    Apartment,

    /// "[Boat Slip](https://ddwiki.reso.org/display/DDW17/Boat+Slip)": A place where you can tie up a boat or house boat.
    BoatSlip,

    /// "[Business](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245891)": The property is designed for any type of business.
    Business,

    /// "[Cabin](https://ddwiki.reso.org/display/DDW17/Cabin)": A single family residence that may have limited utilities.
    Cabin,

    /// "[Condominium](https://ddwiki.reso.org/display/DDW17/Condominium)": A unit within a structure where ownership is on a unit by unit basis.
    Condominium,

    /// "[Deeded Parking](https://ddwiki.reso.org/display/DDW17/Deeded+Parking)": A parking space (or spaces) that are owned and separate from a residence.
    DeededParking,

    /// "[Duplex](https://ddwiki.reso.org/display/DDW17/Duplex)": A multi family structure with two independent units with a shared wall or ceiling/floor.
    Duplex,

    /// "[Farm](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245878)": A place where agricultural and similar activities take place, especially the growing of crops.
    Farm,

    /// "[Hotel/Motel](https://ddwiki.reso.org/display/DDW17/Hotel-Motel)": The property is designed for hotel or motel use.
    HotelMotel,

    /// "[Industrial](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245893)": The property is designed for industrial use.
    Industrial,

    /// "[Manufactured Home](https://ddwiki.reso.org/display/DDW17/Manufactured+Home)": A factory built house that is transported to the lot.
    ManufacturedHome,

    /// "[Manufactured On Land](https://ddwiki.reso.org/display/DDW17/Manufactured+On+Land)": A factory built house that is transported to the lot and sold with the land. The property may or may not have a 433a certification.
    ManufacturedOnLand,

    /// "[Mixed Use](https://ddwiki.reso.org/display/DDW17/Mixed+Use)": The property is designed be used in more than one way.  i.e. Office and Retail.
    MixedUse,

    /// "[Mobile Home](https://ddwiki.reso.org/display/DDW17/Mobile+Home)": A factory built house that is transported to the lot, retains axles and was built prior to June 15, 1976.
    MobileHome,

    /// "[Multi Family](https://ddwiki.reso.org/display/DDW17/Multi+Family)": A structure or complex with 5 or more units that are individual dwellings.
    MultiFamily,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245896)": The property is designed to be used as office space.
    Office,

    /// "[Own Your Own](https://ddwiki.reso.org/display/DDW17/Own+Your+Own)": A unit within a structure where ownership is based on a partial deed and rights to occupy a unit.
    OwnYourOwn,

    /// "[Quadruplex](https://ddwiki.reso.org/display/DDW17/Quadruplex)": A multi family structure with four independent units with shared walls or ceilings/floors.
    Quadruplex,

    /// "[Ranch](https://ddwiki.reso.org/display/DDW17/Ranch)": A place where agricultural and similar activities take place, especially the raising of livestock.
    Ranch,

    /// "[Retail](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245897)": The property designed to be used as retail space.
    Retail,

    /// "[Single Family Residence](https://ddwiki.reso.org/display/DDW17/Single+Family+Residence)": A single family residence on real property.
    SingleFamilyResidence,

    /// "[Stock Cooperative](https://ddwiki.reso.org/display/DDW17/Stock+Cooperative)": A unit within a structure where ownership is based on a share of stock and rights to occupy a unit.
    StockCooperative,

    /// "[Timeshare](https://ddwiki.reso.org/display/DDW17/Timeshare)": A form of property ownership under with a property is held by a number of people, each with the right of possession for a specified time interval.
    Timeshare,

    /// "[Townhouse](https://ddwiki.reso.org/display/DDW17/Townhouse)": A dwelling unit, generally having two or more floors and attached to other similar units via party walls.
    Townhouse,

    /// "[Triplex](https://ddwiki.reso.org/display/DDW17/Triplex)": A multi family structure with three independent units with shared walls or ceilings/floors.
    Triplex,

    /// "[Unimproved Land](https://ddwiki.reso.org/display/DDW17/Unimproved+Land)": Commercial land that has not been built upon or improved.
    UnimprovedLand,

    /// "[Warehouse](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245899)": The property is designed to be used for warehousing.
    Warehouse,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for PropertySubType {
    fn from_str(s: &str) -> PropertySubType {
        match s {
            "Agriculture" => PropertySubType::Agriculture,

            "Apartment" => PropertySubType::Apartment,

            "Boat Slip" => PropertySubType::BoatSlip,

            "Business" => PropertySubType::Business,

            "Cabin" => PropertySubType::Cabin,

            "Condominium" => PropertySubType::Condominium,

            "Deeded Parking" => PropertySubType::DeededParking,

            "Duplex" => PropertySubType::Duplex,

            "Farm" => PropertySubType::Farm,

            "Hotel/Motel" => PropertySubType::HotelMotel,

            "Industrial" => PropertySubType::Industrial,

            "Manufactured Home" => PropertySubType::ManufacturedHome,

            "Manufactured On Land" => PropertySubType::ManufacturedOnLand,

            "Mixed Use" => PropertySubType::MixedUse,

            "Mobile Home" => PropertySubType::MobileHome,

            "Multi Family" => PropertySubType::MultiFamily,

            "Office" => PropertySubType::Office,

            "Own Your Own" => PropertySubType::OwnYourOwn,

            "Quadruplex" => PropertySubType::Quadruplex,

            "Ranch" => PropertySubType::Ranch,

            "Retail" => PropertySubType::Retail,

            "Single Family Residence" => PropertySubType::SingleFamilyResidence,

            "Stock Cooperative" => PropertySubType::StockCooperative,

            "Timeshare" => PropertySubType::Timeshare,

            "Townhouse" => PropertySubType::Townhouse,

            "Triplex" => PropertySubType::Triplex,

            "Unimproved Land" => PropertySubType::UnimprovedLand,

            "Warehouse" => PropertySubType::Warehouse,

            _ => PropertySubType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> PropertySubType {
        match s.as_ref() {
            "Agriculture" => PropertySubType::Agriculture,

            "Apartment" => PropertySubType::Apartment,

            "Boat Slip" => PropertySubType::BoatSlip,

            "Business" => PropertySubType::Business,

            "Cabin" => PropertySubType::Cabin,

            "Condominium" => PropertySubType::Condominium,

            "Deeded Parking" => PropertySubType::DeededParking,

            "Duplex" => PropertySubType::Duplex,

            "Farm" => PropertySubType::Farm,

            "Hotel/Motel" => PropertySubType::HotelMotel,

            "Industrial" => PropertySubType::Industrial,

            "Manufactured Home" => PropertySubType::ManufacturedHome,

            "Manufactured On Land" => PropertySubType::ManufacturedOnLand,

            "Mixed Use" => PropertySubType::MixedUse,

            "Mobile Home" => PropertySubType::MobileHome,

            "Multi Family" => PropertySubType::MultiFamily,

            "Office" => PropertySubType::Office,

            "Own Your Own" => PropertySubType::OwnYourOwn,

            "Quadruplex" => PropertySubType::Quadruplex,

            "Ranch" => PropertySubType::Ranch,

            "Retail" => PropertySubType::Retail,

            "Single Family Residence" => PropertySubType::SingleFamilyResidence,

            "Stock Cooperative" => PropertySubType::StockCooperative,

            "Timeshare" => PropertySubType::Timeshare,

            "Townhouse" => PropertySubType::Townhouse,

            "Triplex" => PropertySubType::Triplex,

            "Unimproved Land" => PropertySubType::UnimprovedLand,

            "Warehouse" => PropertySubType::Warehouse,

            _ => PropertySubType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            PropertySubType::Agriculture => "Agriculture",

            PropertySubType::Apartment => "Apartment",

            PropertySubType::BoatSlip => "Boat Slip",

            PropertySubType::Business => "Business",

            PropertySubType::Cabin => "Cabin",

            PropertySubType::Condominium => "Condominium",

            PropertySubType::DeededParking => "Deeded Parking",

            PropertySubType::Duplex => "Duplex",

            PropertySubType::Farm => "Farm",

            PropertySubType::HotelMotel => "Hotel/Motel",

            PropertySubType::Industrial => "Industrial",

            PropertySubType::ManufacturedHome => "Manufactured Home",

            PropertySubType::ManufacturedOnLand => "Manufactured On Land",

            PropertySubType::MixedUse => "Mixed Use",

            PropertySubType::MobileHome => "Mobile Home",

            PropertySubType::MultiFamily => "Multi Family",

            PropertySubType::Office => "Office",

            PropertySubType::OwnYourOwn => "Own Your Own",

            PropertySubType::Quadruplex => "Quadruplex",

            PropertySubType::Ranch => "Ranch",

            PropertySubType::Retail => "Retail",

            PropertySubType::SingleFamilyResidence => "Single Family Residence",

            PropertySubType::StockCooperative => "Stock Cooperative",

            PropertySubType::Timeshare => "Timeshare",

            PropertySubType::Townhouse => "Townhouse",

            PropertySubType::Triplex => "Triplex",

            PropertySubType::UnimprovedLand => "Unimproved Land",

            PropertySubType::Warehouse => "Warehouse",

            PropertySubType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            PropertySubType::Agriculture => "Agriculture".into(),

            PropertySubType::Apartment => "Apartment".into(),

            PropertySubType::BoatSlip => "Boat Slip".into(),

            PropertySubType::Business => "Business".into(),

            PropertySubType::Cabin => "Cabin".into(),

            PropertySubType::Condominium => "Condominium".into(),

            PropertySubType::DeededParking => "Deeded Parking".into(),

            PropertySubType::Duplex => "Duplex".into(),

            PropertySubType::Farm => "Farm".into(),

            PropertySubType::HotelMotel => "Hotel/Motel".into(),

            PropertySubType::Industrial => "Industrial".into(),

            PropertySubType::ManufacturedHome => "Manufactured Home".into(),

            PropertySubType::ManufacturedOnLand => "Manufactured On Land".into(),

            PropertySubType::MixedUse => "Mixed Use".into(),

            PropertySubType::MobileHome => "Mobile Home".into(),

            PropertySubType::MultiFamily => "Multi Family".into(),

            PropertySubType::Office => "Office".into(),

            PropertySubType::OwnYourOwn => "Own Your Own".into(),

            PropertySubType::Quadruplex => "Quadruplex".into(),

            PropertySubType::Ranch => "Ranch".into(),

            PropertySubType::Retail => "Retail".into(),

            PropertySubType::SingleFamilyResidence => "Single Family Residence".into(),

            PropertySubType::StockCooperative => "Stock Cooperative".into(),

            PropertySubType::Timeshare => "Timeshare".into(),

            PropertySubType::Townhouse => "Townhouse".into(),

            PropertySubType::Triplex => "Triplex".into(),

            PropertySubType::UnimprovedLand => "Unimproved Land".into(),

            PropertySubType::Warehouse => "Warehouse".into(),

            PropertySubType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            PropertySubType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for PropertySubType {
    fn from(s: String) -> PropertySubType {
        match s.as_ref() {
            "Agriculture" => PropertySubType::Agriculture,

            "Apartment" => PropertySubType::Apartment,

            "Boat Slip" => PropertySubType::BoatSlip,

            "Business" => PropertySubType::Business,

            "Cabin" => PropertySubType::Cabin,

            "Condominium" => PropertySubType::Condominium,

            "Deeded Parking" => PropertySubType::DeededParking,

            "Duplex" => PropertySubType::Duplex,

            "Farm" => PropertySubType::Farm,

            "Hotel/Motel" => PropertySubType::HotelMotel,

            "Industrial" => PropertySubType::Industrial,

            "Manufactured Home" => PropertySubType::ManufacturedHome,

            "Manufactured On Land" => PropertySubType::ManufacturedOnLand,

            "Mixed Use" => PropertySubType::MixedUse,

            "Mobile Home" => PropertySubType::MobileHome,

            "Multi Family" => PropertySubType::MultiFamily,

            "Office" => PropertySubType::Office,

            "Own Your Own" => PropertySubType::OwnYourOwn,

            "Quadruplex" => PropertySubType::Quadruplex,

            "Ranch" => PropertySubType::Ranch,

            "Retail" => PropertySubType::Retail,

            "Single Family Residence" => PropertySubType::SingleFamilyResidence,

            "Stock Cooperative" => PropertySubType::StockCooperative,

            "Timeshare" => PropertySubType::Timeshare,

            "Townhouse" => PropertySubType::Townhouse,

            "Triplex" => PropertySubType::Triplex,

            "Unimproved Land" => PropertySubType::UnimprovedLand,

            "Warehouse" => PropertySubType::Warehouse,

            _ => PropertySubType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PropertySubType {
    fn from(s: &str) -> PropertySubType {
        match s {
            "Agriculture" => PropertySubType::Agriculture,

            "Apartment" => PropertySubType::Apartment,

            "Boat Slip" => PropertySubType::BoatSlip,

            "Business" => PropertySubType::Business,

            "Cabin" => PropertySubType::Cabin,

            "Condominium" => PropertySubType::Condominium,

            "Deeded Parking" => PropertySubType::DeededParking,

            "Duplex" => PropertySubType::Duplex,

            "Farm" => PropertySubType::Farm,

            "Hotel/Motel" => PropertySubType::HotelMotel,

            "Industrial" => PropertySubType::Industrial,

            "Manufactured Home" => PropertySubType::ManufacturedHome,

            "Manufactured On Land" => PropertySubType::ManufacturedOnLand,

            "Mixed Use" => PropertySubType::MixedUse,

            "Mobile Home" => PropertySubType::MobileHome,

            "Multi Family" => PropertySubType::MultiFamily,

            "Office" => PropertySubType::Office,

            "Own Your Own" => PropertySubType::OwnYourOwn,

            "Quadruplex" => PropertySubType::Quadruplex,

            "Ranch" => PropertySubType::Ranch,

            "Retail" => PropertySubType::Retail,

            "Single Family Residence" => PropertySubType::SingleFamilyResidence,

            "Stock Cooperative" => PropertySubType::StockCooperative,

            "Timeshare" => PropertySubType::Timeshare,

            "Townhouse" => PropertySubType::Townhouse,

            "Triplex" => PropertySubType::Triplex,

            "Unimproved Land" => PropertySubType::UnimprovedLand,

            "Warehouse" => PropertySubType::Warehouse,

            _ => PropertySubType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PropertySubType> for &'a str {
    fn from(s: &'a PropertySubType) -> &'a str {
        match s {
            PropertySubType::Agriculture => "Agriculture",

            PropertySubType::Apartment => "Apartment",

            PropertySubType::BoatSlip => "Boat Slip",

            PropertySubType::Business => "Business",

            PropertySubType::Cabin => "Cabin",

            PropertySubType::Condominium => "Condominium",

            PropertySubType::DeededParking => "Deeded Parking",

            PropertySubType::Duplex => "Duplex",

            PropertySubType::Farm => "Farm",

            PropertySubType::HotelMotel => "Hotel/Motel",

            PropertySubType::Industrial => "Industrial",

            PropertySubType::ManufacturedHome => "Manufactured Home",

            PropertySubType::ManufacturedOnLand => "Manufactured On Land",

            PropertySubType::MixedUse => "Mixed Use",

            PropertySubType::MobileHome => "Mobile Home",

            PropertySubType::MultiFamily => "Multi Family",

            PropertySubType::Office => "Office",

            PropertySubType::OwnYourOwn => "Own Your Own",

            PropertySubType::Quadruplex => "Quadruplex",

            PropertySubType::Ranch => "Ranch",

            PropertySubType::Retail => "Retail",

            PropertySubType::SingleFamilyResidence => "Single Family Residence",

            PropertySubType::StockCooperative => "Stock Cooperative",

            PropertySubType::Timeshare => "Timeshare",

            PropertySubType::Townhouse => "Townhouse",

            PropertySubType::Triplex => "Triplex",

            PropertySubType::UnimprovedLand => "Unimproved Land",

            PropertySubType::Warehouse => "Warehouse",

            PropertySubType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PropertySubType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PropertySubType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
