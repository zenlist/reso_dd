use crate::case;
use crate::dtd::*;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn strip_trailing_whitespace(path: &Path) -> Result<(), std::io::Error> {
    Command::new("sed")
        .arg("-i")
        .arg("")
        .arg("-e")
        .arg("s/[[:space:]]*//")
        .arg(path)
        .status()?;

    Ok(())
}

impl Root {
    pub fn generate(
        &self,
        root_path: &Path,
        writer: &mut impl Write,
    ) -> Result<(), std::io::Error> {
        writeln!(writer, "// THIS IS A GENERATED FILE")?;
        writeln!(writer, "// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator")?;

        for resource in self.resources.iter() {
            if resource.title.contains("Collection") {
                // TODO: Handle collections.
                continue;
            }
            let mod_name = resource.mod_name();
            let struct_name = resource.struct_name();
            println!("Generating {}...", mod_name);
            let path = root_path.join(format!("{}.rs", mod_name));
            let mut file = File::create(&path)?;
            resource.generate(&mut file)?;
            writeln!(writer, "mod {};", mod_name)?;
            writeln!(writer, "pub use {}::{};", mod_name, struct_name)?;
            strip_trailing_whitespace(&path)?;
        }
        for lookup_field in self.lookup_values.lookup_fields.iter() {
            if lookup_field.title.contains("[") {
                // TODO: There are some enums that are templated.
                continue;
            }
            let mod_name = lookup_field.mod_name();
            let enum_name = lookup_field.enum_name();
            println!("Generating {}...", mod_name);
            let path = root_path.join(format!("{}.rs", mod_name));
            let mut file = File::create(&path)?;
            lookup_field.generate(&mut file)?;
            writeln!(writer, "mod {};", mod_name)?;
            writeln!(writer, "pub use {}::{};", mod_name, enum_name)?;
            strip_trailing_whitespace(&path)?;
        }

        Ok(())
    }
}

impl Resource {
    pub fn struct_name(&self) -> String {
        self.title.replace(" Resource", "")
    }

    pub fn mod_name(&self) -> String {
        format!("struct_{}", case::snake_case(&self.struct_name()))
    }

