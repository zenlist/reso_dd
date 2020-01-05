// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [StateOrProvince Lookups](https://ddwiki.reso.org/display/DDW17/StateOrProvince+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StateOrProvince {
    /// "[AB](https://ddwiki.reso.org/display/DDW17/AB)": The Canadian province in which the listing is located is Alberta.
    AB,

    /// "[AK](https://ddwiki.reso.org/display/DDW17/AK)": The state in which the listing is located is Alaska.
    AK,

    /// "[AL](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246334)": The state in which the listing is located is Alabama.
    AL,

    /// "[AR](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246335)": The state in which the listing is located is Arkansas.
    AR,

    /// "[AZ](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246336)": The state in which the listing is located is Arizona.
    AZ,

    /// "[BC](https://ddwiki.reso.org/display/DDW17/BC)": The Canadian province in which the listing is located is British Columbia.
    BC,

    /// "[CA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246338)": The state in which the listing is located is California.
    CA,

    /// "[CO](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246339)": The state in which the listing is located is Colorado.
    CO,

    /// "[CT](https://ddwiki.reso.org/display/DDW17/CT)": The state in which the listing is located is Connecticut.
    CT,

    /// "[DC](https://ddwiki.reso.org/display/DDW17/DC)": The federal district in which the listing is located is District of Columbia.
    DC,

    /// "[DE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246342)": The state in which the listing is located is Delaware.
    DE,

    /// "[FL](https://ddwiki.reso.org/display/DDW17/FL)": The state in which the listing is located is Florida.
    FL,

    /// "[GA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246344)": The state in which the listing is located is Georgia.
    GA,

    /// "[HI](https://ddwiki.reso.org/display/DDW17/HI)": The state in which the listing is located is Hawaii.
    HI,

    /// "[IA](https://ddwiki.reso.org/display/DDW17/IA)": The state in which the listing is located is Iowa.
    IA,

    /// "[ID](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246347)": The state in which the listing is located is Idaho.
    ID,

    /// "[IL](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246348)": The state in which the listing is located is Illinois.
    IL,

    /// "[IN](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246349)": The state in which the listing is located is Indiana.
    IN,

    /// "[KS](https://ddwiki.reso.org/display/DDW17/KS)": The state in which the listing is located is Kansas.
    KS,

    /// "[KY](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246351)": The state in which the listing is located is Kentucky.
    KY,

    /// "[LA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246352)": The state in which the listing is located is Louisiana.
    LA,

    /// "[MA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246353)": The state in which the listing is located is Massachusetts.
    MA,

    /// "[MB](https://ddwiki.reso.org/display/DDW17/MB)": The Canadian province in which the listing is located is Manitoba.
    MB,

    /// "[MD](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246355)": The state in which the listing is located is Maryland.
    MD,

    /// "[ME](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246356)": The state in which the listing is located is Maine.
    ME,

    /// "[MI](https://ddwiki.reso.org/display/DDW17/MI)": The state in which the listing is located is Michigan.
    MI,

    /// "[MN](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246358)": The state in which the listing is located is Minnesota.
    MN,

    /// "[MO](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246359)": The state in which the listing is located is Missouri.
    MO,

    /// "[MS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246360)": The state in which the listing is located is Mississippi.
    MS,

    /// "[MT](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246361)": The state in which the listing is located is Montana.
    MT,

    /// "[NB](https://ddwiki.reso.org/display/DDW17/NB)": The Canadian province in which the listing is located is New Brunswick.
    NB,

    /// "[NC](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246363)": The state in which the listing is located is North Carolina.
    NC,

    /// "[ND](https://ddwiki.reso.org/display/DDW17/ND)": The state in which the listing is located is North Dakota.
    ND,

    /// "[NE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246365)": The state in which the listing is located is Nebraska.
    NE,

    /// "[NF](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246366)": The Canadian province in which the listing is located is Newfoundland and Labrador.
    NF,

    /// "[NH](https://ddwiki.reso.org/display/DDW17/NH)": The state in which the listing is located is New Hampshire.
    NH,

    /// "[NJ](https://ddwiki.reso.org/display/DDW17/NJ)": The state in which the listing is located is New Jersey.
    NJ,

    /// "[NM](https://ddwiki.reso.org/display/DDW17/NM)": The state in which the listing is located is New Mexico.
    NM,

    /// "[NS](https://ddwiki.reso.org/display/DDW17/NS)": The Canadian province in which the listing is located is Nova Scotia.
    NS,

    /// "[NT](https://ddwiki.reso.org/display/DDW17/NT)": The Canadian territory in which the listing is located is Northwest Territories.
    NT,

    /// "[NU](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246372)": The Canadian territory in which the listing is located is Nunavut.
    NU,

    /// "[NV](https://ddwiki.reso.org/display/DDW17/NV)": The state in which the listing is located is Nevada.
    NV,

    /// "[NY](https://ddwiki.reso.org/display/DDW17/NY)": The state in which the listing is located is New York.
    NY,

    /// "[OH](https://ddwiki.reso.org/display/DDW17/OH)": The state in which the listing is located is Ohio.
    OH,

    /// "[OK](https://ddwiki.reso.org/display/DDW17/OK)": The state in which the listing is located is Oklahoma.
    OK,

    /// "[ON](https://ddwiki.reso.org/display/DDW17/ON)": The Canadian province in which the listing is located is Ontario.
    ON,

    /// "[OR](https://ddwiki.reso.org/display/DDW17/OR)": The state in which the listing is located is Oregon.
    OR,

    /// "[PA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246379)": The state in which the listing is located is Pennsylvania.
    PA,

    /// "[PE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246380)": The Canadian province in which the listing is located is Prince Edward Island.
    PE,

    /// "[QC](https://ddwiki.reso.org/display/DDW17/QC)": The Canadian province in which the listing is located is Quebec.
    QC,

    /// "[RI](https://ddwiki.reso.org/display/DDW17/RI)": The state in which the listing is located is Rhode Island.
    RI,

    /// "[SC](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246383)": The state in which the listing is located is South Carolina.
    SC,

    /// "[SD](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246384)": The state in which the listing is located is South Dakota.
    SD,

    /// "[SK](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246385)": The Canadian province in which the listing is located is Saskatchewan.
    SK,

    /// "[TN](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246386)": The state in which the listing is located is Tennessee.
    TN,

    /// "[TX](https://ddwiki.reso.org/display/DDW17/TX)": The state in which the listing is located is Texas.
    TX,

    /// "[UT](https://ddwiki.reso.org/display/DDW17/UT)": The state in which the listing is located is Utah.
    UT,

    /// "[VA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246389)": The state in which the listing is located is Virginia.
    VA,

    /// "[VI](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246390)": The territory in which the listing is located is Virgin Islands.
    VI,

    /// "[VT](https://ddwiki.reso.org/display/DDW17/VT)": The state in which the listing is located is Vermont.
    VT,

    /// "[WA](https://ddwiki.reso.org/display/DDW17/WA)": The state in which the listing is located is Washington.
    WA,

    /// "[WI](https://ddwiki.reso.org/display/DDW17/WI)": The state in which the listing is located is Wisconsin.
    WI,

    /// "[WV](https://ddwiki.reso.org/display/DDW17/WV)": The state in which the listing is located is West Virginia.
    WV,

    /// "[WY](https://ddwiki.reso.org/display/DDW17/WY)": The state in which the listing is located is Wyoming.
    WY,

    /// "[YT](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246396)": The Canadian territory in which the listing is located is Yukon.
    YT,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for StateOrProvince {
    fn from_str(s: &str) -> StateOrProvince {
        match s {
            "AB" => StateOrProvince::AB,

            "AK" => StateOrProvince::AK,

            "AL" => StateOrProvince::AL,

            "AR" => StateOrProvince::AR,

            "AZ" => StateOrProvince::AZ,

            "BC" => StateOrProvince::BC,

            "CA" => StateOrProvince::CA,

            "CO" => StateOrProvince::CO,

            "CT" => StateOrProvince::CT,

            "DC" => StateOrProvince::DC,

            "DE" => StateOrProvince::DE,

            "FL" => StateOrProvince::FL,

            "GA" => StateOrProvince::GA,

            "HI" => StateOrProvince::HI,

            "IA" => StateOrProvince::IA,

            "ID" => StateOrProvince::ID,

            "IL" => StateOrProvince::IL,

            "IN" => StateOrProvince::IN,

            "KS" => StateOrProvince::KS,

            "KY" => StateOrProvince::KY,

            "LA" => StateOrProvince::LA,

            "MA" => StateOrProvince::MA,

            "MB" => StateOrProvince::MB,

            "MD" => StateOrProvince::MD,

            "ME" => StateOrProvince::ME,

            "MI" => StateOrProvince::MI,

            "MN" => StateOrProvince::MN,

            "MO" => StateOrProvince::MO,

            "MS" => StateOrProvince::MS,

            "MT" => StateOrProvince::MT,

            "NB" => StateOrProvince::NB,

            "NC" => StateOrProvince::NC,

            "ND" => StateOrProvince::ND,

            "NE" => StateOrProvince::NE,

            "NF" => StateOrProvince::NF,

            "NH" => StateOrProvince::NH,

            "NJ" => StateOrProvince::NJ,

            "NM" => StateOrProvince::NM,

            "NS" => StateOrProvince::NS,

            "NT" => StateOrProvince::NT,

            "NU" => StateOrProvince::NU,

            "NV" => StateOrProvince::NV,

            "NY" => StateOrProvince::NY,

            "OH" => StateOrProvince::OH,

            "OK" => StateOrProvince::OK,

            "ON" => StateOrProvince::ON,

            "OR" => StateOrProvince::OR,

            "PA" => StateOrProvince::PA,

            "PE" => StateOrProvince::PE,

            "QC" => StateOrProvince::QC,

            "RI" => StateOrProvince::RI,

            "SC" => StateOrProvince::SC,

            "SD" => StateOrProvince::SD,

            "SK" => StateOrProvince::SK,

            "TN" => StateOrProvince::TN,

            "TX" => StateOrProvince::TX,

            "UT" => StateOrProvince::UT,

            "VA" => StateOrProvince::VA,

            "VI" => StateOrProvince::VI,

            "VT" => StateOrProvince::VT,

            "WA" => StateOrProvince::WA,

            "WI" => StateOrProvince::WI,

            "WV" => StateOrProvince::WV,

            "WY" => StateOrProvince::WY,

            "YT" => StateOrProvince::YT,

            _ => StateOrProvince::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> StateOrProvince {
        match s.as_ref() {
            "AB" => StateOrProvince::AB,

            "AK" => StateOrProvince::AK,

            "AL" => StateOrProvince::AL,

            "AR" => StateOrProvince::AR,

            "AZ" => StateOrProvince::AZ,

            "BC" => StateOrProvince::BC,

            "CA" => StateOrProvince::CA,

            "CO" => StateOrProvince::CO,

            "CT" => StateOrProvince::CT,

            "DC" => StateOrProvince::DC,

            "DE" => StateOrProvince::DE,

            "FL" => StateOrProvince::FL,

            "GA" => StateOrProvince::GA,

            "HI" => StateOrProvince::HI,

            "IA" => StateOrProvince::IA,

            "ID" => StateOrProvince::ID,

            "IL" => StateOrProvince::IL,

            "IN" => StateOrProvince::IN,

            "KS" => StateOrProvince::KS,

            "KY" => StateOrProvince::KY,

            "LA" => StateOrProvince::LA,

            "MA" => StateOrProvince::MA,

            "MB" => StateOrProvince::MB,

            "MD" => StateOrProvince::MD,

            "ME" => StateOrProvince::ME,

            "MI" => StateOrProvince::MI,

            "MN" => StateOrProvince::MN,

            "MO" => StateOrProvince::MO,

            "MS" => StateOrProvince::MS,

            "MT" => StateOrProvince::MT,

            "NB" => StateOrProvince::NB,

            "NC" => StateOrProvince::NC,

            "ND" => StateOrProvince::ND,

            "NE" => StateOrProvince::NE,

            "NF" => StateOrProvince::NF,

            "NH" => StateOrProvince::NH,

            "NJ" => StateOrProvince::NJ,

            "NM" => StateOrProvince::NM,

            "NS" => StateOrProvince::NS,

            "NT" => StateOrProvince::NT,

            "NU" => StateOrProvince::NU,

            "NV" => StateOrProvince::NV,

            "NY" => StateOrProvince::NY,

            "OH" => StateOrProvince::OH,

            "OK" => StateOrProvince::OK,

            "ON" => StateOrProvince::ON,

            "OR" => StateOrProvince::OR,

            "PA" => StateOrProvince::PA,

            "PE" => StateOrProvince::PE,

            "QC" => StateOrProvince::QC,

            "RI" => StateOrProvince::RI,

            "SC" => StateOrProvince::SC,

            "SD" => StateOrProvince::SD,

            "SK" => StateOrProvince::SK,

            "TN" => StateOrProvince::TN,

            "TX" => StateOrProvince::TX,

            "UT" => StateOrProvince::UT,

            "VA" => StateOrProvince::VA,

            "VI" => StateOrProvince::VI,

            "VT" => StateOrProvince::VT,

            "WA" => StateOrProvince::WA,

            "WI" => StateOrProvince::WI,

            "WV" => StateOrProvince::WV,

            "WY" => StateOrProvince::WY,

            "YT" => StateOrProvince::YT,

            _ => StateOrProvince::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            StateOrProvince::AB => "AB",

            StateOrProvince::AK => "AK",

            StateOrProvince::AL => "AL",

            StateOrProvince::AR => "AR",

            StateOrProvince::AZ => "AZ",

            StateOrProvince::BC => "BC",

            StateOrProvince::CA => "CA",

            StateOrProvince::CO => "CO",

            StateOrProvince::CT => "CT",

            StateOrProvince::DC => "DC",

            StateOrProvince::DE => "DE",

            StateOrProvince::FL => "FL",

            StateOrProvince::GA => "GA",

            StateOrProvince::HI => "HI",

            StateOrProvince::IA => "IA",

            StateOrProvince::ID => "ID",

            StateOrProvince::IL => "IL",

            StateOrProvince::IN => "IN",

            StateOrProvince::KS => "KS",

            StateOrProvince::KY => "KY",

            StateOrProvince::LA => "LA",

            StateOrProvince::MA => "MA",

            StateOrProvince::MB => "MB",

            StateOrProvince::MD => "MD",

            StateOrProvince::ME => "ME",

            StateOrProvince::MI => "MI",

            StateOrProvince::MN => "MN",

            StateOrProvince::MO => "MO",

            StateOrProvince::MS => "MS",

            StateOrProvince::MT => "MT",

            StateOrProvince::NB => "NB",

            StateOrProvince::NC => "NC",

            StateOrProvince::ND => "ND",

            StateOrProvince::NE => "NE",

            StateOrProvince::NF => "NF",

            StateOrProvince::NH => "NH",

            StateOrProvince::NJ => "NJ",

            StateOrProvince::NM => "NM",

            StateOrProvince::NS => "NS",

            StateOrProvince::NT => "NT",

            StateOrProvince::NU => "NU",

            StateOrProvince::NV => "NV",

            StateOrProvince::NY => "NY",

            StateOrProvince::OH => "OH",

            StateOrProvince::OK => "OK",

            StateOrProvince::ON => "ON",

            StateOrProvince::OR => "OR",

            StateOrProvince::PA => "PA",

            StateOrProvince::PE => "PE",

            StateOrProvince::QC => "QC",

            StateOrProvince::RI => "RI",

            StateOrProvince::SC => "SC",

            StateOrProvince::SD => "SD",

            StateOrProvince::SK => "SK",

            StateOrProvince::TN => "TN",

            StateOrProvince::TX => "TX",

            StateOrProvince::UT => "UT",

            StateOrProvince::VA => "VA",

            StateOrProvince::VI => "VI",

            StateOrProvince::VT => "VT",

            StateOrProvince::WA => "WA",

            StateOrProvince::WI => "WI",

            StateOrProvince::WV => "WV",

            StateOrProvince::WY => "WY",

            StateOrProvince::YT => "YT",

            StateOrProvince::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            StateOrProvince::AB => "AB".into(),

            StateOrProvince::AK => "AK".into(),

            StateOrProvince::AL => "AL".into(),

            StateOrProvince::AR => "AR".into(),

            StateOrProvince::AZ => "AZ".into(),

            StateOrProvince::BC => "BC".into(),

            StateOrProvince::CA => "CA".into(),

            StateOrProvince::CO => "CO".into(),

            StateOrProvince::CT => "CT".into(),

            StateOrProvince::DC => "DC".into(),

            StateOrProvince::DE => "DE".into(),

            StateOrProvince::FL => "FL".into(),

            StateOrProvince::GA => "GA".into(),

            StateOrProvince::HI => "HI".into(),

            StateOrProvince::IA => "IA".into(),

            StateOrProvince::ID => "ID".into(),

            StateOrProvince::IL => "IL".into(),

            StateOrProvince::IN => "IN".into(),

            StateOrProvince::KS => "KS".into(),

            StateOrProvince::KY => "KY".into(),

            StateOrProvince::LA => "LA".into(),

            StateOrProvince::MA => "MA".into(),

            StateOrProvince::MB => "MB".into(),

            StateOrProvince::MD => "MD".into(),

            StateOrProvince::ME => "ME".into(),

            StateOrProvince::MI => "MI".into(),

            StateOrProvince::MN => "MN".into(),

            StateOrProvince::MO => "MO".into(),

            StateOrProvince::MS => "MS".into(),

            StateOrProvince::MT => "MT".into(),

            StateOrProvince::NB => "NB".into(),

            StateOrProvince::NC => "NC".into(),

            StateOrProvince::ND => "ND".into(),

            StateOrProvince::NE => "NE".into(),

            StateOrProvince::NF => "NF".into(),

            StateOrProvince::NH => "NH".into(),

            StateOrProvince::NJ => "NJ".into(),

            StateOrProvince::NM => "NM".into(),

            StateOrProvince::NS => "NS".into(),

            StateOrProvince::NT => "NT".into(),

            StateOrProvince::NU => "NU".into(),

            StateOrProvince::NV => "NV".into(),

            StateOrProvince::NY => "NY".into(),

            StateOrProvince::OH => "OH".into(),

            StateOrProvince::OK => "OK".into(),

            StateOrProvince::ON => "ON".into(),

            StateOrProvince::OR => "OR".into(),

            StateOrProvince::PA => "PA".into(),

            StateOrProvince::PE => "PE".into(),

            StateOrProvince::QC => "QC".into(),

            StateOrProvince::RI => "RI".into(),

            StateOrProvince::SC => "SC".into(),

            StateOrProvince::SD => "SD".into(),

            StateOrProvince::SK => "SK".into(),

            StateOrProvince::TN => "TN".into(),

            StateOrProvince::TX => "TX".into(),

            StateOrProvince::UT => "UT".into(),

            StateOrProvince::VA => "VA".into(),

            StateOrProvince::VI => "VI".into(),

            StateOrProvince::VT => "VT".into(),

            StateOrProvince::WA => "WA".into(),

            StateOrProvince::WI => "WI".into(),

            StateOrProvince::WV => "WV".into(),

            StateOrProvince::WY => "WY".into(),

            StateOrProvince::YT => "YT".into(),

            StateOrProvince::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            StateOrProvince::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for StateOrProvince {
    fn from(s: String) -> StateOrProvince {
        match s.as_ref() {
            "AB" => StateOrProvince::AB,

            "AK" => StateOrProvince::AK,

            "AL" => StateOrProvince::AL,

            "AR" => StateOrProvince::AR,

            "AZ" => StateOrProvince::AZ,

            "BC" => StateOrProvince::BC,

            "CA" => StateOrProvince::CA,

            "CO" => StateOrProvince::CO,

            "CT" => StateOrProvince::CT,

            "DC" => StateOrProvince::DC,

            "DE" => StateOrProvince::DE,

            "FL" => StateOrProvince::FL,

            "GA" => StateOrProvince::GA,

            "HI" => StateOrProvince::HI,

            "IA" => StateOrProvince::IA,

            "ID" => StateOrProvince::ID,

            "IL" => StateOrProvince::IL,

            "IN" => StateOrProvince::IN,

            "KS" => StateOrProvince::KS,

            "KY" => StateOrProvince::KY,

            "LA" => StateOrProvince::LA,

            "MA" => StateOrProvince::MA,

            "MB" => StateOrProvince::MB,

            "MD" => StateOrProvince::MD,

            "ME" => StateOrProvince::ME,

            "MI" => StateOrProvince::MI,

            "MN" => StateOrProvince::MN,

            "MO" => StateOrProvince::MO,

            "MS" => StateOrProvince::MS,

            "MT" => StateOrProvince::MT,

            "NB" => StateOrProvince::NB,

            "NC" => StateOrProvince::NC,

            "ND" => StateOrProvince::ND,

            "NE" => StateOrProvince::NE,

            "NF" => StateOrProvince::NF,

            "NH" => StateOrProvince::NH,

            "NJ" => StateOrProvince::NJ,

            "NM" => StateOrProvince::NM,

            "NS" => StateOrProvince::NS,

            "NT" => StateOrProvince::NT,

            "NU" => StateOrProvince::NU,

            "NV" => StateOrProvince::NV,

            "NY" => StateOrProvince::NY,

            "OH" => StateOrProvince::OH,

            "OK" => StateOrProvince::OK,

            "ON" => StateOrProvince::ON,

            "OR" => StateOrProvince::OR,

            "PA" => StateOrProvince::PA,

            "PE" => StateOrProvince::PE,

            "QC" => StateOrProvince::QC,

            "RI" => StateOrProvince::RI,

            "SC" => StateOrProvince::SC,

            "SD" => StateOrProvince::SD,

            "SK" => StateOrProvince::SK,

            "TN" => StateOrProvince::TN,

            "TX" => StateOrProvince::TX,

            "UT" => StateOrProvince::UT,

            "VA" => StateOrProvince::VA,

            "VI" => StateOrProvince::VI,

            "VT" => StateOrProvince::VT,

            "WA" => StateOrProvince::WA,

            "WI" => StateOrProvince::WI,

            "WV" => StateOrProvince::WV,

            "WY" => StateOrProvince::WY,

            "YT" => StateOrProvince::YT,

            _ => StateOrProvince::OpenEnumeration(s),
        }
    }
}

impl From<&str> for StateOrProvince {
    fn from(s: &str) -> StateOrProvince {
        match s {
            "AB" => StateOrProvince::AB,

            "AK" => StateOrProvince::AK,

            "AL" => StateOrProvince::AL,

            "AR" => StateOrProvince::AR,

            "AZ" => StateOrProvince::AZ,

            "BC" => StateOrProvince::BC,

            "CA" => StateOrProvince::CA,

            "CO" => StateOrProvince::CO,

            "CT" => StateOrProvince::CT,

            "DC" => StateOrProvince::DC,

            "DE" => StateOrProvince::DE,

            "FL" => StateOrProvince::FL,

            "GA" => StateOrProvince::GA,

            "HI" => StateOrProvince::HI,

            "IA" => StateOrProvince::IA,

            "ID" => StateOrProvince::ID,

            "IL" => StateOrProvince::IL,

            "IN" => StateOrProvince::IN,

            "KS" => StateOrProvince::KS,

            "KY" => StateOrProvince::KY,

            "LA" => StateOrProvince::LA,

            "MA" => StateOrProvince::MA,

            "MB" => StateOrProvince::MB,

            "MD" => StateOrProvince::MD,

            "ME" => StateOrProvince::ME,

            "MI" => StateOrProvince::MI,

            "MN" => StateOrProvince::MN,

            "MO" => StateOrProvince::MO,

            "MS" => StateOrProvince::MS,

            "MT" => StateOrProvince::MT,

            "NB" => StateOrProvince::NB,

            "NC" => StateOrProvince::NC,

            "ND" => StateOrProvince::ND,

            "NE" => StateOrProvince::NE,

            "NF" => StateOrProvince::NF,

            "NH" => StateOrProvince::NH,

            "NJ" => StateOrProvince::NJ,

            "NM" => StateOrProvince::NM,

            "NS" => StateOrProvince::NS,

            "NT" => StateOrProvince::NT,

            "NU" => StateOrProvince::NU,

            "NV" => StateOrProvince::NV,

            "NY" => StateOrProvince::NY,

            "OH" => StateOrProvince::OH,

            "OK" => StateOrProvince::OK,

            "ON" => StateOrProvince::ON,

            "OR" => StateOrProvince::OR,

            "PA" => StateOrProvince::PA,

            "PE" => StateOrProvince::PE,

            "QC" => StateOrProvince::QC,

            "RI" => StateOrProvince::RI,

            "SC" => StateOrProvince::SC,

            "SD" => StateOrProvince::SD,

            "SK" => StateOrProvince::SK,

            "TN" => StateOrProvince::TN,

            "TX" => StateOrProvince::TX,

            "UT" => StateOrProvince::UT,

            "VA" => StateOrProvince::VA,

            "VI" => StateOrProvince::VI,

            "VT" => StateOrProvince::VT,

            "WA" => StateOrProvince::WA,

            "WI" => StateOrProvince::WI,

            "WV" => StateOrProvince::WV,

            "WY" => StateOrProvince::WY,

            "YT" => StateOrProvince::YT,

            _ => StateOrProvince::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a StateOrProvince> for &'a str {
    fn from(s: &'a StateOrProvince) -> &'a str {
        match s {
            StateOrProvince::AB => "AB",

            StateOrProvince::AK => "AK",

            StateOrProvince::AL => "AL",

            StateOrProvince::AR => "AR",

            StateOrProvince::AZ => "AZ",

            StateOrProvince::BC => "BC",

            StateOrProvince::CA => "CA",

            StateOrProvince::CO => "CO",

            StateOrProvince::CT => "CT",

            StateOrProvince::DC => "DC",

            StateOrProvince::DE => "DE",

            StateOrProvince::FL => "FL",

            StateOrProvince::GA => "GA",

            StateOrProvince::HI => "HI",

            StateOrProvince::IA => "IA",

            StateOrProvince::ID => "ID",

            StateOrProvince::IL => "IL",

            StateOrProvince::IN => "IN",

            StateOrProvince::KS => "KS",

            StateOrProvince::KY => "KY",

            StateOrProvince::LA => "LA",

            StateOrProvince::MA => "MA",

            StateOrProvince::MB => "MB",

            StateOrProvince::MD => "MD",

            StateOrProvince::ME => "ME",

            StateOrProvince::MI => "MI",

            StateOrProvince::MN => "MN",

            StateOrProvince::MO => "MO",

            StateOrProvince::MS => "MS",

            StateOrProvince::MT => "MT",

            StateOrProvince::NB => "NB",

            StateOrProvince::NC => "NC",

            StateOrProvince::ND => "ND",

            StateOrProvince::NE => "NE",

            StateOrProvince::NF => "NF",

            StateOrProvince::NH => "NH",

            StateOrProvince::NJ => "NJ",

            StateOrProvince::NM => "NM",

            StateOrProvince::NS => "NS",

            StateOrProvince::NT => "NT",

            StateOrProvince::NU => "NU",

            StateOrProvince::NV => "NV",

            StateOrProvince::NY => "NY",

            StateOrProvince::OH => "OH",

            StateOrProvince::OK => "OK",

            StateOrProvince::ON => "ON",

            StateOrProvince::OR => "OR",

            StateOrProvince::PA => "PA",

            StateOrProvince::PE => "PE",

            StateOrProvince::QC => "QC",

            StateOrProvince::RI => "RI",

            StateOrProvince::SC => "SC",

            StateOrProvince::SD => "SD",

            StateOrProvince::SK => "SK",

            StateOrProvince::TN => "TN",

            StateOrProvince::TX => "TX",

            StateOrProvince::UT => "UT",

            StateOrProvince::VA => "VA",

            StateOrProvince::VI => "VI",

            StateOrProvince::VT => "VT",

            StateOrProvince::WA => "WA",

            StateOrProvince::WI => "WI",

            StateOrProvince::WV => "WV",

            StateOrProvince::WY => "WY",

            StateOrProvince::YT => "YT",

            StateOrProvince::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for StateOrProvince {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StateOrProvince {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
