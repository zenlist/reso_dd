// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Utilities Lookups](https://ddwiki.reso.org/display/DDW17/Utilities+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Utilities {
    /// "[Cable Available](https://ddwiki.reso.org/display/DDW17/Cable+Available)": The property has cable available but is not connected.
    CableAvailable,

    /// "[Cable Connected](https://ddwiki.reso.org/display/DDW17/Cable+Connected)": Cable service is physically connected, but not necessarily paid.
    CableConnected,

    /// "[Cable Not Available](https://ddwiki.reso.org/display/DDW17/Cable+Not+Available)": Cable is not available in the area of the property.
    CableNotAvailable,

    /// "[Electricity Available](https://ddwiki.reso.org/display/DDW17/Electricity+Available)": Electricity is available from the public utility but not connected.
    ElectricityAvailable,

    /// "[Electricity Connected](https://ddwiki.reso.org/display/DDW17/Electricity+Connected)": Electricity from the public utility is available and connected, but not necessarily paid.
    ElectricityConnected,

    /// "[Electricity Not Available](https://ddwiki.reso.org/display/DDW17/Electricity+Not+Available)": Electricity from the public utility is not available.  An independent source of electricity is the only option.
    ElectricityNotAvailable,

    /// "[Natural Gas Available](https://ddwiki.reso.org/display/DDW17/Natural+Gas+Available)": Natural gas is available from the public utility but not connected.
    NaturalGasAvailable,

    /// "[Natural Gas Connected](https://ddwiki.reso.org/display/DDW17/Natural+Gas+Connected)": Natural gas from the public utility is available and connected, but not necessarily paid.
    NaturalGasConnected,

    /// "[Natural Gas Not Available](https://ddwiki.reso.org/display/DDW17/Natural+Gas+Not+Available)": Natural gas from the public utility is not available.  An independent source of gas is the only option.  i.e. propane.
    NaturalGasNotAvailable,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246579)": There are no public utilities currently available or connected.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246580)": There are utilities other than those listed.
    Other,

    /// "[Phone Available](https://ddwiki.reso.org/display/DDW17/Phone+Available)": The property has telephone service available but is not connected.
    PhoneAvailable,

    /// "[Phone Connected](https://ddwiki.reso.org/display/DDW17/Phone+Connected)": Telephone service is physically connected, but not necessarily paid.
    PhoneConnected,

    /// "[Phone Not Available](https://ddwiki.reso.org/display/DDW17/Phone+Not+Available)": Telephone service is not available in the area of the property.
    PhoneNotAvailable,

    /// "[Propane](https://ddwiki.reso.org/display/DDW17/Propane)": The property has a propane system.
    Propane,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246585)": See remarks for details about the public or other utilities available/installed at the property.
    SeeRemarks,

    /// "[Sewer Available](https://ddwiki.reso.org/display/DDW17/Sewer+Available)": Sewer service is available from the public utility but not connected.
    SewerAvailable,

    /// "[Sewer Connected](https://ddwiki.reso.org/display/DDW17/Sewer+Connected)": Sewer service from the public utility is available and connected, but not necessarily paid.
    SewerConnected,

    /// "[Sewer Not Available](https://ddwiki.reso.org/display/DDW17/Sewer+Not+Available)": Sewer service from the public utility is not available.  An independent alternative to sewer is the only option.  i.e. septic.
    SewerNotAvailable,

    /// "[Underground Utilities](https://ddwiki.reso.org/display/DDW17/Underground+Utilities)": All or some of the utilities are run underground.
    UndergroundUtilities,

    /// "[Water Available](https://ddwiki.reso.org/display/DDW17/Water+Available)": Water service is available from the public utility but not connected.
    WaterAvailable,

    /// "[Water Connected](https://ddwiki.reso.org/display/DDW17/Water+Connected)": Water service from the public utility is available and connected, but not necessarily paid.
    WaterConnected,

    /// "[Water Not Available](https://ddwiki.reso.org/display/DDW17/Water+Not+Available)": Water service from the public utility is not available.  An independent source for water is the only option.  i.e. well.
    WaterNotAvailable,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Utilities {
    fn from_str(s: &str) -> Utilities {
        match s {
            "Cable Available" => Utilities::CableAvailable,

            "Cable Connected" => Utilities::CableConnected,

            "Cable Not Available" => Utilities::CableNotAvailable,

            "Electricity Available" => Utilities::ElectricityAvailable,

            "Electricity Connected" => Utilities::ElectricityConnected,

            "Electricity Not Available" => Utilities::ElectricityNotAvailable,

            "Natural Gas Available" => Utilities::NaturalGasAvailable,

            "Natural Gas Connected" => Utilities::NaturalGasConnected,

            "Natural Gas Not Available" => Utilities::NaturalGasNotAvailable,

            "None" => Utilities::None,

            "Other" => Utilities::Other,

            "Phone Available" => Utilities::PhoneAvailable,

            "Phone Connected" => Utilities::PhoneConnected,

            "Phone Not Available" => Utilities::PhoneNotAvailable,

            "Propane" => Utilities::Propane,

            "See Remarks" => Utilities::SeeRemarks,

            "Sewer Available" => Utilities::SewerAvailable,

            "Sewer Connected" => Utilities::SewerConnected,

            "Sewer Not Available" => Utilities::SewerNotAvailable,

            "Underground Utilities" => Utilities::UndergroundUtilities,

            "Water Available" => Utilities::WaterAvailable,

            "Water Connected" => Utilities::WaterConnected,

            "Water Not Available" => Utilities::WaterNotAvailable,

            _ => Utilities::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Utilities {
        match s.as_ref() {
            "Cable Available" => Utilities::CableAvailable,

            "Cable Connected" => Utilities::CableConnected,

            "Cable Not Available" => Utilities::CableNotAvailable,

            "Electricity Available" => Utilities::ElectricityAvailable,

            "Electricity Connected" => Utilities::ElectricityConnected,

            "Electricity Not Available" => Utilities::ElectricityNotAvailable,

            "Natural Gas Available" => Utilities::NaturalGasAvailable,

            "Natural Gas Connected" => Utilities::NaturalGasConnected,

            "Natural Gas Not Available" => Utilities::NaturalGasNotAvailable,

            "None" => Utilities::None,

            "Other" => Utilities::Other,

            "Phone Available" => Utilities::PhoneAvailable,

            "Phone Connected" => Utilities::PhoneConnected,

            "Phone Not Available" => Utilities::PhoneNotAvailable,

            "Propane" => Utilities::Propane,

            "See Remarks" => Utilities::SeeRemarks,

            "Sewer Available" => Utilities::SewerAvailable,

            "Sewer Connected" => Utilities::SewerConnected,

            "Sewer Not Available" => Utilities::SewerNotAvailable,

            "Underground Utilities" => Utilities::UndergroundUtilities,

            "Water Available" => Utilities::WaterAvailable,

            "Water Connected" => Utilities::WaterConnected,

            "Water Not Available" => Utilities::WaterNotAvailable,

            _ => Utilities::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Utilities::CableAvailable => "Cable Available",

            Utilities::CableConnected => "Cable Connected",

            Utilities::CableNotAvailable => "Cable Not Available",

            Utilities::ElectricityAvailable => "Electricity Available",

            Utilities::ElectricityConnected => "Electricity Connected",

            Utilities::ElectricityNotAvailable => "Electricity Not Available",

            Utilities::NaturalGasAvailable => "Natural Gas Available",

            Utilities::NaturalGasConnected => "Natural Gas Connected",

            Utilities::NaturalGasNotAvailable => "Natural Gas Not Available",

            Utilities::None => "None",

            Utilities::Other => "Other",

            Utilities::PhoneAvailable => "Phone Available",

            Utilities::PhoneConnected => "Phone Connected",

            Utilities::PhoneNotAvailable => "Phone Not Available",

            Utilities::Propane => "Propane",

            Utilities::SeeRemarks => "See Remarks",

            Utilities::SewerAvailable => "Sewer Available",

            Utilities::SewerConnected => "Sewer Connected",

            Utilities::SewerNotAvailable => "Sewer Not Available",

            Utilities::UndergroundUtilities => "Underground Utilities",

            Utilities::WaterAvailable => "Water Available",

            Utilities::WaterConnected => "Water Connected",

            Utilities::WaterNotAvailable => "Water Not Available",

            Utilities::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Utilities::CableAvailable => "Cable Available".into(),

            Utilities::CableConnected => "Cable Connected".into(),

            Utilities::CableNotAvailable => "Cable Not Available".into(),

            Utilities::ElectricityAvailable => "Electricity Available".into(),

            Utilities::ElectricityConnected => "Electricity Connected".into(),

            Utilities::ElectricityNotAvailable => "Electricity Not Available".into(),

            Utilities::NaturalGasAvailable => "Natural Gas Available".into(),

            Utilities::NaturalGasConnected => "Natural Gas Connected".into(),

            Utilities::NaturalGasNotAvailable => "Natural Gas Not Available".into(),

            Utilities::None => "None".into(),

            Utilities::Other => "Other".into(),

            Utilities::PhoneAvailable => "Phone Available".into(),

            Utilities::PhoneConnected => "Phone Connected".into(),

            Utilities::PhoneNotAvailable => "Phone Not Available".into(),

            Utilities::Propane => "Propane".into(),

            Utilities::SeeRemarks => "See Remarks".into(),

            Utilities::SewerAvailable => "Sewer Available".into(),

            Utilities::SewerConnected => "Sewer Connected".into(),

            Utilities::SewerNotAvailable => "Sewer Not Available".into(),

            Utilities::UndergroundUtilities => "Underground Utilities".into(),

            Utilities::WaterAvailable => "Water Available".into(),

            Utilities::WaterConnected => "Water Connected".into(),

            Utilities::WaterNotAvailable => "Water Not Available".into(),

            Utilities::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Utilities::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Utilities {
    fn from(s: String) -> Utilities {
        match s.as_ref() {
            "Cable Available" => Utilities::CableAvailable,

            "Cable Connected" => Utilities::CableConnected,

            "Cable Not Available" => Utilities::CableNotAvailable,

            "Electricity Available" => Utilities::ElectricityAvailable,

            "Electricity Connected" => Utilities::ElectricityConnected,

            "Electricity Not Available" => Utilities::ElectricityNotAvailable,

            "Natural Gas Available" => Utilities::NaturalGasAvailable,

            "Natural Gas Connected" => Utilities::NaturalGasConnected,

            "Natural Gas Not Available" => Utilities::NaturalGasNotAvailable,

            "None" => Utilities::None,

            "Other" => Utilities::Other,

            "Phone Available" => Utilities::PhoneAvailable,

            "Phone Connected" => Utilities::PhoneConnected,

            "Phone Not Available" => Utilities::PhoneNotAvailable,

            "Propane" => Utilities::Propane,

            "See Remarks" => Utilities::SeeRemarks,

            "Sewer Available" => Utilities::SewerAvailable,

            "Sewer Connected" => Utilities::SewerConnected,

            "Sewer Not Available" => Utilities::SewerNotAvailable,

            "Underground Utilities" => Utilities::UndergroundUtilities,

            "Water Available" => Utilities::WaterAvailable,

            "Water Connected" => Utilities::WaterConnected,

            "Water Not Available" => Utilities::WaterNotAvailable,

            _ => Utilities::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Utilities {
    fn from(s: &str) -> Utilities {
        match s {
            "Cable Available" => Utilities::CableAvailable,

            "Cable Connected" => Utilities::CableConnected,

            "Cable Not Available" => Utilities::CableNotAvailable,

            "Electricity Available" => Utilities::ElectricityAvailable,

            "Electricity Connected" => Utilities::ElectricityConnected,

            "Electricity Not Available" => Utilities::ElectricityNotAvailable,

            "Natural Gas Available" => Utilities::NaturalGasAvailable,

            "Natural Gas Connected" => Utilities::NaturalGasConnected,

            "Natural Gas Not Available" => Utilities::NaturalGasNotAvailable,

            "None" => Utilities::None,

            "Other" => Utilities::Other,

            "Phone Available" => Utilities::PhoneAvailable,

            "Phone Connected" => Utilities::PhoneConnected,

            "Phone Not Available" => Utilities::PhoneNotAvailable,

            "Propane" => Utilities::Propane,

            "See Remarks" => Utilities::SeeRemarks,

            "Sewer Available" => Utilities::SewerAvailable,

            "Sewer Connected" => Utilities::SewerConnected,

            "Sewer Not Available" => Utilities::SewerNotAvailable,

            "Underground Utilities" => Utilities::UndergroundUtilities,

            "Water Available" => Utilities::WaterAvailable,

            "Water Connected" => Utilities::WaterConnected,

            "Water Not Available" => Utilities::WaterNotAvailable,

            _ => Utilities::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Utilities> for &'a str {
    fn from(s: &'a Utilities) -> &'a str {
        match s {
            Utilities::CableAvailable => "Cable Available",

            Utilities::CableConnected => "Cable Connected",

            Utilities::CableNotAvailable => "Cable Not Available",

            Utilities::ElectricityAvailable => "Electricity Available",

            Utilities::ElectricityConnected => "Electricity Connected",

            Utilities::ElectricityNotAvailable => "Electricity Not Available",

            Utilities::NaturalGasAvailable => "Natural Gas Available",

            Utilities::NaturalGasConnected => "Natural Gas Connected",

            Utilities::NaturalGasNotAvailable => "Natural Gas Not Available",

            Utilities::None => "None",

            Utilities::Other => "Other",

            Utilities::PhoneAvailable => "Phone Available",

            Utilities::PhoneConnected => "Phone Connected",

            Utilities::PhoneNotAvailable => "Phone Not Available",

            Utilities::Propane => "Propane",

            Utilities::SeeRemarks => "See Remarks",

            Utilities::SewerAvailable => "Sewer Available",

            Utilities::SewerConnected => "Sewer Connected",

            Utilities::SewerNotAvailable => "Sewer Not Available",

            Utilities::UndergroundUtilities => "Underground Utilities",

            Utilities::WaterAvailable => "Water Available",

            Utilities::WaterConnected => "Water Connected",

            Utilities::WaterNotAvailable => "Water Not Available",

            Utilities::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Utilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Utilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
