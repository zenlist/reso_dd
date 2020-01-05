/// A standard RESO enumeration set
pub trait ResoEnumeration {
    /// Convert from a `&str` into the enumeration.
    ///
    /// Note that if the `&str` isn't recognized, it will be cloned into a string for use as a
    /// fallback value.
    fn from_str(input: &str) -> Self;

    /// Convert from a `String` into the enumeration.
    ///
    /// If the `String` is recognized, it will be dropped. If it is not recognized, it will be kept
    /// as the fallback value.
    fn from_string(input: String) -> Self;

    /// Convert from the enumeration into a `&str`.
    fn to_str(&self) -> &str;

    /// Convert from the enumeration into a `String`.
    fn into_string(self) -> String;

    /// Get the fallback value, if the enum represents a fallback value.
    fn fallback_value(&self) -> Option<&str>;
}
