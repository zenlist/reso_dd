// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ExistingLeaseType Lookups](https://ddwiki.reso.org/display/DDW17/ExistingLeaseType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExistingLeaseType {
    /// "[Absolute Net](https://ddwiki.reso.org/display/DDW17/Absolute+Net)": Also known as a Bondable Lease, the tenant carries every risk in addition to the costs of a NNN Lease.
    AbsoluteNet,

    /// "[CPI Adjustment](https://ddwiki.reso.org/display/DDW17/CPI+Adjustment)": An escalation clause/provision in a lease to adjust the amount paid by the tenant (lessee) where the adjustment will follow the Consumer Price Index (CPI).
    CPIAdjustment,

    /// "[Escalation Clause](https://ddwiki.reso.org/display/DDW17/Escalation+Clause)": A clause or provision in a lease document that set a formula for how rent will increase over time.
    EscalationClause,

    /// "[Gross](https://ddwiki.reso.org/display/DDW17/Gross)": A lease agreement where the owner (lessor) pays all property changes normal to ownership.  The opposite to net leases where the tenant (lessee) may pay taxes, insurance, maintenance and even for damages that were not caused by the tenant.
    Gross,

    /// "[Ground Lease](https://ddwiki.reso.org/display/DDW17/Ground+Lease)": Typically a long term lease of land where the tenant (lessee) has the right to develop or make improvements.
    GroundLease,

    /// "[Net](https://ddwiki.reso.org/display/DDW17/Net)": A lease agreement where the tenant pays the real estate taxes.
    Net,

    /// "[NN](https://ddwiki.reso.org/display/DDW17/NN)": A lease agreement where the tenant pays real estate taxes and building insurance.
    NN,

    /// "[NNN](https://ddwiki.reso.org/display/DDW17/NNN)": A lease agreement where the tenant pays real estate taxes, building insurance and maintenance.
    NNN,

    /// "[Oral](https://ddwiki.reso.org/display/DDW17/Oral)": The terms of the lease are agreed orally (not in writing) between the lessee and lessor.  Legal restrictions around oral agreements vary from state to state.
    Oral,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for ExistingLeaseType {
    fn from(s: String) -> ExistingLeaseType {
        match s.as_ref() {
            "Absolute Net" => ExistingLeaseType::AbsoluteNet,

            "CPI Adjustment" => ExistingLeaseType::CPIAdjustment,

            "Escalation Clause" => ExistingLeaseType::EscalationClause,

            "Gross" => ExistingLeaseType::Gross,

            "Ground Lease" => ExistingLeaseType::GroundLease,

            "Net" => ExistingLeaseType::Net,

            "NN" => ExistingLeaseType::NN,

            "NNN" => ExistingLeaseType::NNN,

            "Oral" => ExistingLeaseType::Oral,

            _ => ExistingLeaseType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ExistingLeaseType {
    fn from(s: &str) -> ExistingLeaseType {
        match s {
            "Absolute Net" => ExistingLeaseType::AbsoluteNet,

            "CPI Adjustment" => ExistingLeaseType::CPIAdjustment,

            "Escalation Clause" => ExistingLeaseType::EscalationClause,

            "Gross" => ExistingLeaseType::Gross,

            "Ground Lease" => ExistingLeaseType::GroundLease,

            "Net" => ExistingLeaseType::Net,

            "NN" => ExistingLeaseType::NN,

            "NNN" => ExistingLeaseType::NNN,

            "Oral" => ExistingLeaseType::Oral,

            _ => ExistingLeaseType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ExistingLeaseType> for &'a str {
    fn from(s: &'a ExistingLeaseType) -> &'a str {
        match s {
            ExistingLeaseType::AbsoluteNet => "Absolute Net",

            ExistingLeaseType::CPIAdjustment => "CPI Adjustment",

            ExistingLeaseType::EscalationClause => "Escalation Clause",

            ExistingLeaseType::Gross => "Gross",

            ExistingLeaseType::GroundLease => "Ground Lease",

            ExistingLeaseType::Net => "Net",

            ExistingLeaseType::NN => "NN",

            ExistingLeaseType::NNN => "NNN",

            ExistingLeaseType::Oral => "Oral",

            ExistingLeaseType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ExistingLeaseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ExistingLeaseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_existing_lease_type_format {
    use super::ExistingLeaseType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ExistingLeaseType>>,
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
    ) -> Result<Option<Vec<ExistingLeaseType>>, D::Error>
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