    pub fn generate(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        if self.title.contains("Collection") {
            // TODO: Handle collections.
            return Ok(());
        }

        writeln!(
            writer,
            r##"
            // THIS IS A GENERATED FILE
            // If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
            #[allow(unused_imports)]
            use serde::{{Serialize, Deserialize}};

            /// [{title}]({url})
            #[derive(Debug, Default, Clone, Serialize, Deserialize)]
            pub struct {struct_name} {{"##,
            title = self.title,
            url = self.url,
            struct_name = self.struct_name()
        )?;
        for field in self.fields.iter() {
            field.generate(writer)?;
        }
        writeln!(writer, "}}")?;
        writeln!(writer)
    }
}

impl Field {
    pub fn field_name(&self) -> String {
        case::snake_case(&self.standard_name)
    }

    pub fn is_single_enum(&self) -> bool {
        self.simple_data_type == "String List, Single" && self.page_form == "PropResourceField"
    }

    pub fn is_multi_enum(&self) -> bool {
        self.simple_data_type == "String List, Multi" && self.page_form == "PropResourceField"
    }

    pub fn data_type(&self) -> String {
        if self.is_single_enum() {
            return format!(
                "Option<crate::{}>",
                LookupField::build_enum_name(&self.lookup)
            );
        }
        if self.is_multi_enum() {
            return format!(
                "Option<Vec<crate::{}>>",
                LookupField::build_enum_name(&self.lookup)
            );
        }

        match self.simple_data_type.as_ref() {
            "Number" => "Option<f64>".into(),
            "Boolean" => "Option<bool>".into(),
            "String" => "Option<String>".into(),
            "Timestamp" => "Option<chrono::DateTime<chrono::FixedOffset>>".into(),
            "Date" => "Option<chrono::NaiveDate>".into(),
            "String List, Single" | "String List, Multi" => "Option<String>".into(),
            "Collection" => "Option<String>".into(),
            typ => panic!("Unknown data type '{}'", typ),
        }
    }

    pub fn generate(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        if self.standard_name.contains("[") {
            // TODO: there are some fields that are templated.
            return Ok(());
        }

        let extra_serde = if self.is_multi_enum() {
            format!(r##"#[serde(default, with = "crate::comma_delimited")]"##,)
        } else {
            "".into()
        };

        writeln!(
            writer,
            r#"

            /// {definition}
            ///
            /// [{standard_name}]({url})
            #[serde(rename = "{standard_name}", skip_serializing_if = "Option::is_none")]
            {extra_serde}
            pub {field_name}: {data_type},
            "#,
            standard_name = self.standard_name,
            url = self.url,
            extra_serde = extra_serde,
            definition = self.definition,
            field_name = self.field_name(),
            data_type = self.data_type(),
        )
    }
}

impl LookupField {
    pub fn enum_name(&self) -> String {
        Self::build_enum_name(&self.title)
    }

    pub fn build_enum_name(title: &str) -> String {
        title.replace(" Lookups", "")
    }

    pub fn mod_name(&self) -> String {
        format!("enum_{}", case::snake_case(&self.enum_name()))
    }

    pub fn generate(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        if self.title.contains("[") {
            // TODO: There are some enums that are templated.
            return Ok(());
        }

        writeln!(
            writer,
            r#"
            // THIS IS A GENERATED FILE
            // If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
            use serde::{{Serialize, Deserialize}};
            "#
        )?;

        self.generate_enum(writer)?;
        self.generate_impl_reso_enumeration(writer)?;
        self.generate_impl_from_string(writer)?;
        self.generate_impl_from_str(writer)?;
        self.generate_impl_into_str(writer)?;
        self.generate_impl_serde(writer)?;
        // self.generate_mod_opt_vec_item(writer)?;

        Ok(())
    }

    fn generate_enum(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        writeln!(
            writer,
            r#"
            /// [{title}]({url})
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub enum {enum_name} {{
            "#,
            title = self.title,
            url = self.url,
            enum_name = self.enum_name(),
        )?;
        // There's one situation where an element shows up twice. Ugh.
        let mut seen = HashSet::new();
        for value in self.values.iter() {
            if seen.insert(value.lookup_value.clone()) {
                value.generate(writer)?;
            }
        }
        writeln!(
            writer,
            r#"
                /// A value that was not defined by the enumeration
                OpenEnumeration(String),
            }}
            "#
        )?;
        Ok(())
    }

    fn generate_impl_from_string(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        let enum_name = self.enum_name();
        writeln!(
            writer,
            r#"
            impl From<String> for {enum_name} {{
                fn from(s: String) -> {enum_name} {{
                    match s.as_ref() {{
            "#,
            enum_name = enum_name,
        )?;
        // There's one situation where an element shows up twice. Ugh.
        let mut seen = HashSet::new();
        for value in self.values.iter() {
            if seen.insert(value.lookup_value.clone()) {
                write!(
                    writer,
                    r#"
                    "{string_value}" => {enum_name}::{enum_value},
                    "#,
                    enum_name = enum_name,
                    string_value = value.lookup_value,
                    enum_value = value.enum_name()
                )?;
            }
        }
        writeln!(
            writer,
            r#"
                        _ => {enum_name}::OpenEnumeration(s),
                    }}
                }}
            }}
            "#,
            enum_name = enum_name,
        )?;
        Ok(())
    }

    fn generate_impl_from_str(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        let enum_name = self.enum_name();
        writeln!(
            writer,
            r#"
            impl From<&str> for {enum_name} {{
                fn from(s: &str) -> {enum_name} {{
                    match s {{
            "#,
            enum_name = enum_name,
        )?;
        // There's one situation where an element shows up twice. Ugh.
        let mut seen = HashSet::new();
        for value in self.values.iter() {
            if seen.insert(value.lookup_value.clone()) {
                write!(
                    writer,
                    r#"
                    "{string_value}" => {enum_name}::{enum_value},
                    "#,
                    enum_name = enum_name,
                    string_value = value.lookup_value,
                    enum_value = value.enum_name()
                )?;
            }
        }
        writeln!(
            writer,
            r#"
                        _ => {enum_name}::OpenEnumeration(s.into()),
                    }}
                }}
            }}
            "#,
            enum_name = enum_name,
        )?;
        Ok(())
    }

    fn generate_impl_into_str(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        let enum_name = self.enum_name();
        writeln!(
            writer,
            r#"
            impl<'a> From<&'a {enum_name}> for &'a str {{
                fn from(s: &'a {enum_name}) -> &'a str {{
                    match s {{
            "#,
            enum_name = enum_name,
        )?;
        // There's one situation where an element shows up twice. Ugh.
        let mut seen = HashSet::new();
        for value in self.values.iter() {
            if seen.insert(value.lookup_value.clone()) {
                write!(
                    writer,
                    r#"
                    {enum_name}::{enum_value} => "{string_value}",
                    "#,
                    enum_name = enum_name,
                    string_value = value.lookup_value,
                    enum_value = value.enum_name()
                )?;
            }
        }
        writeln!(
            writer,
            r#"
                        {enum_name}::OpenEnumeration(s) => s,
                    }}
                }}
            }}
            "#,
            enum_name = enum_name,
        )?;
        Ok(())
    }

    fn generate_impl_reso_enumeration(
        &self,
        writer: &mut impl Write,
    ) -> Result<(), std::io::Error> {
        let enum_name = self.enum_name();
        writeln!(
            writer,
            r#"
            impl crate::ResoEnumeration for {enum_name} {{
            "#,
            enum_name = enum_name,
        )?;

        writeln!(
            writer,
            r#"
            fn from_str(s: &str) -> {enum_name} {{
                    match s {{
            "#,
            enum_name = enum_name,
        )?;
        // There's one situation where an element shows up twice. Ugh.
        let mut seen = HashSet::new();
        for value in self.values.iter() {
            if seen.insert(value.lookup_value.clone()) {
                write!(
                    writer,
                    r#"
                    "{string_value}" => {enum_name}::{enum_value},
                    "#,
                    enum_name = enum_name,
                    string_value = value.lookup_value,
                    enum_value = value.enum_name()
                )?;
            }
        }
        writeln!(
            writer,
            r#"
                    _ => {enum_name}::OpenEnumeration(s.into()),
                }}
            }}
            "#,
            enum_name = enum_name,
        )?;

        writeln!(
            writer,
            r#"
            fn from_string(s: String) -> {enum_name} {{
                    match s.as_ref() {{
            "#,
            enum_name = enum_name,
        )?;
        // There's one situation where an element shows up twice. Ugh.
        let mut seen = HashSet::new();
        for value in self.values.iter() {
            if seen.insert(value.lookup_value.clone()) {
                write!(
                    writer,
                    r#"
                    "{string_value}" => {enum_name}::{enum_value},
                    "#,
                    enum_name = enum_name,
                    string_value = value.lookup_value,
                    enum_value = value.enum_name()
                )?;
            }
        }
        writeln!(
            writer,
            r#"
                    _ => {enum_name}::OpenEnumeration(s),
                }}
            }}
            "#,
            enum_name = enum_name,
        )?;

        writeln!(
            writer,
            r#"
            fn to_str(&self) -> &str {{
                match self {{
            "#,
        )?;
        // There's one situation where an element shows up twice. Ugh.
        let mut seen = HashSet::new();
        for value in self.values.iter() {
            if seen.insert(value.lookup_value.clone()) {
                write!(
                    writer,
                    r#"
                    {enum_name}::{enum_value} => "{string_value}",
                    "#,
                    enum_name = enum_name,
                    string_value = value.lookup_value,
                    enum_value = value.enum_name()
                )?;
            }
        }
        writeln!(
            writer,
            r#"
                    {enum_name}::OpenEnumeration(ref s) => s,
                }}
            }}
            "#,
            enum_name = enum_name,
        )?;

        writeln!(
            writer,
            r#"
            fn into_string(self) -> String {{
                match self {{
            "#,
        )?;
        // There's one situation where an element shows up twice. Ugh.
        let mut seen = HashSet::new();
        for value in self.values.iter() {
            if seen.insert(value.lookup_value.clone()) {
                write!(
                    writer,
                    r#"
                    {enum_name}::{enum_value} => "{string_value}".into(),
                    "#,
                    enum_name = enum_name,
                    string_value = value.lookup_value,
                    enum_value = value.enum_name()
                )?;
            }
        }
        writeln!(
            writer,
            r#"
                    {enum_name}::OpenEnumeration(s) => s,
                }}
            }}
            "#,
            enum_name = enum_name,
        )?;

        writeln!(
            writer,
            r#"
            fn fallback_value(&self) -> Option<&str> {{
                match self {{
            "#,
        )?;
        writeln!(
            writer,
            r#"
                    {enum_name}::OpenEnumeration(ref s) => Some(s),
                    _ => None,
                }}
            }}
            "#,
            enum_name = enum_name,
        )?;

        writeln!(
            writer,
            r#"
            }}
            "#,
        )?;
        Ok(())
    }

    fn generate_impl_serde(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        writeln!(
            writer,
            r#"
            impl Serialize for {enum_name} {{
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer {{
                    serializer.serialize_str(self.into())
                }}
            }}

            impl<'de> Deserialize<'de> for {enum_name} {{
                    fn deserialize<D>(
                    deserializer: D,
                ) -> Result<Self, D::Error>
                where D: serde::Deserializer<'de>, {{
                    let s = String::deserialize(deserializer)?;
                    Ok(From::from(s))
                }}
            }}
            "#,
            enum_name = self.enum_name()
        )
    }
}

