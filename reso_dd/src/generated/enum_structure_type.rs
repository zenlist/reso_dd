// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [StructureType Lookups](https://ddwiki.reso.org/display/DDW17/StructureType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StructureType {
    /// "[Cabin](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246437)": A single family residence that may have limited utilities and rooms.
    Cabin,

    /// "[Dock](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246438)": A floating or pillar supported structure over water used to park water craft.
    Dock,

    /// "[Duplex](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246439)": A multi family structure with two independent units sharing a common roof.
    Duplex,

    /// "[Flex](https://ddwiki.reso.org/display/DDW17/Flex)": A commercial property that is designed to be used in different ways.  e.g. Office, Retail or Warehouse.
    Flex,

    /// "[Hotel/Motel](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246441)": A commercial structure designed to be a hotel or motel.
    HotelMotel,

    /// "[House](https://ddwiki.reso.org/display/DDW17/House)": A single family residence on real property either attached or detached from another structure.  A house may be modular (aka prefabricated), but not a manufactured home with serial/license number.
    House,

    /// "[Industrial](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246443)": A commercial structure designed for industrial use.
    Industrial,

    /// "[Manufactured House](https://ddwiki.reso.org/display/DDW17/Manufactured+House)": A factory built house that is transported to the lot.  A manufactured home will have a serial/license number, where prefabricated (modular) homes are classified with stick built as "House" in this list.
    ManufacturedHouse,

    /// "[Mixed Use](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246445)": The property is designed be used in more than one way.  This is typically a combination of residential and commercial space.  e.g. a dwelling over a retail space.
    MixedUse,

    /// "[Multi Family](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246446)": A structure or complex with 5 or more units that are individual dwellings.
    MultiFamily,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246447)": The property has no structure.
    None,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246448)": A commercial structure designed to be used as office space.
    Office,

    /// "[Quadruplex](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246449)": A multi family structure with four independent units sharing a common roof.
    Quadruplex,

    /// "[Retail](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246450)": A commercial structure designed to be used for retail space.
    Retail,

    /// "[Townhouse](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246451)": A dwelling unit, generally having two or more floors and attached to other similar units via party walls.
    Townhouse,

    /// "[Triplex](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246452)": A multi family structure with three independent units sharing a common roof.
    Triplex,

    /// "[Warehouse](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246453)": A commercial structure designed for warehousing.
    Warehouse,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for StructureType {
    fn from(s: String) -> StructureType {
        match s.as_ref() {
            "Cabin" => StructureType::Cabin,

            "Dock" => StructureType::Dock,

            "Duplex" => StructureType::Duplex,

            "Flex" => StructureType::Flex,

            "Hotel/Motel" => StructureType::HotelMotel,

            "House" => StructureType::House,

            "Industrial" => StructureType::Industrial,

            "Manufactured House" => StructureType::ManufacturedHouse,

            "Mixed Use" => StructureType::MixedUse,

            "Multi Family" => StructureType::MultiFamily,

            "None" => StructureType::None,

            "Office" => StructureType::Office,

            "Quadruplex" => StructureType::Quadruplex,

            "Retail" => StructureType::Retail,

            "Townhouse" => StructureType::Townhouse,

            "Triplex" => StructureType::Triplex,

            "Warehouse" => StructureType::Warehouse,

            _ => StructureType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for StructureType {
    fn from(s: &str) -> StructureType {
        match s {
            "Cabin" => StructureType::Cabin,

            "Dock" => StructureType::Dock,

            "Duplex" => StructureType::Duplex,

            "Flex" => StructureType::Flex,

            "Hotel/Motel" => StructureType::HotelMotel,

            "House" => StructureType::House,

            "Industrial" => StructureType::Industrial,

            "Manufactured House" => StructureType::ManufacturedHouse,

            "Mixed Use" => StructureType::MixedUse,

            "Multi Family" => StructureType::MultiFamily,

            "None" => StructureType::None,

            "Office" => StructureType::Office,

            "Quadruplex" => StructureType::Quadruplex,

            "Retail" => StructureType::Retail,

            "Townhouse" => StructureType::Townhouse,

            "Triplex" => StructureType::Triplex,

            "Warehouse" => StructureType::Warehouse,

            _ => StructureType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a StructureType> for &'a str {
    fn from(s: &'a StructureType) -> &'a str {
        match s {
            StructureType::Cabin => "Cabin",

            StructureType::Dock => "Dock",

            StructureType::Duplex => "Duplex",

            StructureType::Flex => "Flex",

            StructureType::HotelMotel => "Hotel/Motel",

            StructureType::House => "House",

            StructureType::Industrial => "Industrial",

            StructureType::ManufacturedHouse => "Manufactured House",

            StructureType::MixedUse => "Mixed Use",

            StructureType::MultiFamily => "Multi Family",

            StructureType::None => "None",

            StructureType::Office => "Office",

            StructureType::Quadruplex => "Quadruplex",

            StructureType::Retail => "Retail",

            StructureType::Townhouse => "Townhouse",

            StructureType::Triplex => "Triplex",

            StructureType::Warehouse => "Warehouse",

            StructureType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for StructureType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StructureType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_structure_type_format {
    use super::StructureType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<StructureType>>,
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
    ) -> Result<Option<Vec<StructureType>>, D::Error>
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
