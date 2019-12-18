// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [UnitTypeType Lookups](https://ddwiki.reso.org/display/DDW17/UnitTypeType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitTypeType {
    /// "[1 Bedroom](https://ddwiki.reso.org/display/DDW17/1+Bedroom)": The type of unit is a 1 bedroom.
    _1Bedroom,

    /// "[2 Bedroom](https://ddwiki.reso.org/display/DDW17/2+Bedroom)": The type of unit is a 2 bedroom.
    _2Bedroom,

    /// "[3 Bedroom](https://ddwiki.reso.org/display/DDW17/3+Bedroom)": The type of unit is a 3 bedroom.
    _3Bedroom,

    /// "[4 Bedroom Or More](https://ddwiki.reso.org/display/DDW17/4+Bedroom+Or+More)": The type of unit is a 4 or more bedroom.
    _4BedroomOrMore,

    /// "[Apartments](https://ddwiki.reso.org/display/DDW17/Apartments)": The type of unit is apartments.
    Apartments,

    /// "[Efficiency](https://ddwiki.reso.org/display/DDW17/Efficiency)": The type of unit is an efficiency.
    Efficiency,

    /// "[Loft](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246608)": The type of unit is a loft.
    Loft,

    /// "[Manager's Unit](https://ddwiki.reso.org/display/DDW17/Manager%27s+Unit)": The type of unit is a manager's unit.
    ManagersUnit,

    /// "[Penthouse](https://ddwiki.reso.org/display/DDW17/Penthouse)": The type of unit is a penthouse.
    Penthouse,

    /// "[Studio](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246611)": The type of unit is a studio.
    Studio,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for UnitTypeType {
    fn from(s: String) -> UnitTypeType {
        match s.as_ref() {
            "1 Bedroom" => UnitTypeType::_1Bedroom,

            "2 Bedroom" => UnitTypeType::_2Bedroom,

            "3 Bedroom" => UnitTypeType::_3Bedroom,

            "4 Bedroom Or More" => UnitTypeType::_4BedroomOrMore,

            "Apartments" => UnitTypeType::Apartments,

            "Efficiency" => UnitTypeType::Efficiency,

            "Loft" => UnitTypeType::Loft,

            "Manager's Unit" => UnitTypeType::ManagersUnit,

            "Penthouse" => UnitTypeType::Penthouse,

            "Studio" => UnitTypeType::Studio,

            _ => UnitTypeType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for UnitTypeType {
    fn from(s: &str) -> UnitTypeType {
        match s {
            "1 Bedroom" => UnitTypeType::_1Bedroom,

            "2 Bedroom" => UnitTypeType::_2Bedroom,

            "3 Bedroom" => UnitTypeType::_3Bedroom,

            "4 Bedroom Or More" => UnitTypeType::_4BedroomOrMore,

            "Apartments" => UnitTypeType::Apartments,

            "Efficiency" => UnitTypeType::Efficiency,

            "Loft" => UnitTypeType::Loft,

            "Manager's Unit" => UnitTypeType::ManagersUnit,

            "Penthouse" => UnitTypeType::Penthouse,

            "Studio" => UnitTypeType::Studio,

            _ => UnitTypeType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a UnitTypeType> for &'a str {
    fn from(s: &'a UnitTypeType) -> &'a str {
        match s {
            UnitTypeType::_1Bedroom => "1 Bedroom",

            UnitTypeType::_2Bedroom => "2 Bedroom",

            UnitTypeType::_3Bedroom => "3 Bedroom",

            UnitTypeType::_4BedroomOrMore => "4 Bedroom Or More",

            UnitTypeType::Apartments => "Apartments",

            UnitTypeType::Efficiency => "Efficiency",

            UnitTypeType::Loft => "Loft",

            UnitTypeType::ManagersUnit => "Manager's Unit",

            UnitTypeType::Penthouse => "Penthouse",

            UnitTypeType::Studio => "Studio",

            UnitTypeType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for UnitTypeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UnitTypeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_unit_type_type_format {
    use super::UnitTypeType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<UnitTypeType>>,
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
    ) -> Result<Option<Vec<UnitTypeType>>, D::Error>
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
