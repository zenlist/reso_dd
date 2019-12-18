// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ShowingRequirements Lookups](https://ddwiki.reso.org/display/DDW17/ShowingRequirements+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShowingRequirements {
    /// "[24 Hour Notice](https://ddwiki.reso.org/display/DDW17/24+Hour+Notice)": A 24 hour notice is required to show the property.
    _24HourNotice,

    /// "[Appointment Only](https://ddwiki.reso.org/display/DDW17/Appointment+Only)": Showing of the property is by appointment only.
    AppointmentOnly,

    /// "[Call Listing Agent](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246486)": Call the listing agent to arrange a showing of the property.
    CallListingAgent,

    /// "[Call Listing Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246487)": Call the listing office to arrange a showing of the property.
    CallListingOffice,

    /// "[Call Manager](https://ddwiki.reso.org/display/DDW17/Call+Manager)": Call the property manage to arrange a showing of the property.
    CallManager,

    /// "[Call Owner](https://ddwiki.reso.org/display/DDW17/Call+Owner)": Call the property owner to arrange a showing of the property.
    CallOwner,

    /// "[Call Tenant](https://ddwiki.reso.org/display/DDW17/Call+Tenant)": Call the tenant/occupant directly to arrange a showing of the property.
    CallTenant,

    /// "[Combination Lock Box](https://ddwiki.reso.org/display/DDW17/Combination+Lock+Box)": The property has a combination lock box for showing access.
    CombinationLockBox,

    /// "[Day Sleeper](https://ddwiki.reso.org/display/DDW17/Day+Sleeper)": The property has a tenant/occupant who sleeps during the day.
    DaySleeper,

    /// "[Do Not Show](https://ddwiki.reso.org/display/DDW17/Do+Not+Show)": Do not show this property.
    DoNotShow,

    /// "[Email Listing Agent](https://ddwiki.reso.org/display/DDW17/Email+Listing+Agent)": Email the listing agent for more information about showing the property.
    EmailListingAgent,

    /// "[Key In Office](https://ddwiki.reso.org/display/DDW17/Key+In+Office)": The key to access the property for showing must be retrieved from the listing or manager's office.
    KeyInOffice,

    /// "[Lockbox](https://ddwiki.reso.org/display/DDW17/Lockbox)": The property has an electronic lockbox for showing access.
    Lockbox,

    /// "[No Lockbox](https://ddwiki.reso.org/display/DDW17/No+Lockbox)": There is no lockbox on the property.
    NoLockbox,

    /// "[No Sign](https://ddwiki.reso.org/display/DDW17/No+Sign)": The property has no for sale sign.
    NoSign,

    /// "[Occupied](https://ddwiki.reso.org/display/DDW17/Occupied)": The property is currently occupied.
    Occupied,

    /// "[Pet(s) on Premises](https://ddwiki.reso.org/display/DDW17/Pet%28s%29+on+Premises)": There are currently pets at the property.
    PetsonPremises,

    /// "[Restricted Hours](https://ddwiki.reso.org/display/DDW17/Restricted+Hours)": The times when the property may be shown are restricted.
    RestrictedHours,

    /// "[Security System](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246502)": The property has a security system that is a consideration when showing.
    SecuritySystem,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246503)": See the remarks fields for more information about showing the property.
    SeeRemarks,

    /// "[Showing Service](https://ddwiki.reso.org/display/DDW17/Showing+Service)": A service used by a listing broker to provide showing services of listed properties.
    ShowingService,

    /// "[Text Listing Agent](https://ddwiki.reso.org/display/DDW17/Text+Listing+Agent)": Text message the listing agent to arrange a showing of the property.
    TextListingAgent,

    /// "[To Be Built](https://ddwiki.reso.org/display/DDW17/To+Be+Built)": The property has yet to be built.
    ToBeBuilt,

    /// "[Under Construction](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246506)": The property is under construction.
    UnderConstruction,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for ShowingRequirements {
    fn from(s: String) -> ShowingRequirements {
        match s.as_ref() {
            "24 Hour Notice" => ShowingRequirements::_24HourNotice,

            "Appointment Only" => ShowingRequirements::AppointmentOnly,

            "Call Listing Agent" => ShowingRequirements::CallListingAgent,

            "Call Listing Office" => ShowingRequirements::CallListingOffice,

            "Call Manager" => ShowingRequirements::CallManager,

            "Call Owner" => ShowingRequirements::CallOwner,

            "Call Tenant" => ShowingRequirements::CallTenant,

            "Combination Lock Box" => ShowingRequirements::CombinationLockBox,

            "Day Sleeper" => ShowingRequirements::DaySleeper,

            "Do Not Show" => ShowingRequirements::DoNotShow,

            "Email Listing Agent" => ShowingRequirements::EmailListingAgent,

            "Key In Office" => ShowingRequirements::KeyInOffice,

            "Lockbox" => ShowingRequirements::Lockbox,

            "No Lockbox" => ShowingRequirements::NoLockbox,

            "No Sign" => ShowingRequirements::NoSign,

            "Occupied" => ShowingRequirements::Occupied,

            "Pet(s) on Premises" => ShowingRequirements::PetsonPremises,

            "Restricted Hours" => ShowingRequirements::RestrictedHours,

            "Security System" => ShowingRequirements::SecuritySystem,

            "See Remarks" => ShowingRequirements::SeeRemarks,

            "Showing Service" => ShowingRequirements::ShowingService,

            "Text Listing Agent" => ShowingRequirements::TextListingAgent,

            "To Be Built" => ShowingRequirements::ToBeBuilt,

            "Under Construction" => ShowingRequirements::UnderConstruction,

            _ => ShowingRequirements::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ShowingRequirements {
    fn from(s: &str) -> ShowingRequirements {
        match s {
            "24 Hour Notice" => ShowingRequirements::_24HourNotice,

            "Appointment Only" => ShowingRequirements::AppointmentOnly,

            "Call Listing Agent" => ShowingRequirements::CallListingAgent,

            "Call Listing Office" => ShowingRequirements::CallListingOffice,

            "Call Manager" => ShowingRequirements::CallManager,

            "Call Owner" => ShowingRequirements::CallOwner,

            "Call Tenant" => ShowingRequirements::CallTenant,

            "Combination Lock Box" => ShowingRequirements::CombinationLockBox,

            "Day Sleeper" => ShowingRequirements::DaySleeper,

            "Do Not Show" => ShowingRequirements::DoNotShow,

            "Email Listing Agent" => ShowingRequirements::EmailListingAgent,

            "Key In Office" => ShowingRequirements::KeyInOffice,

            "Lockbox" => ShowingRequirements::Lockbox,

            "No Lockbox" => ShowingRequirements::NoLockbox,

            "No Sign" => ShowingRequirements::NoSign,

            "Occupied" => ShowingRequirements::Occupied,

            "Pet(s) on Premises" => ShowingRequirements::PetsonPremises,

            "Restricted Hours" => ShowingRequirements::RestrictedHours,

            "Security System" => ShowingRequirements::SecuritySystem,

            "See Remarks" => ShowingRequirements::SeeRemarks,

            "Showing Service" => ShowingRequirements::ShowingService,

            "Text Listing Agent" => ShowingRequirements::TextListingAgent,

            "To Be Built" => ShowingRequirements::ToBeBuilt,

            "Under Construction" => ShowingRequirements::UnderConstruction,

            _ => ShowingRequirements::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ShowingRequirements> for &'a str {
    fn from(s: &'a ShowingRequirements) -> &'a str {
        match s {
            ShowingRequirements::_24HourNotice => "24 Hour Notice",

            ShowingRequirements::AppointmentOnly => "Appointment Only",

            ShowingRequirements::CallListingAgent => "Call Listing Agent",

            ShowingRequirements::CallListingOffice => "Call Listing Office",

            ShowingRequirements::CallManager => "Call Manager",

            ShowingRequirements::CallOwner => "Call Owner",

            ShowingRequirements::CallTenant => "Call Tenant",

            ShowingRequirements::CombinationLockBox => "Combination Lock Box",

            ShowingRequirements::DaySleeper => "Day Sleeper",

            ShowingRequirements::DoNotShow => "Do Not Show",

            ShowingRequirements::EmailListingAgent => "Email Listing Agent",

            ShowingRequirements::KeyInOffice => "Key In Office",

            ShowingRequirements::Lockbox => "Lockbox",

            ShowingRequirements::NoLockbox => "No Lockbox",

            ShowingRequirements::NoSign => "No Sign",

            ShowingRequirements::Occupied => "Occupied",

            ShowingRequirements::PetsonPremises => "Pet(s) on Premises",

            ShowingRequirements::RestrictedHours => "Restricted Hours",

            ShowingRequirements::SecuritySystem => "Security System",

            ShowingRequirements::SeeRemarks => "See Remarks",

            ShowingRequirements::ShowingService => "Showing Service",

            ShowingRequirements::TextListingAgent => "Text Listing Agent",

            ShowingRequirements::ToBeBuilt => "To Be Built",

            ShowingRequirements::UnderConstruction => "Under Construction",

            ShowingRequirements::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ShowingRequirements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ShowingRequirements {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_showing_requirements_format {
    use super::ShowingRequirements;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ShowingRequirements>>,
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
    ) -> Result<Option<Vec<ShowingRequirements>>, D::Error>
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
