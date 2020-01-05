// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LotFeatures Lookups](https://ddwiki.reso.org/display/DDW17/LotFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LotFeatures {
    /// "[Agricultural](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245501)": The lot has agricultural features.
    Agricultural,

    /// "[Back Yard](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245502)": The lot has a back yard.
    BackYard,

    /// "[Bluff](https://ddwiki.reso.org/display/DDW17/Bluff)": The lot is on or near a bluff.
    Bluff,

    /// "[City Lot](https://ddwiki.reso.org/display/DDW17/City+Lot)": The lot is in a city/urban setting.
    CityLot,

    /// "[Cleared](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245505)": The lot has been cleared.
    Cleared,

    /// "[Close to Clubhouse](https://ddwiki.reso.org/display/DDW17/Close+to+Clubhouse)": The lot is located close to the community clubhouse.
    ClosetoClubhouse,

    /// "[Corner Lot](https://ddwiki.reso.org/display/DDW17/Corner+Lot)": The lot is located on the corner of an intersection.
    CornerLot,

    /// "[Corners Marked](https://ddwiki.reso.org/display/DDW17/Corners+Marked)": The corners of the lot have been marked.
    CornersMarked,

    /// "[Cul-De-Sac](https://ddwiki.reso.org/display/DDW17/Cul-De-Sac)": The lot is located on street that is closed on one end in a circular shape.  Cul-de-sac translated literally from French is "the bottom of the bag", which helps explain the circular shape.
    CulDeSac,

    /// "[Desert Back](https://ddwiki.reso.org/display/DDW17/Desert+Back)": The back of the lot faces desert.
    DesertBack,

    /// "[Desert Front](https://ddwiki.reso.org/display/DDW17/Desert+Front)": The front of the lot faces desert.
    DesertFront,

    /// "[Farm](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245512)": The lot is, or has characteristics of a farm.
    Farm,

    /// "[Few Trees](https://ddwiki.reso.org/display/DDW17/Few+Trees)": The lot has a few trees.
    FewTrees,

    /// "[Flag Lot](https://ddwiki.reso.org/display/DDW17/Flag+Lot)": Named for the shape, a flag lot has a long driveway leading to the property, together may have the appearance of a pole and flag.  The driveway in a flag lot typically runs between two other properties.
    FlagLot,

    /// "[Front Yard](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245515)": The lot has a front yard.
    FrontYard,

    /// "[Garden](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245516)": The lot has a garden.
    Garden,

    /// "[Gentle Sloping](https://ddwiki.reso.org/display/DDW17/Gentle+Sloping)": The lot's slop is gentle.
    GentleSloping,

    /// "[Greenbelt](https://ddwiki.reso.org/display/DDW17/Greenbelt)": the lot is adjacent to a greenbelt.
    Greenbelt,

    /// "[Interior Lot](https://ddwiki.reso.org/display/DDW17/Interior+Lot)": Also referred to as an inside lot, an interior lot faces street on only one side.
    InteriorLot,

    /// "[Irregular Lot](https://ddwiki.reso.org/display/DDW17/Irregular+Lot)": The lot is not a rectangle.
    IrregularLot,

    /// "[Landscaped](https://ddwiki.reso.org/display/DDW17/Landscaped)": The lot has been fully or partially landscaped.
    Landscaped,

    /// "[Level](https://ddwiki.reso.org/display/DDW17/Level)": The lot is level/flat.
    Level,

    /// "[Many Trees](https://ddwiki.reso.org/display/DDW17/Many+Trees)": The lot has many trees.
    ManyTrees,

    /// "[Meadow](https://ddwiki.reso.org/display/DDW17/Meadow)": The lot has a meadow.
    Meadow,

    /// "[Native Plants](https://ddwiki.reso.org/display/DDW17/Native+Plants)": The lot's landscaping includes native plants.
    NativePlants,

    /// "[Near Golf Course](https://ddwiki.reso.org/display/DDW17/Near+Golf+Course)": The lot is near a golf course.
    NearGolfCourse,

    /// "[Near Public Transit](https://ddwiki.reso.org/display/DDW17/Near+Public+Transit)": The lot is near public transportation.
    NearPublicTransit,

    /// "[On Golf Course](https://ddwiki.reso.org/display/DDW17/On+Golf+Course)": The lot is directly adjacent to a golf course.
    OnGolfCourse,

    /// "[Open Lot](https://ddwiki.reso.org/display/DDW17/Open+Lot)": The lot is open.
    OpenLot,

    /// "[Orchard(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245530)": The lot includes one or more orchards.
    Orchards,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245531)": The lot has features other than those in this list.
    Other,

    /// "[Pasture](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245532)": The lot includes a pasture.
    Pasture,

    /// "[Paved](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245533)": The lot is partially or fully paved.
    Paved,

    /// "[Pie Shaped Lot](https://ddwiki.reso.org/display/DDW17/Pie+Shaped+Lot)": The lot is pie, or triangle shaped.  Typically narrow at the front and wide at the back, the reverse, a wide front, could be referred to as pie shaped or reverse pie shaped.
    PieShapedLot,

    /// "[Private](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245535)": The lot is private or features that provide privacy form adjacent areas such as neighbors or roads.
    Private,

    /// "[Rectangular Lot](https://ddwiki.reso.org/display/DDW17/Rectangular+Lot)": Also known as a regular shaped lot, the lot has is a rectangle or square.
    RectangularLot,

    /// "[Rock Outcropping](https://ddwiki.reso.org/display/DDW17/Rock+Outcropping)": Rock features or barriers that transition a grading in the landscape.
    RockOutcropping,

    /// "[Rolling Slope](https://ddwiki.reso.org/display/DDW17/Rolling+Slope)": The slope of the property varies in a rolling or wavy fashion.
    RollingSlope,

    /// "[Secluded](https://ddwiki.reso.org/display/DDW17/Secluded)": The lot is secluded.
    Secluded,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245540)": See the remarks fields for additional information about the lot's features.
    SeeRemarks,

    /// "[Sloped](https://ddwiki.reso.org/display/DDW17/Sloped)": The lot is sloped.
    Sloped,

    /// "[Sloped Down](https://ddwiki.reso.org/display/DDW17/Sloped+Down)": The lot is sloped down, typically from the perspective of looking at the property from the street.
    SlopedDown,

    /// "[Sloped Up](https://ddwiki.reso.org/display/DDW17/Sloped+Up)": The lot is sloped up, typically from the perspective of looking at the property from the street.
    SlopedUp,

    /// "[Split Possible](https://ddwiki.reso.org/display/DDW17/Split+Possible)": It may be possible that the lot could be split into two or more parcels.
    SplitPossible,

    /// "[Sprinklers In Front](https://ddwiki.reso.org/display/DDW17/Sprinklers+In+Front)": there are irrigation sprinklers on the front of the lot.
    SprinklersInFront,

    /// "[Sprinklers In Rear](https://ddwiki.reso.org/display/DDW17/Sprinklers+In+Rear)": there are irrigation sprinklers to the rear of the lot.
    SprinklersInRear,

    /// "[Steep Slope](https://ddwiki.reso.org/display/DDW17/Steep+Slope)": The lot is sloped steeply.
    SteepSlope,

    /// "[Subdivided](https://ddwiki.reso.org/display/DDW17/Subdivided)": The lot has been subdivided into two or more parcels.
    Subdivided,

    /// "[Views](https://ddwiki.reso.org/display/DDW17/Views)": There are views from the lot.
    Views,

    /// "[Waterfall](https://ddwiki.reso.org/display/DDW17/Waterfall)": The lot has a waterfall.
    Waterfall,

    /// "[Waterfront](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245551)": The lot is located on a waterfront.
    Waterfront,

    /// "[Wetlands](https://ddwiki.reso.org/display/DDW17/Wetlands)": The lot is located near or within wetlands.
    Wetlands,

    /// "[Wooded](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245553)": The lot is wooded.
    Wooded,

    /// "[Zero Lot Line](https://ddwiki.reso.org/display/DDW17/Zero+Lot+Line)": The structure comes up to, or very near the property line.  Attached single family residences, row homes, garden homes, patio homes all may be zero lot line homes.
    ZeroLotLine,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for LotFeatures {
    fn from_str(s: &str) -> LotFeatures {
        match s {
            "Agricultural" => LotFeatures::Agricultural,

            "Back Yard" => LotFeatures::BackYard,

            "Bluff" => LotFeatures::Bluff,

            "City Lot" => LotFeatures::CityLot,

            "Cleared" => LotFeatures::Cleared,

            "Close to Clubhouse" => LotFeatures::ClosetoClubhouse,

            "Corner Lot" => LotFeatures::CornerLot,

            "Corners Marked" => LotFeatures::CornersMarked,

            "Cul-De-Sac" => LotFeatures::CulDeSac,

            "Desert Back" => LotFeatures::DesertBack,

            "Desert Front" => LotFeatures::DesertFront,

            "Farm" => LotFeatures::Farm,

            "Few Trees" => LotFeatures::FewTrees,

            "Flag Lot" => LotFeatures::FlagLot,

            "Front Yard" => LotFeatures::FrontYard,

            "Garden" => LotFeatures::Garden,

            "Gentle Sloping" => LotFeatures::GentleSloping,

            "Greenbelt" => LotFeatures::Greenbelt,

            "Interior Lot" => LotFeatures::InteriorLot,

            "Irregular Lot" => LotFeatures::IrregularLot,

            "Landscaped" => LotFeatures::Landscaped,

            "Level" => LotFeatures::Level,

            "Many Trees" => LotFeatures::ManyTrees,

            "Meadow" => LotFeatures::Meadow,

            "Native Plants" => LotFeatures::NativePlants,

            "Near Golf Course" => LotFeatures::NearGolfCourse,

            "Near Public Transit" => LotFeatures::NearPublicTransit,

            "On Golf Course" => LotFeatures::OnGolfCourse,

            "Open Lot" => LotFeatures::OpenLot,

            "Orchard(s)" => LotFeatures::Orchards,

            "Other" => LotFeatures::Other,

            "Pasture" => LotFeatures::Pasture,

            "Paved" => LotFeatures::Paved,

            "Pie Shaped Lot" => LotFeatures::PieShapedLot,

            "Private" => LotFeatures::Private,

            "Rectangular Lot" => LotFeatures::RectangularLot,

            "Rock Outcropping" => LotFeatures::RockOutcropping,

            "Rolling Slope" => LotFeatures::RollingSlope,

            "Secluded" => LotFeatures::Secluded,

            "See Remarks" => LotFeatures::SeeRemarks,

            "Sloped" => LotFeatures::Sloped,

            "Sloped Down" => LotFeatures::SlopedDown,

            "Sloped Up" => LotFeatures::SlopedUp,

            "Split Possible" => LotFeatures::SplitPossible,

            "Sprinklers In Front" => LotFeatures::SprinklersInFront,

            "Sprinklers In Rear" => LotFeatures::SprinklersInRear,

            "Steep Slope" => LotFeatures::SteepSlope,

            "Subdivided" => LotFeatures::Subdivided,

            "Views" => LotFeatures::Views,

            "Waterfall" => LotFeatures::Waterfall,

            "Waterfront" => LotFeatures::Waterfront,

            "Wetlands" => LotFeatures::Wetlands,

            "Wooded" => LotFeatures::Wooded,

            "Zero Lot Line" => LotFeatures::ZeroLotLine,

            _ => LotFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> LotFeatures {
        match s.as_ref() {
            "Agricultural" => LotFeatures::Agricultural,

            "Back Yard" => LotFeatures::BackYard,

            "Bluff" => LotFeatures::Bluff,

            "City Lot" => LotFeatures::CityLot,

            "Cleared" => LotFeatures::Cleared,

            "Close to Clubhouse" => LotFeatures::ClosetoClubhouse,

            "Corner Lot" => LotFeatures::CornerLot,

            "Corners Marked" => LotFeatures::CornersMarked,

            "Cul-De-Sac" => LotFeatures::CulDeSac,

            "Desert Back" => LotFeatures::DesertBack,

            "Desert Front" => LotFeatures::DesertFront,

            "Farm" => LotFeatures::Farm,

            "Few Trees" => LotFeatures::FewTrees,

            "Flag Lot" => LotFeatures::FlagLot,

            "Front Yard" => LotFeatures::FrontYard,

            "Garden" => LotFeatures::Garden,

            "Gentle Sloping" => LotFeatures::GentleSloping,

            "Greenbelt" => LotFeatures::Greenbelt,

            "Interior Lot" => LotFeatures::InteriorLot,

            "Irregular Lot" => LotFeatures::IrregularLot,

            "Landscaped" => LotFeatures::Landscaped,

            "Level" => LotFeatures::Level,

            "Many Trees" => LotFeatures::ManyTrees,

            "Meadow" => LotFeatures::Meadow,

            "Native Plants" => LotFeatures::NativePlants,

            "Near Golf Course" => LotFeatures::NearGolfCourse,

            "Near Public Transit" => LotFeatures::NearPublicTransit,

            "On Golf Course" => LotFeatures::OnGolfCourse,

            "Open Lot" => LotFeatures::OpenLot,

            "Orchard(s)" => LotFeatures::Orchards,

            "Other" => LotFeatures::Other,

            "Pasture" => LotFeatures::Pasture,

            "Paved" => LotFeatures::Paved,

            "Pie Shaped Lot" => LotFeatures::PieShapedLot,

            "Private" => LotFeatures::Private,

            "Rectangular Lot" => LotFeatures::RectangularLot,

            "Rock Outcropping" => LotFeatures::RockOutcropping,

            "Rolling Slope" => LotFeatures::RollingSlope,

            "Secluded" => LotFeatures::Secluded,

            "See Remarks" => LotFeatures::SeeRemarks,

            "Sloped" => LotFeatures::Sloped,

            "Sloped Down" => LotFeatures::SlopedDown,

            "Sloped Up" => LotFeatures::SlopedUp,

            "Split Possible" => LotFeatures::SplitPossible,

            "Sprinklers In Front" => LotFeatures::SprinklersInFront,

            "Sprinklers In Rear" => LotFeatures::SprinklersInRear,

            "Steep Slope" => LotFeatures::SteepSlope,

            "Subdivided" => LotFeatures::Subdivided,

            "Views" => LotFeatures::Views,

            "Waterfall" => LotFeatures::Waterfall,

            "Waterfront" => LotFeatures::Waterfront,

            "Wetlands" => LotFeatures::Wetlands,

            "Wooded" => LotFeatures::Wooded,

            "Zero Lot Line" => LotFeatures::ZeroLotLine,

            _ => LotFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            LotFeatures::Agricultural => "Agricultural",

            LotFeatures::BackYard => "Back Yard",

            LotFeatures::Bluff => "Bluff",

            LotFeatures::CityLot => "City Lot",

            LotFeatures::Cleared => "Cleared",

            LotFeatures::ClosetoClubhouse => "Close to Clubhouse",

            LotFeatures::CornerLot => "Corner Lot",

            LotFeatures::CornersMarked => "Corners Marked",

            LotFeatures::CulDeSac => "Cul-De-Sac",

            LotFeatures::DesertBack => "Desert Back",

            LotFeatures::DesertFront => "Desert Front",

            LotFeatures::Farm => "Farm",

            LotFeatures::FewTrees => "Few Trees",

            LotFeatures::FlagLot => "Flag Lot",

            LotFeatures::FrontYard => "Front Yard",

            LotFeatures::Garden => "Garden",

            LotFeatures::GentleSloping => "Gentle Sloping",

            LotFeatures::Greenbelt => "Greenbelt",

            LotFeatures::InteriorLot => "Interior Lot",

            LotFeatures::IrregularLot => "Irregular Lot",

            LotFeatures::Landscaped => "Landscaped",

            LotFeatures::Level => "Level",

            LotFeatures::ManyTrees => "Many Trees",

            LotFeatures::Meadow => "Meadow",

            LotFeatures::NativePlants => "Native Plants",

            LotFeatures::NearGolfCourse => "Near Golf Course",

            LotFeatures::NearPublicTransit => "Near Public Transit",

            LotFeatures::OnGolfCourse => "On Golf Course",

            LotFeatures::OpenLot => "Open Lot",

            LotFeatures::Orchards => "Orchard(s)",

            LotFeatures::Other => "Other",

            LotFeatures::Pasture => "Pasture",

            LotFeatures::Paved => "Paved",

            LotFeatures::PieShapedLot => "Pie Shaped Lot",

            LotFeatures::Private => "Private",

            LotFeatures::RectangularLot => "Rectangular Lot",

            LotFeatures::RockOutcropping => "Rock Outcropping",

            LotFeatures::RollingSlope => "Rolling Slope",

            LotFeatures::Secluded => "Secluded",

            LotFeatures::SeeRemarks => "See Remarks",

            LotFeatures::Sloped => "Sloped",

            LotFeatures::SlopedDown => "Sloped Down",

            LotFeatures::SlopedUp => "Sloped Up",

            LotFeatures::SplitPossible => "Split Possible",

            LotFeatures::SprinklersInFront => "Sprinklers In Front",

            LotFeatures::SprinklersInRear => "Sprinklers In Rear",

            LotFeatures::SteepSlope => "Steep Slope",

            LotFeatures::Subdivided => "Subdivided",

            LotFeatures::Views => "Views",

            LotFeatures::Waterfall => "Waterfall",

            LotFeatures::Waterfront => "Waterfront",

            LotFeatures::Wetlands => "Wetlands",

            LotFeatures::Wooded => "Wooded",

            LotFeatures::ZeroLotLine => "Zero Lot Line",

            LotFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            LotFeatures::Agricultural => "Agricultural".into(),

            LotFeatures::BackYard => "Back Yard".into(),

            LotFeatures::Bluff => "Bluff".into(),

            LotFeatures::CityLot => "City Lot".into(),

            LotFeatures::Cleared => "Cleared".into(),

            LotFeatures::ClosetoClubhouse => "Close to Clubhouse".into(),

            LotFeatures::CornerLot => "Corner Lot".into(),

            LotFeatures::CornersMarked => "Corners Marked".into(),

            LotFeatures::CulDeSac => "Cul-De-Sac".into(),

            LotFeatures::DesertBack => "Desert Back".into(),

            LotFeatures::DesertFront => "Desert Front".into(),

            LotFeatures::Farm => "Farm".into(),

            LotFeatures::FewTrees => "Few Trees".into(),

            LotFeatures::FlagLot => "Flag Lot".into(),

            LotFeatures::FrontYard => "Front Yard".into(),

            LotFeatures::Garden => "Garden".into(),

            LotFeatures::GentleSloping => "Gentle Sloping".into(),

            LotFeatures::Greenbelt => "Greenbelt".into(),

            LotFeatures::InteriorLot => "Interior Lot".into(),

            LotFeatures::IrregularLot => "Irregular Lot".into(),

            LotFeatures::Landscaped => "Landscaped".into(),

            LotFeatures::Level => "Level".into(),

            LotFeatures::ManyTrees => "Many Trees".into(),

            LotFeatures::Meadow => "Meadow".into(),

            LotFeatures::NativePlants => "Native Plants".into(),

            LotFeatures::NearGolfCourse => "Near Golf Course".into(),

            LotFeatures::NearPublicTransit => "Near Public Transit".into(),

            LotFeatures::OnGolfCourse => "On Golf Course".into(),

            LotFeatures::OpenLot => "Open Lot".into(),

            LotFeatures::Orchards => "Orchard(s)".into(),

            LotFeatures::Other => "Other".into(),

            LotFeatures::Pasture => "Pasture".into(),

            LotFeatures::Paved => "Paved".into(),

            LotFeatures::PieShapedLot => "Pie Shaped Lot".into(),

            LotFeatures::Private => "Private".into(),

            LotFeatures::RectangularLot => "Rectangular Lot".into(),

            LotFeatures::RockOutcropping => "Rock Outcropping".into(),

            LotFeatures::RollingSlope => "Rolling Slope".into(),

            LotFeatures::Secluded => "Secluded".into(),

            LotFeatures::SeeRemarks => "See Remarks".into(),

            LotFeatures::Sloped => "Sloped".into(),

            LotFeatures::SlopedDown => "Sloped Down".into(),

            LotFeatures::SlopedUp => "Sloped Up".into(),

            LotFeatures::SplitPossible => "Split Possible".into(),

            LotFeatures::SprinklersInFront => "Sprinklers In Front".into(),

            LotFeatures::SprinklersInRear => "Sprinklers In Rear".into(),

            LotFeatures::SteepSlope => "Steep Slope".into(),

            LotFeatures::Subdivided => "Subdivided".into(),

            LotFeatures::Views => "Views".into(),

            LotFeatures::Waterfall => "Waterfall".into(),

            LotFeatures::Waterfront => "Waterfront".into(),

            LotFeatures::Wetlands => "Wetlands".into(),

            LotFeatures::Wooded => "Wooded".into(),

            LotFeatures::ZeroLotLine => "Zero Lot Line".into(),

            LotFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            LotFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LotFeatures {
    fn from(s: String) -> LotFeatures {
        match s.as_ref() {
            "Agricultural" => LotFeatures::Agricultural,

            "Back Yard" => LotFeatures::BackYard,

            "Bluff" => LotFeatures::Bluff,

            "City Lot" => LotFeatures::CityLot,

            "Cleared" => LotFeatures::Cleared,

            "Close to Clubhouse" => LotFeatures::ClosetoClubhouse,

            "Corner Lot" => LotFeatures::CornerLot,

            "Corners Marked" => LotFeatures::CornersMarked,

            "Cul-De-Sac" => LotFeatures::CulDeSac,

            "Desert Back" => LotFeatures::DesertBack,

            "Desert Front" => LotFeatures::DesertFront,

            "Farm" => LotFeatures::Farm,

            "Few Trees" => LotFeatures::FewTrees,

            "Flag Lot" => LotFeatures::FlagLot,

            "Front Yard" => LotFeatures::FrontYard,

            "Garden" => LotFeatures::Garden,

            "Gentle Sloping" => LotFeatures::GentleSloping,

            "Greenbelt" => LotFeatures::Greenbelt,

            "Interior Lot" => LotFeatures::InteriorLot,

            "Irregular Lot" => LotFeatures::IrregularLot,

            "Landscaped" => LotFeatures::Landscaped,

            "Level" => LotFeatures::Level,

            "Many Trees" => LotFeatures::ManyTrees,

            "Meadow" => LotFeatures::Meadow,

            "Native Plants" => LotFeatures::NativePlants,

            "Near Golf Course" => LotFeatures::NearGolfCourse,

            "Near Public Transit" => LotFeatures::NearPublicTransit,

            "On Golf Course" => LotFeatures::OnGolfCourse,

            "Open Lot" => LotFeatures::OpenLot,

            "Orchard(s)" => LotFeatures::Orchards,

            "Other" => LotFeatures::Other,

            "Pasture" => LotFeatures::Pasture,

            "Paved" => LotFeatures::Paved,

            "Pie Shaped Lot" => LotFeatures::PieShapedLot,

            "Private" => LotFeatures::Private,

            "Rectangular Lot" => LotFeatures::RectangularLot,

            "Rock Outcropping" => LotFeatures::RockOutcropping,

            "Rolling Slope" => LotFeatures::RollingSlope,

            "Secluded" => LotFeatures::Secluded,

            "See Remarks" => LotFeatures::SeeRemarks,

            "Sloped" => LotFeatures::Sloped,

            "Sloped Down" => LotFeatures::SlopedDown,

            "Sloped Up" => LotFeatures::SlopedUp,

            "Split Possible" => LotFeatures::SplitPossible,

            "Sprinklers In Front" => LotFeatures::SprinklersInFront,

            "Sprinklers In Rear" => LotFeatures::SprinklersInRear,

            "Steep Slope" => LotFeatures::SteepSlope,

            "Subdivided" => LotFeatures::Subdivided,

            "Views" => LotFeatures::Views,

            "Waterfall" => LotFeatures::Waterfall,

            "Waterfront" => LotFeatures::Waterfront,

            "Wetlands" => LotFeatures::Wetlands,

            "Wooded" => LotFeatures::Wooded,

            "Zero Lot Line" => LotFeatures::ZeroLotLine,

            _ => LotFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LotFeatures {
    fn from(s: &str) -> LotFeatures {
        match s {
            "Agricultural" => LotFeatures::Agricultural,

            "Back Yard" => LotFeatures::BackYard,

            "Bluff" => LotFeatures::Bluff,

            "City Lot" => LotFeatures::CityLot,

            "Cleared" => LotFeatures::Cleared,

            "Close to Clubhouse" => LotFeatures::ClosetoClubhouse,

            "Corner Lot" => LotFeatures::CornerLot,

            "Corners Marked" => LotFeatures::CornersMarked,

            "Cul-De-Sac" => LotFeatures::CulDeSac,

            "Desert Back" => LotFeatures::DesertBack,

            "Desert Front" => LotFeatures::DesertFront,

            "Farm" => LotFeatures::Farm,

            "Few Trees" => LotFeatures::FewTrees,

            "Flag Lot" => LotFeatures::FlagLot,

            "Front Yard" => LotFeatures::FrontYard,

            "Garden" => LotFeatures::Garden,

            "Gentle Sloping" => LotFeatures::GentleSloping,

            "Greenbelt" => LotFeatures::Greenbelt,

            "Interior Lot" => LotFeatures::InteriorLot,

            "Irregular Lot" => LotFeatures::IrregularLot,

            "Landscaped" => LotFeatures::Landscaped,

            "Level" => LotFeatures::Level,

            "Many Trees" => LotFeatures::ManyTrees,

            "Meadow" => LotFeatures::Meadow,

            "Native Plants" => LotFeatures::NativePlants,

            "Near Golf Course" => LotFeatures::NearGolfCourse,

            "Near Public Transit" => LotFeatures::NearPublicTransit,

            "On Golf Course" => LotFeatures::OnGolfCourse,

            "Open Lot" => LotFeatures::OpenLot,

            "Orchard(s)" => LotFeatures::Orchards,

            "Other" => LotFeatures::Other,

            "Pasture" => LotFeatures::Pasture,

            "Paved" => LotFeatures::Paved,

            "Pie Shaped Lot" => LotFeatures::PieShapedLot,

            "Private" => LotFeatures::Private,

            "Rectangular Lot" => LotFeatures::RectangularLot,

            "Rock Outcropping" => LotFeatures::RockOutcropping,

            "Rolling Slope" => LotFeatures::RollingSlope,

            "Secluded" => LotFeatures::Secluded,

            "See Remarks" => LotFeatures::SeeRemarks,

            "Sloped" => LotFeatures::Sloped,

            "Sloped Down" => LotFeatures::SlopedDown,

            "Sloped Up" => LotFeatures::SlopedUp,

            "Split Possible" => LotFeatures::SplitPossible,

            "Sprinklers In Front" => LotFeatures::SprinklersInFront,

            "Sprinklers In Rear" => LotFeatures::SprinklersInRear,

            "Steep Slope" => LotFeatures::SteepSlope,

            "Subdivided" => LotFeatures::Subdivided,

            "Views" => LotFeatures::Views,

            "Waterfall" => LotFeatures::Waterfall,

            "Waterfront" => LotFeatures::Waterfront,

            "Wetlands" => LotFeatures::Wetlands,

            "Wooded" => LotFeatures::Wooded,

            "Zero Lot Line" => LotFeatures::ZeroLotLine,

            _ => LotFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LotFeatures> for &'a str {
    fn from(s: &'a LotFeatures) -> &'a str {
        match s {
            LotFeatures::Agricultural => "Agricultural",

            LotFeatures::BackYard => "Back Yard",

            LotFeatures::Bluff => "Bluff",

            LotFeatures::CityLot => "City Lot",

            LotFeatures::Cleared => "Cleared",

            LotFeatures::ClosetoClubhouse => "Close to Clubhouse",

            LotFeatures::CornerLot => "Corner Lot",

            LotFeatures::CornersMarked => "Corners Marked",

            LotFeatures::CulDeSac => "Cul-De-Sac",

            LotFeatures::DesertBack => "Desert Back",

            LotFeatures::DesertFront => "Desert Front",

            LotFeatures::Farm => "Farm",

            LotFeatures::FewTrees => "Few Trees",

            LotFeatures::FlagLot => "Flag Lot",

            LotFeatures::FrontYard => "Front Yard",

            LotFeatures::Garden => "Garden",

            LotFeatures::GentleSloping => "Gentle Sloping",

            LotFeatures::Greenbelt => "Greenbelt",

            LotFeatures::InteriorLot => "Interior Lot",

            LotFeatures::IrregularLot => "Irregular Lot",

            LotFeatures::Landscaped => "Landscaped",

            LotFeatures::Level => "Level",

            LotFeatures::ManyTrees => "Many Trees",

            LotFeatures::Meadow => "Meadow",

            LotFeatures::NativePlants => "Native Plants",

            LotFeatures::NearGolfCourse => "Near Golf Course",

            LotFeatures::NearPublicTransit => "Near Public Transit",

            LotFeatures::OnGolfCourse => "On Golf Course",

            LotFeatures::OpenLot => "Open Lot",

            LotFeatures::Orchards => "Orchard(s)",

            LotFeatures::Other => "Other",

            LotFeatures::Pasture => "Pasture",

            LotFeatures::Paved => "Paved",

            LotFeatures::PieShapedLot => "Pie Shaped Lot",

            LotFeatures::Private => "Private",

            LotFeatures::RectangularLot => "Rectangular Lot",

            LotFeatures::RockOutcropping => "Rock Outcropping",

            LotFeatures::RollingSlope => "Rolling Slope",

            LotFeatures::Secluded => "Secluded",

            LotFeatures::SeeRemarks => "See Remarks",

            LotFeatures::Sloped => "Sloped",

            LotFeatures::SlopedDown => "Sloped Down",

            LotFeatures::SlopedUp => "Sloped Up",

            LotFeatures::SplitPossible => "Split Possible",

            LotFeatures::SprinklersInFront => "Sprinklers In Front",

            LotFeatures::SprinklersInRear => "Sprinklers In Rear",

            LotFeatures::SteepSlope => "Steep Slope",

            LotFeatures::Subdivided => "Subdivided",

            LotFeatures::Views => "Views",

            LotFeatures::Waterfall => "Waterfall",

            LotFeatures::Waterfront => "Waterfront",

            LotFeatures::Wetlands => "Wetlands",

            LotFeatures::Wooded => "Wooded",

            LotFeatures::ZeroLotLine => "Zero Lot Line",

            LotFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LotFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LotFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
