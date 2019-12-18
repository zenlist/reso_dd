use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Root {
    #[serde(rename = "Resource")]
    pub resources: Vec<Resource>,
    #[serde(rename = "LookupValues")]
    pub lookup_values: LookupValues,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Resource {
    #[serde(rename = "WikiPageTitle")]
    pub title: String,
    #[serde(rename = "WikiPageURL")]
    pub url: String,
    #[serde(rename = "Field")]
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Field {
    #[serde(rename = "PageForm")]
    pub page_form: String,
    #[serde(rename = "WikiPageURL")]
    pub url: String,
    pub standard_name: String,
    pub definition: String,
    // TODO: Groups
    pub simple_data_type: String,
    pub sug_max_length: Option<String>,
    // TODO: Synonym
    pub element_status: String,
    #[serde(rename = "BEDES")]
    pub bedes: String,
    pub certification_level: String,
    #[serde(rename = "RecordID")]
    pub record_id: String,
    pub lookup_status: String,
    pub lookup: String,
    pub collection: Option<String>,
    pub sug_max_precision: String,
    pub repeating_element: String,
    // TODO: PropertyTypes
    // TODO: Payloads
    pub spanish_standard_name: String,
    pub status_change_date: String,
    pub revised_date: String,
    pub added_in_version: String,
    pub modification_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LookupValues {
    #[serde(rename = "WikiPageURL")]
    pub url: String,
    #[serde(rename = "LookupField")]
    pub lookup_fields: Vec<LookupField>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LookupField {
    #[serde(rename = "WikiPageTitle")]
    pub title: String,
    #[serde(rename = "WikiPageURL")]
    pub url: String,
    #[serde(rename = "LookupValue")]
    pub values: Vec<LookupValue>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct LookupValue {
    #[serde(rename = "WikiPageTitle")]
    pub title: String,
    #[serde(rename = "WikiPageURL")]
    pub url: String,
    pub lookup_value: String,
    pub lookup_field: String,
    pub definition: String,
    pub synonym: String,
    #[serde(rename = "BEDES")]
    pub bedes: String,
    // TODO: References
    pub element_status: String,
    #[serde(rename = "LookupID")]
    pub lookup_id: usize,
    #[serde(rename = "LookupFieldID")]
    pub lookup_field_id: usize,
    pub spanish_lookup_field: String,
    pub spanish_lookup_value: String,
    pub status_change_date: String,
    pub revised_date: String,
    pub added_in_version: String,
    pub modification_timestamp: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_xml_rs::from_str;

    #[test]
    fn parse_dtd() {
        let dtd = include_str!("../../reso-ddwiki-export-2019-11-26-DDW17-v1.7.xml");

        let result = from_str::<Root>(dtd);

        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}
