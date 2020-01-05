// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [FinancialDataSource Lookups](https://ddwiki.reso.org/display/DDW17/FinancialDataSource+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FinancialDataSource {
    /// "[Accountant](https://ddwiki.reso.org/display/DDW17/Accountant)": The financial data in the listing was provided by an accountant.
    Accountant,

    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244728)": the financial data in the listing was provided by the owner.
    Owner,

    /// "[Property Manager](https://ddwiki.reso.org/display/DDW17/Property+Manager)": the financial data in the listing was provided by the property manager.
    PropertyManager,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for FinancialDataSource {
    fn from_str(s: &str) -> FinancialDataSource {
        match s {
            "Accountant" => FinancialDataSource::Accountant,

            "Owner" => FinancialDataSource::Owner,

            "Property Manager" => FinancialDataSource::PropertyManager,

            _ => FinancialDataSource::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> FinancialDataSource {
        match s.as_ref() {
            "Accountant" => FinancialDataSource::Accountant,

            "Owner" => FinancialDataSource::Owner,

            "Property Manager" => FinancialDataSource::PropertyManager,

            _ => FinancialDataSource::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            FinancialDataSource::Accountant => "Accountant",

            FinancialDataSource::Owner => "Owner",

            FinancialDataSource::PropertyManager => "Property Manager",

            FinancialDataSource::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            FinancialDataSource::Accountant => "Accountant".into(),

            FinancialDataSource::Owner => "Owner".into(),

            FinancialDataSource::PropertyManager => "Property Manager".into(),

            FinancialDataSource::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            FinancialDataSource::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for FinancialDataSource {
    fn from(s: String) -> FinancialDataSource {
        match s.as_ref() {
            "Accountant" => FinancialDataSource::Accountant,

            "Owner" => FinancialDataSource::Owner,

            "Property Manager" => FinancialDataSource::PropertyManager,

            _ => FinancialDataSource::OpenEnumeration(s),
        }
    }
}

impl From<&str> for FinancialDataSource {
    fn from(s: &str) -> FinancialDataSource {
        match s {
            "Accountant" => FinancialDataSource::Accountant,

            "Owner" => FinancialDataSource::Owner,

            "Property Manager" => FinancialDataSource::PropertyManager,

            _ => FinancialDataSource::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a FinancialDataSource> for &'a str {
    fn from(s: &'a FinancialDataSource) -> &'a str {
        match s {
            FinancialDataSource::Accountant => "Accountant",

            FinancialDataSource::Owner => "Owner",

            FinancialDataSource::PropertyManager => "Property Manager",

            FinancialDataSource::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for FinancialDataSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FinancialDataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
