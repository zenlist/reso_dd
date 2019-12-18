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

pub(crate) mod option_vec_office_type_format {
    use super::OfficeType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OfficeType>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<OfficeType>>, D::Error>
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
