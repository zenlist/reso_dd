// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [AccessibilityFeatures Lookups](https://ddwiki.reso.org/display/DDW17/AccessibilityFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AccessibilityFeatures {
    /// "[Accessible Approach with Ramp](https://ddwiki.reso.org/display/DDW17/Accessible+Approach+with+Ramp)": A minimum of one entrance to the structure with clear, evenly-paved walkway from parking area or pedestrian arrival area; Path of travel does not include a running slope in excess of 1:12 (8.33%); a cross slope exceeding 1:50 (2%); nor level changes of more than 1/2 inch; if slope is over 5%, handrails are required.  Level landing; 32 inch clear width opening doors; and adequate lighting on pathway and landing.  This is required to be considered "Visitable".
    AccessibleApproachwithRamp,

    /// "[Accessible Bedroom](https://ddwiki.reso.org/display/DDW17/Accessible+Bedroom)": Bedroom has adequate turnaround of 60 inches or other approved turnaround configuration; Closet doors have 32 inch clearance.  Accessible environmental controls. Multiple lighting fixtures.  This is required to be considered Enhanced Accessible.  Optional: Some lower-height storage in closet.   Remote control of lighting and environmental controls.
    AccessibleBedroom,

    /// "[Accessible Central Living Area](https://ddwiki.reso.org/display/DDW17/Accessible+Central+Living+Area)": Hard surface flooring or low-pile carpet, securely attached along edges. Entrances to all rooms on all floors provide 36 inch clear passage. Multiple lighting fixtures installed to provide adaptable lighting for general purpose and tasks.  This is required to be considered Enhanced Accessible.
    AccessibleCentralLivingArea,

    /// "[Accessible Closets](https://ddwiki.reso.org/display/DDW17/Accessible+Closets)": Closet doors are 32” clearance throughout Central Living Area.
    AccessibleClosets,

    /// "[Accessible Common Area](https://ddwiki.reso.org/display/DDW17/Accessible+Common+Area)": Common Area, used for entertaining guests, is level, with 36-inch passage through and around the space.  Required to be considered Visitable.
    AccessibleCommonArea,

    /// "[Accessible Doors](https://ddwiki.reso.org/display/DDW17/Accessible+Doors)": Minimum 32 inches clear passage; levered handle; threshold, if present, maximum 1/2-inch, but beveled on both sides when over 1/4 inch. Required to be considered Visitable or Enhanced Accessible.
    AccessibleDoors,

    /// "[Accessible Electrical and Environmental Controls](https://ddwiki.reso.org/display/DDW17/Accessible+Electrical+and+Environmental+Controls)": Thermostats and security system controls located on floor with central living area. Control devices  for light switches and thermostats at 42-48 inches height off floor and side- to-side. Electrical plugs minimum of 18 inches above floor. This is required to be considered Enhanced Accessible.  Optional: Rocker-style light switches; Lighted switches; Automatic/remote control for environmental controls.
    AccessibleElectricalandEnvironmentalControls,

    /// "[Accessible Elevator Installed](https://ddwiki.reso.org/display/DDW17/Accessible+Elevator+Installed)": Elevator with minimum 32" door and minimum  36" x 48" turning radius.
    AccessibleElevatorInstalled,

    /// "[Accessible Entrance](https://ddwiki.reso.org/display/DDW17/Accessible+Entrance)": Entrance door is a minimum of 32 inches wide; threshold , when present,  maximum 1/2-inch, but when over 1/4-inch, is beveled on both sides. The entry door has lever handle for egress.   This is required to be considered Visitable.  Optional: Entryway is covered; Bench near outside entry door; Motion-detection outside lights. Accessible peephole or other method for inside viewing of anyone outside the entry door.  House number easily visible from street by emergency responders.
    AccessibleEntrance,

    /// "[Accessible for Hearing-Impairment](https://ddwiki.reso.org/display/DDW17/Accessible+for+Hearing-Impairment)": Home is wired for  flashing lights and/or vibrating smoke alarm, door bell, other alerting  features.
    AccessibleforHearingImpairment,

    /// "[Accessible Full Bath](https://ddwiki.reso.org/display/DDW17/Accessible+Full+Bath)": Bathroom has adequate turnaround: 60 inches or other approved turnaround configuration.  Accessible bathing area:  Roll-in shower or space for shower chair or transfer bench in bathtub. Slip-resistant surface on bathroom floor and bathtub/shower.  Roll-under or side-access to sink, with padded trap. Lighting directly over shower/bathing areas, in addition to general bathroom lighting.  This is required to be considered Enhanced Accessible.  Optional: Removable cabinet doors and base under sink.  Grab bars with shear force of 250 pounds, installed around toilet and shower/tub, with proper backing.  Hand-held and/or height-adjustable shower head; automatic water temperature controlled (anti-scald) tub/shower; offset tub/shower controls toward front edge of tub/shower for easy access.   Raised toilet (17-19 inches minimum).
    AccessibleFullBath,

    /// "[Accessible Hallway(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243658)": Hallway is minimum 36, preferred 42 inches wide  (or adequate alternative based on individual configuration). At least one lighting fixture is present.
    AccessibleHallways,

    /// "[Accessible Kitchen](https://ddwiki.reso.org/display/DDW17/Accessible+Kitchen)": 40" clear turn-around, or 36" clear with clear under-counter space for T-turn space in kitchen, unimpeded by fixtures.  Roll-under/adaptable sink with padded trap or side-access to sink. Roll-out shelves in at least 50% of lower cabinets, Roll-under work area (36" clearance). Lighting fixtures directed over all task areas with adaptable control.  This is required to be considered Enhanced Accessible.  Optional:  Adaptable under-sink cabinet with removable doors and cabinet base.  Accessible features for upper kitchen cabinets.  Space for side-by-side refrigerator (minimum 36 inches);  Contrasting color counter edges for vison accessibility.
    AccessibleKitchen,

    /// "[Accessible Kitchen Appliances](https://ddwiki.reso.org/display/DDW17/Accessible+Kitchen+Appliances)": Stove controls in front or side, at counter top height; Oven with side-access door at counter level; Microwave is at counter level.  This is required to be considered Enhanced Accessible.  Optional:  Raised dishwasher.  Microwave  has clear work area below or to the right side.
    AccessibleKitchenAppliances,

    /// "[Accessible Stairway](https://ddwiki.reso.org/display/DDW17/Accessible+Stairway)": Handrails on both sides of stairs, extended when possible, with shear force of 250 pounds. Interior and exterior stairs have adequate number of light fixtures for full coverage. Non-slip stair treads.  If stairs are carpeted, covering is non-moveable, low-pile carpet.  This is required to be considered Enhanced Accessible.  Optional: Stair treads are in high contrast colors for increased visibility.
    AccessibleStairway,

    /// "[Accessible Washer/Dryer](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243662)": Raised clothes washer and/or dryer, front controls, side opening, both open to center.
    AccessibleWasherDryer,

    /// "[Adaptable Bathroom Walls](https://ddwiki.reso.org/display/DDW17/Adaptable+Bathroom+Walls)": Reinforced main bathroom walls, including bath or shower, to permit installation of grab bars (with shear force of 250 pounds) and/or fixtures in the future.   This is required to be considered Enhanced Accessible.
    AdaptableBathroomWalls,

    /// "[Adaptable For Elevator](https://ddwiki.reso.org/display/DDW17/Adaptable+For+Elevator)": Stacked closets in a multi-story house for possible future conversion to an elevator.
    AdaptableForElevator,

    /// "[Ceiling Track](https://ddwiki.reso.org/display/DDW17/Ceiling+Track)": Track installed in ceiling for lift chair (Hoyer lift).
    CeilingTrack,

    /// "[Central Living Area](https://ddwiki.reso.org/display/DDW17/Central+Living+Area)": Central Living Area includes: Common Area, hallway(s), full or 3/4 bathroom,  kitchen, at least one bedroom, access to environmental controls, and access to floors above main floor, if necessary.
    CentralLivingArea,

    /// "[Common Area](https://ddwiki.reso.org/display/DDW17/Common+Area)": The Common Area is the portion of the home near accessible entrance, used for entertaining guests.
    CommonArea,

    /// "[Customized Wheelchair Accessible](https://ddwiki.reso.org/display/DDW17/Customized+Wheelchair+Accessible)": Customized accessibility for specific size or style of wheelchair or scooter.
    CustomizedWheelchairAccessible,

    /// "[Electronic Environmental Controls](https://ddwiki.reso.org/display/DDW17/Electronic+Environmental+Controls)": Programmable electronic controls for thermostat, lights, security system and automatic doors.
    ElectronicEnvironmentalControls,

    /// "[Enhanced Accessible](https://ddwiki.reso.org/display/DDW17/Enhanced+Accessible)": The Central Living Area is fully accessible for lifelong living by all residents, no matter their ability. A person in a wheel chair or with other disability is able to  perform all personal and housekeeping tasks.  Persons without disabilities are also able to perform basic tasks with greater ease.  To be considered "Enhanced Accessible" the home also includes all "Visitable" features.
    EnhancedAccessible,

    /// "[Exterior Wheelchair Lift](https://ddwiki.reso.org/display/DDW17/Exterior+Wheelchair+Lift)": Mechanical wheelchair lift is installed outside the home to facilitate barrier-free approach.
    ExteriorWheelchairLift,

    /// "[Grip-Accessible Features](https://ddwiki.reso.org/display/DDW17/Grip-Accessible+Features)": All doors, faucets and other mechanisms throughout central living area are lever, hands-free or other style that can be  controlled with a closed, clenched fist or weak hands.  This is required to be considered Enhanced Accessible.  Optional:  Wire pull (D-ring) handles or equivalent or easy touch latches on cabinets and drawers.
    GripAccessibleFeatures,

    /// "[Reinforced Floors](https://ddwiki.reso.org/display/DDW17/Reinforced+Floors)": Reinforced floors for bariatric needs, power wheelchairs, therapeutic tub or heavy medical equipment.
    ReinforcedFloors,

    /// "[Safe Emergency Egress from Home](https://ddwiki.reso.org/display/DDW17/Safe+Emergency+Egress+from+Home)": Minimum two, no-step, accessible egresses from Central Living Area.  Window locks are  19 to 54 inches from the floor and can be opened with a closed fist. Emergency egress windows in sleeping areas require minimal effort to open.  This is required to be considered Enhanced Accessible.
    SafeEmergencyEgressfromHome,

    /// "[Smart Technology](https://ddwiki.reso.org/display/DDW17/Smart+Technology)": Smart Home (computer-controlled) and/or smart products— for example, voice activated controls, voice reminder, remote monitoring of individuals with dementia.
    SmartTechnology,

    /// "[Stair Lift](https://ddwiki.reso.org/display/DDW17/Stair+Lift)": Stair lift with motorized rail to climb interior or exterior stairway installed professionally.
    StairLift,

    /// "[Standby Generator](https://ddwiki.reso.org/display/DDW17/Standby+Generator)": Backup generator for refrigeration of medications, life-sustaining medical  equipment or essential room temperature control.
    StandbyGenerator,

    /// "[Therapeutic Whirlpool](https://ddwiki.reso.org/display/DDW17/Therapeutic+Whirlpool)": Therapeutic whirlpool, properly installed.
    TherapeuticWhirlpool,

    /// "[Visitable](https://ddwiki.reso.org/display/DDW17/Visitable)": The home is “visitable” for all guests: a person in a wheel chair can easily enter the home and  access the main Common Area, a half-bathroom at minimum, and the hall leading to that bathroom.
    Visitable,

    /// "[Visitor Bathroom](https://ddwiki.reso.org/display/DDW17/Visitor+Bathroom)": Bathroom that is closest to Common Area. Minimum half bath. Door has minimum 32-inch clear width opening; lever handles. Minimum 60-inch turnaround or other approved turnaround configuration (30" x 48" clear space if door opens out.). Grab bar installed in toilet area with proper blocking. This is required to be considered Visitable.
    VisitorBathroom,

    /// "[Walker-Accessible Stairs](https://ddwiki.reso.org/display/DDW17/Walker-Accessible+Stairs)": Treads are minimum 25 inches deep, with maximum 4 inches rise, minimum 36 inches wide.  May be a custom feature in addition to approach with ramp.
    WalkerAccessibleStairs,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for AccessibilityFeatures {
    fn from_str(s: &str) -> AccessibilityFeatures {
        match s {
            "Accessible Approach with Ramp" => AccessibilityFeatures::AccessibleApproachwithRamp,

            "Accessible Bedroom" => AccessibilityFeatures::AccessibleBedroom,

            "Accessible Central Living Area" => AccessibilityFeatures::AccessibleCentralLivingArea,

            "Accessible Closets" => AccessibilityFeatures::AccessibleClosets,

            "Accessible Common Area" => AccessibilityFeatures::AccessibleCommonArea,

            "Accessible Doors" => AccessibilityFeatures::AccessibleDoors,

            "Accessible Electrical and Environmental Controls" => {
                AccessibilityFeatures::AccessibleElectricalandEnvironmentalControls
            }

            "Accessible Elevator Installed" => AccessibilityFeatures::AccessibleElevatorInstalled,

            "Accessible Entrance" => AccessibilityFeatures::AccessibleEntrance,

            "Accessible for Hearing-Impairment" => {
                AccessibilityFeatures::AccessibleforHearingImpairment
            }

            "Accessible Full Bath" => AccessibilityFeatures::AccessibleFullBath,

            "Accessible Hallway(s)" => AccessibilityFeatures::AccessibleHallways,

            "Accessible Kitchen" => AccessibilityFeatures::AccessibleKitchen,

            "Accessible Kitchen Appliances" => AccessibilityFeatures::AccessibleKitchenAppliances,

            "Accessible Stairway" => AccessibilityFeatures::AccessibleStairway,

            "Accessible Washer/Dryer" => AccessibilityFeatures::AccessibleWasherDryer,

            "Adaptable Bathroom Walls" => AccessibilityFeatures::AdaptableBathroomWalls,

            "Adaptable For Elevator" => AccessibilityFeatures::AdaptableForElevator,

            "Ceiling Track" => AccessibilityFeatures::CeilingTrack,

            "Central Living Area" => AccessibilityFeatures::CentralLivingArea,

            "Common Area" => AccessibilityFeatures::CommonArea,

            "Customized Wheelchair Accessible" => {
                AccessibilityFeatures::CustomizedWheelchairAccessible
            }

            "Electronic Environmental Controls" => {
                AccessibilityFeatures::ElectronicEnvironmentalControls
            }

            "Enhanced Accessible" => AccessibilityFeatures::EnhancedAccessible,

            "Exterior Wheelchair Lift" => AccessibilityFeatures::ExteriorWheelchairLift,

            "Grip-Accessible Features" => AccessibilityFeatures::GripAccessibleFeatures,

            "Reinforced Floors" => AccessibilityFeatures::ReinforcedFloors,

            "Safe Emergency Egress from Home" => AccessibilityFeatures::SafeEmergencyEgressfromHome,

            "Smart Technology" => AccessibilityFeatures::SmartTechnology,

            "Stair Lift" => AccessibilityFeatures::StairLift,

            "Standby Generator" => AccessibilityFeatures::StandbyGenerator,

            "Therapeutic Whirlpool" => AccessibilityFeatures::TherapeuticWhirlpool,

            "Visitable" => AccessibilityFeatures::Visitable,

            "Visitor Bathroom" => AccessibilityFeatures::VisitorBathroom,

            "Walker-Accessible Stairs" => AccessibilityFeatures::WalkerAccessibleStairs,

            _ => AccessibilityFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> AccessibilityFeatures {
        match s.as_ref() {
            "Accessible Approach with Ramp" => AccessibilityFeatures::AccessibleApproachwithRamp,

            "Accessible Bedroom" => AccessibilityFeatures::AccessibleBedroom,

            "Accessible Central Living Area" => AccessibilityFeatures::AccessibleCentralLivingArea,

            "Accessible Closets" => AccessibilityFeatures::AccessibleClosets,

            "Accessible Common Area" => AccessibilityFeatures::AccessibleCommonArea,

            "Accessible Doors" => AccessibilityFeatures::AccessibleDoors,

            "Accessible Electrical and Environmental Controls" => {
                AccessibilityFeatures::AccessibleElectricalandEnvironmentalControls
            }

            "Accessible Elevator Installed" => AccessibilityFeatures::AccessibleElevatorInstalled,

            "Accessible Entrance" => AccessibilityFeatures::AccessibleEntrance,

            "Accessible for Hearing-Impairment" => {
                AccessibilityFeatures::AccessibleforHearingImpairment
            }

            "Accessible Full Bath" => AccessibilityFeatures::AccessibleFullBath,

            "Accessible Hallway(s)" => AccessibilityFeatures::AccessibleHallways,

            "Accessible Kitchen" => AccessibilityFeatures::AccessibleKitchen,

            "Accessible Kitchen Appliances" => AccessibilityFeatures::AccessibleKitchenAppliances,

            "Accessible Stairway" => AccessibilityFeatures::AccessibleStairway,

            "Accessible Washer/Dryer" => AccessibilityFeatures::AccessibleWasherDryer,

            "Adaptable Bathroom Walls" => AccessibilityFeatures::AdaptableBathroomWalls,

            "Adaptable For Elevator" => AccessibilityFeatures::AdaptableForElevator,

            "Ceiling Track" => AccessibilityFeatures::CeilingTrack,

            "Central Living Area" => AccessibilityFeatures::CentralLivingArea,

            "Common Area" => AccessibilityFeatures::CommonArea,

            "Customized Wheelchair Accessible" => {
                AccessibilityFeatures::CustomizedWheelchairAccessible
            }

            "Electronic Environmental Controls" => {
                AccessibilityFeatures::ElectronicEnvironmentalControls
            }

            "Enhanced Accessible" => AccessibilityFeatures::EnhancedAccessible,

            "Exterior Wheelchair Lift" => AccessibilityFeatures::ExteriorWheelchairLift,

            "Grip-Accessible Features" => AccessibilityFeatures::GripAccessibleFeatures,

            "Reinforced Floors" => AccessibilityFeatures::ReinforcedFloors,

            "Safe Emergency Egress from Home" => AccessibilityFeatures::SafeEmergencyEgressfromHome,

            "Smart Technology" => AccessibilityFeatures::SmartTechnology,

            "Stair Lift" => AccessibilityFeatures::StairLift,

            "Standby Generator" => AccessibilityFeatures::StandbyGenerator,

            "Therapeutic Whirlpool" => AccessibilityFeatures::TherapeuticWhirlpool,

            "Visitable" => AccessibilityFeatures::Visitable,

            "Visitor Bathroom" => AccessibilityFeatures::VisitorBathroom,

            "Walker-Accessible Stairs" => AccessibilityFeatures::WalkerAccessibleStairs,

            _ => AccessibilityFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            AccessibilityFeatures::AccessibleApproachwithRamp => "Accessible Approach with Ramp",

            AccessibilityFeatures::AccessibleBedroom => "Accessible Bedroom",

            AccessibilityFeatures::AccessibleCentralLivingArea => "Accessible Central Living Area",

            AccessibilityFeatures::AccessibleClosets => "Accessible Closets",

            AccessibilityFeatures::AccessibleCommonArea => "Accessible Common Area",

            AccessibilityFeatures::AccessibleDoors => "Accessible Doors",

            AccessibilityFeatures::AccessibleElectricalandEnvironmentalControls => {
                "Accessible Electrical and Environmental Controls"
            }

            AccessibilityFeatures::AccessibleElevatorInstalled => "Accessible Elevator Installed",

            AccessibilityFeatures::AccessibleEntrance => "Accessible Entrance",

            AccessibilityFeatures::AccessibleforHearingImpairment => {
                "Accessible for Hearing-Impairment"
            }

            AccessibilityFeatures::AccessibleFullBath => "Accessible Full Bath",

            AccessibilityFeatures::AccessibleHallways => "Accessible Hallway(s)",

            AccessibilityFeatures::AccessibleKitchen => "Accessible Kitchen",

            AccessibilityFeatures::AccessibleKitchenAppliances => "Accessible Kitchen Appliances",

            AccessibilityFeatures::AccessibleStairway => "Accessible Stairway",

            AccessibilityFeatures::AccessibleWasherDryer => "Accessible Washer/Dryer",

            AccessibilityFeatures::AdaptableBathroomWalls => "Adaptable Bathroom Walls",

            AccessibilityFeatures::AdaptableForElevator => "Adaptable For Elevator",

            AccessibilityFeatures::CeilingTrack => "Ceiling Track",

            AccessibilityFeatures::CentralLivingArea => "Central Living Area",

            AccessibilityFeatures::CommonArea => "Common Area",

            AccessibilityFeatures::CustomizedWheelchairAccessible => {
                "Customized Wheelchair Accessible"
            }

            AccessibilityFeatures::ElectronicEnvironmentalControls => {
                "Electronic Environmental Controls"
            }

            AccessibilityFeatures::EnhancedAccessible => "Enhanced Accessible",

            AccessibilityFeatures::ExteriorWheelchairLift => "Exterior Wheelchair Lift",

            AccessibilityFeatures::GripAccessibleFeatures => "Grip-Accessible Features",

            AccessibilityFeatures::ReinforcedFloors => "Reinforced Floors",

            AccessibilityFeatures::SafeEmergencyEgressfromHome => "Safe Emergency Egress from Home",

            AccessibilityFeatures::SmartTechnology => "Smart Technology",

            AccessibilityFeatures::StairLift => "Stair Lift",

            AccessibilityFeatures::StandbyGenerator => "Standby Generator",

            AccessibilityFeatures::TherapeuticWhirlpool => "Therapeutic Whirlpool",

            AccessibilityFeatures::Visitable => "Visitable",

            AccessibilityFeatures::VisitorBathroom => "Visitor Bathroom",

            AccessibilityFeatures::WalkerAccessibleStairs => "Walker-Accessible Stairs",

            AccessibilityFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            AccessibilityFeatures::AccessibleApproachwithRamp => {
                "Accessible Approach with Ramp".into()
            }

            AccessibilityFeatures::AccessibleBedroom => "Accessible Bedroom".into(),

            AccessibilityFeatures::AccessibleCentralLivingArea => {
                "Accessible Central Living Area".into()
            }

            AccessibilityFeatures::AccessibleClosets => "Accessible Closets".into(),

            AccessibilityFeatures::AccessibleCommonArea => "Accessible Common Area".into(),

            AccessibilityFeatures::AccessibleDoors => "Accessible Doors".into(),

            AccessibilityFeatures::AccessibleElectricalandEnvironmentalControls => {
                "Accessible Electrical and Environmental Controls".into()
            }

            AccessibilityFeatures::AccessibleElevatorInstalled => {
                "Accessible Elevator Installed".into()
            }

            AccessibilityFeatures::AccessibleEntrance => "Accessible Entrance".into(),

            AccessibilityFeatures::AccessibleforHearingImpairment => {
                "Accessible for Hearing-Impairment".into()
            }

            AccessibilityFeatures::AccessibleFullBath => "Accessible Full Bath".into(),

            AccessibilityFeatures::AccessibleHallways => "Accessible Hallway(s)".into(),

            AccessibilityFeatures::AccessibleKitchen => "Accessible Kitchen".into(),

            AccessibilityFeatures::AccessibleKitchenAppliances => {
                "Accessible Kitchen Appliances".into()
            }

            AccessibilityFeatures::AccessibleStairway => "Accessible Stairway".into(),

            AccessibilityFeatures::AccessibleWasherDryer => "Accessible Washer/Dryer".into(),

            AccessibilityFeatures::AdaptableBathroomWalls => "Adaptable Bathroom Walls".into(),

            AccessibilityFeatures::AdaptableForElevator => "Adaptable For Elevator".into(),

            AccessibilityFeatures::CeilingTrack => "Ceiling Track".into(),

            AccessibilityFeatures::CentralLivingArea => "Central Living Area".into(),

            AccessibilityFeatures::CommonArea => "Common Area".into(),

            AccessibilityFeatures::CustomizedWheelchairAccessible => {
                "Customized Wheelchair Accessible".into()
            }

            AccessibilityFeatures::ElectronicEnvironmentalControls => {
                "Electronic Environmental Controls".into()
            }

            AccessibilityFeatures::EnhancedAccessible => "Enhanced Accessible".into(),

            AccessibilityFeatures::ExteriorWheelchairLift => "Exterior Wheelchair Lift".into(),

            AccessibilityFeatures::GripAccessibleFeatures => "Grip-Accessible Features".into(),

            AccessibilityFeatures::ReinforcedFloors => "Reinforced Floors".into(),

            AccessibilityFeatures::SafeEmergencyEgressfromHome => {
                "Safe Emergency Egress from Home".into()
            }

            AccessibilityFeatures::SmartTechnology => "Smart Technology".into(),

            AccessibilityFeatures::StairLift => "Stair Lift".into(),

            AccessibilityFeatures::StandbyGenerator => "Standby Generator".into(),

            AccessibilityFeatures::TherapeuticWhirlpool => "Therapeutic Whirlpool".into(),

            AccessibilityFeatures::Visitable => "Visitable".into(),

            AccessibilityFeatures::VisitorBathroom => "Visitor Bathroom".into(),

            AccessibilityFeatures::WalkerAccessibleStairs => "Walker-Accessible Stairs".into(),

            AccessibilityFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            AccessibilityFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for AccessibilityFeatures {
    fn from(s: String) -> AccessibilityFeatures {
        match s.as_ref() {
            "Accessible Approach with Ramp" => AccessibilityFeatures::AccessibleApproachwithRamp,

            "Accessible Bedroom" => AccessibilityFeatures::AccessibleBedroom,

            "Accessible Central Living Area" => AccessibilityFeatures::AccessibleCentralLivingArea,

            "Accessible Closets" => AccessibilityFeatures::AccessibleClosets,

            "Accessible Common Area" => AccessibilityFeatures::AccessibleCommonArea,

            "Accessible Doors" => AccessibilityFeatures::AccessibleDoors,

            "Accessible Electrical and Environmental Controls" => {
                AccessibilityFeatures::AccessibleElectricalandEnvironmentalControls
            }

            "Accessible Elevator Installed" => AccessibilityFeatures::AccessibleElevatorInstalled,

            "Accessible Entrance" => AccessibilityFeatures::AccessibleEntrance,

            "Accessible for Hearing-Impairment" => {
                AccessibilityFeatures::AccessibleforHearingImpairment
            }

            "Accessible Full Bath" => AccessibilityFeatures::AccessibleFullBath,

            "Accessible Hallway(s)" => AccessibilityFeatures::AccessibleHallways,

            "Accessible Kitchen" => AccessibilityFeatures::AccessibleKitchen,

            "Accessible Kitchen Appliances" => AccessibilityFeatures::AccessibleKitchenAppliances,

            "Accessible Stairway" => AccessibilityFeatures::AccessibleStairway,

            "Accessible Washer/Dryer" => AccessibilityFeatures::AccessibleWasherDryer,

            "Adaptable Bathroom Walls" => AccessibilityFeatures::AdaptableBathroomWalls,

            "Adaptable For Elevator" => AccessibilityFeatures::AdaptableForElevator,

            "Ceiling Track" => AccessibilityFeatures::CeilingTrack,

            "Central Living Area" => AccessibilityFeatures::CentralLivingArea,

            "Common Area" => AccessibilityFeatures::CommonArea,

            "Customized Wheelchair Accessible" => {
                AccessibilityFeatures::CustomizedWheelchairAccessible
            }

            "Electronic Environmental Controls" => {
                AccessibilityFeatures::ElectronicEnvironmentalControls
            }

            "Enhanced Accessible" => AccessibilityFeatures::EnhancedAccessible,

            "Exterior Wheelchair Lift" => AccessibilityFeatures::ExteriorWheelchairLift,

            "Grip-Accessible Features" => AccessibilityFeatures::GripAccessibleFeatures,

            "Reinforced Floors" => AccessibilityFeatures::ReinforcedFloors,

            "Safe Emergency Egress from Home" => AccessibilityFeatures::SafeEmergencyEgressfromHome,

            "Smart Technology" => AccessibilityFeatures::SmartTechnology,

            "Stair Lift" => AccessibilityFeatures::StairLift,

            "Standby Generator" => AccessibilityFeatures::StandbyGenerator,

            "Therapeutic Whirlpool" => AccessibilityFeatures::TherapeuticWhirlpool,

            "Visitable" => AccessibilityFeatures::Visitable,

            "Visitor Bathroom" => AccessibilityFeatures::VisitorBathroom,

            "Walker-Accessible Stairs" => AccessibilityFeatures::WalkerAccessibleStairs,

            _ => AccessibilityFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for AccessibilityFeatures {
    fn from(s: &str) -> AccessibilityFeatures {
        match s {
            "Accessible Approach with Ramp" => AccessibilityFeatures::AccessibleApproachwithRamp,

            "Accessible Bedroom" => AccessibilityFeatures::AccessibleBedroom,

            "Accessible Central Living Area" => AccessibilityFeatures::AccessibleCentralLivingArea,

            "Accessible Closets" => AccessibilityFeatures::AccessibleClosets,

            "Accessible Common Area" => AccessibilityFeatures::AccessibleCommonArea,

            "Accessible Doors" => AccessibilityFeatures::AccessibleDoors,

            "Accessible Electrical and Environmental Controls" => {
                AccessibilityFeatures::AccessibleElectricalandEnvironmentalControls
            }

            "Accessible Elevator Installed" => AccessibilityFeatures::AccessibleElevatorInstalled,

            "Accessible Entrance" => AccessibilityFeatures::AccessibleEntrance,

            "Accessible for Hearing-Impairment" => {
                AccessibilityFeatures::AccessibleforHearingImpairment
            }

            "Accessible Full Bath" => AccessibilityFeatures::AccessibleFullBath,

            "Accessible Hallway(s)" => AccessibilityFeatures::AccessibleHallways,

            "Accessible Kitchen" => AccessibilityFeatures::AccessibleKitchen,

            "Accessible Kitchen Appliances" => AccessibilityFeatures::AccessibleKitchenAppliances,

            "Accessible Stairway" => AccessibilityFeatures::AccessibleStairway,

            "Accessible Washer/Dryer" => AccessibilityFeatures::AccessibleWasherDryer,

            "Adaptable Bathroom Walls" => AccessibilityFeatures::AdaptableBathroomWalls,

            "Adaptable For Elevator" => AccessibilityFeatures::AdaptableForElevator,

            "Ceiling Track" => AccessibilityFeatures::CeilingTrack,

            "Central Living Area" => AccessibilityFeatures::CentralLivingArea,

            "Common Area" => AccessibilityFeatures::CommonArea,

            "Customized Wheelchair Accessible" => {
                AccessibilityFeatures::CustomizedWheelchairAccessible
            }

            "Electronic Environmental Controls" => {
                AccessibilityFeatures::ElectronicEnvironmentalControls
            }

            "Enhanced Accessible" => AccessibilityFeatures::EnhancedAccessible,

            "Exterior Wheelchair Lift" => AccessibilityFeatures::ExteriorWheelchairLift,

            "Grip-Accessible Features" => AccessibilityFeatures::GripAccessibleFeatures,

            "Reinforced Floors" => AccessibilityFeatures::ReinforcedFloors,

            "Safe Emergency Egress from Home" => AccessibilityFeatures::SafeEmergencyEgressfromHome,

            "Smart Technology" => AccessibilityFeatures::SmartTechnology,

            "Stair Lift" => AccessibilityFeatures::StairLift,

            "Standby Generator" => AccessibilityFeatures::StandbyGenerator,

            "Therapeutic Whirlpool" => AccessibilityFeatures::TherapeuticWhirlpool,

            "Visitable" => AccessibilityFeatures::Visitable,

            "Visitor Bathroom" => AccessibilityFeatures::VisitorBathroom,

            "Walker-Accessible Stairs" => AccessibilityFeatures::WalkerAccessibleStairs,

            _ => AccessibilityFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a AccessibilityFeatures> for &'a str {
    fn from(s: &'a AccessibilityFeatures) -> &'a str {
        match s {
            AccessibilityFeatures::AccessibleApproachwithRamp => "Accessible Approach with Ramp",

            AccessibilityFeatures::AccessibleBedroom => "Accessible Bedroom",

            AccessibilityFeatures::AccessibleCentralLivingArea => "Accessible Central Living Area",

            AccessibilityFeatures::AccessibleClosets => "Accessible Closets",

            AccessibilityFeatures::AccessibleCommonArea => "Accessible Common Area",

            AccessibilityFeatures::AccessibleDoors => "Accessible Doors",

            AccessibilityFeatures::AccessibleElectricalandEnvironmentalControls => {
                "Accessible Electrical and Environmental Controls"
            }

            AccessibilityFeatures::AccessibleElevatorInstalled => "Accessible Elevator Installed",

            AccessibilityFeatures::AccessibleEntrance => "Accessible Entrance",

            AccessibilityFeatures::AccessibleforHearingImpairment => {
                "Accessible for Hearing-Impairment"
            }

            AccessibilityFeatures::AccessibleFullBath => "Accessible Full Bath",

            AccessibilityFeatures::AccessibleHallways => "Accessible Hallway(s)",

            AccessibilityFeatures::AccessibleKitchen => "Accessible Kitchen",

            AccessibilityFeatures::AccessibleKitchenAppliances => "Accessible Kitchen Appliances",

            AccessibilityFeatures::AccessibleStairway => "Accessible Stairway",

            AccessibilityFeatures::AccessibleWasherDryer => "Accessible Washer/Dryer",

            AccessibilityFeatures::AdaptableBathroomWalls => "Adaptable Bathroom Walls",

            AccessibilityFeatures::AdaptableForElevator => "Adaptable For Elevator",

            AccessibilityFeatures::CeilingTrack => "Ceiling Track",

            AccessibilityFeatures::CentralLivingArea => "Central Living Area",

            AccessibilityFeatures::CommonArea => "Common Area",

            AccessibilityFeatures::CustomizedWheelchairAccessible => {
                "Customized Wheelchair Accessible"
            }

            AccessibilityFeatures::ElectronicEnvironmentalControls => {
                "Electronic Environmental Controls"
            }

            AccessibilityFeatures::EnhancedAccessible => "Enhanced Accessible",

            AccessibilityFeatures::ExteriorWheelchairLift => "Exterior Wheelchair Lift",

            AccessibilityFeatures::GripAccessibleFeatures => "Grip-Accessible Features",

            AccessibilityFeatures::ReinforcedFloors => "Reinforced Floors",

            AccessibilityFeatures::SafeEmergencyEgressfromHome => "Safe Emergency Egress from Home",

            AccessibilityFeatures::SmartTechnology => "Smart Technology",

            AccessibilityFeatures::StairLift => "Stair Lift",

            AccessibilityFeatures::StandbyGenerator => "Standby Generator",

            AccessibilityFeatures::TherapeuticWhirlpool => "Therapeutic Whirlpool",

            AccessibilityFeatures::Visitable => "Visitable",

            AccessibilityFeatures::VisitorBathroom => "Visitor Bathroom",

            AccessibilityFeatures::WalkerAccessibleStairs => "Walker-Accessible Stairs",

            AccessibilityFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for AccessibilityFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AccessibilityFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
