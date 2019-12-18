// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [GreenBuildingVerificationType Lookups](https://ddwiki.reso.org/display/DDW17/GreenBuildingVerificationType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreenBuildingVerificationType {
    /// "[Certified Passive House](https://ddwiki.reso.org/display/DDW17/Certified+Passive+House)": Super-insulated new homes that have been built to meet certification requirements demonstrating minimal or no heating and cooling system.
    CertifiedPassiveHouse,

    /// "[ENERGY STAR Certified Homes](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Certified+Homes)": EPA ENERGY STAR Certified Homes is a set of optional construction practices and technologies (above minimum code requirements) that builders can follow to upgrade a new home's energy efficiency beyond minimum code requirements. Guidelines are outlined in the National Performance Path; or the National Prescriptive Path; This whole-house label differs from the ENERGY STAR products label. To achieve the ENERGY STAR Certified Homes label, a home's energy efficiency must be verified by a third-party organization.
    ENERGYSTARCertifiedHomes,

    /// "[EnerPHit](https://ddwiki.reso.org/display/DDW17/EnerPHit)": Super-insulated existing homes that have been remodeled to meet certification requirements demonstrating minimal or no heating and cooling system.
    EnerPHit,

    /// "[HERS Index Score](https://ddwiki.reso.org/display/DDW17/HERS+Index+Score)": The HERS Index is the nationally recognized scoring system for measuring a home's energy performance. To calculate a home's HERS Index Score, a certified RESNET home energy rater will do a home energy rating and compare the data against a reference home (a design-modeled home of the same size and shape as the actual home), so the HERS Index Score is always relative to the size, shape, and type of the house. The lower the number, the more energy efficient the home.
    HERSIndexScore,

    /// "[Home Energy Score](https://ddwiki.reso.org/display/DDW17/Home+Energy+Score)": The Home Energy Score, managed by the US DOE, is a national system that allows homes to receive an energy efficiency rating, similar to the MPG rating available for cars. The Home Energy Score uses a 10-point scale to reflect how much energy a home is expected to use under standard operating conditions. Homes that are expected to use the least amount of energy (and are considered the most energy efficient) score a 10, and homes that are expected to use the most amount of energy (and are considered the least energy efficient) score a 1. The Home Energy Score uses a standard calculation method and takes into account the home's structure and envelope (walls, windows, foundation) and its heating, cooling, and hot water systems. Only Qualified Assessors who pass a DOE exam are allowed to provide the Home Energy Score.
    HomeEnergyScore,

    /// "[Home Energy Upgrade Certificate of Energy Efficiency Improvements](https://ddwiki.reso.org/display/DDW17/Home+Energy+Upgrade+Certificate+of+Energy+Efficiency+Improvements)": Buildings Performance Institute BPI- 2101 Standard Requirements for a Certificate of Completion for Whole-House Energy Efficiency Upgrades specifies a standard way of describing the improvements made to an existing home through a home energy upgrade (HEU). Certificates are provided by a local energy efficiency program sponsor.
    HomeEnergyUpgradeCertificateofEnergyEfficiencyImprovements,

    /// "[Home Energy Upgrade Certificate of Energy Efficiency Performance](https://ddwiki.reso.org/display/DDW17/Home+Energy+Upgrade+Certificate+of+Energy+Efficiency+Performance)": Buildings Performance Institute BPI- 2101 Standard Requirements for a Certificate of Completion for Whole-House Energy Efficiency Upgrades specifies a standard way of describing the improvements made to an existing home through a home energy upgrade (HEU) and provides one or more measures of a home's performance. Measures of performance may include a HERS rating, a Home Energy Score, an indication of projected or actual energy consumption, or other systems. Certificates are provided by a local energy efficiency program sponsor.
    HomeEnergyUpgradeCertificateofEnergyEfficiencyPerformance,

    /// "[Home Performance with ENERGY STAR](https://ddwiki.reso.org/display/DDW17/Home+Performance+with+ENERGY+STAR)": Home Performance with ENERGY STAR offers whole-house solutions to high energy bills and homes with comfort problems. The program is managed by a local sponsor that recruits home improvement contractors who are qualified to perform comprehensive home assessments and improvements.   Local sponsors must follow specific guidelines to participate as outlined in the HPwES Sponsor Guide and Reference Manual.
    HomePerformancewithENERGYSTAR,

    /// "[Indoor airPLUS](https://ddwiki.reso.org/display/DDW17/Indoor+airPLUS)": EPA Indoor airPLUS is a set of optional construction practices and technologies builders can follow to reduce indoor air pollutants and improve the indoor air quality in a new home beyond minimum code requirements. It is only available to homes that first meet ENERGY STAR Certified Homes certification.
    IndoorairPLUS,

    /// "[LEED For Homes](https://ddwiki.reso.org/display/DDW17/LEED+For+Homes)": USGBC's residential rating system, LEED for Homes, was launched in 2008. The LEED rating systems are developed through an open, consensus-based process led by LEED committees. LEED is a voluntary program that provides independent, third-party verification that a home was designed and built using methods for achieving high performance in multiple areas of sustainability including energy, water, waste management, indoor air quality, and sustainable site development. There are multiple rating systems for all types of buildings including Existing Buildings and Homes for residential projects. Homes may achieve different levels of certification (platinum, gold, silver, certified) depending on the number of LEED prerequisites and credits that are met.
    LEEDForHomes,

    /// "[Living Building Challenge](https://ddwiki.reso.org/display/DDW17/Living+Building+Challenge)": Living Future Institute's Homes certified by a third-party that they produce as much or more energy than they use. Super-insulated homes that have met certification requirements demonstrating minimal or no heating and cooling system.
    LivingBuildingChallenge,

    /// "[NGBS New Construction](https://ddwiki.reso.org/display/DDW17/NGBS+New+Construction)": Home Innovation Research Labs certifies homes to the ICC-700 National Green Building Standard(tm) (NGBS), which has undergone the full consensus process and received approval from the American National Standards Institute (ANSI). Home Innovation Research Labs provides project certification to the NGBS. NGBS applies to both single-family homes and multifamily buildings. Certification options also exist for new construction, remodel projects (both whole-home and functional areas such as a kitchen or bathroom) and land development/subdivision. Homes may qualify for a bronze, silver, gold, or emerald certification level depending on the number of green building practices met.
    NGBSNewConstruction,

    /// "[NGBS Small Projects Remodel](https://ddwiki.reso.org/display/DDW17/NGBS+Small+Projects+Remodel)": Home Innovation Research Labs provides project certification to the NGBS. NGBS applies to both single-family homes and multifamily buildings. Certification options also exist for new construction, remodel projects (both whole-home and functional areas such as a kitchen or bathroom), and land development/subdivision. For the Small Projects Remodel, the functional area (kitchen, bathroom, basement, addition) is either certified or not. Unlike other NGBS certifications, multiple certification levels (bronze, silver, gold, or emerald) do not exist for this certification option.
    NGBSSmallProjectsRemodel,

    /// "[NGBS Whole-Home Remodel](https://ddwiki.reso.org/display/DDW17/NGBS+Whole-Home+Remodel)": Home Innovation Research Labs provides project certification to the NGBS. NGBS applies to both single-family homes and multifamily buildings. Certification options also exist for new construction, remodel projects (both whole-home and functional areas such as a kitchen or bathroom), and land development/subdivision. Homes may qualify for a bronze, silver, gold, or emerald certification level depending on the number of green building practices met.
    NGBSWholeHomeRemodel,

    /// "[PHIUS+](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244887)": Super-insulated homes that have met certification requirements demonstrating minimal or no heating and cooling system.
    PHIUSPlus,

    /// "[WaterSense](https://ddwiki.reso.org/display/DDW17/WaterSense)": EPA WaterSense is a set of optional construction practices and technologies (above minimum code requirements) that builders can follow to ensure a home uses less water while still providing the same level of comfort and convenience. WaterSense also applies to specific plumbing fixtures (see InteriorFeatures) and should not be confused with the whole-house label.
    WaterSense,

    /// "[Zero Energy Ready Home](https://ddwiki.reso.org/display/DDW17/Zero+Energy+Ready+Home)": DOE Zero Energy Ready Home is a set of optional construction practices and technologies (above minimum code and ENERGY STAR Certified Home requirements) that builders can follow to ensure high-performance homes so energy efficient all or most annual energy consumption can be offset with renewable energy. Guidelines are outlined in the "DOE Zero Energy Ready Home National Program Requirements."
    ZeroEnergyReadyHome,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for GreenBuildingVerificationType {
    fn from(s: String) -> GreenBuildingVerificationType {
        match s.as_ref() {


"Certified Passive House" => GreenBuildingVerificationType::CertifiedPassiveHouse,

"ENERGY STAR Certified Homes" => GreenBuildingVerificationType::ENERGYSTARCertifiedHomes,

"EnerPHit" => GreenBuildingVerificationType::EnerPHit,

"HERS Index Score" => GreenBuildingVerificationType::HERSIndexScore,

"Home Energy Score" => GreenBuildingVerificationType::HomeEnergyScore,

"Home Energy Upgrade Certificate of Energy Efficiency Improvements" => GreenBuildingVerificationType::HomeEnergyUpgradeCertificateofEnergyEfficiencyImprovements,

"Home Energy Upgrade Certificate of Energy Efficiency Performance" => GreenBuildingVerificationType::HomeEnergyUpgradeCertificateofEnergyEfficiencyPerformance,

"Home Performance with ENERGY STAR" => GreenBuildingVerificationType::HomePerformancewithENERGYSTAR,

"Indoor airPLUS" => GreenBuildingVerificationType::IndoorairPLUS,

"LEED For Homes" => GreenBuildingVerificationType::LEEDForHomes,

"Living Building Challenge" => GreenBuildingVerificationType::LivingBuildingChallenge,

"NGBS New Construction" => GreenBuildingVerificationType::NGBSNewConstruction,

"NGBS Small Projects Remodel" => GreenBuildingVerificationType::NGBSSmallProjectsRemodel,

"NGBS Whole-Home Remodel" => GreenBuildingVerificationType::NGBSWholeHomeRemodel,

"PHIUS+" => GreenBuildingVerificationType::PHIUSPlus,

"WaterSense" => GreenBuildingVerificationType::WaterSense,

"Zero Energy Ready Home" => GreenBuildingVerificationType::ZeroEnergyReadyHome,

_ => GreenBuildingVerificationType::OpenEnumeration(s),
}
    }
}

