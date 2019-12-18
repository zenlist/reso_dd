// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LockBoxType Lookups](https://ddwiki.reso.org/display/DDW17/LockBoxType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LockBoxType {
    /// "[Call Listing Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245409)": Call the listing office for information about accessing the property.
    CallListingOffice,

    /// "[Call Seller Direct](https://ddwiki.reso.org/display/DDW17/Call+Seller+Direct)": Call the seller directly to arrange for access to the property.
    CallSellerDirect,

    /// "[Combo](https://ddwiki.reso.org/display/DDW17/Combo)": The lockbox on the property is opened via combination.  See remarks or contact the agent/office for the combination.
    Combo,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245412)": There is no lockbox on the property.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245416)": A lock box type not included in this list.
    Other,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245413)": See remarks for details about the lockbox and accessing the property.
    SeeRemarks,

    /// "[SentriLock](https://ddwiki.reso.org/display/DDW17/SentriLock)": The lockbox is from SentriLock and requires a SentriLock key or access code.
    SentriLock,

    /// "[Supra](https://ddwiki.reso.org/display/DDW17/Supra)": The lockbox is from Supra and requires a Supra key.
    Supra,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for LockBoxType {
    fn from(s: String) -> LockBoxType {
        match s.as_ref() {
            "Call Listing Office" => LockBoxType::CallListingOffice,

            "Call Seller Direct" => LockBoxType::CallSellerDirect,

            "Combo" => LockBoxType::Combo,

            "None" => LockBoxType::None,

            "Other" => LockBoxType::Other,

            "See Remarks" => LockBoxType::SeeRemarks,

            "SentriLock" => LockBoxType::SentriLock,

            "Supra" => LockBoxType::Supra,

            _ => LockBoxType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LockBoxType {
    fn from(s: &str) -> LockBoxType {
        match s {
            "Call Listing Office" => LockBoxType::CallListingOffice,

            "Call Seller Direct" => LockBoxType::CallSellerDirect,

            "Combo" => LockBoxType::Combo,

            "None" => LockBoxType::None,

            "Other" => LockBoxType::Other,

            "See Remarks" => LockBoxType::SeeRemarks,

            "SentriLock" => LockBoxType::SentriLock,

            "Supra" => LockBoxType::Supra,

            _ => LockBoxType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LockBoxType> for &'a str {
    fn from(s: &'a LockBoxType) -> &'a str {
        match s {
            LockBoxType::CallListingOffice => "Call Listing Office",

            LockBoxType::CallSellerDirect => "Call Seller Direct",

            LockBoxType::Combo => "Combo",

            LockBoxType::None => "None",

            LockBoxType::Other => "Other",

            LockBoxType::SeeRemarks => "See Remarks",

            LockBoxType::SentriLock => "SentriLock",

            LockBoxType::Supra => "Supra",

            LockBoxType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LockBoxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LockBoxType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_lock_box_type_format {
    use super::LockBoxType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<LockBoxType>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<LockBoxType>>, D::Error>
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
