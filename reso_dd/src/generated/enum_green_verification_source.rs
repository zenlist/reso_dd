// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [GreenVerificationSource Lookups](https://ddwiki.reso.org/display/DDW17/GreenVerificationSource+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreenVerificationSource {
    /// "[Administrator](https://ddwiki.reso.org/display/DDW17/Administrator)": An administrator such as a utility, governmental entity, etc. provided data such as photovoltaic characteristics, or a verified score, certification, label, etc.
    Administrator,

    /// "[Assessor](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244921)": The assessor provided data such as photovoltaic characteristics, or a verified score, certification, label, etc.
    Assessor,

    /// "[Builder](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244922)": The builder provided data such as photovoltaic characteristics, or a verified score, certification, label, etc.
    Builder,

    /// "[Contractor or Installer](https://ddwiki.reso.org/display/DDW17/Contractor+or+Installer)": The contractor or installer provided data such as photovoltaic characteristics, or a verified score, certification, label, etc.
    ContractororInstaller,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244924)": Data such as photovoltaic characteristics, or a verified score, certification, label, etc. was received from another party not listed.
    Other,

    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244925)": The owner provided data such as photovoltaic characteristics, or a verified score, certification, label, etc.
    Owner,

    /// "[Program Sponsor](https://ddwiki.reso.org/display/DDW17/Program+Sponsor)": The program sponsor provided data such as photovoltaic characteristics, or a verified score, certification, label, etc.
    ProgramSponsor,

    /// "[Program Verifier](https://ddwiki.reso.org/display/DDW17/Program+Verifier)": The program verifier hired as a third-party provided data such as photovoltaic characteristics, or a verified score, certification, label, etc.
    ProgramVerifier,

    /// "[Public Records](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244928)": Data such as photovoltaic characteristics, or a verified score, certification, label, etc. was received from public record such as a building permit.
    PublicRecords,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244929)": See remarks for information about the source of data such as photovoltaic characteristics, or a verified score, certification, label, etc.
    SeeRemarks,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for GreenVerificationSource {
    fn from(s: String) -> GreenVerificationSource {
        match s.as_ref() {
            "Administrator" => GreenVerificationSource::Administrator,

            "Assessor" => GreenVerificationSource::Assessor,

            "Builder" => GreenVerificationSource::Builder,

            "Contractor or Installer" => GreenVerificationSource::ContractororInstaller,

            "Other" => GreenVerificationSource::Other,

            "Owner" => GreenVerificationSource::Owner,

            "Program Sponsor" => GreenVerificationSource::ProgramSponsor,

            "Program Verifier" => GreenVerificationSource::ProgramVerifier,

            "Public Records" => GreenVerificationSource::PublicRecords,

            "See Remarks" => GreenVerificationSource::SeeRemarks,

            _ => GreenVerificationSource::OpenEnumeration(s),
        }
    }
}

impl From<&str> for GreenVerificationSource {
    fn from(s: &str) -> GreenVerificationSource {
        match s {
            "Administrator" => GreenVerificationSource::Administrator,

            "Assessor" => GreenVerificationSource::Assessor,

            "Builder" => GreenVerificationSource::Builder,

            "Contractor or Installer" => GreenVerificationSource::ContractororInstaller,

            "Other" => GreenVerificationSource::Other,

            "Owner" => GreenVerificationSource::Owner,

            "Program Sponsor" => GreenVerificationSource::ProgramSponsor,

            "Program Verifier" => GreenVerificationSource::ProgramVerifier,

            "Public Records" => GreenVerificationSource::PublicRecords,

            "See Remarks" => GreenVerificationSource::SeeRemarks,

            _ => GreenVerificationSource::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a GreenVerificationSource> for &'a str {
    fn from(s: &'a GreenVerificationSource) -> &'a str {
        match s {
            GreenVerificationSource::Administrator => "Administrator",

            GreenVerificationSource::Assessor => "Assessor",

            GreenVerificationSource::Builder => "Builder",

            GreenVerificationSource::ContractororInstaller => "Contractor or Installer",

            GreenVerificationSource::Other => "Other",

            GreenVerificationSource::Owner => "Owner",

            GreenVerificationSource::ProgramSponsor => "Program Sponsor",

            GreenVerificationSource::ProgramVerifier => "Program Verifier",

            GreenVerificationSource::PublicRecords => "Public Records",

            GreenVerificationSource::SeeRemarks => "See Remarks",

            GreenVerificationSource::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for GreenVerificationSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for GreenVerificationSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_green_verification_source_format {
    use super::GreenVerificationSource;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<GreenVerificationSource>>,
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
    ) -> Result<Option<Vec<GreenVerificationSource>>, D::Error>
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
