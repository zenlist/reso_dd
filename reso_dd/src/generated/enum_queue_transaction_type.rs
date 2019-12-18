// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [QueueTransactionType Lookups](https://ddwiki.reso.org/display/DDW17/QueueTransactionType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QueueTransactionType {
    /// "[Add](https://ddwiki.reso.org/display/DDW17/Add)": The resource record being referenced by the queue does not yet exist in the target and is an addition.
    Add,

    /// "[Change](https://ddwiki.reso.org/display/DDW17/Change)": The resource record being referenced by the queue exists in the target and is being modified.
    Change,

    /// "[Delete](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246733)": The resource record being referenced by the queue exists in the target and is to be removed.
    Delete,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for QueueTransactionType {
    fn from(s: String) -> QueueTransactionType {
        match s.as_ref() {
            "Add" => QueueTransactionType::Add,

            "Change" => QueueTransactionType::Change,

            "Delete" => QueueTransactionType::Delete,

            _ => QueueTransactionType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for QueueTransactionType {
    fn from(s: &str) -> QueueTransactionType {
        match s {
            "Add" => QueueTransactionType::Add,

            "Change" => QueueTransactionType::Change,

            "Delete" => QueueTransactionType::Delete,

            _ => QueueTransactionType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a QueueTransactionType> for &'a str {
    fn from(s: &'a QueueTransactionType) -> &'a str {
        match s {
            QueueTransactionType::Add => "Add",

            QueueTransactionType::Change => "Change",

            QueueTransactionType::Delete => "Delete",

            QueueTransactionType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for QueueTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for QueueTransactionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_queue_transaction_type_format {
    use super::QueueTransactionType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<QueueTransactionType>>,
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
    ) -> Result<Option<Vec<QueueTransactionType>>, D::Error>
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
