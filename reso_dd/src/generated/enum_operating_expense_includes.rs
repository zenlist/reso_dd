// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OperatingExpenseIncludes Lookups](https://ddwiki.reso.org/display/DDW17/OperatingExpenseIncludes+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OperatingExpenseIncludes {
    /// "[Accounting](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245673)": The operating expense amount includes accounting costs.
    Accounting,

    /// "[Advertising](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245674)": The operating expense amount includes advertising costs.
    Advertising,

    /// "[Association](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245675)": The operating expense amount includes association costs.
    Association,

    /// "[Cable TV](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245676)": The operating expense amount includes cable TV costs.
    CableTV,

    /// "[Capital Improvements](https://ddwiki.reso.org/display/DDW17/Capital+Improvements)": The operating expense amount includes capital improvements costs.
    CapitalImprovements,

    /// "[Depreciation](https://ddwiki.reso.org/display/DDW17/Depreciation)": The operating expense amount includes depreciation costs.
    Depreciation,

    /// "[Equipment Rental](https://ddwiki.reso.org/display/DDW17/Equipment+Rental)": The operating expense amount includes equipment rental costs.
    EquipmentRental,

    /// "[Fuel](https://ddwiki.reso.org/display/DDW17/Fuel)": The operating expense amount includes fuel costs.
    Fuel,

    /// "[Furniture Replacement](https://ddwiki.reso.org/display/DDW17/Furniture+Replacement)": The operating expense amount includes furniture replacement costs.
    FurnitureReplacement,

    /// "[Gardener](https://ddwiki.reso.org/display/DDW17/Gardener)": The operating expense amount includes gardener costs.
    Gardener,

    /// "[Insurance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245683)": The operating expense amount includes insurance costs.
    Insurance,

    /// "[Legal](https://ddwiki.reso.org/display/DDW17/Legal)": The operating expense amount includes legal costs.
    Legal,

    /// "[Licenses](https://ddwiki.reso.org/display/DDW17/Licenses)": The operating expense amount includes licenses costs.
    Licenses,

    /// "[Maintenance](https://ddwiki.reso.org/display/DDW17/Maintenance)": The operating expense amount includes maintenance costs.
    Maintenance,

    /// "[Maintenance Grounds](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245687)": The operating expense amount includes maintenance grounds costs.
    MaintenanceGrounds,

    /// "[Maintenance Structure](https://ddwiki.reso.org/display/DDW17/Maintenance+Structure)": The operating expense amount includes maintenance structure costs.
    MaintenanceStructure,

    /// "[Manager](https://ddwiki.reso.org/display/DDW17/Manager)": The operating expense amount includes manager costs.
    Manager,

    /// "[Mortgage/Loans](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245690)": The operating expense amount includes mortgage/loans costs.
    MortgageLoans,

    /// "[New Tax](https://ddwiki.reso.org/display/DDW17/New+Tax)": The operating expense amount includes new tax costs.
    NewTax,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245692)": The operating expense amount includes other costs.
    Other,

    /// "[Parking](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245693)": The operating expense amount includes parking costs.
    Parking,

    /// "[Pest Control](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245694)": The operating expense amount includes pest control costs.
    PestControl,

    /// "[Pool/Spa](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245695)": The operating expense amount includes pool/spa costs.
    PoolSpa,

    /// "[Professional Management](https://ddwiki.reso.org/display/DDW17/Professional+Management)": The operating expense amount includes professional management costs.
    ProfessionalManagement,

    /// "[Security](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245697)": The operating expense amount includes security costs.
    Security,

    /// "[Snow Removal](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245698)": The operating expense amount includes snow removal costs.
    SnowRemoval,

    /// "[Staff](https://ddwiki.reso.org/display/DDW17/Staff)": The operating expense amount includes staff costs.
    Staff,

    /// "[Supplies](https://ddwiki.reso.org/display/DDW17/Supplies)": The operating expense amount includes supplies costs.
    Supplies,

    /// "[Trash](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245701)": The operating expense amount includes trash costs.
    Trash,

    /// "[Utilities](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245702)": The operating expense amount includes utilities costs.
    Utilities,

    /// "[Vacancy Allowance](https://ddwiki.reso.org/display/DDW17/Vacancy+Allowance)": The operating expense amount includes vacancy allowance costs.
    VacancyAllowance,

    /// "[Water/Sewer](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245704)": The operating expense amount includes water/sewer costs.
    WaterSewer,

    /// "[Workmans Compensation](https://ddwiki.reso.org/display/DDW17/Workmans+Compensation)": The operating expense amount includes workman's compensation costs.
    WorkmansCompensation,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OperatingExpenseIncludes {
    fn from_str(s: &str) -> OperatingExpenseIncludes {
        match s {
            "Accounting" => OperatingExpenseIncludes::Accounting,

            "Advertising" => OperatingExpenseIncludes::Advertising,

            "Association" => OperatingExpenseIncludes::Association,

            "Cable TV" => OperatingExpenseIncludes::CableTV,

            "Capital Improvements" => OperatingExpenseIncludes::CapitalImprovements,

            "Depreciation" => OperatingExpenseIncludes::Depreciation,

            "Equipment Rental" => OperatingExpenseIncludes::EquipmentRental,

            "Fuel" => OperatingExpenseIncludes::Fuel,

            "Furniture Replacement" => OperatingExpenseIncludes::FurnitureReplacement,

            "Gardener" => OperatingExpenseIncludes::Gardener,

            "Insurance" => OperatingExpenseIncludes::Insurance,

            "Legal" => OperatingExpenseIncludes::Legal,

            "Licenses" => OperatingExpenseIncludes::Licenses,

            "Maintenance" => OperatingExpenseIncludes::Maintenance,

            "Maintenance Grounds" => OperatingExpenseIncludes::MaintenanceGrounds,

            "Maintenance Structure" => OperatingExpenseIncludes::MaintenanceStructure,

            "Manager" => OperatingExpenseIncludes::Manager,

            "Mortgage/Loans" => OperatingExpenseIncludes::MortgageLoans,

            "New Tax" => OperatingExpenseIncludes::NewTax,

            "Other" => OperatingExpenseIncludes::Other,

            "Parking" => OperatingExpenseIncludes::Parking,

            "Pest Control" => OperatingExpenseIncludes::PestControl,

            "Pool/Spa" => OperatingExpenseIncludes::PoolSpa,

            "Professional Management" => OperatingExpenseIncludes::ProfessionalManagement,

            "Security" => OperatingExpenseIncludes::Security,

            "Snow Removal" => OperatingExpenseIncludes::SnowRemoval,

            "Staff" => OperatingExpenseIncludes::Staff,

            "Supplies" => OperatingExpenseIncludes::Supplies,

            "Trash" => OperatingExpenseIncludes::Trash,

            "Utilities" => OperatingExpenseIncludes::Utilities,

            "Vacancy Allowance" => OperatingExpenseIncludes::VacancyAllowance,

            "Water/Sewer" => OperatingExpenseIncludes::WaterSewer,

            "Workmans Compensation" => OperatingExpenseIncludes::WorkmansCompensation,

            _ => OperatingExpenseIncludes::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OperatingExpenseIncludes {
        match s.as_ref() {
            "Accounting" => OperatingExpenseIncludes::Accounting,

            "Advertising" => OperatingExpenseIncludes::Advertising,

            "Association" => OperatingExpenseIncludes::Association,

            "Cable TV" => OperatingExpenseIncludes::CableTV,

            "Capital Improvements" => OperatingExpenseIncludes::CapitalImprovements,

            "Depreciation" => OperatingExpenseIncludes::Depreciation,

            "Equipment Rental" => OperatingExpenseIncludes::EquipmentRental,

            "Fuel" => OperatingExpenseIncludes::Fuel,

            "Furniture Replacement" => OperatingExpenseIncludes::FurnitureReplacement,

            "Gardener" => OperatingExpenseIncludes::Gardener,

            "Insurance" => OperatingExpenseIncludes::Insurance,

            "Legal" => OperatingExpenseIncludes::Legal,

            "Licenses" => OperatingExpenseIncludes::Licenses,

            "Maintenance" => OperatingExpenseIncludes::Maintenance,

            "Maintenance Grounds" => OperatingExpenseIncludes::MaintenanceGrounds,

            "Maintenance Structure" => OperatingExpenseIncludes::MaintenanceStructure,

            "Manager" => OperatingExpenseIncludes::Manager,

            "Mortgage/Loans" => OperatingExpenseIncludes::MortgageLoans,

            "New Tax" => OperatingExpenseIncludes::NewTax,

            "Other" => OperatingExpenseIncludes::Other,

            "Parking" => OperatingExpenseIncludes::Parking,

            "Pest Control" => OperatingExpenseIncludes::PestControl,

            "Pool/Spa" => OperatingExpenseIncludes::PoolSpa,

            "Professional Management" => OperatingExpenseIncludes::ProfessionalManagement,

            "Security" => OperatingExpenseIncludes::Security,

            "Snow Removal" => OperatingExpenseIncludes::SnowRemoval,

            "Staff" => OperatingExpenseIncludes::Staff,

            "Supplies" => OperatingExpenseIncludes::Supplies,

            "Trash" => OperatingExpenseIncludes::Trash,

            "Utilities" => OperatingExpenseIncludes::Utilities,

            "Vacancy Allowance" => OperatingExpenseIncludes::VacancyAllowance,

            "Water/Sewer" => OperatingExpenseIncludes::WaterSewer,

            "Workmans Compensation" => OperatingExpenseIncludes::WorkmansCompensation,

            _ => OperatingExpenseIncludes::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OperatingExpenseIncludes::Accounting => "Accounting",

            OperatingExpenseIncludes::Advertising => "Advertising",

            OperatingExpenseIncludes::Association => "Association",

            OperatingExpenseIncludes::CableTV => "Cable TV",

            OperatingExpenseIncludes::CapitalImprovements => "Capital Improvements",

            OperatingExpenseIncludes::Depreciation => "Depreciation",

            OperatingExpenseIncludes::EquipmentRental => "Equipment Rental",

            OperatingExpenseIncludes::Fuel => "Fuel",

            OperatingExpenseIncludes::FurnitureReplacement => "Furniture Replacement",

            OperatingExpenseIncludes::Gardener => "Gardener",

            OperatingExpenseIncludes::Insurance => "Insurance",

            OperatingExpenseIncludes::Legal => "Legal",

            OperatingExpenseIncludes::Licenses => "Licenses",

            OperatingExpenseIncludes::Maintenance => "Maintenance",

            OperatingExpenseIncludes::MaintenanceGrounds => "Maintenance Grounds",

            OperatingExpenseIncludes::MaintenanceStructure => "Maintenance Structure",

            OperatingExpenseIncludes::Manager => "Manager",

            OperatingExpenseIncludes::MortgageLoans => "Mortgage/Loans",

            OperatingExpenseIncludes::NewTax => "New Tax",

            OperatingExpenseIncludes::Other => "Other",

            OperatingExpenseIncludes::Parking => "Parking",

            OperatingExpenseIncludes::PestControl => "Pest Control",

            OperatingExpenseIncludes::PoolSpa => "Pool/Spa",

            OperatingExpenseIncludes::ProfessionalManagement => "Professional Management",

            OperatingExpenseIncludes::Security => "Security",

            OperatingExpenseIncludes::SnowRemoval => "Snow Removal",

            OperatingExpenseIncludes::Staff => "Staff",

            OperatingExpenseIncludes::Supplies => "Supplies",

            OperatingExpenseIncludes::Trash => "Trash",

            OperatingExpenseIncludes::Utilities => "Utilities",

            OperatingExpenseIncludes::VacancyAllowance => "Vacancy Allowance",

            OperatingExpenseIncludes::WaterSewer => "Water/Sewer",

            OperatingExpenseIncludes::WorkmansCompensation => "Workmans Compensation",

            OperatingExpenseIncludes::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OperatingExpenseIncludes::Accounting => "Accounting".into(),

            OperatingExpenseIncludes::Advertising => "Advertising".into(),

            OperatingExpenseIncludes::Association => "Association".into(),

            OperatingExpenseIncludes::CableTV => "Cable TV".into(),

            OperatingExpenseIncludes::CapitalImprovements => "Capital Improvements".into(),

            OperatingExpenseIncludes::Depreciation => "Depreciation".into(),

            OperatingExpenseIncludes::EquipmentRental => "Equipment Rental".into(),

            OperatingExpenseIncludes::Fuel => "Fuel".into(),

            OperatingExpenseIncludes::FurnitureReplacement => "Furniture Replacement".into(),

            OperatingExpenseIncludes::Gardener => "Gardener".into(),

            OperatingExpenseIncludes::Insurance => "Insurance".into(),

            OperatingExpenseIncludes::Legal => "Legal".into(),

            OperatingExpenseIncludes::Licenses => "Licenses".into(),

            OperatingExpenseIncludes::Maintenance => "Maintenance".into(),

            OperatingExpenseIncludes::MaintenanceGrounds => "Maintenance Grounds".into(),

            OperatingExpenseIncludes::MaintenanceStructure => "Maintenance Structure".into(),

            OperatingExpenseIncludes::Manager => "Manager".into(),

            OperatingExpenseIncludes::MortgageLoans => "Mortgage/Loans".into(),

            OperatingExpenseIncludes::NewTax => "New Tax".into(),

            OperatingExpenseIncludes::Other => "Other".into(),

            OperatingExpenseIncludes::Parking => "Parking".into(),

            OperatingExpenseIncludes::PestControl => "Pest Control".into(),

            OperatingExpenseIncludes::PoolSpa => "Pool/Spa".into(),

            OperatingExpenseIncludes::ProfessionalManagement => "Professional Management".into(),

            OperatingExpenseIncludes::Security => "Security".into(),

            OperatingExpenseIncludes::SnowRemoval => "Snow Removal".into(),

            OperatingExpenseIncludes::Staff => "Staff".into(),

            OperatingExpenseIncludes::Supplies => "Supplies".into(),

            OperatingExpenseIncludes::Trash => "Trash".into(),

            OperatingExpenseIncludes::Utilities => "Utilities".into(),

            OperatingExpenseIncludes::VacancyAllowance => "Vacancy Allowance".into(),

            OperatingExpenseIncludes::WaterSewer => "Water/Sewer".into(),

            OperatingExpenseIncludes::WorkmansCompensation => "Workmans Compensation".into(),

            OperatingExpenseIncludes::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OperatingExpenseIncludes::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OperatingExpenseIncludes {
    fn from(s: String) -> OperatingExpenseIncludes {
        match s.as_ref() {
            "Accounting" => OperatingExpenseIncludes::Accounting,

            "Advertising" => OperatingExpenseIncludes::Advertising,

            "Association" => OperatingExpenseIncludes::Association,

            "Cable TV" => OperatingExpenseIncludes::CableTV,

            "Capital Improvements" => OperatingExpenseIncludes::CapitalImprovements,

            "Depreciation" => OperatingExpenseIncludes::Depreciation,

            "Equipment Rental" => OperatingExpenseIncludes::EquipmentRental,

            "Fuel" => OperatingExpenseIncludes::Fuel,

            "Furniture Replacement" => OperatingExpenseIncludes::FurnitureReplacement,

            "Gardener" => OperatingExpenseIncludes::Gardener,

            "Insurance" => OperatingExpenseIncludes::Insurance,

            "Legal" => OperatingExpenseIncludes::Legal,

            "Licenses" => OperatingExpenseIncludes::Licenses,

            "Maintenance" => OperatingExpenseIncludes::Maintenance,

            "Maintenance Grounds" => OperatingExpenseIncludes::MaintenanceGrounds,

            "Maintenance Structure" => OperatingExpenseIncludes::MaintenanceStructure,

            "Manager" => OperatingExpenseIncludes::Manager,

            "Mortgage/Loans" => OperatingExpenseIncludes::MortgageLoans,

            "New Tax" => OperatingExpenseIncludes::NewTax,

            "Other" => OperatingExpenseIncludes::Other,

            "Parking" => OperatingExpenseIncludes::Parking,

            "Pest Control" => OperatingExpenseIncludes::PestControl,

            "Pool/Spa" => OperatingExpenseIncludes::PoolSpa,

            "Professional Management" => OperatingExpenseIncludes::ProfessionalManagement,

            "Security" => OperatingExpenseIncludes::Security,

            "Snow Removal" => OperatingExpenseIncludes::SnowRemoval,

            "Staff" => OperatingExpenseIncludes::Staff,

            "Supplies" => OperatingExpenseIncludes::Supplies,

            "Trash" => OperatingExpenseIncludes::Trash,

            "Utilities" => OperatingExpenseIncludes::Utilities,

            "Vacancy Allowance" => OperatingExpenseIncludes::VacancyAllowance,

            "Water/Sewer" => OperatingExpenseIncludes::WaterSewer,

            "Workmans Compensation" => OperatingExpenseIncludes::WorkmansCompensation,

            _ => OperatingExpenseIncludes::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OperatingExpenseIncludes {
    fn from(s: &str) -> OperatingExpenseIncludes {
        match s {
            "Accounting" => OperatingExpenseIncludes::Accounting,

            "Advertising" => OperatingExpenseIncludes::Advertising,

            "Association" => OperatingExpenseIncludes::Association,

            "Cable TV" => OperatingExpenseIncludes::CableTV,

            "Capital Improvements" => OperatingExpenseIncludes::CapitalImprovements,

            "Depreciation" => OperatingExpenseIncludes::Depreciation,

            "Equipment Rental" => OperatingExpenseIncludes::EquipmentRental,

            "Fuel" => OperatingExpenseIncludes::Fuel,

            "Furniture Replacement" => OperatingExpenseIncludes::FurnitureReplacement,

            "Gardener" => OperatingExpenseIncludes::Gardener,

            "Insurance" => OperatingExpenseIncludes::Insurance,

            "Legal" => OperatingExpenseIncludes::Legal,

            "Licenses" => OperatingExpenseIncludes::Licenses,

            "Maintenance" => OperatingExpenseIncludes::Maintenance,

            "Maintenance Grounds" => OperatingExpenseIncludes::MaintenanceGrounds,

            "Maintenance Structure" => OperatingExpenseIncludes::MaintenanceStructure,

            "Manager" => OperatingExpenseIncludes::Manager,

            "Mortgage/Loans" => OperatingExpenseIncludes::MortgageLoans,

            "New Tax" => OperatingExpenseIncludes::NewTax,

            "Other" => OperatingExpenseIncludes::Other,

            "Parking" => OperatingExpenseIncludes::Parking,

            "Pest Control" => OperatingExpenseIncludes::PestControl,

            "Pool/Spa" => OperatingExpenseIncludes::PoolSpa,

            "Professional Management" => OperatingExpenseIncludes::ProfessionalManagement,

            "Security" => OperatingExpenseIncludes::Security,

            "Snow Removal" => OperatingExpenseIncludes::SnowRemoval,

            "Staff" => OperatingExpenseIncludes::Staff,

            "Supplies" => OperatingExpenseIncludes::Supplies,

            "Trash" => OperatingExpenseIncludes::Trash,

            "Utilities" => OperatingExpenseIncludes::Utilities,

            "Vacancy Allowance" => OperatingExpenseIncludes::VacancyAllowance,

            "Water/Sewer" => OperatingExpenseIncludes::WaterSewer,

            "Workmans Compensation" => OperatingExpenseIncludes::WorkmansCompensation,

            _ => OperatingExpenseIncludes::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OperatingExpenseIncludes> for &'a str {
    fn from(s: &'a OperatingExpenseIncludes) -> &'a str {
        match s {
            OperatingExpenseIncludes::Accounting => "Accounting",

            OperatingExpenseIncludes::Advertising => "Advertising",

            OperatingExpenseIncludes::Association => "Association",

            OperatingExpenseIncludes::CableTV => "Cable TV",

            OperatingExpenseIncludes::CapitalImprovements => "Capital Improvements",

            OperatingExpenseIncludes::Depreciation => "Depreciation",

            OperatingExpenseIncludes::EquipmentRental => "Equipment Rental",

            OperatingExpenseIncludes::Fuel => "Fuel",

            OperatingExpenseIncludes::FurnitureReplacement => "Furniture Replacement",

            OperatingExpenseIncludes::Gardener => "Gardener",

            OperatingExpenseIncludes::Insurance => "Insurance",

            OperatingExpenseIncludes::Legal => "Legal",

            OperatingExpenseIncludes::Licenses => "Licenses",

            OperatingExpenseIncludes::Maintenance => "Maintenance",

            OperatingExpenseIncludes::MaintenanceGrounds => "Maintenance Grounds",

            OperatingExpenseIncludes::MaintenanceStructure => "Maintenance Structure",

            OperatingExpenseIncludes::Manager => "Manager",

            OperatingExpenseIncludes::MortgageLoans => "Mortgage/Loans",

            OperatingExpenseIncludes::NewTax => "New Tax",

            OperatingExpenseIncludes::Other => "Other",

            OperatingExpenseIncludes::Parking => "Parking",

            OperatingExpenseIncludes::PestControl => "Pest Control",

            OperatingExpenseIncludes::PoolSpa => "Pool/Spa",

            OperatingExpenseIncludes::ProfessionalManagement => "Professional Management",

            OperatingExpenseIncludes::Security => "Security",

            OperatingExpenseIncludes::SnowRemoval => "Snow Removal",

            OperatingExpenseIncludes::Staff => "Staff",

            OperatingExpenseIncludes::Supplies => "Supplies",

            OperatingExpenseIncludes::Trash => "Trash",

            OperatingExpenseIncludes::Utilities => "Utilities",

            OperatingExpenseIncludes::VacancyAllowance => "Vacancy Allowance",

            OperatingExpenseIncludes::WaterSewer => "Water/Sewer",

            OperatingExpenseIncludes::WorkmansCompensation => "Workmans Compensation",

            OperatingExpenseIncludes::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OperatingExpenseIncludes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OperatingExpenseIncludes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
