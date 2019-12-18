use crate::case;
use crate::dtd::*;
use std::collections::HashSet;
use std::io::Write;

impl Root {
    pub fn generate(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        writeln!(writer, "// THIS IS A GENERATED FILE")?;
        writeln!(writer, "// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator")?;
        writeln!(writer, "use serde::{{Serialize, Deserialize}};")?;

        for resource in self.resources.iter() {
            resource.generate(writer)?;
        }
        for lookup_field in self.lookup_values.lookup_fields.iter() {
            lookup_field.generate(writer)?;
        }

        Ok(())
    }
}

impl Resource {
    pub fn struct_name(&self) -> String {
        self.title.replace(" Resource", "")
    }

    pub fn generate(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        if self.title.contains("Collection") {
            // TODO: Handle collections.
            return Ok(());
        }

        writeln!(
            writer,
            r##"
            /// [{title}]({url})
            #[derive(Debug, Default, Serialize, Deserialize)]
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
            return format!("Option<{}>", LookupField::build_enum_name(&self.lookup));
        }
        if self.is_multi_enum() {
            return format!(
                "Option<Vec<{}>>",
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
            format!(
                r##"#[serde(default, with = "{serde_module}")]"##,
                serde_module = LookupField::build_multiple_items_format_mod_name(&self.lookup)
            )
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

    pub fn multiple_items_format_mod_name(&self) -> String {
        Self::build_multiple_items_format_mod_name(&self.title)
    }

    pub fn build_multiple_items_format_mod_name(title: &str) -> String {
        format!(
            "option_vec_{}_format",
            case::snake_case(&Self::build_enum_name(title))
        )
    }

    pub fn generate(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        if self.title.contains("[") {
            // TODO: There are some enums that are templated.
            return Ok(());
        }

        self.generate_enum(writer)?;
        self.generate_impl_from_string(writer)?;
        self.generate_impl_from_str(writer)?;
        self.generate_impl_into_str(writer)?;
        self.generate_impl_serde(writer)?;
        self.generate_mod_opt_vec_item(writer)?;

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

    fn generate_mod_opt_vec_item(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        writeln!(
            writer,
            r#"
            pub(crate) mod {multiple_items_format_mod_name} {{
                use super::{enum_name};
                use serde::{{Deserialize, Serializer, Deserializer}};

                pub(crate) fn serialize<S>(
                    items: &Option<Vec<{enum_name}>>,
                    serializer: S,
                ) -> Result<S::Ok, S::Error>
                where S: Serializer,
                {{
                    match items {{
                        None => return serializer.serialize_none(),
                        Some(ref vec) if vec.len() == 0 => serializer.serialize_str(""),
                        Some(ref vec) => {{
                            let items: Vec<&str> = vec.iter().map(|item| item.into()).collect();
                            let joined = items.join(",");
                            serializer.serialize_str(&joined)
                        }}
                    }}
                }}

                pub(crate) fn deserialize<'de, D>(
                    deserializer: D,
                ) -> Result<Option<Vec<{enum_name}>>, D::Error>
                where D: Deserializer<'de>,
                {{
                    let s = String::deserialize(deserializer)?;
                    if s == "" {{
                        return Ok(Some(vec![]));
                    }}

                    let items = s.split(",").map(|i| From::<&str>::from(i)).collect();
                    Ok(Some(items))
                }}
            }}
            "#,
            enum_name = self.enum_name(),
            multiple_items_format_mod_name = self.multiple_items_format_mod_name(),
        )
    }
}

impl LookupValue {
    pub fn enum_name(&self) -> String {
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

        let result = case::pascal_case(&self.lookup_value);
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
