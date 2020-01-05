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

impl crate::ResoEnumeration for ExistingLeaseType {
    fn from_str(s: &str) -> ExistingLeaseType {
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

    fn from_string(s: String) -> ExistingLeaseType {
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

    fn to_str(&self) -> &str {
        match self {
            ExistingLeaseType::AbsoluteNet => "Absolute Net",

            ExistingLeaseType::CPIAdjustment => "CPI Adjustment",

            ExistingLeaseType::EscalationClause => "Escalation Clause",

            ExistingLeaseType::Gross => "Gross",

            ExistingLeaseType::GroundLease => "Ground Lease",

            ExistingLeaseType::Net => "Net",

            ExistingLeaseType::NN => "NN",

            ExistingLeaseType::NNN => "NNN",

            ExistingLeaseType::Oral => "Oral",

            ExistingLeaseType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ExistingLeaseType::AbsoluteNet => "Absolute Net".into(),

            ExistingLeaseType::CPIAdjustment => "CPI Adjustment".into(),

            ExistingLeaseType::EscalationClause => "Escalation Clause".into(),

            ExistingLeaseType::Gross => "Gross".into(),

            ExistingLeaseType::GroundLease => "Ground Lease".into(),

            ExistingLeaseType::Net => "Net".into(),

            ExistingLeaseType::NN => "NN".into(),

            ExistingLeaseType::NNN => "NNN".into(),

            ExistingLeaseType::Oral => "Oral".into(),

            ExistingLeaseType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ExistingLeaseType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
