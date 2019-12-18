// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [SecurityFeatures Lookups](https://ddwiki.reso.org/display/DDW17/SecurityFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SecurityFeatures {
    /// "[24 Hour Security](https://ddwiki.reso.org/display/DDW17/24+Hour+Security)": The property has 24 hour security.
    _24HourSecurity,

    /// "[Building Security](https://ddwiki.reso.org/display/DDW17/Building+Security)": The property has building security.
    BuildingSecurity,

    /// "[Carbon Monoxide Detector(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246457)": The property has carbon monoxide detector(s).
    CarbonMonoxideDetectors,

    /// "[Closed Circuit Camera(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246458)": The property has closed circuit camera(s).
    ClosedCircuitCameras,

    /// "[Fire Alarm](https://ddwiki.reso.org/display/DDW17/Fire+Alarm)": The property has fire alarm(s).
    FireAlarm,

    /// "[Fire Escape](https://ddwiki.reso.org/display/DDW17/Fire+Escape)": The property has a fire escape.
    FireEscape,

    /// "[Fire Sprinkler System](https://ddwiki.reso.org/display/DDW17/Fire+Sprinkler+System)": The property has a fire sprinkler system.
    FireSprinklerSystem,

    /// "[Firewall(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246462)": The property has firewall(s).
    Firewalls,

    /// "[Gated Community](https://ddwiki.reso.org/display/DDW17/Gated+Community)": The property is in a gated community.
    GatedCommunity,

    /// "[Gated with Guard](https://ddwiki.reso.org/display/DDW17/Gated+with+Guard)": The property is in a gated community/area with guard service.
    GatedwithGuard,

    /// "[Key Card Entry](https://ddwiki.reso.org/display/DDW17/Key+Card+Entry)": The property or community has key card entry.
    KeyCardEntry,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246466)": The property has security features other than those in this list.
    Other,

    /// "[Panic Alarm](https://ddwiki.reso.org/display/DDW17/Panic+Alarm)": The property has a panic alarm.
    PanicAlarm,

    /// "[Prewired](https://ddwiki.reso.org/display/DDW17/Prewired)": The property is prewired for a security system.
    Prewired,

    /// "[Secured Garage/Parking](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246469)": The property has a secured garage or parking area.
    SecuredGarageParking,

    /// "[Security Fence](https://ddwiki.reso.org/display/DDW17/Security+Fence)": The property has a security fence.
    SecurityFence,

    /// "[Security Gate](https://ddwiki.reso.org/display/DDW17/Security+Gate)": The property has a security gate.
    SecurityGate,

    /// "[Security Guard](https://ddwiki.reso.org/display/DDW17/Security+Guard)": The property or community has a security guard.
    SecurityGuard,

    /// "[Security Lights](https://ddwiki.reso.org/display/DDW17/Security+Lights)": The property has security lights.
    SecurityLights,

    /// "[Security Service](https://ddwiki.reso.org/display/DDW17/Security+Service)": The property has a security service.
    SecurityService,

    /// "[Security System](https://ddwiki.reso.org/display/DDW17/Security+System)": The property has a security system.
    SecuritySystem,

    /// "[Security System Leased](https://ddwiki.reso.org/display/DDW17/Security+System+Leased)": The property has a leased security system.
    SecuritySystemLeased,

    /// "[Security System Owned](https://ddwiki.reso.org/display/DDW17/Security+System+Owned)": The property has an owned security system.
    SecuritySystemOwned,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246478)": See the remarks fields for more information about the security features of the property.
    SeeRemarks,

    /// "[Smoke Detector(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246479)": The property has smoke detector(s).
    SmokeDetectors,

    /// "[Varies By Unit](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246480)": The security features vary from unit to unit.
    VariesByUnit,

    /// "[Window Bars](https://ddwiki.reso.org/display/DDW17/Window+Bars)": The property has window bars.
    WindowBars,

    /// "[Window Bars with Quick Release](https://ddwiki.reso.org/display/DDW17/Window+Bars+with+Quick+Release)": The property has window bars with a quick release mechanism.
    WindowBarswithQuickRelease,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for SecurityFeatures {
    fn from(s: String) -> SecurityFeatures {
        match s.as_ref() {
            "24 Hour Security" => SecurityFeatures::_24HourSecurity,

            "Building Security" => SecurityFeatures::BuildingSecurity,

            "Carbon Monoxide Detector(s)" => SecurityFeatures::CarbonMonoxideDetectors,

            "Closed Circuit Camera(s)" => SecurityFeatures::ClosedCircuitCameras,

            "Fire Alarm" => SecurityFeatures::FireAlarm,

            "Fire Escape" => SecurityFeatures::FireEscape,

            "Fire Sprinkler System" => SecurityFeatures::FireSprinklerSystem,

            "Firewall(s)" => SecurityFeatures::Firewalls,

            "Gated Community" => SecurityFeatures::GatedCommunity,

            "Gated with Guard" => SecurityFeatures::GatedwithGuard,

            "Key Card Entry" => SecurityFeatures::KeyCardEntry,

            "Other" => SecurityFeatures::Other,

            "Panic Alarm" => SecurityFeatures::PanicAlarm,

            "Prewired" => SecurityFeatures::Prewired,

            "Secured Garage/Parking" => SecurityFeatures::SecuredGarageParking,

            "Security Fence" => SecurityFeatures::SecurityFence,

            "Security Gate" => SecurityFeatures::SecurityGate,

            "Security Guard" => SecurityFeatures::SecurityGuard,

            "Security Lights" => SecurityFeatures::SecurityLights,

            "Security Service" => SecurityFeatures::SecurityService,

            "Security System" => SecurityFeatures::SecuritySystem,

            "Security System Leased" => SecurityFeatures::SecuritySystemLeased,

            "Security System Owned" => SecurityFeatures::SecuritySystemOwned,

            "See Remarks" => SecurityFeatures::SeeRemarks,

            "Smoke Detector(s)" => SecurityFeatures::SmokeDetectors,

            "Varies By Unit" => SecurityFeatures::VariesByUnit,

            "Window Bars" => SecurityFeatures::WindowBars,

            "Window Bars with Quick Release" => SecurityFeatures::WindowBarswithQuickRelease,

            _ => SecurityFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for SecurityFeatures {
    fn from(s: &str) -> SecurityFeatures {
        match s {
            "24 Hour Security" => SecurityFeatures::_24HourSecurity,

            "Building Security" => SecurityFeatures::BuildingSecurity,

            "Carbon Monoxide Detector(s)" => SecurityFeatures::CarbonMonoxideDetectors,

            "Closed Circuit Camera(s)" => SecurityFeatures::ClosedCircuitCameras,

            "Fire Alarm" => SecurityFeatures::FireAlarm,

            "Fire Escape" => SecurityFeatures::FireEscape,

            "Fire Sprinkler System" => SecurityFeatures::FireSprinklerSystem,

            "Firewall(s)" => SecurityFeatures::Firewalls,

            "Gated Community" => SecurityFeatures::GatedCommunity,

            "Gated with Guard" => SecurityFeatures::GatedwithGuard,

            "Key Card Entry" => SecurityFeatures::KeyCardEntry,

            "Other" => SecurityFeatures::Other,

            "Panic Alarm" => SecurityFeatures::PanicAlarm,

            "Prewired" => SecurityFeatures::Prewired,

            "Secured Garage/Parking" => SecurityFeatures::SecuredGarageParking,

            "Security Fence" => SecurityFeatures::SecurityFence,

            "Security Gate" => SecurityFeatures::SecurityGate,

            "Security Guard" => SecurityFeatures::SecurityGuard,

            "Security Lights" => SecurityFeatures::SecurityLights,

            "Security Service" => SecurityFeatures::SecurityService,

            "Security System" => SecurityFeatures::SecuritySystem,

            "Security System Leased" => SecurityFeatures::SecuritySystemLeased,

            "Security System Owned" => SecurityFeatures::SecuritySystemOwned,

            "See Remarks" => SecurityFeatures::SeeRemarks,

            "Smoke Detector(s)" => SecurityFeatures::SmokeDetectors,

            "Varies By Unit" => SecurityFeatures::VariesByUnit,

            "Window Bars" => SecurityFeatures::WindowBars,

            "Window Bars with Quick Release" => SecurityFeatures::WindowBarswithQuickRelease,

            _ => SecurityFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a SecurityFeatures> for &'a str {
    fn from(s: &'a SecurityFeatures) -> &'a str {
        match s {
            SecurityFeatures::_24HourSecurity => "24 Hour Security",

            SecurityFeatures::BuildingSecurity => "Building Security",

            SecurityFeatures::CarbonMonoxideDetectors => "Carbon Monoxide Detector(s)",

            SecurityFeatures::ClosedCircuitCameras => "Closed Circuit Camera(s)",

            SecurityFeatures::FireAlarm => "Fire Alarm",

            SecurityFeatures::FireEscape => "Fire Escape",

            SecurityFeatures::FireSprinklerSystem => "Fire Sprinkler System",

            SecurityFeatures::Firewalls => "Firewall(s)",

            SecurityFeatures::GatedCommunity => "Gated Community",

            SecurityFeatures::GatedwithGuard => "Gated with Guard",

            SecurityFeatures::KeyCardEntry => "Key Card Entry",

            SecurityFeatures::Other => "Other",

            SecurityFeatures::PanicAlarm => "Panic Alarm",

            SecurityFeatures::Prewired => "Prewired",

            SecurityFeatures::SecuredGarageParking => "Secured Garage/Parking",

            SecurityFeatures::SecurityFence => "Security Fence",

            SecurityFeatures::SecurityGate => "Security Gate",

            SecurityFeatures::SecurityGuard => "Security Guard",

            SecurityFeatures::SecurityLights => "Security Lights",

            SecurityFeatures::SecurityService => "Security Service",

            SecurityFeatures::SecuritySystem => "Security System",

            SecurityFeatures::SecuritySystemLeased => "Security System Leased",

            SecurityFeatures::SecuritySystemOwned => "Security System Owned",

            SecurityFeatures::SeeRemarks => "See Remarks",

            SecurityFeatures::SmokeDetectors => "Smoke Detector(s)",

            SecurityFeatures::VariesByUnit => "Varies By Unit",

            SecurityFeatures::WindowBars => "Window Bars",

            SecurityFeatures::WindowBarswithQuickRelease => "Window Bars with Quick Release",

            SecurityFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for SecurityFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SecurityFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_security_features_format {
    use super::SecurityFeatures;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<SecurityFeatures>>,
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
    ) -> Result<Option<Vec<SecurityFeatures>>, D::Error>
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
