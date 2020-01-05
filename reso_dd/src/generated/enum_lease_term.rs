// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LeaseTerm Lookups](https://ddwiki.reso.org/display/DDW17/LeaseTerm+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LeaseTerm {
    /// "[12 Months](https://ddwiki.reso.org/display/DDW17/12+Months)": The length of the lease is 12 months.
    _12Months,

    /// "[24 Months](https://ddwiki.reso.org/display/DDW17/24+Months)": The length of the lease is 24 months.
    _24Months,

    /// "[6 Months](https://ddwiki.reso.org/display/DDW17/6+Months)": The length of the lease is 6 months.
    _6Months,

    /// "[Month To Month](https://ddwiki.reso.org/display/DDW17/Month+To+Month)": The length of the lease is month to month.
    MonthToMonth,

    /// "[Negotiable](https://ddwiki.reso.org/display/DDW17/Negotiable)": The length of the lease is negotiable.
    Negotiable,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245355)": There is no stated term to the lease.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245356)": The term of the lease is something other than is available in this list.
    Other,

    /// "[Renewal Option](https://ddwiki.reso.org/display/DDW17/Renewal+Option)": The lease has a renewal option.
    RenewalOption,

    /// "[Short Term Lease](https://ddwiki.reso.org/display/DDW17/Short+Term+Lease)": The lease is short term.
    ShortTermLease,

    /// "[Weekly](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245359)": The length of the lease is weekly.
    Weekly,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for LeaseTerm {
    fn from_str(s: &str) -> LeaseTerm {
        match s {
            "12 Months" => LeaseTerm::_12Months,

            "24 Months" => LeaseTerm::_24Months,

            "6 Months" => LeaseTerm::_6Months,

            "Month To Month" => LeaseTerm::MonthToMonth,

            "Negotiable" => LeaseTerm::Negotiable,

            "None" => LeaseTerm::None,

            "Other" => LeaseTerm::Other,

            "Renewal Option" => LeaseTerm::RenewalOption,

            "Short Term Lease" => LeaseTerm::ShortTermLease,

            "Weekly" => LeaseTerm::Weekly,

            _ => LeaseTerm::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> LeaseTerm {
        match s.as_ref() {
            "12 Months" => LeaseTerm::_12Months,

            "24 Months" => LeaseTerm::_24Months,

            "6 Months" => LeaseTerm::_6Months,

            "Month To Month" => LeaseTerm::MonthToMonth,

            "Negotiable" => LeaseTerm::Negotiable,

            "None" => LeaseTerm::None,

            "Other" => LeaseTerm::Other,

            "Renewal Option" => LeaseTerm::RenewalOption,

            "Short Term Lease" => LeaseTerm::ShortTermLease,

            "Weekly" => LeaseTerm::Weekly,

            _ => LeaseTerm::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            LeaseTerm::_12Months => "12 Months",

            LeaseTerm::_24Months => "24 Months",

            LeaseTerm::_6Months => "6 Months",

            LeaseTerm::MonthToMonth => "Month To Month",

            LeaseTerm::Negotiable => "Negotiable",

            LeaseTerm::None => "None",

            LeaseTerm::Other => "Other",

            LeaseTerm::RenewalOption => "Renewal Option",

            LeaseTerm::ShortTermLease => "Short Term Lease",

            LeaseTerm::Weekly => "Weekly",

            LeaseTerm::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            LeaseTerm::_12Months => "12 Months".into(),

            LeaseTerm::_24Months => "24 Months".into(),

            LeaseTerm::_6Months => "6 Months".into(),

            LeaseTerm::MonthToMonth => "Month To Month".into(),

            LeaseTerm::Negotiable => "Negotiable".into(),

            LeaseTerm::None => "None".into(),

            LeaseTerm::Other => "Other".into(),

            LeaseTerm::RenewalOption => "Renewal Option".into(),

            LeaseTerm::ShortTermLease => "Short Term Lease".into(),

            LeaseTerm::Weekly => "Weekly".into(),

            LeaseTerm::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            LeaseTerm::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LeaseTerm {
    fn from(s: String) -> LeaseTerm {
        match s.as_ref() {
            "12 Months" => LeaseTerm::_12Months,

            "24 Months" => LeaseTerm::_24Months,

            "6 Months" => LeaseTerm::_6Months,

            "Month To Month" => LeaseTerm::MonthToMonth,

            "Negotiable" => LeaseTerm::Negotiable,

            "None" => LeaseTerm::None,

            "Other" => LeaseTerm::Other,

            "Renewal Option" => LeaseTerm::RenewalOption,

            "Short Term Lease" => LeaseTerm::ShortTermLease,

            "Weekly" => LeaseTerm::Weekly,

            _ => LeaseTerm::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LeaseTerm {
    fn from(s: &str) -> LeaseTerm {
        match s {
            "12 Months" => LeaseTerm::_12Months,

            "24 Months" => LeaseTerm::_24Months,

            "6 Months" => LeaseTerm::_6Months,

            "Month To Month" => LeaseTerm::MonthToMonth,

            "Negotiable" => LeaseTerm::Negotiable,

            "None" => LeaseTerm::None,

            "Other" => LeaseTerm::Other,

            "Renewal Option" => LeaseTerm::RenewalOption,

            "Short Term Lease" => LeaseTerm::ShortTermLease,

            "Weekly" => LeaseTerm::Weekly,

            _ => LeaseTerm::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LeaseTerm> for &'a str {
    fn from(s: &'a LeaseTerm) -> &'a str {
        match s {
            LeaseTerm::_12Months => "12 Months",

            LeaseTerm::_24Months => "24 Months",

            LeaseTerm::_6Months => "6 Months",

            LeaseTerm::MonthToMonth => "Month To Month",

            LeaseTerm::Negotiable => "Negotiable",

            LeaseTerm::None => "None",

            LeaseTerm::Other => "Other",

            LeaseTerm::RenewalOption => "Renewal Option",

            LeaseTerm::ShortTermLease => "Short Term Lease",

            LeaseTerm::Weekly => "Weekly",

            LeaseTerm::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LeaseTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LeaseTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
