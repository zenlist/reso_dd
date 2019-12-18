// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [GreenSustainability Lookups](https://ddwiki.reso.org/display/DDW17/GreenSustainability+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreenSustainability {
    /// "[Conserving Methods](https://ddwiki.reso.org/display/DDW17/Conserving+Methods)": Construction is planned to require fewer materials while maintaining structural integrity. May include advanced wall framing as documented in several major green building programs. May also include indigenous construction methods such as straw bale, sod, clay, etc., based on local climate, materials, and practices.
    ConservingMethods,

    /// "[Onsite Recycling Center](https://ddwiki.reso.org/display/DDW17/Onsite+Recycling+Center)": Property includes sufficient built-in storage space and/ or containers for temporary storage of recyclable materials and/or compost collection.
    OnsiteRecyclingCenter,

    /// "[Recyclable Materials](https://ddwiki.reso.org/display/DDW17/Recyclable+Materials)": Structure includes multiple products that have a significant amount of postconsumer recycled content. Major green building programs tend to use 25 percent postconsumer recycled content as a threshold. Postindustrial recycled content tends to count toward overall content, but to a less scale. Some more common examples of recycled content materials include masonry, paving stones, or foundations with fly ash; aluminum gutters and downspouts; decking; drywall fibers, insulation, and house wrap; vinyl plastics; countertops; and cabinets, interior doors, or trim. More details are available on Home Innovation Research Labs “Browse Green Certified Products” tool or LEED Environmentally Preferred Products credit table.
    RecyclableMaterials,

    /// "[Recycled Materials](https://ddwiki.reso.org/display/DDW17/Recycled+Materials)": Structure includes multiple products that have a significant amount of postconsumer recycled content. Major green building programs tend to use 25 percent postconsumer recycled content as a threshold. Postindustrial recycled content tends to count toward overall content, but to a less scale. Some more common examples of recycled content materials include masonry, paving stones, or foundations with fly ash; aluminum gutters and downspouts; decking; drywall fibers, insulation, and house wrap; vinyl plastics; countertops; and cabinets, interior doors, or trim. More details are available on Home Innovation Research Labs “Browse Green Certified Products” tool or LEED Environmentally Preferred Products credit table.
    RecycledMaterials,

    /// "[Regionally-Sourced Materials](https://ddwiki.reso.org/display/DDW17/Regionally-Sourced+Materials)": Refers to building materials that were manufactured, extracted, harvested, or recovered within 500 miles of the building. Several major green building programs define regionally sourced as within a 500-mile radius.
    RegionallySourcedMaterials,

    /// "[Renewable Materials](https://ddwiki.reso.org/display/DDW17/Renewable+Materials)": Structure includes materials that are naturally occurring, abundant, and/or fast-growing materials. Some products are certified to come from fast-growing or otherwise renewable sources such as flooring and wood products. Some materials may be bio-based, which means they have been processed from once-living materials such as paper, straw, soy, natural fibers, and crops.
    RenewableMaterials,

    /// "[Salvaged Materials](https://ddwiki.reso.org/display/DDW17/Salvaged+Materials)": Structure incorporates materials that were diverted from a landfill and/or sourced to give an otherwise unused item fresh use as an attached fixture. May include bricks, timbers, roofing, flooring, walls, cabinets, doors, etc.
    SalvagedMaterials,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for GreenSustainability {
    fn from(s: String) -> GreenSustainability {
        match s.as_ref() {
            "Conserving Methods" => GreenSustainability::ConservingMethods,

            "Onsite Recycling Center" => GreenSustainability::OnsiteRecyclingCenter,

            "Recyclable Materials" => GreenSustainability::RecyclableMaterials,

            "Recycled Materials" => GreenSustainability::RecycledMaterials,

            "Regionally-Sourced Materials" => GreenSustainability::RegionallySourcedMaterials,

            "Renewable Materials" => GreenSustainability::RenewableMaterials,

            "Salvaged Materials" => GreenSustainability::SalvagedMaterials,

            _ => GreenSustainability::OpenEnumeration(s),
        }
    }
}

impl From<&str> for GreenSustainability {
    fn from(s: &str) -> GreenSustainability {
        match s {
            "Conserving Methods" => GreenSustainability::ConservingMethods,

            "Onsite Recycling Center" => GreenSustainability::OnsiteRecyclingCenter,

            "Recyclable Materials" => GreenSustainability::RecyclableMaterials,

            "Recycled Materials" => GreenSustainability::RecycledMaterials,

            "Regionally-Sourced Materials" => GreenSustainability::RegionallySourcedMaterials,

            "Renewable Materials" => GreenSustainability::RenewableMaterials,

            "Salvaged Materials" => GreenSustainability::SalvagedMaterials,

            _ => GreenSustainability::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a GreenSustainability> for &'a str {
    fn from(s: &'a GreenSustainability) -> &'a str {
        match s {
            GreenSustainability::ConservingMethods => "Conserving Methods",

            GreenSustainability::OnsiteRecyclingCenter => "Onsite Recycling Center",

            GreenSustainability::RecyclableMaterials => "Recyclable Materials",

            GreenSustainability::RecycledMaterials => "Recycled Materials",

            GreenSustainability::RegionallySourcedMaterials => "Regionally-Sourced Materials",

            GreenSustainability::RenewableMaterials => "Renewable Materials",

            GreenSustainability::SalvagedMaterials => "Salvaged Materials",

            GreenSustainability::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for GreenSustainability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for GreenSustainability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_green_sustainability_format {
    use super::GreenSustainability;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<GreenSustainability>>,
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
    ) -> Result<Option<Vec<GreenSustainability>>, D::Error>
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
