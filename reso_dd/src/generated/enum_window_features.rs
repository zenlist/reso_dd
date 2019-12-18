// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [WindowFeatures Lookups](https://ddwiki.reso.org/display/DDW17/WindowFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WindowFeatures {
    /// "[Aluminum Frames](https://ddwiki.reso.org/display/DDW17/Aluminum+Frames)": The windows have aluminum frames.
    AluminumFrames,

    /// "[Bay Window(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246644)": The property has one or more bay windows.
    BayWindows,

    /// "[Blinds](https://ddwiki.reso.org/display/DDW17/Blinds)": The property has window blinds.
    Blinds,

    /// "[Display Window(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246646)": The property has one or more windows that would normally be used to display goods or products.
    DisplayWindows,

    /// "[Double Pane Windows](https://ddwiki.reso.org/display/DDW17/Double+Pane+Windows)": The property has windows with two panes of glass.
    DoublePaneWindows,

    /// "[Drapes](https://ddwiki.reso.org/display/DDW17/Drapes)": The property has drapes.
    Drapes,

    /// "[ENERGY STAR Qualified Windows](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Windows)": The property has ENERGY STAR Qualified windows.
    ENERGYSTARQualifiedWindows,

    /// "[Garden Window(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246650)": The property has one or more garden windows.
    GardenWindows,

    /// "[Insulated Windows](https://ddwiki.reso.org/display/DDW17/Insulated+Windows)": The property has insulated windows.
    InsulatedWindows,

    /// "[Low Emissivity Windows](https://ddwiki.reso.org/display/DDW17/Low+Emissivity+Windows)": The property has low emissivity windows.
    LowEmissivityWindows,

    /// "[Plantation Shutters](https://ddwiki.reso.org/display/DDW17/Plantation+Shutters)": The property has plantation shutters.
    PlantationShutters,

    /// "[Screens](https://ddwiki.reso.org/display/DDW17/Screens)": The property has screens.
    Screens,

    /// "[Shutters](https://ddwiki.reso.org/display/DDW17/Shutters)": The property has shutters.
    Shutters,

    /// "[Skylight(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246656)": The property has skylight(s).
    Skylights,

    /// "[Solar Screens](https://ddwiki.reso.org/display/DDW17/Solar+Screens)": The property has solar screens.
    SolarScreens,

    /// "[Storm Window(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246658)": The property has storm window(s).
    StormWindows,

    /// "[Tinted Windows](https://ddwiki.reso.org/display/DDW17/Tinted+Windows)": The property has tinted windows.
    TintedWindows,

    /// "[Triple Pane Windows](https://ddwiki.reso.org/display/DDW17/Triple+Pane+Windows)": The property has triple pane windows.
    TriplePaneWindows,

    /// "[Window Coverings](https://ddwiki.reso.org/display/DDW17/Window+Coverings)": The property has window coverings.
    WindowCoverings,

    /// "[Window Treatments](https://ddwiki.reso.org/display/DDW17/Window+Treatments)": The property has window treatments.
    WindowTreatments,

    /// "[Wood Frames](https://ddwiki.reso.org/display/DDW17/Wood+Frames)": The property has wood framed windows.
    WoodFrames,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for WindowFeatures {
    fn from(s: String) -> WindowFeatures {
        match s.as_ref() {
            "Aluminum Frames" => WindowFeatures::AluminumFrames,

            "Bay Window(s)" => WindowFeatures::BayWindows,

            "Blinds" => WindowFeatures::Blinds,

            "Display Window(s)" => WindowFeatures::DisplayWindows,

            "Double Pane Windows" => WindowFeatures::DoublePaneWindows,

            "Drapes" => WindowFeatures::Drapes,

            "ENERGY STAR Qualified Windows" => WindowFeatures::ENERGYSTARQualifiedWindows,

            "Garden Window(s)" => WindowFeatures::GardenWindows,

            "Insulated Windows" => WindowFeatures::InsulatedWindows,

            "Low Emissivity Windows" => WindowFeatures::LowEmissivityWindows,

            "Plantation Shutters" => WindowFeatures::PlantationShutters,

            "Screens" => WindowFeatures::Screens,

            "Shutters" => WindowFeatures::Shutters,

            "Skylight(s)" => WindowFeatures::Skylights,

            "Solar Screens" => WindowFeatures::SolarScreens,

            "Storm Window(s)" => WindowFeatures::StormWindows,

            "Tinted Windows" => WindowFeatures::TintedWindows,

            "Triple Pane Windows" => WindowFeatures::TriplePaneWindows,

            "Window Coverings" => WindowFeatures::WindowCoverings,

            "Window Treatments" => WindowFeatures::WindowTreatments,

            "Wood Frames" => WindowFeatures::WoodFrames,

            _ => WindowFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for WindowFeatures {
    fn from(s: &str) -> WindowFeatures {
        match s {
            "Aluminum Frames" => WindowFeatures::AluminumFrames,

            "Bay Window(s)" => WindowFeatures::BayWindows,

            "Blinds" => WindowFeatures::Blinds,

            "Display Window(s)" => WindowFeatures::DisplayWindows,

            "Double Pane Windows" => WindowFeatures::DoublePaneWindows,

            "Drapes" => WindowFeatures::Drapes,

            "ENERGY STAR Qualified Windows" => WindowFeatures::ENERGYSTARQualifiedWindows,

            "Garden Window(s)" => WindowFeatures::GardenWindows,

            "Insulated Windows" => WindowFeatures::InsulatedWindows,

            "Low Emissivity Windows" => WindowFeatures::LowEmissivityWindows,

            "Plantation Shutters" => WindowFeatures::PlantationShutters,

            "Screens" => WindowFeatures::Screens,

            "Shutters" => WindowFeatures::Shutters,

            "Skylight(s)" => WindowFeatures::Skylights,

            "Solar Screens" => WindowFeatures::SolarScreens,

            "Storm Window(s)" => WindowFeatures::StormWindows,

            "Tinted Windows" => WindowFeatures::TintedWindows,

            "Triple Pane Windows" => WindowFeatures::TriplePaneWindows,

            "Window Coverings" => WindowFeatures::WindowCoverings,

            "Window Treatments" => WindowFeatures::WindowTreatments,

            "Wood Frames" => WindowFeatures::WoodFrames,

            _ => WindowFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a WindowFeatures> for &'a str {
    fn from(s: &'a WindowFeatures) -> &'a str {
        match s {
            WindowFeatures::AluminumFrames => "Aluminum Frames",

            WindowFeatures::BayWindows => "Bay Window(s)",

            WindowFeatures::Blinds => "Blinds",

            WindowFeatures::DisplayWindows => "Display Window(s)",

            WindowFeatures::DoublePaneWindows => "Double Pane Windows",

            WindowFeatures::Drapes => "Drapes",

            WindowFeatures::ENERGYSTARQualifiedWindows => "ENERGY STAR Qualified Windows",

            WindowFeatures::GardenWindows => "Garden Window(s)",

            WindowFeatures::InsulatedWindows => "Insulated Windows",

            WindowFeatures::LowEmissivityWindows => "Low Emissivity Windows",

            WindowFeatures::PlantationShutters => "Plantation Shutters",

            WindowFeatures::Screens => "Screens",

            WindowFeatures::Shutters => "Shutters",

            WindowFeatures::Skylights => "Skylight(s)",

            WindowFeatures::SolarScreens => "Solar Screens",

            WindowFeatures::StormWindows => "Storm Window(s)",

            WindowFeatures::TintedWindows => "Tinted Windows",

            WindowFeatures::TriplePaneWindows => "Triple Pane Windows",

            WindowFeatures::WindowCoverings => "Window Coverings",

            WindowFeatures::WindowTreatments => "Window Treatments",

            WindowFeatures::WoodFrames => "Wood Frames",

            WindowFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for WindowFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for WindowFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_window_features_format {
    use super::WindowFeatures;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<WindowFeatures>>,
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
    ) -> Result<Option<Vec<WindowFeatures>>, D::Error>
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
