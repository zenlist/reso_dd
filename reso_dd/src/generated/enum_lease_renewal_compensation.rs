// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LeaseRenewalCompensation Lookups](https://ddwiki.reso.org/display/DDW17/LeaseRenewalCompensation+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LeaseRenewalCompensation {
    /// "[Call Listing Agent](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245344)": For details on about additional selling office compensation for lease renewals, contact the listing agent.
    CallListingAgent,

    /// "[Call Listing Office](https://ddwiki.reso.org/display/DDW17/Call+Listing+Office)": For details on about additional selling office compensation for lease renewals, contact the listing office.
    CallListingOffice,

    /// "[Commission Paid On Tenant Purchase](https://ddwiki.reso.org/display/DDW17/Commission+Paid+On+Tenant+Purchase)": Additional commission is paid in the event the tenant purchase the property.
    CommissionPaidOnTenantPurchase,

    /// "[No Renewal Commission](https://ddwiki.reso.org/display/DDW17/No+Renewal+Commission)": There is no additional commission if the tenant renews or extends the lease.
    NoRenewalCommission,

    /// "[Renewal Commission Paid](https://ddwiki.reso.org/display/DDW17/Renewal+Commission+Paid)": There is additional commission paid if the tenant renews the lease.
    RenewalCommissionPaid,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for LeaseRenewalCompensation {
    fn from_str(s: &str) -> LeaseRenewalCompensation {
        match s {
            "Call Listing Agent" => LeaseRenewalCompensation::CallListingAgent,

            "Call Listing Office" => LeaseRenewalCompensation::CallListingOffice,

            "Commission Paid On Tenant Purchase" => {
                LeaseRenewalCompensation::CommissionPaidOnTenantPurchase
            }

            "No Renewal Commission" => LeaseRenewalCompensation::NoRenewalCommission,

            "Renewal Commission Paid" => LeaseRenewalCompensation::RenewalCommissionPaid,

            _ => LeaseRenewalCompensation::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> LeaseRenewalCompensation {
        match s.as_ref() {
            "Call Listing Agent" => LeaseRenewalCompensation::CallListingAgent,

            "Call Listing Office" => LeaseRenewalCompensation::CallListingOffice,

            "Commission Paid On Tenant Purchase" => {
                LeaseRenewalCompensation::CommissionPaidOnTenantPurchase
            }

            "No Renewal Commission" => LeaseRenewalCompensation::NoRenewalCommission,

            "Renewal Commission Paid" => LeaseRenewalCompensation::RenewalCommissionPaid,

            _ => LeaseRenewalCompensation::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            LeaseRenewalCompensation::CallListingAgent => "Call Listing Agent",

            LeaseRenewalCompensation::CallListingOffice => "Call Listing Office",

            LeaseRenewalCompensation::CommissionPaidOnTenantPurchase => {
                "Commission Paid On Tenant Purchase"
            }

            LeaseRenewalCompensation::NoRenewalCommission => "No Renewal Commission",

            LeaseRenewalCompensation::RenewalCommissionPaid => "Renewal Commission Paid",

            LeaseRenewalCompensation::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            LeaseRenewalCompensation::CallListingAgent => "Call Listing Agent".into(),

            LeaseRenewalCompensation::CallListingOffice => "Call Listing Office".into(),

            LeaseRenewalCompensation::CommissionPaidOnTenantPurchase => {
                "Commission Paid On Tenant Purchase".into()
            }

            LeaseRenewalCompensation::NoRenewalCommission => "No Renewal Commission".into(),

            LeaseRenewalCompensation::RenewalCommissionPaid => "Renewal Commission Paid".into(),

            LeaseRenewalCompensation::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            LeaseRenewalCompensation::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LeaseRenewalCompensation {
    fn from(s: String) -> LeaseRenewalCompensation {
        match s.as_ref() {
            "Call Listing Agent" => LeaseRenewalCompensation::CallListingAgent,

            "Call Listing Office" => LeaseRenewalCompensation::CallListingOffice,

            "Commission Paid On Tenant Purchase" => {
                LeaseRenewalCompensation::CommissionPaidOnTenantPurchase
            }

            "No Renewal Commission" => LeaseRenewalCompensation::NoRenewalCommission,

            "Renewal Commission Paid" => LeaseRenewalCompensation::RenewalCommissionPaid,

            _ => LeaseRenewalCompensation::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LeaseRenewalCompensation {
    fn from(s: &str) -> LeaseRenewalCompensation {
        match s {
            "Call Listing Agent" => LeaseRenewalCompensation::CallListingAgent,

            "Call Listing Office" => LeaseRenewalCompensation::CallListingOffice,

            "Commission Paid On Tenant Purchase" => {
                LeaseRenewalCompensation::CommissionPaidOnTenantPurchase
            }

            "No Renewal Commission" => LeaseRenewalCompensation::NoRenewalCommission,

            "Renewal Commission Paid" => LeaseRenewalCompensation::RenewalCommissionPaid,

            _ => LeaseRenewalCompensation::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LeaseRenewalCompensation> for &'a str {
    fn from(s: &'a LeaseRenewalCompensation) -> &'a str {
        match s {
            LeaseRenewalCompensation::CallListingAgent => "Call Listing Agent",

            LeaseRenewalCompensation::CallListingOffice => "Call Listing Office",

            LeaseRenewalCompensation::CommissionPaidOnTenantPurchase => {
                "Commission Paid On Tenant Purchase"
            }

            LeaseRenewalCompensation::NoRenewalCommission => "No Renewal Commission",

            LeaseRenewalCompensation::RenewalCommissionPaid => "Renewal Commission Paid",

            LeaseRenewalCompensation::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LeaseRenewalCompensation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LeaseRenewalCompensation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
