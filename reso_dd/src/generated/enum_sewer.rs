// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Sewer Lookups](https://ddwiki.reso.org/display/DDW17/Sewer+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Sewer {
    /// "[Aerobic Septic](https://ddwiki.reso.org/display/DDW17/Aerobic+Septic)": The property has an aerobic septic.
    AerobicSeptic,

    /// "[Cesspool](https://ddwiki.reso.org/display/DDW17/Cesspool)": The property has a cesspool.
    Cesspool,

    /// "[Engineered Septic](https://ddwiki.reso.org/display/DDW17/Engineered+Septic)": The property has an engineered septic.
    EngineeredSeptic,

    /// "[Holding Tank](https://ddwiki.reso.org/display/DDW17/Holding+Tank)": The property has a holding tank.
    HoldingTank,

    /// "[Mound Septic](https://ddwiki.reso.org/display/DDW17/Mound+Septic)": The property has a mound septic.
    MoundSeptic,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246259)": The property has no sewer, septic or cesspool.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246260)": The property has a system other than sewer, sceptic or cesspool in this list.
    Other,

    /// "[Perc Test On File](https://ddwiki.reso.org/display/DDW17/Perc+Test+On+File)": The property has a perc test on file.
    PercTestOnFile,

    /// "[Perc Test Required](https://ddwiki.reso.org/display/DDW17/Perc+Test+Required)": The property requires a perc test.
    PercTestRequired,

    /// "[Private Sewer](https://ddwiki.reso.org/display/DDW17/Private+Sewer)": The property has a private sewer.
    PrivateSewer,

    /// "[Public Sewer](https://ddwiki.reso.org/display/DDW17/Public+Sewer)": The property has a public sewer.
    PublicSewer,

    /// "[Septic Needed](https://ddwiki.reso.org/display/DDW17/Septic+Needed)": The property needs a septic system.
    SepticNeeded,

    /// "[Septic Tank](https://ddwiki.reso.org/display/DDW17/Septic+Tank)": The property has a septic tank.
    SepticTank,

    /// "[Shared Septic](https://ddwiki.reso.org/display/DDW17/Shared+Septic)": The property has a shared septic.
    SharedSeptic,

    /// "[Unknown](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246268)": The property's sewer/septic is unknown.
    Unknown,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Sewer {
    fn from_str(s: &str) -> Sewer {
        match s {
            "Aerobic Septic" => Sewer::AerobicSeptic,

            "Cesspool" => Sewer::Cesspool,

            "Engineered Septic" => Sewer::EngineeredSeptic,

            "Holding Tank" => Sewer::HoldingTank,

            "Mound Septic" => Sewer::MoundSeptic,

            "None" => Sewer::None,

            "Other" => Sewer::Other,

            "Perc Test On File" => Sewer::PercTestOnFile,

            "Perc Test Required" => Sewer::PercTestRequired,

            "Private Sewer" => Sewer::PrivateSewer,

            "Public Sewer" => Sewer::PublicSewer,

            "Septic Needed" => Sewer::SepticNeeded,

            "Septic Tank" => Sewer::SepticTank,

            "Shared Septic" => Sewer::SharedSeptic,

            "Unknown" => Sewer::Unknown,

            _ => Sewer::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Sewer {
        match s.as_ref() {
            "Aerobic Septic" => Sewer::AerobicSeptic,

            "Cesspool" => Sewer::Cesspool,

            "Engineered Septic" => Sewer::EngineeredSeptic,

            "Holding Tank" => Sewer::HoldingTank,

            "Mound Septic" => Sewer::MoundSeptic,

            "None" => Sewer::None,

            "Other" => Sewer::Other,

            "Perc Test On File" => Sewer::PercTestOnFile,

            "Perc Test Required" => Sewer::PercTestRequired,

            "Private Sewer" => Sewer::PrivateSewer,

            "Public Sewer" => Sewer::PublicSewer,

            "Septic Needed" => Sewer::SepticNeeded,

            "Septic Tank" => Sewer::SepticTank,

            "Shared Septic" => Sewer::SharedSeptic,

            "Unknown" => Sewer::Unknown,

            _ => Sewer::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Sewer::AerobicSeptic => "Aerobic Septic",

            Sewer::Cesspool => "Cesspool",

            Sewer::EngineeredSeptic => "Engineered Septic",

            Sewer::HoldingTank => "Holding Tank",

            Sewer::MoundSeptic => "Mound Septic",

            Sewer::None => "None",

            Sewer::Other => "Other",

            Sewer::PercTestOnFile => "Perc Test On File",

            Sewer::PercTestRequired => "Perc Test Required",

            Sewer::PrivateSewer => "Private Sewer",

            Sewer::PublicSewer => "Public Sewer",

            Sewer::SepticNeeded => "Septic Needed",

            Sewer::SepticTank => "Septic Tank",

            Sewer::SharedSeptic => "Shared Septic",

            Sewer::Unknown => "Unknown",

            Sewer::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Sewer::AerobicSeptic => "Aerobic Septic".into(),

            Sewer::Cesspool => "Cesspool".into(),

            Sewer::EngineeredSeptic => "Engineered Septic".into(),

            Sewer::HoldingTank => "Holding Tank".into(),

            Sewer::MoundSeptic => "Mound Septic".into(),

            Sewer::None => "None".into(),

            Sewer::Other => "Other".into(),

            Sewer::PercTestOnFile => "Perc Test On File".into(),

            Sewer::PercTestRequired => "Perc Test Required".into(),

            Sewer::PrivateSewer => "Private Sewer".into(),

            Sewer::PublicSewer => "Public Sewer".into(),

            Sewer::SepticNeeded => "Septic Needed".into(),

            Sewer::SepticTank => "Septic Tank".into(),

            Sewer::SharedSeptic => "Shared Septic".into(),

            Sewer::Unknown => "Unknown".into(),

            Sewer::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Sewer::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Sewer {
    fn from(s: String) -> Sewer {
        match s.as_ref() {
            "Aerobic Septic" => Sewer::AerobicSeptic,

            "Cesspool" => Sewer::Cesspool,

            "Engineered Septic" => Sewer::EngineeredSeptic,

            "Holding Tank" => Sewer::HoldingTank,

            "Mound Septic" => Sewer::MoundSeptic,

            "None" => Sewer::None,

            "Other" => Sewer::Other,

            "Perc Test On File" => Sewer::PercTestOnFile,

            "Perc Test Required" => Sewer::PercTestRequired,

            "Private Sewer" => Sewer::PrivateSewer,

            "Public Sewer" => Sewer::PublicSewer,

            "Septic Needed" => Sewer::SepticNeeded,

            "Septic Tank" => Sewer::SepticTank,

            "Shared Septic" => Sewer::SharedSeptic,

            "Unknown" => Sewer::Unknown,

            _ => Sewer::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Sewer {
    fn from(s: &str) -> Sewer {
        match s {
            "Aerobic Septic" => Sewer::AerobicSeptic,

            "Cesspool" => Sewer::Cesspool,

            "Engineered Septic" => Sewer::EngineeredSeptic,

            "Holding Tank" => Sewer::HoldingTank,

            "Mound Septic" => Sewer::MoundSeptic,

            "None" => Sewer::None,

            "Other" => Sewer::Other,

            "Perc Test On File" => Sewer::PercTestOnFile,

            "Perc Test Required" => Sewer::PercTestRequired,

            "Private Sewer" => Sewer::PrivateSewer,

            "Public Sewer" => Sewer::PublicSewer,

            "Septic Needed" => Sewer::SepticNeeded,

            "Septic Tank" => Sewer::SepticTank,

            "Shared Septic" => Sewer::SharedSeptic,

            "Unknown" => Sewer::Unknown,

            _ => Sewer::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Sewer> for &'a str {
    fn from(s: &'a Sewer) -> &'a str {
        match s {
            Sewer::AerobicSeptic => "Aerobic Septic",

            Sewer::Cesspool => "Cesspool",

            Sewer::EngineeredSeptic => "Engineered Septic",

            Sewer::HoldingTank => "Holding Tank",

            Sewer::MoundSeptic => "Mound Septic",

            Sewer::None => "None",

            Sewer::Other => "Other",

            Sewer::PercTestOnFile => "Perc Test On File",

            Sewer::PercTestRequired => "Perc Test Required",

            Sewer::PrivateSewer => "Private Sewer",

            Sewer::PublicSewer => "Public Sewer",

            Sewer::SepticNeeded => "Septic Needed",

            Sewer::SepticTank => "Septic Tank",

            Sewer::SharedSeptic => "Shared Septic",

            Sewer::Unknown => "Unknown",

            Sewer::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Sewer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Sewer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
