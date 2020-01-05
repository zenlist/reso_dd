// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// [Rules Resource](https://ddwiki.reso.org/display/DDW17/Rules+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rules {
    /// The class or table to which the Rule refers. i.e. Residential, Residential Lease, Income, Mobile, etc.
    ///
    /// [ClassName](https://ddwiki.reso.org/display/DDW17/ClassName+%28Rules%29+Field)
    #[serde(rename = "ClassName", skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// The unique identifier of the field to which the Rule applies. This is a foreign key relating to the field found in the resource per the ResourceName.
    ///
    /// [FieldKey](https://ddwiki.reso.org/display/DDW17/FieldKey+%28Rules%29+Field)
    #[serde(rename = "FieldKey", skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,

    /// The unique identifier of the field to which the Rule applies. This is a foreign key relating to the field found in the resource per the ResourceName. This is the numeric only key and used as an alternative to the FieldKey field.
    ///
    /// [FieldKeyNumeric](https://ddwiki.reso.org/display/DDW17/FieldKeyNumeric+%28Rules%29+Field)
    #[serde(rename = "FieldKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub field_key_numeric: Option<f64>,

    /// The name of the field to which the Rule applies.
    ///
    /// [FieldName](https://ddwiki.reso.org/display/DDW17/FieldName+%28Rules%29+Field)
    #[serde(rename = "FieldName", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the rule was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+%28Rules%29+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the rule was initially entered.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+%28Rules%29+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider. The Originating system is the system with authoritative control over the record. For example; the name of the MLS where the Rule originated. In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+%28Rules%29+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The name of the Originating record provider. Most commonly the name of the MLS. The place where the Rules is originally input. The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+%28Rules%29+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// The system key, a unique record identifier, from the Originating system. The Originating system is the system with authoritative control over the record. For example, the Multiple Listing Service where the Rule originated. There may be cases where the Source System (how you received the record) is not the Originating System. See Source System Key for more information.
    ///
    /// [OriginatingSystemRuleKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemRuleKey+Field)
    #[serde(
        rename = "OriginatingSystemRuleKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_rule_key: Option<String>,

    /// The resource to which the Rule refers. E.g. Property, Member, Office, Open House, etc.
    ///
    /// [ResourceName](https://ddwiki.reso.org/display/DDW17/ResourceName+%28Rules%29+Field)
    #[serde(rename = "ResourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,

    /// The action to be taken when processing the rule.
    ///
    /// [RuleAction](https://ddwiki.reso.org/display/DDW17/RuleAction+Field)
    #[serde(rename = "RuleAction", skip_serializing_if = "Option::is_none")]
    pub rule_action: Option<String>,

    /// A detailed textual description of the rule.
    ///
    /// [RuleDescription](https://ddwiki.reso.org/display/DDW17/RuleDescription+Field)
    #[serde(rename = "RuleDescription", skip_serializing_if = "Option::is_none")]
    pub rule_description: Option<String>,

    /// Is the rule currently enabled?
    ///
    /// [RuleEnabledYN](https://ddwiki.reso.org/display/DDW17/RuleEnabledYN+Field)
    #[serde(rename = "RuleEnabledYN", skip_serializing_if = "Option::is_none")]
    pub rule_enabled_yn: Option<bool>,

    /// Textual information conveyed when the given rule is in error or fails. (e.g. The listing price must be greater than 0.)
    ///
    /// [RuleErrorText](https://ddwiki.reso.org/display/DDW17/RuleErrorText+Field)
    #[serde(rename = "RuleErrorText", skip_serializing_if = "Option::is_none")]
    pub rule_error_text: Option<String>,

    /// The expression or details of the rule.
    ///
    /// [RuleExpression](https://ddwiki.reso.org/display/DDW17/RuleExpression+Field)
    #[serde(rename = "RuleExpression", skip_serializing_if = "Option::is_none")]
    pub rule_expression: Option<String>,

    /// $filter, JavaScript, RETS1.8, REBR, etc.… ?
    ///
    /// [RuleFormat](https://ddwiki.reso.org/display/DDW17/RuleFormat+Field)
    #[serde(rename = "RuleFormat", skip_serializing_if = "Option::is_none")]
    pub rule_format: Option<String>,

    /// The text that might be displayed on a form that helps the user fix the rule (e.g. enter phone number in the 10 digit format ###-###-####.)
    ///
    /// [RuleHelpText](https://ddwiki.reso.org/display/DDW17/RuleHelpText+Field)
    #[serde(rename = "RuleHelpText", skip_serializing_if = "Option::is_none")]
    pub rule_help_text: Option<String>,

    /// <ac:structured-macro ac:name="tooltip" ac:schema-version="1" ac:macro-id="d9385e45-37c2-4bf8-9f64-6a4568d6b0b4"><ac:parameter ac:name="tip">A single designation identifying what category of fields to which the given field belongs.</ac:parameter><ac:parameter ac:name="text">Group</ac:parameter></ac:structured-macro><ac:link ac:anchor="Group"><ri:page ri:content-title="Data Dictionary Terms and Meta Definitions" /><ac:plain-text-link-body><![CDATA[?]]></ac:plain-text-link-body></ac:link>: <ac:link><ri:page ri:content-title="Rules Resource" /><ac:link-body>Rules Resource</ac:link-body></ac:link>
    ///
    /// [RuleKey](https://ddwiki.reso.org/display/DDW17/RuleKey+Field)
    #[serde(rename = "RuleKey", skip_serializing_if = "Option::is_none")]
    pub rule_key: Option<String>,

    /// <ac:structured-macro ac:name="tooltip" ac:schema-version="1" ac:macro-id="dddc1b50-f511-45b5-8c68-773cf136e7b5"><ac:parameter ac:name="tip">A single designation identifying what category of fields to which the given field belongs.</ac:parameter><ac:parameter ac:name="text">Group</ac:parameter></ac:structured-macro><ac:link ac:anchor="Group"><ri:page ri:content-title="Data Dictionary Terms and Meta Definitions" /><ac:plain-text-link-body><![CDATA[?]]></ac:plain-text-link-body></ac:link>: <ac:link><ri:page ri:content-title="Rules Resource" /><ac:link-body>Rules Resource</ac:link-body></ac:link>
    ///
    /// [RuleKeyNumeric](https://ddwiki.reso.org/display/DDW17/RuleKeyNumeric+Field)
    #[serde(rename = "RuleKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub rule_key_numeric: Option<f64>,

    /// A descriptive name for the rule.
    ///
    /// [RuleName](https://ddwiki.reso.org/display/DDW17/RuleName+Field)
    #[serde(rename = "RuleName", skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,

    /// When in use, execution of rules are to follow the order specified by this field. Any rule that references another field will need to be ordered.
    ///
    /// [RuleOrder](https://ddwiki.reso.org/display/DDW17/RuleOrder+Field)
    #[serde(rename = "RuleOrder", skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<f64>,

    /// Validation, Required, Warning, etc.
    ///
    /// [RuleType](https://ddwiki.reso.org/display/DDW17/RuleType+Field)
    #[serde(rename = "RuleType", skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,

    /// A semantically-versioned rule version. https://semver.org/
    ///
    /// [RuleVersion](https://ddwiki.reso.org/display/DDW17/RuleVersion+Field)
    #[serde(rename = "RuleVersion", skip_serializing_if = "Option::is_none")]
    pub rule_version: Option<String>,

    /// Textual information conveyed when a given rule has met a condition that warrants a warning message. e.g. you've entered a sale price that is has a difference from the listing price greater than 25%.
    ///
    /// [RuleWarningText](https://ddwiki.reso.org/display/DDW17/RuleWarningText+Field)
    #[serde(rename = "RuleWarningText", skip_serializing_if = "Option::is_none")]
    pub rule_warning_text: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemHistoryKey](https://ddwiki.reso.org/display/DDW17/SourceSystemHistoryKey+%28Rules%29+Field)
    #[serde(
        rename = "SourceSystemHistoryKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_system_history_key: Option<String>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+%28Rules%29+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The name of the Rule record provider. The system from which the record was directly received. The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+%28Rules%29+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,
}
