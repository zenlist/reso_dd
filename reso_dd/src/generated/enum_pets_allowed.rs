// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PetsAllowed Lookups](https://ddwiki.reso.org/display/DDW17/PetsAllowed+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PetsAllowed {
    /// "[Breed Restrictions](https://ddwiki.reso.org/display/DDW17/Breed+Restrictions)": There are breed restrictions on allowed pets.
    BreedRestrictions,

    /// "[Call](https://ddwiki.reso.org/display/DDW17/Call)": Call to inquire about pet restrictions.
    Call,

    /// "[Cats OK](https://ddwiki.reso.org/display/DDW17/Cats+OK)": Cats are allowed.
    CatsOK,

    /// "[Dogs OK](https://ddwiki.reso.org/display/DDW17/Dogs+OK)": Dogs are allowed.
    DogsOK,

    /// "[No](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245842)": No pets are allowed.
    No,

    /// "[Number Limit](https://ddwiki.reso.org/display/DDW17/Number+Limit)": There is a limit on the number of pets allowed.
    NumberLimit,

    /// "[Size Limit](https://ddwiki.reso.org/display/DDW17/Size+Limit)": There are size restrictions on allowed pets.
    SizeLimit,

    /// "[Yes](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245845)": All pets are allowed.
    Yes,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for PetsAllowed {
    fn from(s: String) -> PetsAllowed {
        match s.as_ref() {
            "Breed Restrictions" => PetsAllowed::BreedRestrictions,

            "Call" => PetsAllowed::Call,

            "Cats OK" => PetsAllowed::CatsOK,

            "Dogs OK" => PetsAllowed::DogsOK,

            "No" => PetsAllowed::No,

            "Number Limit" => PetsAllowed::NumberLimit,

            "Size Limit" => PetsAllowed::SizeLimit,

            "Yes" => PetsAllowed::Yes,

            _ => PetsAllowed::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PetsAllowed {
    fn from(s: &str) -> PetsAllowed {
        match s {
            "Breed Restrictions" => PetsAllowed::BreedRestrictions,

            "Call" => PetsAllowed::Call,

            "Cats OK" => PetsAllowed::CatsOK,

            "Dogs OK" => PetsAllowed::DogsOK,

            "No" => PetsAllowed::No,

            "Number Limit" => PetsAllowed::NumberLimit,

            "Size Limit" => PetsAllowed::SizeLimit,

            "Yes" => PetsAllowed::Yes,

            _ => PetsAllowed::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PetsAllowed> for &'a str {
    fn from(s: &'a PetsAllowed) -> &'a str {
        match s {
            PetsAllowed::BreedRestrictions => "Breed Restrictions",

            PetsAllowed::Call => "Call",

            PetsAllowed::CatsOK => "Cats OK",

            PetsAllowed::DogsOK => "Dogs OK",

            PetsAllowed::No => "No",

            PetsAllowed::NumberLimit => "Number Limit",

            PetsAllowed::SizeLimit => "Size Limit",

            PetsAllowed::Yes => "Yes",

            PetsAllowed::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PetsAllowed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PetsAllowed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_pets_allowed_format {
    use super::PetsAllowed;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<PetsAllowed>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<PetsAllowed>>, D::Error>
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
