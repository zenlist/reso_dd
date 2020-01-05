// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Electric Lookups](https://ddwiki.reso.org/display/DDW17/Electric+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Electric {
    /// "[100 Amp Service](https://ddwiki.reso.org/display/DDW17/100+Amp+Service)": The electrical features of the property include 100 amp service.
    _100AmpService,

    /// "[150 Amp Service](https://ddwiki.reso.org/display/DDW17/150+Amp+Service)": The electrical features of the property include 150 amp service.
    _150AmpService,

    /// "[200+ Amp Service](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244609)": The electrical features of the property include 200+ amp service.
    _200PlusAmpService,

    /// "[220 Volts](https://ddwiki.reso.org/display/DDW17/220+Volts)": The electrical features of the property include 220 volts.
    _220Volts,

    /// "[220 Volts For Spa](https://ddwiki.reso.org/display/DDW17/220+Volts+For+Spa)": The electrical features of the property include 220 volts for spa.
    _220VoltsForSpa,

    /// "[220 Volts in Garage](https://ddwiki.reso.org/display/DDW17/220+Volts+in+Garage)": The electrical features of the property include 220 volts in garage.
    _220VoltsinGarage,

    /// "[220 Volts in Kitchen](https://ddwiki.reso.org/display/DDW17/220+Volts+in+Kitchen)": The electrical features of the property include 220 volts in kitchen.
    _220VoltsinKitchen,

    /// "[220 Volts in Laundry](https://ddwiki.reso.org/display/DDW17/220+Volts+in+Laundry)": The electrical features of the property include 220 volts in laundry.
    _220VoltsinLaundry,

    /// "[220 Volts in Workshop](https://ddwiki.reso.org/display/DDW17/220+Volts+in+Workshop)": The electrical features of the property include 220 volts in workshop.
    _220VoltsinWorkshop,

    /// "[440 Volts](https://ddwiki.reso.org/display/DDW17/440+Volts)": The electrical features of the property include 440 volts.
    _440Volts,

    /// "[Circuit Breakers](https://ddwiki.reso.org/display/DDW17/Circuit+Breakers)": The electrical features of the property include circuit breakers.
    CircuitBreakers,

    /// "[Energy Storage Device](https://ddwiki.reso.org/display/DDW17/Energy+Storage+Device)": Device(s) that capture energy at one time to be used at a later time.  Most commonly these refer to single or groups of stand-alone batteries, such as could be used as back-up power, but it also might include flywheels or other devices to store power.
    EnergyStorageDevice,

    /// "[Fuses](https://ddwiki.reso.org/display/DDW17/Fuses)": The electrical features of the property include fuses.
    Fuses,

    /// "[Generator](https://ddwiki.reso.org/display/DDW17/Generator)": The electrical features of the property include generator.
    Generator,

    /// "[Net Meter](https://ddwiki.reso.org/display/DDW17/Net+Meter)": Net metering is an electric service that allows electricity generated on a consumer’s site (“on-site”) to offset that consumer’s use.  This generation can include (generally small) renewable energy facilities (such as wind, solar power, fuel cells or hydro).  Net meters might also be used with energy storage devices such as batteries (stand alone or for electric vehicles). Net meters can “spin backwards” such that at the end of the billing period, the consumer only pays for its use, less what it produced (i.e., the “net”).
    NetMeter,

    /// "[Photovoltaics Seller Owned](https://ddwiki.reso.org/display/DDW17/Photovoltaics+Seller+Owned)": The electrical features of the property include a solar photovoltaic system that is owned by the seller.
    PhotovoltaicsSellerOwned,

    /// "[Photovoltaics Third-Party Owned](https://ddwiki.reso.org/display/DDW17/Photovoltaics+Third-Party+Owned)": The electrical features of the property include a solar photovoltaic system owned by a third party.  This is typically a lease but may be some other arrangement where the property owner does not own the photovoltaic system.
    PhotovoltaicsThirdPartyOwned,

    /// "[Pre-Wired for Renewables](https://ddwiki.reso.org/display/DDW17/Pre-Wired+for+Renewables)": Indicates the electric infrastructure on the property has been extended to more easily incorporate an on-site electric generation facility in the future. This would often include, for example, installing conduit and wire from the generation facility to the electric panel, designating circuits on the panel for that generation, and/or leaving room near the panel for future components, such as an inverter.
    PreWiredforRenewables,

    /// "[Ready for Renewables](https://ddwiki.reso.org/display/DDW17/Ready+for+Renewables)": Indicates a comprehensive infrastructure is in place on the property to more easily incorporate an on-site electric generation facility in the future. Can be confirmed via supporting documentation such as a checklist provided by the DOE Zero Energy Ready Homes program. Solar-PV ready, for example, would often include extensive efficiency measures such as insulation and appliances, architectural drawings that design for a clear roof space, installing conduit from the attic to the electric panel, dedicated circuits on the electric panel, and leaving room near the panel for future components of a solar electric system, such as an inverter. Local requirements may vary. (source: DOE Zero Energy Ready Home http://energy.gov/sites/prod/files/2015/05/f22/PV-Ready%20Checklist.pdf )
    ReadyforRenewables,

    /// "[Underground](https://ddwiki.reso.org/display/DDW17/Underground)": The electrical features of the property include underground.
    Underground,

    /// "[Wind Turbine Seller Owned](https://ddwiki.reso.org/display/DDW17/Wind+Turbine+Seller+Owned)": A wind turbine is provided on the property to generate electricity. Seller owned turbines are typically considered real property and can be transferred with the property.
    WindTurbineSellerOwned,

    /// "[Wind Turbine Third-Party Owned](https://ddwiki.reso.org/display/DDW17/Wind+Turbine+Third-Party+Owned)": A wind turbine is provided on the property to generate electricity.  The homeowner enters a lease agreement with the owner of the wind turbine(s). Third-Party Owned turbines indicate a lease or a Power Purchase Agreement (PPA) exists.  The lease/PPA can often be transferred but the financing company has to agree. See CurrentFinancing field for important further definition of these models.
    WindTurbineThirdPartyOwned,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Electric {
    fn from_str(s: &str) -> Electric {
        match s {
            "100 Amp Service" => Electric::_100AmpService,

            "150 Amp Service" => Electric::_150AmpService,

            "200+ Amp Service" => Electric::_200PlusAmpService,

            "220 Volts" => Electric::_220Volts,

            "220 Volts For Spa" => Electric::_220VoltsForSpa,

            "220 Volts in Garage" => Electric::_220VoltsinGarage,

            "220 Volts in Kitchen" => Electric::_220VoltsinKitchen,

            "220 Volts in Laundry" => Electric::_220VoltsinLaundry,

            "220 Volts in Workshop" => Electric::_220VoltsinWorkshop,

            "440 Volts" => Electric::_440Volts,

            "Circuit Breakers" => Electric::CircuitBreakers,

            "Energy Storage Device" => Electric::EnergyStorageDevice,

            "Fuses" => Electric::Fuses,

            "Generator" => Electric::Generator,

            "Net Meter" => Electric::NetMeter,

            "Photovoltaics Seller Owned" => Electric::PhotovoltaicsSellerOwned,

            "Photovoltaics Third-Party Owned" => Electric::PhotovoltaicsThirdPartyOwned,

            "Pre-Wired for Renewables" => Electric::PreWiredforRenewables,

            "Ready for Renewables" => Electric::ReadyforRenewables,

            "Underground" => Electric::Underground,

            "Wind Turbine Seller Owned" => Electric::WindTurbineSellerOwned,

            "Wind Turbine Third-Party Owned" => Electric::WindTurbineThirdPartyOwned,

            _ => Electric::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Electric {
        match s.as_ref() {
            "100 Amp Service" => Electric::_100AmpService,

            "150 Amp Service" => Electric::_150AmpService,

            "200+ Amp Service" => Electric::_200PlusAmpService,

            "220 Volts" => Electric::_220Volts,

            "220 Volts For Spa" => Electric::_220VoltsForSpa,

            "220 Volts in Garage" => Electric::_220VoltsinGarage,

            "220 Volts in Kitchen" => Electric::_220VoltsinKitchen,

            "220 Volts in Laundry" => Electric::_220VoltsinLaundry,

            "220 Volts in Workshop" => Electric::_220VoltsinWorkshop,

            "440 Volts" => Electric::_440Volts,

            "Circuit Breakers" => Electric::CircuitBreakers,

            "Energy Storage Device" => Electric::EnergyStorageDevice,

            "Fuses" => Electric::Fuses,

            "Generator" => Electric::Generator,

            "Net Meter" => Electric::NetMeter,

            "Photovoltaics Seller Owned" => Electric::PhotovoltaicsSellerOwned,

            "Photovoltaics Third-Party Owned" => Electric::PhotovoltaicsThirdPartyOwned,

            "Pre-Wired for Renewables" => Electric::PreWiredforRenewables,

            "Ready for Renewables" => Electric::ReadyforRenewables,

            "Underground" => Electric::Underground,

            "Wind Turbine Seller Owned" => Electric::WindTurbineSellerOwned,

            "Wind Turbine Third-Party Owned" => Electric::WindTurbineThirdPartyOwned,

            _ => Electric::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Electric::_100AmpService => "100 Amp Service",

            Electric::_150AmpService => "150 Amp Service",

            Electric::_200PlusAmpService => "200+ Amp Service",

            Electric::_220Volts => "220 Volts",

            Electric::_220VoltsForSpa => "220 Volts For Spa",

            Electric::_220VoltsinGarage => "220 Volts in Garage",

            Electric::_220VoltsinKitchen => "220 Volts in Kitchen",

            Electric::_220VoltsinLaundry => "220 Volts in Laundry",

            Electric::_220VoltsinWorkshop => "220 Volts in Workshop",

            Electric::_440Volts => "440 Volts",

            Electric::CircuitBreakers => "Circuit Breakers",

            Electric::EnergyStorageDevice => "Energy Storage Device",

            Electric::Fuses => "Fuses",

            Electric::Generator => "Generator",

            Electric::NetMeter => "Net Meter",

            Electric::PhotovoltaicsSellerOwned => "Photovoltaics Seller Owned",

            Electric::PhotovoltaicsThirdPartyOwned => "Photovoltaics Third-Party Owned",

            Electric::PreWiredforRenewables => "Pre-Wired for Renewables",

            Electric::ReadyforRenewables => "Ready for Renewables",

            Electric::Underground => "Underground",

            Electric::WindTurbineSellerOwned => "Wind Turbine Seller Owned",

            Electric::WindTurbineThirdPartyOwned => "Wind Turbine Third-Party Owned",

            Electric::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Electric::_100AmpService => "100 Amp Service".into(),

            Electric::_150AmpService => "150 Amp Service".into(),

            Electric::_200PlusAmpService => "200+ Amp Service".into(),

            Electric::_220Volts => "220 Volts".into(),

            Electric::_220VoltsForSpa => "220 Volts For Spa".into(),

            Electric::_220VoltsinGarage => "220 Volts in Garage".into(),

            Electric::_220VoltsinKitchen => "220 Volts in Kitchen".into(),

            Electric::_220VoltsinLaundry => "220 Volts in Laundry".into(),

            Electric::_220VoltsinWorkshop => "220 Volts in Workshop".into(),

            Electric::_440Volts => "440 Volts".into(),

            Electric::CircuitBreakers => "Circuit Breakers".into(),

            Electric::EnergyStorageDevice => "Energy Storage Device".into(),

            Electric::Fuses => "Fuses".into(),

            Electric::Generator => "Generator".into(),

            Electric::NetMeter => "Net Meter".into(),

            Electric::PhotovoltaicsSellerOwned => "Photovoltaics Seller Owned".into(),

            Electric::PhotovoltaicsThirdPartyOwned => "Photovoltaics Third-Party Owned".into(),

            Electric::PreWiredforRenewables => "Pre-Wired for Renewables".into(),

            Electric::ReadyforRenewables => "Ready for Renewables".into(),

            Electric::Underground => "Underground".into(),

            Electric::WindTurbineSellerOwned => "Wind Turbine Seller Owned".into(),

            Electric::WindTurbineThirdPartyOwned => "Wind Turbine Third-Party Owned".into(),

            Electric::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Electric::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Electric {
    fn from(s: String) -> Electric {
        match s.as_ref() {
            "100 Amp Service" => Electric::_100AmpService,

            "150 Amp Service" => Electric::_150AmpService,

            "200+ Amp Service" => Electric::_200PlusAmpService,

            "220 Volts" => Electric::_220Volts,

            "220 Volts For Spa" => Electric::_220VoltsForSpa,

            "220 Volts in Garage" => Electric::_220VoltsinGarage,

            "220 Volts in Kitchen" => Electric::_220VoltsinKitchen,

            "220 Volts in Laundry" => Electric::_220VoltsinLaundry,

            "220 Volts in Workshop" => Electric::_220VoltsinWorkshop,

            "440 Volts" => Electric::_440Volts,

            "Circuit Breakers" => Electric::CircuitBreakers,

            "Energy Storage Device" => Electric::EnergyStorageDevice,

            "Fuses" => Electric::Fuses,

            "Generator" => Electric::Generator,

            "Net Meter" => Electric::NetMeter,

            "Photovoltaics Seller Owned" => Electric::PhotovoltaicsSellerOwned,

            "Photovoltaics Third-Party Owned" => Electric::PhotovoltaicsThirdPartyOwned,

            "Pre-Wired for Renewables" => Electric::PreWiredforRenewables,

            "Ready for Renewables" => Electric::ReadyforRenewables,

            "Underground" => Electric::Underground,

            "Wind Turbine Seller Owned" => Electric::WindTurbineSellerOwned,

            "Wind Turbine Third-Party Owned" => Electric::WindTurbineThirdPartyOwned,

            _ => Electric::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Electric {
    fn from(s: &str) -> Electric {
        match s {
            "100 Amp Service" => Electric::_100AmpService,

            "150 Amp Service" => Electric::_150AmpService,

            "200+ Amp Service" => Electric::_200PlusAmpService,

            "220 Volts" => Electric::_220Volts,

            "220 Volts For Spa" => Electric::_220VoltsForSpa,

            "220 Volts in Garage" => Electric::_220VoltsinGarage,

            "220 Volts in Kitchen" => Electric::_220VoltsinKitchen,

            "220 Volts in Laundry" => Electric::_220VoltsinLaundry,

            "220 Volts in Workshop" => Electric::_220VoltsinWorkshop,

            "440 Volts" => Electric::_440Volts,

            "Circuit Breakers" => Electric::CircuitBreakers,

            "Energy Storage Device" => Electric::EnergyStorageDevice,

            "Fuses" => Electric::Fuses,

            "Generator" => Electric::Generator,

            "Net Meter" => Electric::NetMeter,

            "Photovoltaics Seller Owned" => Electric::PhotovoltaicsSellerOwned,

            "Photovoltaics Third-Party Owned" => Electric::PhotovoltaicsThirdPartyOwned,

            "Pre-Wired for Renewables" => Electric::PreWiredforRenewables,

            "Ready for Renewables" => Electric::ReadyforRenewables,

            "Underground" => Electric::Underground,

            "Wind Turbine Seller Owned" => Electric::WindTurbineSellerOwned,

            "Wind Turbine Third-Party Owned" => Electric::WindTurbineThirdPartyOwned,

            _ => Electric::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Electric> for &'a str {
    fn from(s: &'a Electric) -> &'a str {
        match s {
            Electric::_100AmpService => "100 Amp Service",

            Electric::_150AmpService => "150 Amp Service",

            Electric::_200PlusAmpService => "200+ Amp Service",

            Electric::_220Volts => "220 Volts",

            Electric::_220VoltsForSpa => "220 Volts For Spa",

            Electric::_220VoltsinGarage => "220 Volts in Garage",

            Electric::_220VoltsinKitchen => "220 Volts in Kitchen",

            Electric::_220VoltsinLaundry => "220 Volts in Laundry",

            Electric::_220VoltsinWorkshop => "220 Volts in Workshop",

            Electric::_440Volts => "440 Volts",

            Electric::CircuitBreakers => "Circuit Breakers",

            Electric::EnergyStorageDevice => "Energy Storage Device",

            Electric::Fuses => "Fuses",

            Electric::Generator => "Generator",

            Electric::NetMeter => "Net Meter",

            Electric::PhotovoltaicsSellerOwned => "Photovoltaics Seller Owned",

            Electric::PhotovoltaicsThirdPartyOwned => "Photovoltaics Third-Party Owned",

            Electric::PreWiredforRenewables => "Pre-Wired for Renewables",

            Electric::ReadyforRenewables => "Ready for Renewables",

            Electric::Underground => "Underground",

            Electric::WindTurbineSellerOwned => "Wind Turbine Seller Owned",

            Electric::WindTurbineThirdPartyOwned => "Wind Turbine Third-Party Owned",

            Electric::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Electric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Electric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
