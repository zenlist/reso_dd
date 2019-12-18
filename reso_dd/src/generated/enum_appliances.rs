// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Appliances Lookups](https://ddwiki.reso.org/display/DDW17/Appliances+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Appliances {
    /// "[Bar Fridge](https://ddwiki.reso.org/display/DDW17/Bar+Fridge)": A refrigerator that is sized and/or built to be part of a bar.
    BarFridge,

    /// "[Built-In Electric Oven](https://ddwiki.reso.org/display/DDW17/Built-In+Electric+Oven)": A built-in electric oven.
    BuiltInElectricOven,

    /// "[Built-In Electric Range](https://ddwiki.reso.org/display/DDW17/Built-In+Electric+Range)": A built-in electric range.
    BuiltInElectricRange,

    /// "[Built-In Freezer](https://ddwiki.reso.org/display/DDW17/Built-In+Freezer)": A built-in freezer.
    BuiltInFreezer,

    /// "[Built-In Gas Oven](https://ddwiki.reso.org/display/DDW17/Built-In+Gas+Oven)": A built-in gas oven
    BuiltInGasOven,

    /// "[Built-In Gas Range](https://ddwiki.reso.org/display/DDW17/Built-In+Gas+Range)": A built-in gas range.
    BuiltInGasRange,

    /// "[Built-In Range](https://ddwiki.reso.org/display/DDW17/Built-In+Range)": A built-in range where the fuel type is not specified.
    BuiltInRange,

    /// "[Built-In Refrigerator](https://ddwiki.reso.org/display/DDW17/Built-In+Refrigerator)": A built-in refrigerator
    BuiltInRefrigerator,

    /// "[Convection Oven](https://ddwiki.reso.org/display/DDW17/Convection+Oven)": A convection oven (also known as a fan-assisted oven or simply a fan oven) is an oven that has fans to circulate air around food.
    ConvectionOven,

    /// "[Cooktop](https://ddwiki.reso.org/display/DDW17/Cooktop)": A kitchen stove, often called simply a stove or a cooker, is a kitchen appliance designed for the purpose of cooking food. Kitchen stoves rely on the application of direct heat for the cooking process.
    Cooktop,

    /// "[Dishwasher](https://ddwiki.reso.org/display/DDW17/Dishwasher)": A dishwasher is a mechanical device for cleaning dishware and cutlery.
    Dishwasher,

    /// "[Disposal](https://ddwiki.reso.org/display/DDW17/Disposal)": A garbage disposal unit (also known as a garbage disposal, waste disposal unit, garbage disposer, or in Canadian English a garburator) is a device, usually electrically powered, installed under a kitchen sink between the sink's drain and the trap. The disposal unit shreds food waste into pieces small enough, generally less than 2 mm (0.079 in), to pass through plumbing.
    Disposal,

    /// "[Double Oven](https://ddwiki.reso.org/display/DDW17/Double+Oven)": A built-in oven fixture that has either two ovens, or one oven and one microwave oven. It is usually built into the kitchen cabinet.
    DoubleOven,

    /// "[Down Draft](https://ddwiki.reso.org/display/DDW17/Down+Draft)": A vent that is part of the surface of a cook top that has a fan which sucks cooking fumes/smoke down.  This is an alternative to a hooded cooktop/range.
    DownDraft,

    /// "[Dryer](https://ddwiki.reso.org/display/DDW17/Dryer)": A cloths dryer.
    Dryer,

    /// "[Electric Cooktop](https://ddwiki.reso.org/display/DDW17/Electric+Cooktop)": A cooktop or stove that produces heat by way of electricity rather than gas.  An induction cooktop is electric, but not all electric cooktops are induction.
    ElectricCooktop,

    /// "[Electric Oven](https://ddwiki.reso.org/display/DDW17/Electric+Oven)": An oven that is heated by electricity, typically by way of heating coils.
    ElectricOven,

    /// "[Electric Range](https://ddwiki.reso.org/display/DDW17/Electric+Range)": An oven and cooktop that generates heat by way of electricity.
    ElectricRange,

    /// "[Electric Water Heater](https://ddwiki.reso.org/display/DDW17/Electric+Water+Heater)": A water heater that heats the water by way of electricity.
    ElectricWaterHeater,

    /// "[ENERGY STAR Qualified Appliances](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Appliances)": The property includes Energy Star qualified appliances.
    ENERGYSTARQualifiedAppliances,

    /// "[ENERGY STAR Qualified Dishwasher](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Dishwasher)": The property includes an Energy Star qualified dishwasher.
    ENERGYSTARQualifiedDishwasher,

    /// "[ENERGY STAR Qualified Dryer](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Dryer)": The property includes an Energy Star qualified cloths dryer.
    ENERGYSTARQualifiedDryer,

    /// "[ENERGY STAR Qualified Freezer](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Freezer)": The property includes an Energy Star qualified freezer.
    ENERGYSTARQualifiedFreezer,

    /// "[ENERGY STAR Qualified Refrigerator](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Refrigerator)": The property includes an Energy Star qualified refrigerator.
    ENERGYSTARQualifiedRefrigerator,

    /// "[ENERGY STAR Qualified Washer](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Washer)": The property includes an Energy Star qualified cloths washer.
    ENERGYSTARQualifiedWasher,

    /// "[ENERGY STAR Qualified Water Heater](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Water+Heater)": The property includes an Energy Star qualified water heater.
    ENERGYSTARQualifiedWaterHeater,

    /// "[Exhaust Fan](https://ddwiki.reso.org/display/DDW17/Exhaust+Fan)": The cooktop has an exhaust fan.
    ExhaustFan,

    /// "[Free-Standing Electric Oven](https://ddwiki.reso.org/display/DDW17/Free-Standing+Electric+Oven)": The oven is free standing, not built-in, and uses electricity to produce heat.
    FreeStandingElectricOven,

    /// "[Free-Standing Electric Range](https://ddwiki.reso.org/display/DDW17/Free-Standing+Electric+Range)": The range is free standing, not built-in, and uses electricity to produce heat for its oven and cooktop.
    FreeStandingElectricRange,

    /// "[Free-Standing Freezer](https://ddwiki.reso.org/display/DDW17/Free-Standing+Freezer)": The freezer is free standing and not built-in.
    FreeStandingFreezer,

    /// "[Free-Standing Gas Oven](https://ddwiki.reso.org/display/DDW17/Free-Standing+Gas+Oven)": The oven is free standing, not built-in, and uses gas to produce heat.
    FreeStandingGasOven,

    /// "[Free-Standing Gas Range](https://ddwiki.reso.org/display/DDW17/Free-Standing+Gas+Range)": The range is free standing, not built-in, and uses gas to produce heat for its oven and cooktop.
    FreeStandingGasRange,

    /// "[Free-Standing Range](https://ddwiki.reso.org/display/DDW17/Free-Standing+Range)": The range is free standing, not built-in.
    FreeStandingRange,

    /// "[Free-Standing Refrigerator](https://ddwiki.reso.org/display/DDW17/Free-Standing+Refrigerator)": The refrigerator is free-standing, not built-in.
    FreeStandingRefrigerator,

    /// "[Freezer](https://ddwiki.reso.org/display/DDW17/Freezer)": The property includes a freezer.
    Freezer,

    /// "[Gas Cooktop](https://ddwiki.reso.org/display/DDW17/Gas+Cooktop)": A cooktop or stove that produces heat by way of gas rather than electricity.  An induction cooktop is electric, but not all electric cooktops are induction.
    GasCooktop,

    /// "[Gas Oven](https://ddwiki.reso.org/display/DDW17/Gas+Oven)": An oven that is heated by gas.
    GasOven,

    /// "[Gas Range](https://ddwiki.reso.org/display/DDW17/Gas+Range)": An oven and cooktop that generates heat by way of gas.
    GasRange,

    /// "[Gas Water Heater](https://ddwiki.reso.org/display/DDW17/Gas+Water+Heater)": A water heater that heats the water with gas.
    GasWaterHeater,

    /// "[Humidifier](https://ddwiki.reso.org/display/DDW17/Humidifier)": The property includes a humidity control device or system.
    Humidifier,

    /// "[Ice Maker](https://ddwiki.reso.org/display/DDW17/Ice+Maker)": The property includes an ice maker.
    IceMaker,

    /// "[Indoor Grill](https://ddwiki.reso.org/display/DDW17/Indoor+Grill)": The property has an indoor grill.
    IndoorGrill,

    /// "[Induction Cooktop](https://ddwiki.reso.org/display/DDW17/Induction+Cooktop)": The electric cooktop is based on magnetic induction rather than heating coils.
    InductionCooktop,

    /// "[Instant Hot Water](https://ddwiki.reso.org/display/DDW17/Instant+Hot+Water)": The property has a circulatory or instant hot water system.
    InstantHotWater,

    /// "[Microwave](https://ddwiki.reso.org/display/DDW17/Microwave)": The property includes a microwave.
    Microwave,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243734)": The property includes no appliances.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243735)": The property includes appliances other than those available in this list.
    Other,

    /// "[Oven](https://ddwiki.reso.org/display/DDW17/Oven)": The property includes an oven.
    Oven,

    /// "[Plumbed For Ice Maker](https://ddwiki.reso.org/display/DDW17/Plumbed+For+Ice+Maker)": The property has plumbing for an ice maker.
    PlumbedForIceMaker,

    /// "[Portable Dishwasher](https://ddwiki.reso.org/display/DDW17/Portable+Dishwasher)": The property includes a portable dishwasher.
    PortableDishwasher,

    /// "[Propane Cooktop](https://ddwiki.reso.org/display/DDW17/Propane+Cooktop)": The gas cooktop uses propane as its fuel and either has a local tank or runs on a house wide propane system.
    PropaneCooktop,

    /// "[Range](https://ddwiki.reso.org/display/DDW17/Range)": The property includes a range, which is a single unit that has both an oven and a cooktop.
    Range,

    /// "[Range Hood](https://ddwiki.reso.org/display/DDW17/Range+Hood)": The range has a hooded exhaust.
    RangeHood,

    /// "[Refrigerator](https://ddwiki.reso.org/display/DDW17/Refrigerator)": The property includes a refrigerator.
    Refrigerator,

    /// "[Self Cleaning Oven](https://ddwiki.reso.org/display/DDW17/Self+Cleaning+Oven)": The oven included with the property has a self-cleaning feature.
    SelfCleaningOven,

    /// "[Solar Hot Water](https://ddwiki.reso.org/display/DDW17/Solar+Hot+Water)": The hot water heater has either a passive or active solar component.
    SolarHotWater,

    /// "[Stainless Steel Appliance(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243745)": Some or all of the appliances included in the property are stainless steal.
    StainlessSteelAppliances,

    /// "[Tankless Water Heater](https://ddwiki.reso.org/display/DDW17/Tankless+Water+Heater)": A tankless water heater is included with the property.
    TanklessWaterHeater,

    /// "[Trash Compactor](https://ddwiki.reso.org/display/DDW17/Trash+Compactor)": The property has a trash compactor.
    TrashCompactor,

    /// "[Vented Exhaust Fan](https://ddwiki.reso.org/display/DDW17/Vented+Exhaust+Fan)": The cooktop has an exhaust fan that is vented.
    VentedExhaustFan,

    /// "[Warming Drawer](https://ddwiki.reso.org/display/DDW17/Warming+Drawer)": The property has a warming drawer.
    WarmingDrawer,

    /// "[Washer](https://ddwiki.reso.org/display/DDW17/Washer)": The property includes a cloths washer.
    Washer,

    /// "[Washer/Dryer](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243751)": The property includes a cloths washer and dryer.
    WasherDryer,

    /// "[Washer/Dryer Stacked](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243752)": The property has a stacked cloths washer and dryer.
    WasherDryerStacked,

    /// "[Water Heater](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243753)": The property has a water heater.
    WaterHeater,

    /// "[Water Purifier](https://ddwiki.reso.org/display/DDW17/Water+Purifier)": The property has a water purifier.
    WaterPurifier,

    /// "[Water Purifier Owned](https://ddwiki.reso.org/display/DDW17/Water+Purifier+Owned)": The property has a water purifier that is owned and not rented/leased.
    WaterPurifierOwned,

    /// "[Water Purifier Rented](https://ddwiki.reso.org/display/DDW17/Water+Purifier+Rented)": The property has a water purifier that is on a rental or lease agreement.
    WaterPurifierRented,

    /// "[Water Softener](https://ddwiki.reso.org/display/DDW17/Water+Softener)": The property has a water softening system.
    WaterSoftener,

    /// "[Water Softener Owned](https://ddwiki.reso.org/display/DDW17/Water+Softener+Owned)": The property has a water softening system that is owned and not rented/leased.
    WaterSoftenerOwned,

    /// "[Water Softener Rented](https://ddwiki.reso.org/display/DDW17/Water+Softener+Rented)": The property has a water softening system that is on a rental or lease agreement.
    WaterSoftenerRented,

    /// "[Wine Cooler](https://ddwiki.reso.org/display/DDW17/Wine+Cooler)": The property includes a wine cooler.
    WineCooler,

    /// "[Wine Refrigerator](https://ddwiki.reso.org/display/DDW17/Wine+Refrigerator)": The property includes a wine refrigerator.
    WineRefrigerator,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Appliances {
    fn from(s: String) -> Appliances {
        match s.as_ref() {
            "Bar Fridge" => Appliances::BarFridge,

            "Built-In Electric Oven" => Appliances::BuiltInElectricOven,

            "Built-In Electric Range" => Appliances::BuiltInElectricRange,

            "Built-In Freezer" => Appliances::BuiltInFreezer,

            "Built-In Gas Oven" => Appliances::BuiltInGasOven,

            "Built-In Gas Range" => Appliances::BuiltInGasRange,

            "Built-In Range" => Appliances::BuiltInRange,

            "Built-In Refrigerator" => Appliances::BuiltInRefrigerator,

            "Convection Oven" => Appliances::ConvectionOven,

            "Cooktop" => Appliances::Cooktop,

            "Dishwasher" => Appliances::Dishwasher,

            "Disposal" => Appliances::Disposal,

            "Double Oven" => Appliances::DoubleOven,

            "Down Draft" => Appliances::DownDraft,

            "Dryer" => Appliances::Dryer,

            "Electric Cooktop" => Appliances::ElectricCooktop,

            "Electric Oven" => Appliances::ElectricOven,

            "Electric Range" => Appliances::ElectricRange,

            "Electric Water Heater" => Appliances::ElectricWaterHeater,

            "ENERGY STAR Qualified Appliances" => Appliances::ENERGYSTARQualifiedAppliances,

            "ENERGY STAR Qualified Dishwasher" => Appliances::ENERGYSTARQualifiedDishwasher,

            "ENERGY STAR Qualified Dryer" => Appliances::ENERGYSTARQualifiedDryer,

            "ENERGY STAR Qualified Freezer" => Appliances::ENERGYSTARQualifiedFreezer,

            "ENERGY STAR Qualified Refrigerator" => Appliances::ENERGYSTARQualifiedRefrigerator,

            "ENERGY STAR Qualified Washer" => Appliances::ENERGYSTARQualifiedWasher,

            "ENERGY STAR Qualified Water Heater" => Appliances::ENERGYSTARQualifiedWaterHeater,

            "Exhaust Fan" => Appliances::ExhaustFan,

            "Free-Standing Electric Oven" => Appliances::FreeStandingElectricOven,

            "Free-Standing Electric Range" => Appliances::FreeStandingElectricRange,

            "Free-Standing Freezer" => Appliances::FreeStandingFreezer,

            "Free-Standing Gas Oven" => Appliances::FreeStandingGasOven,

            "Free-Standing Gas Range" => Appliances::FreeStandingGasRange,

            "Free-Standing Range" => Appliances::FreeStandingRange,

            "Free-Standing Refrigerator" => Appliances::FreeStandingRefrigerator,

            "Freezer" => Appliances::Freezer,

            "Gas Cooktop" => Appliances::GasCooktop,

            "Gas Oven" => Appliances::GasOven,

            "Gas Range" => Appliances::GasRange,

            "Gas Water Heater" => Appliances::GasWaterHeater,

            "Humidifier" => Appliances::Humidifier,

            "Ice Maker" => Appliances::IceMaker,

            "Indoor Grill" => Appliances::IndoorGrill,

            "Induction Cooktop" => Appliances::InductionCooktop,

            "Instant Hot Water" => Appliances::InstantHotWater,

            "Microwave" => Appliances::Microwave,

            "None" => Appliances::None,

            "Other" => Appliances::Other,

            "Oven" => Appliances::Oven,

            "Plumbed For Ice Maker" => Appliances::PlumbedForIceMaker,

            "Portable Dishwasher" => Appliances::PortableDishwasher,

            "Propane Cooktop" => Appliances::PropaneCooktop,

            "Range" => Appliances::Range,

            "Range Hood" => Appliances::RangeHood,

            "Refrigerator" => Appliances::Refrigerator,

            "Self Cleaning Oven" => Appliances::SelfCleaningOven,

            "Solar Hot Water" => Appliances::SolarHotWater,

            "Stainless Steel Appliance(s)" => Appliances::StainlessSteelAppliances,

            "Tankless Water Heater" => Appliances::TanklessWaterHeater,

            "Trash Compactor" => Appliances::TrashCompactor,

            "Vented Exhaust Fan" => Appliances::VentedExhaustFan,

            "Warming Drawer" => Appliances::WarmingDrawer,

            "Washer" => Appliances::Washer,

            "Washer/Dryer" => Appliances::WasherDryer,

            "Washer/Dryer Stacked" => Appliances::WasherDryerStacked,

            "Water Heater" => Appliances::WaterHeater,

            "Water Purifier" => Appliances::WaterPurifier,

            "Water Purifier Owned" => Appliances::WaterPurifierOwned,

            "Water Purifier Rented" => Appliances::WaterPurifierRented,

            "Water Softener" => Appliances::WaterSoftener,

            "Water Softener Owned" => Appliances::WaterSoftenerOwned,

            "Water Softener Rented" => Appliances::WaterSoftenerRented,

            "Wine Cooler" => Appliances::WineCooler,

            "Wine Refrigerator" => Appliances::WineRefrigerator,

            _ => Appliances::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Appliances {
    fn from(s: &str) -> Appliances {
        match s {
            "Bar Fridge" => Appliances::BarFridge,

            "Built-In Electric Oven" => Appliances::BuiltInElectricOven,

            "Built-In Electric Range" => Appliances::BuiltInElectricRange,

            "Built-In Freezer" => Appliances::BuiltInFreezer,

            "Built-In Gas Oven" => Appliances::BuiltInGasOven,

            "Built-In Gas Range" => Appliances::BuiltInGasRange,

            "Built-In Range" => Appliances::BuiltInRange,

            "Built-In Refrigerator" => Appliances::BuiltInRefrigerator,

            "Convection Oven" => Appliances::ConvectionOven,

            "Cooktop" => Appliances::Cooktop,

            "Dishwasher" => Appliances::Dishwasher,

            "Disposal" => Appliances::Disposal,

            "Double Oven" => Appliances::DoubleOven,

            "Down Draft" => Appliances::DownDraft,

            "Dryer" => Appliances::Dryer,

            "Electric Cooktop" => Appliances::ElectricCooktop,

            "Electric Oven" => Appliances::ElectricOven,

            "Electric Range" => Appliances::ElectricRange,

            "Electric Water Heater" => Appliances::ElectricWaterHeater,

            "ENERGY STAR Qualified Appliances" => Appliances::ENERGYSTARQualifiedAppliances,

            "ENERGY STAR Qualified Dishwasher" => Appliances::ENERGYSTARQualifiedDishwasher,

            "ENERGY STAR Qualified Dryer" => Appliances::ENERGYSTARQualifiedDryer,

            "ENERGY STAR Qualified Freezer" => Appliances::ENERGYSTARQualifiedFreezer,

            "ENERGY STAR Qualified Refrigerator" => Appliances::ENERGYSTARQualifiedRefrigerator,

            "ENERGY STAR Qualified Washer" => Appliances::ENERGYSTARQualifiedWasher,

            "ENERGY STAR Qualified Water Heater" => Appliances::ENERGYSTARQualifiedWaterHeater,

            "Exhaust Fan" => Appliances::ExhaustFan,

            "Free-Standing Electric Oven" => Appliances::FreeStandingElectricOven,

            "Free-Standing Electric Range" => Appliances::FreeStandingElectricRange,

            "Free-Standing Freezer" => Appliances::FreeStandingFreezer,

            "Free-Standing Gas Oven" => Appliances::FreeStandingGasOven,

            "Free-Standing Gas Range" => Appliances::FreeStandingGasRange,

            "Free-Standing Range" => Appliances::FreeStandingRange,

            "Free-Standing Refrigerator" => Appliances::FreeStandingRefrigerator,

            "Freezer" => Appliances::Freezer,

            "Gas Cooktop" => Appliances::GasCooktop,

            "Gas Oven" => Appliances::GasOven,

            "Gas Range" => Appliances::GasRange,

            "Gas Water Heater" => Appliances::GasWaterHeater,

            "Humidifier" => Appliances::Humidifier,

            "Ice Maker" => Appliances::IceMaker,

            "Indoor Grill" => Appliances::IndoorGrill,

            "Induction Cooktop" => Appliances::InductionCooktop,

            "Instant Hot Water" => Appliances::InstantHotWater,

            "Microwave" => Appliances::Microwave,

            "None" => Appliances::None,

            "Other" => Appliances::Other,

            "Oven" => Appliances::Oven,

            "Plumbed For Ice Maker" => Appliances::PlumbedForIceMaker,

            "Portable Dishwasher" => Appliances::PortableDishwasher,

            "Propane Cooktop" => Appliances::PropaneCooktop,

            "Range" => Appliances::Range,

            "Range Hood" => Appliances::RangeHood,

            "Refrigerator" => Appliances::Refrigerator,

            "Self Cleaning Oven" => Appliances::SelfCleaningOven,

            "Solar Hot Water" => Appliances::SolarHotWater,

            "Stainless Steel Appliance(s)" => Appliances::StainlessSteelAppliances,

            "Tankless Water Heater" => Appliances::TanklessWaterHeater,

            "Trash Compactor" => Appliances::TrashCompactor,

            "Vented Exhaust Fan" => Appliances::VentedExhaustFan,

            "Warming Drawer" => Appliances::WarmingDrawer,

            "Washer" => Appliances::Washer,

            "Washer/Dryer" => Appliances::WasherDryer,

            "Washer/Dryer Stacked" => Appliances::WasherDryerStacked,

            "Water Heater" => Appliances::WaterHeater,

            "Water Purifier" => Appliances::WaterPurifier,

            "Water Purifier Owned" => Appliances::WaterPurifierOwned,

            "Water Purifier Rented" => Appliances::WaterPurifierRented,

            "Water Softener" => Appliances::WaterSoftener,

            "Water Softener Owned" => Appliances::WaterSoftenerOwned,

            "Water Softener Rented" => Appliances::WaterSoftenerRented,

            "Wine Cooler" => Appliances::WineCooler,

            "Wine Refrigerator" => Appliances::WineRefrigerator,

            _ => Appliances::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Appliances> for &'a str {
    fn from(s: &'a Appliances) -> &'a str {
        match s {
            Appliances::BarFridge => "Bar Fridge",

            Appliances::BuiltInElectricOven => "Built-In Electric Oven",

            Appliances::BuiltInElectricRange => "Built-In Electric Range",

            Appliances::BuiltInFreezer => "Built-In Freezer",

            Appliances::BuiltInGasOven => "Built-In Gas Oven",

            Appliances::BuiltInGasRange => "Built-In Gas Range",

            Appliances::BuiltInRange => "Built-In Range",

            Appliances::BuiltInRefrigerator => "Built-In Refrigerator",

            Appliances::ConvectionOven => "Convection Oven",

            Appliances::Cooktop => "Cooktop",

            Appliances::Dishwasher => "Dishwasher",

            Appliances::Disposal => "Disposal",

            Appliances::DoubleOven => "Double Oven",

            Appliances::DownDraft => "Down Draft",

            Appliances::Dryer => "Dryer",

            Appliances::ElectricCooktop => "Electric Cooktop",

            Appliances::ElectricOven => "Electric Oven",

            Appliances::ElectricRange => "Electric Range",

            Appliances::ElectricWaterHeater => "Electric Water Heater",

            Appliances::ENERGYSTARQualifiedAppliances => "ENERGY STAR Qualified Appliances",

            Appliances::ENERGYSTARQualifiedDishwasher => "ENERGY STAR Qualified Dishwasher",

            Appliances::ENERGYSTARQualifiedDryer => "ENERGY STAR Qualified Dryer",

            Appliances::ENERGYSTARQualifiedFreezer => "ENERGY STAR Qualified Freezer",

            Appliances::ENERGYSTARQualifiedRefrigerator => "ENERGY STAR Qualified Refrigerator",

            Appliances::ENERGYSTARQualifiedWasher => "ENERGY STAR Qualified Washer",

            Appliances::ENERGYSTARQualifiedWaterHeater => "ENERGY STAR Qualified Water Heater",

            Appliances::ExhaustFan => "Exhaust Fan",

            Appliances::FreeStandingElectricOven => "Free-Standing Electric Oven",

            Appliances::FreeStandingElectricRange => "Free-Standing Electric Range",

            Appliances::FreeStandingFreezer => "Free-Standing Freezer",

            Appliances::FreeStandingGasOven => "Free-Standing Gas Oven",

            Appliances::FreeStandingGasRange => "Free-Standing Gas Range",

            Appliances::FreeStandingRange => "Free-Standing Range",

            Appliances::FreeStandingRefrigerator => "Free-Standing Refrigerator",

            Appliances::Freezer => "Freezer",

            Appliances::GasCooktop => "Gas Cooktop",

            Appliances::GasOven => "Gas Oven",

            Appliances::GasRange => "Gas Range",

            Appliances::GasWaterHeater => "Gas Water Heater",

            Appliances::Humidifier => "Humidifier",

            Appliances::IceMaker => "Ice Maker",

            Appliances::IndoorGrill => "Indoor Grill",

            Appliances::InductionCooktop => "Induction Cooktop",

            Appliances::InstantHotWater => "Instant Hot Water",

            Appliances::Microwave => "Microwave",

            Appliances::None => "None",

            Appliances::Other => "Other",

            Appliances::Oven => "Oven",

            Appliances::PlumbedForIceMaker => "Plumbed For Ice Maker",

            Appliances::PortableDishwasher => "Portable Dishwasher",

            Appliances::PropaneCooktop => "Propane Cooktop",

            Appliances::Range => "Range",

            Appliances::RangeHood => "Range Hood",

            Appliances::Refrigerator => "Refrigerator",

            Appliances::SelfCleaningOven => "Self Cleaning Oven",

            Appliances::SolarHotWater => "Solar Hot Water",

            Appliances::StainlessSteelAppliances => "Stainless Steel Appliance(s)",

            Appliances::TanklessWaterHeater => "Tankless Water Heater",

            Appliances::TrashCompactor => "Trash Compactor",

            Appliances::VentedExhaustFan => "Vented Exhaust Fan",

            Appliances::WarmingDrawer => "Warming Drawer",

            Appliances::Washer => "Washer",

            Appliances::WasherDryer => "Washer/Dryer",

            Appliances::WasherDryerStacked => "Washer/Dryer Stacked",

            Appliances::WaterHeater => "Water Heater",

            Appliances::WaterPurifier => "Water Purifier",

            Appliances::WaterPurifierOwned => "Water Purifier Owned",

            Appliances::WaterPurifierRented => "Water Purifier Rented",

            Appliances::WaterSoftener => "Water Softener",

            Appliances::WaterSoftenerOwned => "Water Softener Owned",

            Appliances::WaterSoftenerRented => "Water Softener Rented",

            Appliances::WineCooler => "Wine Cooler",

            Appliances::WineRefrigerator => "Wine Refrigerator",

            Appliances::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Appliances {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Appliances {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_appliances_format {
    use super::Appliances;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Appliances>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Appliances>>, D::Error>
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
