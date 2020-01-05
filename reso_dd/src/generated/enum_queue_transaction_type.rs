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

impl crate::ResoEnumeration for QueueTransactionType {
    fn from_str(s: &str) -> QueueTransactionType {
        match s {
            "Add" => QueueTransactionType::Add,

            "Change" => QueueTransactionType::Change,

            "Delete" => QueueTransactionType::Delete,

            _ => QueueTransactionType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> QueueTransactionType {
        match s.as_ref() {
            "Add" => QueueTransactionType::Add,

            "Change" => QueueTransactionType::Change,

            "Delete" => QueueTransactionType::Delete,

            _ => QueueTransactionType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            QueueTransactionType::Add => "Add",

            QueueTransactionType::Change => "Change",

            QueueTransactionType::Delete => "Delete",

            QueueTransactionType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            QueueTransactionType::Add => "Add".into(),

            QueueTransactionType::Change => "Change".into(),

            QueueTransactionType::Delete => "Delete".into(),

            QueueTransactionType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            QueueTransactionType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
