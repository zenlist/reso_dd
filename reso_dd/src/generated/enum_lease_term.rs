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

pub(crate) mod option_vec_lease_term_format {
    use super::LeaseTerm;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<LeaseTerm>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<LeaseTerm>>, D::Error>
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
