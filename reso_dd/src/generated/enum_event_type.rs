// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [EventType Lookups](https://ddwiki.reso.org/display/DDW17/EventType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EventType {
    /// "[Click to Primary Hosted Site](https://ddwiki.reso.org/display/DDW17/Click+to+Primary+Hosted+Site)": The Actor was referred to the Object's primary hosted website
    ClicktoPrimaryHostedSite,

    /// "[Clicked on Email Address](https://ddwiki.reso.org/display/DDW17/Clicked+on+Email+Address)": The Actor engaged in the act of emailing to the Object's email address (note: does not indicate an email was sent)
    ClickedonEmailAddress,

    /// "[Clicked on Phone Number](https://ddwiki.reso.org/display/DDW17/Clicked+on+Phone+Number)": The Actor clicked on a phone number link associated with the Object
    ClickedonPhoneNumber,

    /// "[Comments](https://ddwiki.reso.org/display/DDW17/Comments)": Comments were made on the Object
    Comments,

    /// "[Detailed View](https://ddwiki.reso.org/display/DDW17/Detailed+View)": The object was the main focal point in the Actor's view.
    DetailedView,

    /// "[Discard](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244665)": The Actor has reacted "negatively" to the object.
    Discard,

    /// "[Driving Directions](https://ddwiki.reso.org/display/DDW17/Driving+Directions)": The Actor engaged in driving directions with the Object
    DrivingDirections,

    /// "[Exit Detailed View](https://ddwiki.reso.org/display/DDW17/Exit+Detailed+View)": The actor left the detailed view.
    ExitDetailedView,

    /// "[Favorited](https://ddwiki.reso.org/display/DDW17/Favorited)": The Actor has reacted "positively" to the object.
    Favorited,

    /// "[Maybe](https://ddwiki.reso.org/display/DDW17/Maybe)": The Actor has reacted "possibly positive" to the object.
    Maybe,

    /// "[Non-Detailed View](https://ddwiki.reso.org/display/DDW17/Non-Detailed+View)": The object appeared in the Actor's view but was not the main focal point.®
    NonDetailedView,

    /// "[Object Modified](https://ddwiki.reso.org/display/DDW17/Object+Modified)": The tracking Obect was modified in some way.
    ObjectModified,

    /// "[Photo Gallery](https://ddwiki.reso.org/display/DDW17/Photo+Gallery)": The Actor participated in a photo gallery display
    PhotoGallery,

    /// "[Printed](https://ddwiki.reso.org/display/DDW17/Printed)": The Actor printed the object
    Printed,

    /// "[Property Videos](https://ddwiki.reso.org/display/DDW17/Property+Videos)": The Actor has interacted with a property video with the Object
    PropertyVideos,

    /// "[Search](https://ddwiki.reso.org/display/DDW17/Search)": The tracking Object is data is part of a search and will contain more than one result.  Normally, the search result data will be presented in the ObjectCollection field and include ALL the listings that were part of the search.
    Search,

    /// "[Share](https://ddwiki.reso.org/display/DDW17/Share)": The sharing of a listing to another media or entity (includes social media sites, IM's, email and SMS messages)
    Share,

    /// "[Submission of Lead Form](https://ddwiki.reso.org/display/DDW17/Submission+of+Lead+Form)": The Actor has submitted a lead form
    SubmissionofLeadForm,

    /// "[Virtual Tour](https://ddwiki.reso.org/display/DDW17/Virtual+Tour)": The Actor viewed the Object's virtual tour
    VirtualTour,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for EventType {
    fn from(s: String) -> EventType {
        match s.as_ref() {
            "Click to Primary Hosted Site" => EventType::ClicktoPrimaryHostedSite,

            "Clicked on Email Address" => EventType::ClickedonEmailAddress,

            "Clicked on Phone Number" => EventType::ClickedonPhoneNumber,

            "Comments" => EventType::Comments,

            "Detailed View" => EventType::DetailedView,

            "Discard" => EventType::Discard,

            "Driving Directions" => EventType::DrivingDirections,

            "Exit Detailed View" => EventType::ExitDetailedView,

            "Favorited" => EventType::Favorited,

            "Maybe" => EventType::Maybe,

            "Non-Detailed View" => EventType::NonDetailedView,

            "Object Modified" => EventType::ObjectModified,

            "Photo Gallery" => EventType::PhotoGallery,

            "Printed" => EventType::Printed,

            "Property Videos" => EventType::PropertyVideos,

            "Search" => EventType::Search,

            "Share" => EventType::Share,

            "Submission of Lead Form" => EventType::SubmissionofLeadForm,

            "Virtual Tour" => EventType::VirtualTour,

            _ => EventType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for EventType {
    fn from(s: &str) -> EventType {
        match s {
            "Click to Primary Hosted Site" => EventType::ClicktoPrimaryHostedSite,

            "Clicked on Email Address" => EventType::ClickedonEmailAddress,

            "Clicked on Phone Number" => EventType::ClickedonPhoneNumber,

            "Comments" => EventType::Comments,

            "Detailed View" => EventType::DetailedView,

            "Discard" => EventType::Discard,

            "Driving Directions" => EventType::DrivingDirections,

            "Exit Detailed View" => EventType::ExitDetailedView,

            "Favorited" => EventType::Favorited,

            "Maybe" => EventType::Maybe,

            "Non-Detailed View" => EventType::NonDetailedView,

            "Object Modified" => EventType::ObjectModified,

            "Photo Gallery" => EventType::PhotoGallery,

            "Printed" => EventType::Printed,

            "Property Videos" => EventType::PropertyVideos,

            "Search" => EventType::Search,

            "Share" => EventType::Share,

            "Submission of Lead Form" => EventType::SubmissionofLeadForm,

            "Virtual Tour" => EventType::VirtualTour,

            _ => EventType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a EventType> for &'a str {
    fn from(s: &'a EventType) -> &'a str {
        match s {
            EventType::ClicktoPrimaryHostedSite => "Click to Primary Hosted Site",

            EventType::ClickedonEmailAddress => "Clicked on Email Address",

            EventType::ClickedonPhoneNumber => "Clicked on Phone Number",

            EventType::Comments => "Comments",

            EventType::DetailedView => "Detailed View",

            EventType::Discard => "Discard",

            EventType::DrivingDirections => "Driving Directions",

            EventType::ExitDetailedView => "Exit Detailed View",

            EventType::Favorited => "Favorited",

            EventType::Maybe => "Maybe",

            EventType::NonDetailedView => "Non-Detailed View",

            EventType::ObjectModified => "Object Modified",

            EventType::PhotoGallery => "Photo Gallery",

            EventType::Printed => "Printed",

            EventType::PropertyVideos => "Property Videos",

            EventType::Search => "Search",

            EventType::Share => "Share",

            EventType::SubmissionofLeadForm => "Submission of Lead Form",

            EventType::VirtualTour => "Virtual Tour",

            EventType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_event_type_format {
    use super::EventType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<EventType>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<EventType>>, D::Error>
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
