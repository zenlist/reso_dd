// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [View Lookups](https://ddwiki.reso.org/display/DDW17/View+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum View {
    /// "[Bay](https://ddwiki.reso.org/display/DDW17/Bay)": The property has a bay view.
    Bay,

    /// "[Beach](https://ddwiki.reso.org/display/DDW17/Beach)": The property has a beach view.
    Beach,

    /// "[Bridge(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246689)": The property has a bridge(s) view.
    Bridges,

    /// "[Canal](https://ddwiki.reso.org/display/DDW17/Canal)": The property has a canal view.
    Canal,

    /// "[Canyon](https://ddwiki.reso.org/display/DDW17/Canyon)": The property has a canyon view.
    Canyon,

    /// "[City](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246692)": The property has a city view.
    City,

    /// "[City Lights](https://ddwiki.reso.org/display/DDW17/City+Lights)": The property has a view of the city lights.
    CityLights,

    /// "[Creek/Stream](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246694)": The property has a creek/stream view.
    CreekStream,

    /// "[Desert](https://ddwiki.reso.org/display/DDW17/Desert)": The property has a desert view.
    Desert,

    /// "[Downtown](https://ddwiki.reso.org/display/DDW17/Downtown)": The property has a downtown view.
    Downtown,

    /// "[Forest](https://ddwiki.reso.org/display/DDW17/Forest)": The property has a forest view.
    Forest,

    /// "[Garden](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246698)": The property has a garden view.
    Garden,

    /// "[Golf Course](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246699)": The property has a view of the golf course.
    GolfCourse,

    /// "[Hills](https://ddwiki.reso.org/display/DDW17/Hills)": The property has a view of the hills.
    Hills,

    /// "[Lake](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246701)": The property has a lake view.
    Lake,

    /// "[Marina](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246702)": The property has a marina view.
    Marina,

    /// "[Meadow](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246703)": The property has a view of the meadow.
    Meadow,

    /// "[Mountain(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246704)": The property has a mountain view.
    Mountains,

    /// "[Neighborhood](https://ddwiki.reso.org/display/DDW17/Neighborhood)": The property has a view of the neighborhood.
    Neighborhood,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246706)": The property has no view.
    None,

    /// "[Ocean](https://ddwiki.reso.org/display/DDW17/Ocean)": The property has an ocean view.
    Ocean,

    /// "[Orchard](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246708)": The property has a view of the orchard(s).
    Orchard,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246709)": The property has a view other than those in this list.
    Other,

    /// "[Panoramic](https://ddwiki.reso.org/display/DDW17/Panoramic)": The property has a panoramic view.
    Panoramic,

    /// "[Park/Greenbelt](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246711)": The property has a park/greenbelt view.
    ParkGreenbelt,

    /// "[Pasture](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246712)": The property has a view of the pasture.
    Pasture,

    /// "[Pond](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246713)": The property has a view of a pond.
    Pond,

    /// "[Pool](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246714)": The property has a view of the pool.
    Pool,

    /// "[Ridge](https://ddwiki.reso.org/display/DDW17/Ridge)": The property has a view of the ridge.
    Ridge,

    /// "[River](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246716)": The property has a river view.
    River,

    /// "[Rural](https://ddwiki.reso.org/display/DDW17/Rural)": The property has a rural view.
    Rural,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246718)": See the remarks fields for more information about the view from the property.
    SeeRemarks,

    /// "[Skyline](https://ddwiki.reso.org/display/DDW17/Skyline)": The property has a view of the skyline.
    Skyline,

    /// "[Territorial](https://ddwiki.reso.org/display/DDW17/Territorial)": The property has a view of the surrounding area.
    Territorial,

    /// "[Trees/Woods](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246721)": The property has a view of the trees or woods.
    TreesWoods,

    /// "[Valley](https://ddwiki.reso.org/display/DDW17/Valley)": The property has a view of the valley.
    Valley,

    /// "[Vineyard](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246723)": The property has a view of the vineyard(s).
    Vineyard,

    /// "[Water](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246724)": The property has a water view.
    Water,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for View {
    fn from(s: String) -> View {
        match s.as_ref() {
            "Bay" => View::Bay,

            "Beach" => View::Beach,

            "Bridge(s)" => View::Bridges,

            "Canal" => View::Canal,

            "Canyon" => View::Canyon,

            "City" => View::City,

            "City Lights" => View::CityLights,

            "Creek/Stream" => View::CreekStream,

            "Desert" => View::Desert,

            "Downtown" => View::Downtown,

            "Forest" => View::Forest,

            "Garden" => View::Garden,

            "Golf Course" => View::GolfCourse,

            "Hills" => View::Hills,

            "Lake" => View::Lake,

            "Marina" => View::Marina,

            "Meadow" => View::Meadow,

            "Mountain(s)" => View::Mountains,

            "Neighborhood" => View::Neighborhood,

            "None" => View::None,

            "Ocean" => View::Ocean,

            "Orchard" => View::Orchard,

            "Other" => View::Other,

            "Panoramic" => View::Panoramic,

            "Park/Greenbelt" => View::ParkGreenbelt,

            "Pasture" => View::Pasture,

            "Pond" => View::Pond,

            "Pool" => View::Pool,

            "Ridge" => View::Ridge,

            "River" => View::River,

            "Rural" => View::Rural,

            "See Remarks" => View::SeeRemarks,

            "Skyline" => View::Skyline,

            "Territorial" => View::Territorial,

            "Trees/Woods" => View::TreesWoods,

            "Valley" => View::Valley,

            "Vineyard" => View::Vineyard,

            "Water" => View::Water,

            _ => View::OpenEnumeration(s),
        }
    }
}