impl LookupValue {
    pub fn enum_name(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[-/\s&'().,]").unwrap();
            static ref UPPERCASE: Regex = Regex::new(r"^.").unwrap();
        }

        if self.lookup_value == "%" {
            return "Percent".into();
        }
        if self.lookup_value == "$" {
            return "Dollar".into();
        }
        if self.lookup_value == "$filter" {
            return "Filter".into();
        }
        if self.lookup_value == "Open -8 Hours/Day" {
            return "OpenLessThan8HoursDay".into();
        }

        let result = RE.replace_all(&self.lookup_value, "");
        let result = result.replace("+", "Plus");
        let result = UPPERCASE.replace(&result, |caps: &Captures| caps[0].to_uppercase());
        let result = result.into_owned();

        if result.chars().nth(0).unwrap().is_alphabetic() {
            result
        } else {
            format!("_{}", result)
        }
    }

    pub fn generate(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        writeln!(
            writer,
            r#"
            /// "[{lookup_value}]({url})": {definition}
            {enum_name},
            "#,
            lookup_value = self.lookup_value,
            url = self.url,
            definition = self.definition,
            enum_name = self.enum_name(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_field_names() {
        let field = Field {
            standard_name: "AboveGradeFinishedArea".into(),
            ..Default::default()
        };
        assert_eq!(field.field_name(), "above_grade_finished_area");

        let field = Field {
            standard_name: "AdditionalParcelsYN".into(),
            ..Default::default()
        };
        assert_eq!(field.field_name(), "additional_parcels_yn");

        let field = Field {
            standard_name: "BuyerAgentURL".into(),
            ..Default::default()
        };
        assert_eq!(field.field_name(), "buyer_agent_url");

        let field = Field {
            standard_name: "BuyerAgentURL".into(),
            ..Default::default()
        };
        assert_eq!(field.field_name(), "buyer_agent_url");

        let field = Field {
            standard_name: "DOH1".into(),
            ..Default::default()
        };
        assert_eq!(field.field_name(), "doh1");
    }
}
