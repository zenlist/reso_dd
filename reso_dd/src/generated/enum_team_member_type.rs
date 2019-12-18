// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [TeamMemberType Lookups](https://ddwiki.reso.org/display/DDW17/TeamMemberType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TeamMemberType {
    /// "[Administration Assistant](https://ddwiki.reso.org/display/DDW17/Administration+Assistant)": The member of a team who assists with administrative tasks.
    AdministrationAssistant,

    /// "[Buyer Agent](https://ddwiki.reso.org/display/DDW17/Buyer+Agent)": A member of the real estate team.
    BuyerAgent,

    /// "[Lead Manager](https://ddwiki.reso.org/display/DDW17/Lead+Manager)": The member of the team who is the lead manager.
    LeadManager,

    /// "[Listing Agent](https://ddwiki.reso.org/display/DDW17/Listing+Agent)": The member of a team who lists properties.
    ListingAgent,

    /// "[Marketing Assistant](https://ddwiki.reso.org/display/DDW17/Marketing+Assistant)": The member of a team who assists with marketing.
    MarketingAssistant,

    /// "[Operations Manager](https://ddwiki.reso.org/display/DDW17/Operations+Manager)": The member of the team who manages operations.
    OperationsManager,

    /// "[Team Lead](https://ddwiki.reso.org/display/DDW17/Team+Lead)": The leading member of a team.
    TeamLead,

    /// "[Team Member](https://ddwiki.reso.org/display/DDW17/Team+Member+Lead)": A member of the real estate team.
    TeamMember,

    /// "[Transaction Coordinator](https://ddwiki.reso.org/display/DDW17/Transaction+Coordinator)": The member of a team who handles transaction details.
    TransactionCoordinator,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for TeamMemberType {
    fn from(s: String) -> TeamMemberType {
        match s.as_ref() {
            "Administration Assistant" => TeamMemberType::AdministrationAssistant,

            "Buyer Agent" => TeamMemberType::BuyerAgent,

            "Lead Manager" => TeamMemberType::LeadManager,

            "Listing Agent" => TeamMemberType::ListingAgent,

            "Marketing Assistant" => TeamMemberType::MarketingAssistant,

            "Operations Manager" => TeamMemberType::OperationsManager,

            "Team Lead" => TeamMemberType::TeamLead,

            "Team Member" => TeamMemberType::TeamMember,

            "Transaction Coordinator" => TeamMemberType::TransactionCoordinator,

            _ => TeamMemberType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for TeamMemberType {
    fn from(s: &str) -> TeamMemberType {
        match s {
            "Administration Assistant" => TeamMemberType::AdministrationAssistant,

            "Buyer Agent" => TeamMemberType::BuyerAgent,

            "Lead Manager" => TeamMemberType::LeadManager,

            "Listing Agent" => TeamMemberType::ListingAgent,

            "Marketing Assistant" => TeamMemberType::MarketingAssistant,

            "Operations Manager" => TeamMemberType::OperationsManager,

            "Team Lead" => TeamMemberType::TeamLead,

            "Team Member" => TeamMemberType::TeamMember,

            "Transaction Coordinator" => TeamMemberType::TransactionCoordinator,

            _ => TeamMemberType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a TeamMemberType> for &'a str {
    fn from(s: &'a TeamMemberType) -> &'a str {
        match s {
            TeamMemberType::AdministrationAssistant => "Administration Assistant",

            TeamMemberType::BuyerAgent => "Buyer Agent",

            TeamMemberType::LeadManager => "Lead Manager",

            TeamMemberType::ListingAgent => "Listing Agent",

            TeamMemberType::MarketingAssistant => "Marketing Assistant",

            TeamMemberType::OperationsManager => "Operations Manager",

            TeamMemberType::TeamLead => "Team Lead",

            TeamMemberType::TeamMember => "Team Member",

            TeamMemberType::TransactionCoordinator => "Transaction Coordinator",

            TeamMemberType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for TeamMemberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TeamMemberType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_team_member_type_format {
    use super::TeamMemberType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<TeamMemberType>>,
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
    ) -> Result<Option<Vec<TeamMemberType>>, D::Error>
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
