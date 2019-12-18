// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [AssociationFeeIncludes Lookups](https://ddwiki.reso.org/display/DDW17/AssociationFeeIncludes+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AssociationFeeIncludes {
    /// "[Cable TV](https://ddwiki.reso.org/display/DDW17/Cable+TV)": Cable TV is included in the fee paid to the Association.
    CableTV,

    /// "[Earthquake Insurance](https://ddwiki.reso.org/display/DDW17/Earthquake+Insurance)": Earthquake Insurance is included in the fee paid to the Association.
    EarthquakeInsurance,

    /// "[Electricity](https://ddwiki.reso.org/display/DDW17/Electricity)": Electricity is included in the fee paid to the Association.
    Electricity,

    /// "[Gas](https://ddwiki.reso.org/display/DDW17/Gas)": Gas is included in the fee paid to the Association.
    Gas,

    /// "[Insurance](https://ddwiki.reso.org/display/DDW17/Insurance)": Insurance is included in the fee paid to the Association.
    Insurance,

    /// "[Internet](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243641)": Internet access is included with the Home Owner’s Association dues paid by the owner.  Questions about the means of access (e.g. wifi, ethernet), the speed of the access and other information about the Internet Service Provider (ISP) may be directed to the Home Owner’s Association.
    Internet,

    /// "[Maintenance Grounds](https://ddwiki.reso.org/display/DDW17/Maintenance+Grounds)": Maintenance of the grounds including lawns and common areas but not including exterior structures.
    MaintenanceGrounds,

    /// "[Maintenance Structure](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243632)": Maintenance of the exterior of the structure including roofing, walls, exterior structures and does not include the grounds.
    MaintenanceStructure,

    /// "[Pest Control](https://ddwiki.reso.org/display/DDW17/Pest+Control)": Pest Control is included in the fee paid to the Association.
    PestControl,

    /// "[Security](https://ddwiki.reso.org/display/DDW17/Security)": Security is included in the fee paid to the Association.
    Security,

    /// "[Sewer](https://ddwiki.reso.org/display/DDW17/Sewer)": Sewer is included in the fee paid to the Association.
    Sewer,

    /// "[Snow Removal](https://ddwiki.reso.org/display/DDW17/Snow+Removal)": Snow Removal is included in the fee paid to the Association.
    SnowRemoval,

    /// "[Trash](https://ddwiki.reso.org/display/DDW17/Trash)": Trash is included in the fee paid to the Association.
    Trash,

    /// "[Utilities](https://ddwiki.reso.org/display/DDW17/Utilities)": Utilities is included in the fee paid to the Association.
    Utilities,

    /// "[Water](https://ddwiki.reso.org/display/DDW17/Water)": Water is included in the fee paid to the Association.
    Water,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for AssociationFeeIncludes {
    fn from(s: String) -> AssociationFeeIncludes {
        match s.as_ref() {
            "Cable TV" => AssociationFeeIncludes::CableTV,

            "Earthquake Insurance" => AssociationFeeIncludes::EarthquakeInsurance,

            "Electricity" => AssociationFeeIncludes::Electricity,

            "Gas" => AssociationFeeIncludes::Gas,

            "Insurance" => AssociationFeeIncludes::Insurance,

            "Internet" => AssociationFeeIncludes::Internet,

            "Maintenance Grounds" => AssociationFeeIncludes::MaintenanceGrounds,

            "Maintenance Structure" => AssociationFeeIncludes::MaintenanceStructure,

            "Pest Control" => AssociationFeeIncludes::PestControl,

            "Security" => AssociationFeeIncludes::Security,

            "Sewer" => AssociationFeeIncludes::Sewer,

            "Snow Removal" => AssociationFeeIncludes::SnowRemoval,

            "Trash" => AssociationFeeIncludes::Trash,

            "Utilities" => AssociationFeeIncludes::Utilities,

            "Water" => AssociationFeeIncludes::Water,

            _ => AssociationFeeIncludes::OpenEnumeration(s),
        }
    }
}

impl From<&str> for AssociationFeeIncludes {
    fn from(s: &str) -> AssociationFeeIncludes {
        match s {
            "Cable TV" => AssociationFeeIncludes::CableTV,

            "Earthquake Insurance" => AssociationFeeIncludes::EarthquakeInsurance,

            "Electricity" => AssociationFeeIncludes::Electricity,

            "Gas" => AssociationFeeIncludes::Gas,

            "Insurance" => AssociationFeeIncludes::Insurance,

            "Internet" => AssociationFeeIncludes::Internet,

            "Maintenance Grounds" => AssociationFeeIncludes::MaintenanceGrounds,

            "Maintenance Structure" => AssociationFeeIncludes::MaintenanceStructure,

            "Pest Control" => AssociationFeeIncludes::PestControl,

            "Security" => AssociationFeeIncludes::Security,

            "Sewer" => AssociationFeeIncludes::Sewer,

            "Snow Removal" => AssociationFeeIncludes::SnowRemoval,

            "Trash" => AssociationFeeIncludes::Trash,

            "Utilities" => AssociationFeeIncludes::Utilities,

            "Water" => AssociationFeeIncludes::Water,

            _ => AssociationFeeIncludes::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a AssociationFeeIncludes> for &'a str {
    fn from(s: &'a AssociationFeeIncludes) -> &'a str {
        match s {
            AssociationFeeIncludes::CableTV => "Cable TV",

            AssociationFeeIncludes::EarthquakeInsurance => "Earthquake Insurance",

            AssociationFeeIncludes::Electricity => "Electricity",

            AssociationFeeIncludes::Gas => "Gas",

            AssociationFeeIncludes::Insurance => "Insurance",

            AssociationFeeIncludes::Internet => "Internet",

            AssociationFeeIncludes::MaintenanceGrounds => "Maintenance Grounds",

            AssociationFeeIncludes::MaintenanceStructure => "Maintenance Structure",

            AssociationFeeIncludes::PestControl => "Pest Control",

            AssociationFeeIncludes::Security => "Security",

            AssociationFeeIncludes::Sewer => "Sewer",

            AssociationFeeIncludes::SnowRemoval => "Snow Removal",

            AssociationFeeIncludes::Trash => "Trash",

            AssociationFeeIncludes::Utilities => "Utilities",

            AssociationFeeIncludes::Water => "Water",

            AssociationFeeIncludes::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for AssociationFeeIncludes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AssociationFeeIncludes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_association_fee_includes_format {
    use super::AssociationFeeIncludes;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<AssociationFeeIncludes>>,
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
    ) -> Result<Option<Vec<AssociationFeeIncludes>>, D::Error>
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