impl From<&str> for GreenBuildingVerificationType {
    fn from(s: &str) -> GreenBuildingVerificationType {
        match s {


"Certified Passive House" => GreenBuildingVerificationType::CertifiedPassiveHouse,

"ENERGY STAR Certified Homes" => GreenBuildingVerificationType::ENERGYSTARCertifiedHomes,

"EnerPHit" => GreenBuildingVerificationType::EnerPHit,

"HERS Index Score" => GreenBuildingVerificationType::HERSIndexScore,

"Home Energy Score" => GreenBuildingVerificationType::HomeEnergyScore,

"Home Energy Upgrade Certificate of Energy Efficiency Improvements" => GreenBuildingVerificationType::HomeEnergyUpgradeCertificateofEnergyEfficiencyImprovements,

"Home Energy Upgrade Certificate of Energy Efficiency Performance" => GreenBuildingVerificationType::HomeEnergyUpgradeCertificateofEnergyEfficiencyPerformance,

"Home Performance with ENERGY STAR" => GreenBuildingVerificationType::HomePerformancewithENERGYSTAR,

"Indoor airPLUS" => GreenBuildingVerificationType::IndoorairPLUS,

"LEED For Homes" => GreenBuildingVerificationType::LEEDForHomes,

"Living Building Challenge" => GreenBuildingVerificationType::LivingBuildingChallenge,

"NGBS New Construction" => GreenBuildingVerificationType::NGBSNewConstruction,

"NGBS Small Projects Remodel" => GreenBuildingVerificationType::NGBSSmallProjectsRemodel,

"NGBS Whole-Home Remodel" => GreenBuildingVerificationType::NGBSWholeHomeRemodel,

"PHIUS+" => GreenBuildingVerificationType::PHIUSPlus,

"WaterSense" => GreenBuildingVerificationType::WaterSense,

"Zero Energy Ready Home" => GreenBuildingVerificationType::ZeroEnergyReadyHome,

_ => GreenBuildingVerificationType::OpenEnumeration(s.into()),
}
    }
}

