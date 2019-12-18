// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [MemberType Lookups](https://ddwiki.reso.org/display/DDW17/MemberType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemberType {
    /// "[Assistant](https://ddwiki.reso.org/display/DDW17/Assistant)": The member is an assistant. The status of the assistant being licensed is not known or possibly not required.
    Assistant,

    /// "[Association Staff](https://ddwiki.reso.org/display/DDW17/Association+Staff)": The member is a member of the association's staff.
    AssociationStaff,

    /// "[Designated REALTOR Appraiser](https://ddwiki.reso.org/display/DDW17/Designated+REALTOR+Appraiser)": The member is a designated appraiser and a member of NAR.
    DesignatedREALTORAppraiser,

    /// "[Designated REALTOR Participant](https://ddwiki.reso.org/display/DDW17/Designated+REALTOR+Participant)": The member is a designated broker and a member of NAR.
    DesignatedREALTORParticipant,

    /// "[Licensed Assistant](https://ddwiki.reso.org/display/DDW17/Licensed+Assistant)": The member is an assistant. The status of the assistant being licensed is not known or possibly not required.
    LicensedAssistant,

    /// "[MLS Only Appraiser](https://ddwiki.reso.org/display/DDW17/MLS+Only+Appraiser)": The member is an appraiser and not a member of NAR, receiving MLS services only.
    MLSOnlyAppraiser,

    /// "[MLS Only Broker](https://ddwiki.reso.org/display/DDW17/MLS+Only+Broker)": The member is a broker and not a member of NAR, receiving MLS services only.
    MLSOnlyBroker,

    /// "[MLS Only Salesperson](https://ddwiki.reso.org/display/DDW17/MLS+Only+Salesperson)": The member is a sales person and not a member of NAR, receiving MLS services only.
    MLSOnlySalesperson,

    /// "[MLS Staff](https://ddwiki.reso.org/display/DDW17/MLS+Staff)": The individual is a member of MLS staff.
    MLSStaff,

    /// "[Non Member/Vendor](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245594)": The individual is not a member or is a vendor.
    NonMemberVendor,

    /// "[Office Manager](https://ddwiki.reso.org/display/DDW17/Office+Manager)": The member is a license office manager.
    OfficeManager,

    /// "[REALTOR Appraiser](https://ddwiki.reso.org/display/DDW17/REALTOR+Appraiser)": The member is an appraiser and a member of NAR.
    REALTORAppraiser,

    /// "[REALTOR Salesperson](https://ddwiki.reso.org/display/DDW17/REALTOR+Salesperson)": The member is a sales person and a member of NAR.
    REALTORSalesperson,

    /// "[Unlicensed Assistant](https://ddwiki.reso.org/display/DDW17/Unlicensed+Assistant)": The member is an unlicensed assistant.
    UnlicensedAssistant,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for MemberType {
    fn from(s: String) -> MemberType {
        match s.as_ref() {
            "Assistant" => MemberType::Assistant,

            "Association Staff" => MemberType::AssociationStaff,

            "Designated REALTOR Appraiser" => MemberType::DesignatedREALTORAppraiser,

            "Designated REALTOR Participant" => MemberType::DesignatedREALTORParticipant,

            "Licensed Assistant" => MemberType::LicensedAssistant,

            "MLS Only Appraiser" => MemberType::MLSOnlyAppraiser,

            "MLS Only Broker" => MemberType::MLSOnlyBroker,

            "MLS Only Salesperson" => MemberType::MLSOnlySalesperson,

            "MLS Staff" => MemberType::MLSStaff,

            "Non Member/Vendor" => MemberType::NonMemberVendor,

            "Office Manager" => MemberType::OfficeManager,

            "REALTOR Appraiser" => MemberType::REALTORAppraiser,

            "REALTOR Salesperson" => MemberType::REALTORSalesperson,

            "Unlicensed Assistant" => MemberType::UnlicensedAssistant,

            _ => MemberType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for MemberType {
    fn from(s: &str) -> MemberType {
        match s {
            "Assistant" => MemberType::Assistant,

            "Association Staff" => MemberType::AssociationStaff,

            "Designated REALTOR Appraiser" => MemberType::DesignatedREALTORAppraiser,

            "Designated REALTOR Participant" => MemberType::DesignatedREALTORParticipant,

            "Licensed Assistant" => MemberType::LicensedAssistant,

            "MLS Only Appraiser" => MemberType::MLSOnlyAppraiser,

            "MLS Only Broker" => MemberType::MLSOnlyBroker,

            "MLS Only Salesperson" => MemberType::MLSOnlySalesperson,

            "MLS Staff" => MemberType::MLSStaff,

            "Non Member/Vendor" => MemberType::NonMemberVendor,

            "Office Manager" => MemberType::OfficeManager,

            "REALTOR Appraiser" => MemberType::REALTORAppraiser,

            "REALTOR Salesperson" => MemberType::REALTORSalesperson,

            "Unlicensed Assistant" => MemberType::UnlicensedAssistant,

            _ => MemberType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a MemberType> for &'a str {
    fn from(s: &'a MemberType) -> &'a str {
        match s {
            MemberType::Assistant => "Assistant",

            MemberType::AssociationStaff => "Association Staff",

            MemberType::DesignatedREALTORAppraiser => "Designated REALTOR Appraiser",

            MemberType::DesignatedREALTORParticipant => "Designated REALTOR Participant",

            MemberType::LicensedAssistant => "Licensed Assistant",

            MemberType::MLSOnlyAppraiser => "MLS Only Appraiser",

            MemberType::MLSOnlyBroker => "MLS Only Broker",

            MemberType::MLSOnlySalesperson => "MLS Only Salesperson",

            MemberType::MLSStaff => "MLS Staff",

            MemberType::NonMemberVendor => "Non Member/Vendor",

            MemberType::OfficeManager => "Office Manager",

            MemberType::REALTORAppraiser => "REALTOR Appraiser",

            MemberType::REALTORSalesperson => "REALTOR Salesperson",

            MemberType::UnlicensedAssistant => "Unlicensed Assistant",

            MemberType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for MemberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MemberType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_member_type_format {
    use super::MemberType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<MemberType>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<MemberType>>, D::Error>
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