impl From<&str> for View {
    fn from(s: &str) -> View {
        match s {
            "Bay" => View::Bay,

            "Beach" => View::Beach,

            "Bridge(s)" => View::Bridges,

            "Canal" => View::Canal,

            "Canyon" => View::Canyon,

            "City" => View::City,

            "City Lights" => View::CityLights,

            "Creek/Stream" => View::CreekStream,

            "Desert" => View::Desert,

            "Downtown" => View::Downtown,

            "Forest" => View::Forest,

            "Garden" => View::Garden,

            "Golf Course" => View::GolfCourse,

            "Hills" => View::Hills,

            "Lake" => View::Lake,

            "Marina" => View::Marina,

            "Meadow" => View::Meadow,

            "Mountain(s)" => View::Mountains,

            "Neighborhood" => View::Neighborhood,

            "None" => View::None,

            "Ocean" => View::Ocean,

            "Orchard" => View::Orchard,

            "Other" => View::Other,

            "Panoramic" => View::Panoramic,

            "Park/Greenbelt" => View::ParkGreenbelt,

            "Pasture" => View::Pasture,

            "Pond" => View::Pond,

            "Pool" => View::Pool,

            "Ridge" => View::Ridge,

            "River" => View::River,

            "Rural" => View::Rural,

            "See Remarks" => View::SeeRemarks,

            "Skyline" => View::Skyline,

            "Territorial" => View::Territorial,

            "Trees/Woods" => View::TreesWoods,

            "Valley" => View::Valley,

            "Vineyard" => View::Vineyard,

            "Water" => View::Water,

            _ => View::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a View> for &'a str {
    fn from(s: &'a View) -> &'a str {
        match s {
            View::Bay => "Bay",

            View::Beach => "Beach",

            View::Bridges => "Bridge(s)",

            View::Canal => "Canal",

            View::Canyon => "Canyon",

            View::City => "City",

            View::CityLights => "City Lights",

            View::CreekStream => "Creek/Stream",

            View::Desert => "Desert",

            View::Downtown => "Downtown",

            View::Forest => "Forest",

            View::Garden => "Garden",

            View::GolfCourse => "Golf Course",

            View::Hills => "Hills",

            View::Lake => "Lake",

            View::Marina => "Marina",

            View::Meadow => "Meadow",

            View::Mountains => "Mountain(s)",

            View::Neighborhood => "Neighborhood",

            View::None => "None",

            View::Ocean => "Ocean",

            View::Orchard => "Orchard",

            View::Other => "Other",

            View::Panoramic => "Panoramic",

            View::ParkGreenbelt => "Park/Greenbelt",

            View::Pasture => "Pasture",

            View::Pond => "Pond",

            View::Pool => "Pool",

            View::Ridge => "Ridge",

            View::River => "River",

            View::Rural => "Rural",

            View::SeeRemarks => "See Remarks",

            View::Skyline => "Skyline",

            View::Territorial => "Territorial",

            View::TreesWoods => "Trees/Woods",

            View::Valley => "Valley",

            View::Vineyard => "Vineyard",

            View::Water => "Water",

            View::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for View {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for View {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_view_format {
    use super::View;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(items: &Option<Vec<View>>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<View>>, D::Error>
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
