// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OfficeType Lookups](https://ddwiki.reso.org/display/DDW17/OfficeType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OfficeType {
    /// "[Affiliate](https://ddwiki.reso.org/display/DDW17/Affiliate)": The record in the office roster is an affiliate office.
    Affiliate,

    /// "[Appraiser](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245654)": The record in the office roster is an appraiser office.
    Appraiser,

    /// "[Association](https://ddwiki.reso.org/display/DDW17/Association)": The record in the office roster is an association office.
    Association,

    /// "[MLS](https://ddwiki.reso.org/display/DDW17/MLS)": The record in the office roster is an MLS office.
    MLS,

    /// "[MLS Only Branch](https://ddwiki.reso.org/display/DDW17/MLS+Only+Branch)": The record in the office roster is a broker branch office who receives only MLS service.
    MLSOnlyBranch,

    /// "[MLS Only Firm](https://ddwiki.reso.org/display/DDW17/MLS+Only+Firm)": The record in the office roster is a broker Firm office who receives only MLS service.
    MLSOnlyFirm,

    /// "[MLS Only Office](https://ddwiki.reso.org/display/DDW17/MLS+Only+Office)": The record in the office roster is a broker office who receives only MLS service.
    MLSOnlyOffice,

    /// "[Non Member/Vendor](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245660)": The record in the office roster is an non member/vendor office.
    NonMemberVendor,

    /// "[Realtor Branch Office](https://ddwiki.reso.org/display/DDW17/Realtor+Branch+Office)": The record in the office roster is an realtor branch office .
    RealtorBranchOffice,

    /// "[Realtor Firm](https://ddwiki.reso.org/display/DDW17/Realtor+Firm)": The record in the office roster is an realtor firm office.
    RealtorFirm,

    /// "[Realtor Office](https://ddwiki.reso.org/display/DDW17/Realtor+Office)": The record in the office roster is an realtor office.
    RealtorOffice,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OfficeType {
    fn from_str(s: &str) -> OfficeType {
        match s {
            "Affiliate" => OfficeType::Affiliate,

            "Appraiser" => OfficeType::Appraiser,

            "Association" => OfficeType::Association,

            "MLS" => OfficeType::MLS,

            "MLS Only Branch" => OfficeType::MLSOnlyBranch,

            "MLS Only Firm" => OfficeType::MLSOnlyFirm,

            "MLS Only Office" => OfficeType::MLSOnlyOffice,

            "Non Member/Vendor" => OfficeType::NonMemberVendor,

            "Realtor Branch Office" => OfficeType::RealtorBranchOffice,

            "Realtor Firm" => OfficeType::RealtorFirm,

            "Realtor Office" => OfficeType::RealtorOffice,

            _ => OfficeType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OfficeType {
        match s.as_ref() {
            "Affiliate" => OfficeType::Affiliate,

            "Appraiser" => OfficeType::Appraiser,

            "Association" => OfficeType::Association,

            "MLS" => OfficeType::MLS,

            "MLS Only Branch" => OfficeType::MLSOnlyBranch,

            "MLS Only Firm" => OfficeType::MLSOnlyFirm,

            "MLS Only Office" => OfficeType::MLSOnlyOffice,

            "Non Member/Vendor" => OfficeType::NonMemberVendor,

            "Realtor Branch Office" => OfficeType::RealtorBranchOffice,

            "Realtor Firm" => OfficeType::RealtorFirm,

            "Realtor Office" => OfficeType::RealtorOffice,

            _ => OfficeType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OfficeType::Affiliate => "Affiliate",

            OfficeType::Appraiser => "Appraiser",

            OfficeType::Association => "Association",

            OfficeType::MLS => "MLS",

            OfficeType::MLSOnlyBranch => "MLS Only Branch",

            OfficeType::MLSOnlyFirm => "MLS Only Firm",

            OfficeType::MLSOnlyOffice => "MLS Only Office",

            OfficeType::NonMemberVendor => "Non Member/Vendor",

            OfficeType::RealtorBranchOffice => "Realtor Branch Office",

            OfficeType::RealtorFirm => "Realtor Firm",

            OfficeType::RealtorOffice => "Realtor Office",

            OfficeType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OfficeType::Affiliate => "Affiliate".into(),

            OfficeType::Appraiser => "Appraiser".into(),

            OfficeType::Association => "Association".into(),

            OfficeType::MLS => "MLS".into(),

            OfficeType::MLSOnlyBranch => "MLS Only Branch".into(),

            OfficeType::MLSOnlyFirm => "MLS Only Firm".into(),

            OfficeType::MLSOnlyOffice => "MLS Only Office".into(),

            OfficeType::NonMemberVendor => "Non Member/Vendor".into(),

            OfficeType::RealtorBranchOffice => "Realtor Branch Office".into(),

            OfficeType::RealtorFirm => "Realtor Firm".into(),

            OfficeType::RealtorOffice => "Realtor Office".into(),

            OfficeType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OfficeType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OfficeType {
    fn from(s: String) -> OfficeType {
        match s.as_ref() {
            "Affiliate" => OfficeType::Affiliate,

            "Appraiser" => OfficeType::Appraiser,

            "Association" => OfficeType::Association,

            "MLS" => OfficeType::MLS,

            "MLS Only Branch" => OfficeType::MLSOnlyBranch,

            "MLS Only Firm" => OfficeType::MLSOnlyFirm,

            "MLS Only Office" => OfficeType::MLSOnlyOffice,

            "Non Member/Vendor" => OfficeType::NonMemberVendor,

            "Realtor Branch Office" => OfficeType::RealtorBranchOffice,

            "Realtor Firm" => OfficeType::RealtorFirm,

            "Realtor Office" => OfficeType::RealtorOffice,

            _ => OfficeType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OfficeType {
    fn from(s: &str) -> OfficeType {
        match s {
            "Affiliate" => OfficeType::Affiliate,

            "Appraiser" => OfficeType::Appraiser,

            "Association" => OfficeType::Association,

            "MLS" => OfficeType::MLS,

            "MLS Only Branch" => OfficeType::MLSOnlyBranch,

            "MLS Only Firm" => OfficeType::MLSOnlyFirm,

            "MLS Only Office" => OfficeType::MLSOnlyOffice,

            "Non Member/Vendor" => OfficeType::NonMemberVendor,

            "Realtor Branch Office" => OfficeType::RealtorBranchOffice,

            "Realtor Firm" => OfficeType::RealtorFirm,

            "Realtor Office" => OfficeType::RealtorOffice,

            _ => OfficeType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OfficeType> for &'a str {
    fn from(s: &'a OfficeType) -> &'a str {
        match s {
            OfficeType::Affiliate => "Affiliate",

            OfficeType::Appraiser => "Appraiser",

            OfficeType::Association => "Association",

            OfficeType::MLS => "MLS",

            OfficeType::MLSOnlyBranch => "MLS Only Branch",

            OfficeType::MLSOnlyFirm => "MLS Only Firm",

            OfficeType::MLSOnlyOffice => "MLS Only Office",

            OfficeType::NonMemberVendor => "Non Member/Vendor",

            OfficeType::RealtorBranchOffice => "Realtor Branch Office",

            OfficeType::RealtorFirm => "Realtor Firm",

            OfficeType::RealtorOffice => "Realtor Office",

            OfficeType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OfficeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OfficeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