impl<'a> From<&'a GreenBuildingVerificationType> for &'a str {
    fn from(s: &'a GreenBuildingVerificationType) -> &'a str {
        match s {


GreenBuildingVerificationType::CertifiedPassiveHouse => "Certified Passive House",

GreenBuildingVerificationType::ENERGYSTARCertifiedHomes => "ENERGY STAR Certified Homes",

GreenBuildingVerificationType::EnerPHit => "EnerPHit",

GreenBuildingVerificationType::HERSIndexScore => "HERS Index Score",

GreenBuildingVerificationType::HomeEnergyScore => "Home Energy Score",

GreenBuildingVerificationType::HomeEnergyUpgradeCertificateofEnergyEfficiencyImprovements => "Home Energy Upgrade Certificate of Energy Efficiency Improvements",

GreenBuildingVerificationType::HomeEnergyUpgradeCertificateofEnergyEfficiencyPerformance => "Home Energy Upgrade Certificate of Energy Efficiency Performance",

GreenBuildingVerificationType::HomePerformancewithENERGYSTAR => "Home Performance with ENERGY STAR",

GreenBuildingVerificationType::IndoorairPLUS => "Indoor airPLUS",

GreenBuildingVerificationType::LEEDForHomes => "LEED For Homes",

GreenBuildingVerificationType::LivingBuildingChallenge => "Living Building Challenge",

GreenBuildingVerificationType::NGBSNewConstruction => "NGBS New Construction",

GreenBuildingVerificationType::NGBSSmallProjectsRemodel => "NGBS Small Projects Remodel",

GreenBuildingVerificationType::NGBSWholeHomeRemodel => "NGBS Whole-Home Remodel",

GreenBuildingVerificationType::PHIUSPlus => "PHIUS+",

GreenBuildingVerificationType::WaterSense => "WaterSense",

GreenBuildingVerificationType::ZeroEnergyReadyHome => "Zero Energy Ready Home",

GreenBuildingVerificationType::OpenEnumeration(s) => s,
}
    }
}

impl Serialize for GreenBuildingVerificationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for GreenBuildingVerificationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_green_building_verification_type_format {
    use super::GreenBuildingVerificationType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<GreenBuildingVerificationType>>,
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
    ) -> Result<Option<Vec<GreenBuildingVerificationType>>, D::Error>
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
