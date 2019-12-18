// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [SpecialLicenses Lookups](https://ddwiki.reso.org/display/DDW17/SpecialLicenses+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpecialLicenses {
    /// "[Beer/Wine](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246295)": The business being sold uses/requires a Beer/Wine license.
    BeerWine,

    /// "[Class H](https://ddwiki.reso.org/display/DDW17/Class+H)": The business being sold uses/requires a Class H license.
    ClassH,

    /// "[Entertainment](https://ddwiki.reso.org/display/DDW17/Entertainment)": The business being sold uses/requires an Entertainment license.
    Entertainment,

    /// "[Franchise](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246298)": The business being sold uses/requires a Franchise license.
    Franchise,

    /// "[Gambling](https://ddwiki.reso.org/display/DDW17/Gambling)": The business being sold uses/requires a Gambling license.
    Gambling,

    /// "[Liquor](https://ddwiki.reso.org/display/DDW17/Liquor)": The business being sold uses/requires a Liquor license.
    Liquor,

    /// "[Liquor 5 Years Or Less](https://ddwiki.reso.org/display/DDW17/Liquor+5+Years+Or+Less)": The business being sold uses/requires a Liquor 5 years or less license.
    Liquor5YearsOrLess,

    /// "[Liquor 5 Years Or More](https://ddwiki.reso.org/display/DDW17/Liquor+5+Years+Or+More)": The business being sold uses/requires a Liquor 5 years or more license.
    Liquor5YearsOrMore,

    /// "[Liquor-Off Sale](https://ddwiki.reso.org/display/DDW17/Liquor-Off+Sale)": The business being sold uses/requires a Liquor-Off Sale license.
    LiquorOffSale,

    /// "[Liquor-On Sale](https://ddwiki.reso.org/display/DDW17/Liquor-On+Sale)": The business being sold uses/requires a Liquor-On Sale license.
    LiquorOnSale,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246305)": The business being sold uses/requires/has no license.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246306)": The business being sold uses/requires an other license.
    Other,

    /// "[Professional](https://ddwiki.reso.org/display/DDW17/Professional)": The business being sold uses/requires a Professional license.
    Professional,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for SpecialLicenses {
    fn from(s: String) -> SpecialLicenses {
        match s.as_ref() {
            "Beer/Wine" => SpecialLicenses::BeerWine,

            "Class H" => SpecialLicenses::ClassH,

            "Entertainment" => SpecialLicenses::Entertainment,

            "Franchise" => SpecialLicenses::Franchise,

            "Gambling" => SpecialLicenses::Gambling,

            "Liquor" => SpecialLicenses::Liquor,

            "Liquor 5 Years Or Less" => SpecialLicenses::Liquor5YearsOrLess,

            "Liquor 5 Years Or More" => SpecialLicenses::Liquor5YearsOrMore,

            "Liquor-Off Sale" => SpecialLicenses::LiquorOffSale,

            "Liquor-On Sale" => SpecialLicenses::LiquorOnSale,

            "None" => SpecialLicenses::None,

            "Other" => SpecialLicenses::Other,

            "Professional" => SpecialLicenses::Professional,

            _ => SpecialLicenses::OpenEnumeration(s),
        }
    }
}

impl From<&str> for SpecialLicenses {
    fn from(s: &str) -> SpecialLicenses {
        match s {
            "Beer/Wine" => SpecialLicenses::BeerWine,

            "Class H" => SpecialLicenses::ClassH,

            "Entertainment" => SpecialLicenses::Entertainment,

            "Franchise" => SpecialLicenses::Franchise,

            "Gambling" => SpecialLicenses::Gambling,

            "Liquor" => SpecialLicenses::Liquor,

            "Liquor 5 Years Or Less" => SpecialLicenses::Liquor5YearsOrLess,

            "Liquor 5 Years Or More" => SpecialLicenses::Liquor5YearsOrMore,

            "Liquor-Off Sale" => SpecialLicenses::LiquorOffSale,

            "Liquor-On Sale" => SpecialLicenses::LiquorOnSale,

            "None" => SpecialLicenses::None,

            "Other" => SpecialLicenses::Other,

            "Professional" => SpecialLicenses::Professional,

            _ => SpecialLicenses::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a SpecialLicenses> for &'a str {
    fn from(s: &'a SpecialLicenses) -> &'a str {
        match s {
            SpecialLicenses::BeerWine => "Beer/Wine",

            SpecialLicenses::ClassH => "Class H",

            SpecialLicenses::Entertainment => "Entertainment",

            SpecialLicenses::Franchise => "Franchise",

            SpecialLicenses::Gambling => "Gambling",

            SpecialLicenses::Liquor => "Liquor",

            SpecialLicenses::Liquor5YearsOrLess => "Liquor 5 Years Or Less",

            SpecialLicenses::Liquor5YearsOrMore => "Liquor 5 Years Or More",

            SpecialLicenses::LiquorOffSale => "Liquor-Off Sale",

            SpecialLicenses::LiquorOnSale => "Liquor-On Sale",

            SpecialLicenses::None => "None",

            SpecialLicenses::Other => "Other",

            SpecialLicenses::Professional => "Professional",

            SpecialLicenses::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for SpecialLicenses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SpecialLicenses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_special_licenses_format {
    use super::SpecialLicenses;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<SpecialLicenses>>,
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
    ) -> Result<Option<Vec<SpecialLicenses>>, D::Error>
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
