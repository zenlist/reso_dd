// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [MediaType Lookups](https://ddwiki.reso.org/display/DDW17/MediaType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MediaType {
    /// "[doc](https://ddwiki.reso.org/display/DDW17/doc)": The media is a doc file type.
    Doc,

    /// "[docx](https://ddwiki.reso.org/display/DDW17/docx)": The media is a docx file type.
    Docx,

    /// "[gif](https://ddwiki.reso.org/display/DDW17/gif)": The media is a gif file type.
    Gif,

    /// "[jpeg](https://ddwiki.reso.org/display/DDW17/jpeg)": The media is a jpeg file type.
    Jpeg,

    /// "[mov](https://ddwiki.reso.org/display/DDW17/mov)": The media is a mov file type.
    Mov,

    /// "[mp4](https://ddwiki.reso.org/display/DDW17/mp4)": The media is an mp4 file type.
    Mp4,

    /// "[mpeg](https://ddwiki.reso.org/display/DDW17/mpeg)": The media is an mpeg file type.
    Mpeg,

    /// "[pdf](https://ddwiki.reso.org/display/DDW17/pdf)": The media is a pdf file type.
    Pdf,

    /// "[png](https://ddwiki.reso.org/display/DDW17/png)": The media is a png file type.
    Png,

    /// "[quicktime](https://ddwiki.reso.org/display/DDW17/quicktime)": The media is a QuickTime file type.
    Quicktime,

    /// "[rtf](https://ddwiki.reso.org/display/DDW17/rtf)": The media is a rtf file type.
    Rtf,

    /// "[tiff](https://ddwiki.reso.org/display/DDW17/tiff)": The media is a tiff file type.
    Tiff,

    /// "[txt](https://ddwiki.reso.org/display/DDW17/txt)": The media is a txt file type.
    Txt,

    /// "[wmv](https://ddwiki.reso.org/display/DDW17/wmv)": The media is a wmv file type.
    Wmv,

    /// "[xls](https://ddwiki.reso.org/display/DDW17/xls)": The media is a xls file type.
    Xls,

    /// "[xlsx](https://ddwiki.reso.org/display/DDW17/xlsx)": The media is a xlsx file type.
    Xlsx,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for MediaType {
    fn from_str(s: &str) -> MediaType {
        match s {
            "doc" => MediaType::Doc,

            "docx" => MediaType::Docx,

            "gif" => MediaType::Gif,

            "jpeg" => MediaType::Jpeg,

            "mov" => MediaType::Mov,

            "mp4" => MediaType::Mp4,

            "mpeg" => MediaType::Mpeg,

            "pdf" => MediaType::Pdf,

            "png" => MediaType::Png,

            "quicktime" => MediaType::Quicktime,

            "rtf" => MediaType::Rtf,

            "tiff" => MediaType::Tiff,

            "txt" => MediaType::Txt,

            "wmv" => MediaType::Wmv,

            "xls" => MediaType::Xls,

            "xlsx" => MediaType::Xlsx,

            _ => MediaType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> MediaType {
        match s.as_ref() {
            "doc" => MediaType::Doc,

            "docx" => MediaType::Docx,

            "gif" => MediaType::Gif,

            "jpeg" => MediaType::Jpeg,

            "mov" => MediaType::Mov,

            "mp4" => MediaType::Mp4,

            "mpeg" => MediaType::Mpeg,

            "pdf" => MediaType::Pdf,

            "png" => MediaType::Png,

            "quicktime" => MediaType::Quicktime,

            "rtf" => MediaType::Rtf,

            "tiff" => MediaType::Tiff,

            "txt" => MediaType::Txt,

            "wmv" => MediaType::Wmv,

            "xls" => MediaType::Xls,

            "xlsx" => MediaType::Xlsx,

            _ => MediaType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            MediaType::Doc => "doc",

            MediaType::Docx => "docx",

            MediaType::Gif => "gif",

            MediaType::Jpeg => "jpeg",

            MediaType::Mov => "mov",

            MediaType::Mp4 => "mp4",

            MediaType::Mpeg => "mpeg",

            MediaType::Pdf => "pdf",

            MediaType::Png => "png",

            MediaType::Quicktime => "quicktime",

            MediaType::Rtf => "rtf",

            MediaType::Tiff => "tiff",

            MediaType::Txt => "txt",

            MediaType::Wmv => "wmv",

            MediaType::Xls => "xls",

            MediaType::Xlsx => "xlsx",

            MediaType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            MediaType::Doc => "doc".into(),

            MediaType::Docx => "docx".into(),

            MediaType::Gif => "gif".into(),

            MediaType::Jpeg => "jpeg".into(),

            MediaType::Mov => "mov".into(),

            MediaType::Mp4 => "mp4".into(),

            MediaType::Mpeg => "mpeg".into(),

            MediaType::Pdf => "pdf".into(),

            MediaType::Png => "png".into(),

            MediaType::Quicktime => "quicktime".into(),

            MediaType::Rtf => "rtf".into(),

            MediaType::Tiff => "tiff".into(),

            MediaType::Txt => "txt".into(),

            MediaType::Wmv => "wmv".into(),

            MediaType::Xls => "xls".into(),

            MediaType::Xlsx => "xlsx".into(),

            MediaType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            MediaType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for MediaType {
    fn from(s: String) -> MediaType {
        match s.as_ref() {
            "doc" => MediaType::Doc,

            "docx" => MediaType::Docx,

            "gif" => MediaType::Gif,

            "jpeg" => MediaType::Jpeg,

            "mov" => MediaType::Mov,

            "mp4" => MediaType::Mp4,

            "mpeg" => MediaType::Mpeg,

            "pdf" => MediaType::Pdf,

            "png" => MediaType::Png,

            "quicktime" => MediaType::Quicktime,

            "rtf" => MediaType::Rtf,

            "tiff" => MediaType::Tiff,

            "txt" => MediaType::Txt,

            "wmv" => MediaType::Wmv,

            "xls" => MediaType::Xls,

            "xlsx" => MediaType::Xlsx,

            _ => MediaType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for MediaType {
    fn from(s: &str) -> MediaType {
        match s {
            "doc" => MediaType::Doc,

            "docx" => MediaType::Docx,

            "gif" => MediaType::Gif,

            "jpeg" => MediaType::Jpeg,

            "mov" => MediaType::Mov,

            "mp4" => MediaType::Mp4,

            "mpeg" => MediaType::Mpeg,

            "pdf" => MediaType::Pdf,

            "png" => MediaType::Png,

            "quicktime" => MediaType::Quicktime,

            "rtf" => MediaType::Rtf,

            "tiff" => MediaType::Tiff,

            "txt" => MediaType::Txt,

            "wmv" => MediaType::Wmv,

            "xls" => MediaType::Xls,

            "xlsx" => MediaType::Xlsx,

            _ => MediaType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a MediaType> for &'a str {
    fn from(s: &'a MediaType) -> &'a str {
        match s {
            MediaType::Doc => "doc",

            MediaType::Docx => "docx",

            MediaType::Gif => "gif",

            MediaType::Jpeg => "jpeg",

            MediaType::Mov => "mov",

            MediaType::Mp4 => "mp4",

            MediaType::Mpeg => "mpeg",

            MediaType::Pdf => "pdf",

            MediaType::Png => "png",

            MediaType::Quicktime => "quicktime",

            MediaType::Rtf => "rtf",

            MediaType::Tiff => "tiff",

            MediaType::Txt => "txt",

            MediaType::Wmv => "wmv",

            MediaType::Xls => "xls",

            MediaType::Xlsx => "xlsx",

            MediaType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
