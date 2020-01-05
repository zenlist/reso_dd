use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn snake_case(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[A-Z]+").unwrap();
    }
    let owned = RE
        .replace_all(s, |caps: &Captures| format!("_{}", caps[0].to_lowercase()))
        .into_owned();

    if let Some('_') = owned.chars().nth(0) {
        (&owned[1..]).into()
    } else {
        owned
    }
}
