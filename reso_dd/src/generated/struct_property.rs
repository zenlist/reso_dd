// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// [Property Resource](https://ddwiki.reso.org/display/DDW17/Property+Resource)
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Property {
    /// Finished area within the structure that is at or above the surface of the ground.
    ///
    /// [AboveGradeFinishedArea](https://ddwiki.reso.org/display/DDW17/AboveGradeFinishedArea+Field)
    #[serde(
        rename = "AboveGradeFinishedArea",
        skip_serializing_if = "Option::is_none"
    )]
    pub above_grade_finished_area: Option<f64>,

    /// The source of the measurements. This is a pick list of options showing the source of the measurement. i.e. Agent, Assessor, Estimate, etc.
    ///
    /// [AboveGradeFinishedAreaSource](https://ddwiki.reso.org/display/DDW17/AboveGradeFinishedAreaSource+Field)
    #[serde(
        rename = "AboveGradeFinishedAreaSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub above_grade_finished_area_source: Option<crate::AreaSource>,

    /// A pick list of the unit of measurement for the area. i.e. Square Feet, Square Meters, Acres, etc.
    ///
    /// [AboveGradeFinishedAreaUnits](https://ddwiki.reso.org/display/DDW17/AboveGradeFinishedAreaUnits+Field)
    #[serde(
        rename = "AboveGradeFinishedAreaUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub above_grade_finished_area_units: Option<crate::AreaUnits>,

    /// If the property is located behind an unmanned security gate such as in a Gated Community, what is the code to gain access through the secured gate.
    ///
    /// [AccessCode](https://ddwiki.reso.org/display/DDW17/AccessCode+Field)
    #[serde(rename = "AccessCode", skip_serializing_if = "Option::is_none")]
    pub access_code: Option<String>,

    /// A list or description of the accessibility features included in the sale/lease.
    ///
    /// [AccessibilityFeatures](https://ddwiki.reso.org/display/DDW17/AccessibilityFeatures+Field)
    #[serde(
        rename = "AccessibilityFeatures",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub accessibility_features: Option<Vec<crate::AccessibilityFeatures>>,

    /// If additional parcels are included in the sale, a list of those parcel's IDs separated by commas.  Do not include the first or primary parcel number, that should be located in the Parcel Number field.
    ///
    /// [AdditionalParcelsDescription](https://ddwiki.reso.org/display/DDW17/AdditionalParcelsDescription+Field)
    #[serde(
        rename = "AdditionalParcelsDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_parcels_description: Option<String>,

    /// Are there more than one parcel or lot included in the sale?
    ///
    /// [AdditionalParcelsYN](https://ddwiki.reso.org/display/DDW17/AdditionalParcelsYN+Field)
    #[serde(
        rename = "AdditionalParcelsYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_parcels_yn: Option<bool>,

    /// The main or most notable tenants as well as other tenants of the shopping center or mall in which the commercial property is located.
    ///
    /// [AnchorsCoTenants](https://ddwiki.reso.org/display/DDW17/AnchorsCoTenants+Field)
    #[serde(rename = "AnchorsCoTenants", skip_serializing_if = "Option::is_none")]
    pub anchors_co_tenants: Option<String>,

    /// A list of the appliances that will be included in the sale/lease of the property.
    ///
    /// [Appliances](https://ddwiki.reso.org/display/DDW17/Appliances+Field)
    #[serde(rename = "Appliances", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub appliances: Option<Vec<crate::Appliances>>,

    /// A list describing the style of the structure. For example, Victorian, Ranch, Craftsman, etc.
    ///
    /// [ArchitecturalStyle](https://ddwiki.reso.org/display/DDW17/ArchitecturalStyle+Field)
    #[serde(rename = "ArchitecturalStyle", skip_serializing_if = "Option::is_none")]
    pub architectural_style: Option<String>,

    /// Amenities provided by the Home Owners Association, Mobile Park or Complex. For example Pool, Clubhouse, etc.
    ///
    /// [AssociationAmenities](https://ddwiki.reso.org/display/DDW17/AssociationAmenities+Field)
    #[serde(
        rename = "AssociationAmenities",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub association_amenities: Option<Vec<crate::AssociationAmenities>>,

    /// A fee paid by the homeowner to the Home Owners Association which is used for the upkeep of the common area, neighborhood or other association related benefits.
    ///
    /// [AssociationFee](https://ddwiki.reso.org/display/DDW17/AssociationFee+Field)
    #[serde(rename = "AssociationFee", skip_serializing_if = "Option::is_none")]
    pub association_fee: Option<f64>,

    /// A fee paid by the homeowner to the second of two Home Owners Associations, which is used for the upkeep of the common area, neighborhood or other association related benefits.
    ///
    /// [AssociationFee2](https://ddwiki.reso.org/display/DDW17/AssociationFee2+Field)
    #[serde(rename = "AssociationFee2", skip_serializing_if = "Option::is_none")]
    pub association_fee2: Option<f64>,

    /// The frequency the association fee is paid.  For example, Weekly, Monthly, Annually, Bi-Monthly, One Time, etc.
    ///
    /// [AssociationFee2Frequency](https://ddwiki.reso.org/display/DDW17/AssociationFee2Frequency+Field)
    #[serde(
        rename = "AssociationFee2Frequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub association_fee2_frequency: Option<crate::FeeFrequency>,

    /// The frequency the association fee is paid.  For example, Weekly, Monthly, Annually, Bi-Monthly, One Time, etc.
    ///
    /// [AssociationFeeFrequency](https://ddwiki.reso.org/display/DDW17/AssociationFeeFrequency+Field)
    #[serde(
        rename = "AssociationFeeFrequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub association_fee_frequency: Option<crate::FeeFrequency>,

    /// Services included with the association fee.  For example Landscaping, Trash, Water, etc.
    ///
    /// [AssociationFeeIncludes](https://ddwiki.reso.org/display/DDW17/AssociationFeeIncludes+Field)
    #[serde(
        rename = "AssociationFeeIncludes",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub association_fee_includes: Option<Vec<crate::AssociationFeeIncludes>>,

    /// The name of the Home Owners Association.
    ///
    /// [AssociationName](https://ddwiki.reso.org/display/DDW17/AssociationName+Field)
    #[serde(rename = "AssociationName", skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,

    /// The name of the second of two Home Owners Association.
    ///
    /// [AssociationName2](https://ddwiki.reso.org/display/DDW17/AssociationName2+Field)
    #[serde(rename = "AssociationName2", skip_serializing_if = "Option::is_none")]
    pub association_name2: Option<String>,

    /// The phone number of the Home Owners Association. North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [AssociationPhone](https://ddwiki.reso.org/display/DDW17/AssociationPhone+Field)
    #[serde(rename = "AssociationPhone", skip_serializing_if = "Option::is_none")]
    pub association_phone: Option<String>,

    /// The phone number of the second of two Home Owners Association. North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [AssociationPhone2](https://ddwiki.reso.org/display/DDW17/AssociationPhone2+Field)
    #[serde(rename = "AssociationPhone2", skip_serializing_if = "Option::is_none")]
    pub association_phone2: Option<String>,

    /// Is there a Home Owners Association.  A separate Y/N field is needed because not all associations have dues.
    ///
    /// [AssociationYN](https://ddwiki.reso.org/display/DDW17/AssociationYN+Field)
    #[serde(rename = "AssociationYN", skip_serializing_if = "Option::is_none")]
    pub association_yn: Option<bool>,

    /// A flag indicating that the garage attached to the dwelling.
    ///
    /// [AttachedGarageYN](https://ddwiki.reso.org/display/DDW17/AttachedGarageYN+Field)
    #[serde(rename = "AttachedGarageYN", skip_serializing_if = "Option::is_none")]
    pub attached_garage_yn: Option<bool>,

    /// The date the property will be available for possession/occupation.
    ///
    /// [AvailabilityDate](https://ddwiki.reso.org/display/DDW17/AvailabilityDate+Field)
    #[serde(rename = "AvailabilityDate", skip_serializing_if = "Option::is_none")]
    pub availability_date: Option<chrono::NaiveDate>,

    /// A list of information and features about the basement. i.e. None/Slab, Finished, Partially Finished, Crawl Space, Dirt, Outside Entrance, Radon Mitigation
    ///
    /// [Basement](https://ddwiki.reso.org/display/DDW17/Basement+Field)
    #[serde(rename = "Basement", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub basement: Option<Vec<crate::Basement>>,

    /// Does the property have a basement?
    ///
    /// [BasementYN](https://ddwiki.reso.org/display/DDW17/BasementYN+Field)
    #[serde(rename = "BasementYN", skip_serializing_if = "Option::is_none")]
    pub basement_yn: Option<bool>,

    /// A room containing all 4 of the 4 elements constituting a bath, which are; Toilet, Sink, Bathtub or Shower Head.  A Full Bath will typically contain four elements; Sink, Toilet, Tub and Shower Head (in tub or stall).  However, some may considered a Sink, Toilet and Tub (without a shower) a Full Bath, others consider this to be a Three Quarter Bath.  In the event that BathroomsThreeQuarter is not in use, this field may represent the sum of all Full and Three Quarter bathrooms.
    ///
    /// [BathroomsFull](https://ddwiki.reso.org/display/DDW17/BathroomsFull+Field)
    #[serde(rename = "BathroomsFull", skip_serializing_if = "Option::is_none")]
    pub bathrooms_full: Option<f64>,

    /// A room containing 2 of the 4 elements constituting a bath, which are; Toilet, Sink, Bathtub or Shower Head.  A Half Bath will typically contain a Sink and Toilet.
    ///
    /// [BathroomsHalf](https://ddwiki.reso.org/display/DDW17/BathroomsHalf+Field)
    #[serde(rename = "BathroomsHalf", skip_serializing_if = "Option::is_none")]
    pub bathrooms_half: Option<f64>,

    /// A room containing 1 of the 4 elements constituting a bath which are; Toilet, Sink, Bathtub or Shower Head.  Examples are a vanity with a sink or a WC (Water Closet, which is a room with only a toilet).
    ///
    /// [BathroomsOneQuarter](https://ddwiki.reso.org/display/DDW17/BathroomsOneQuarter+Field)
    #[serde(
        rename = "BathroomsOneQuarter",
        skip_serializing_if = "Option::is_none"
    )]
    pub bathrooms_one_quarter: Option<f64>,

    /// The number of partial bathrooms in the property being sold/leased.  When used in combination with the BathroomsFull field, this replaces (or is the sum of) all Half and One Quarter bathrooms; and in the event BathroomsThreeQuarter is not used, BathroomsFull replaces (or is the sum of) all Full and Three Quarter baths.  This field should not be used in combination with the BathroomsOneQuarter or the BathroomsHalf.
    ///
    /// [BathroomsPartial](https://ddwiki.reso.org/display/DDW17/BathroomsPartial+Field)
    #[serde(rename = "BathroomsPartial", skip_serializing_if = "Option::is_none")]
    pub bathrooms_partial: Option<f64>,

    /// A room containing 3 of the 4 elements constituting a bath, which are; Toilet, Sink, Bathtub or Shower Head. A typical Three Quarter Bath will contain Sink, Toilet and Shower.  Some may considered a Sink, Toilet and Tub (without a shower) a Three Quarter Bath, others consider this to be a Full Bath.
    ///
    /// [BathroomsThreeQuarter](https://ddwiki.reso.org/display/DDW17/BathroomsThreeQuarter+Field)
    #[serde(
        rename = "BathroomsThreeQuarter",
        skip_serializing_if = "Option::is_none"
    )]
    pub bathrooms_three_quarter: Option<f64>,

    /// The simple sum of the number of bathrooms.  For example for a property with two Full Bathrooms and one Half Bathroom, the Bathrooms Total Integer will be 3.  Expressing this example as 2.5, you will need to use a non-standard field name.  Decimal based bathrooms are not recommended but possible via the Dictionary's extensibility.
    ///
    /// [BathroomsTotalInteger](https://ddwiki.reso.org/display/DDW17/BathroomsTotalInteger+Field)
    #[serde(
        rename = "BathroomsTotalInteger",
        skip_serializing_if = "Option::is_none"
    )]
    pub bathrooms_total_integer: Option<f64>,

    /// The sum of BedroomsTotal plus other rooms that may be used as a bedroom but are not defined as bedroom per local policy.
    ///
    /// [BedroomsPossible](https://ddwiki.reso.org/display/DDW17/BedroomsPossible+Field)
    #[serde(rename = "BedroomsPossible", skip_serializing_if = "Option::is_none")]
    pub bedrooms_possible: Option<f64>,

    /// The total number of bedrooms in the dwelling.
    ///
    /// [BedroomsTotal](https://ddwiki.reso.org/display/DDW17/BedroomsTotal+Field)
    #[serde(rename = "BedroomsTotal", skip_serializing_if = "Option::is_none")]
    pub bedrooms_total: Option<f64>,

    /// Finished area within the structure that is below ground.
    ///
    /// [BelowGradeFinishedArea](https://ddwiki.reso.org/display/DDW17/BelowGradeFinishedArea+Field)
    #[serde(
        rename = "BelowGradeFinishedArea",
        skip_serializing_if = "Option::is_none"
    )]
    pub below_grade_finished_area: Option<f64>,

    /// The source of the measurements. This is a pick list of options showing the source of the measurement. i.e. Agent, Assessor, Estimate, etc.
    ///
    /// [BelowGradeFinishedAreaSource](https://ddwiki.reso.org/display/DDW17/BelowGradeFinishedAreaSource+Field)
    #[serde(
        rename = "BelowGradeFinishedAreaSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub below_grade_finished_area_source: Option<crate::AreaSource>,

    /// A pick list of the unit of measurement for the area.  i.e. Square Feet, Square Meters, Acres, etc.
    ///
    /// [BelowGradeFinishedAreaUnits](https://ddwiki.reso.org/display/DDW17/BelowGradeFinishedAreaUnits+Field)
    #[serde(
        rename = "BelowGradeFinishedAreaUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub below_grade_finished_area_units: Option<crate::AreaUnits>,

    /// Type of mobile home.
    ///
    /// [BodyType](https://ddwiki.reso.org/display/DDW17/BodyType+Field)
    #[serde(rename = "BodyType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub body_type: Option<Vec<crate::BodyType>>,

    /// The builders model name or number for the property.
    ///
    /// [BuilderModel](https://ddwiki.reso.org/display/DDW17/BuilderModel+Field)
    #[serde(rename = "BuilderModel", skip_serializing_if = "Option::is_none")]
    pub builder_model: Option<String>,

    /// Name of the builder of the property or builder's tract.
    ///
    /// [BuilderName](https://ddwiki.reso.org/display/DDW17/BuilderName+Field)
    #[serde(rename = "BuilderName", skip_serializing_if = "Option::is_none")]
    pub builder_name: Option<String>,

    /// The source of the measurements. This is a pick list of options showing the source of the measurement. i.e. Agent, Assessor, Estimate, etc.
    ///
    /// [BuildingAreaSource](https://ddwiki.reso.org/display/DDW17/BuildingAreaSource+Field)
    #[serde(rename = "BuildingAreaSource", skip_serializing_if = "Option::is_none")]
    pub building_area_source: Option<crate::AreaSource>,

    /// Total area of the structure. Includes both finished and unfinished areas.
    ///
    /// [BuildingAreaTotal](https://ddwiki.reso.org/display/DDW17/BuildingAreaTotal+Field)
    #[serde(rename = "BuildingAreaTotal", skip_serializing_if = "Option::is_none")]
    pub building_area_total: Option<f64>,

    /// A pick list of the unit of measurement for the area.  i.e. Square Feet, Square Meters, Acres, etc.
    ///
    /// [BuildingAreaUnits](https://ddwiki.reso.org/display/DDW17/BuildingAreaUnits+Field)
    #[serde(rename = "BuildingAreaUnits", skip_serializing_if = "Option::is_none")]
    pub building_area_units: Option<crate::AreaUnits>,

    /// Features or amenities of the building or business park.
    ///
    /// [BuildingFeatures](https://ddwiki.reso.org/display/DDW17/BuildingFeatures+Field)
    #[serde(rename = "BuildingFeatures", skip_serializing_if = "Option::is_none")]
    pub building_features: Option<String>,

    /// Name of the building or business park.
    ///
    /// [BuildingName](https://ddwiki.reso.org/display/DDW17/BuildingName+Field)
    #[serde(rename = "BuildingName", skip_serializing_if = "Option::is_none")]
    pub building_name: Option<String>,

    /// Name of the business being sold.
    ///
    /// [BusinessName](https://ddwiki.reso.org/display/DDW17/BusinessName+Field)
    #[serde(rename = "BusinessName", skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,

    /// The type of business being sold. Retail, Wholesale, Grocery, Food & Bev, etc.
    ///
    /// [BusinessType](https://ddwiki.reso.org/display/DDW17/BusinessType+Field)
    #[serde(rename = "BusinessType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub business_type: Option<Vec<crate::BusinessType>>,

    /// The total commission to be paid for this sale, expressed as either a percentage or a constant currency amount.
    ///
    /// [BuyerAgencyCompensation](https://ddwiki.reso.org/display/DDW17/BuyerAgencyCompensation+Field)
    #[serde(
        rename = "BuyerAgencyCompensation",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agency_compensation: Option<String>,

    /// A list of types to clarify the value entered in the BuyerAgencyCompensation field.  For example $, % or some other clarification of the BuyerAgencyCompensation.
    ///
    /// [BuyerAgencyCompensationType](https://ddwiki.reso.org/display/DDW17/BuyerAgencyCompensationType+Field)
    #[serde(
        rename = "BuyerAgencyCompensationType",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agency_compensation_type: Option<crate::CompensationType>,

    /// The Buyer's Agent's Board or Association of REALTORS.
    ///
    /// [BuyerAgentAOR](https://ddwiki.reso.org/display/DDW17/BuyerAgentAOR+Field)
    #[serde(rename = "BuyerAgentAOR", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_aor: Option<String>,

    /// Designations and certifications acknowledging experience and expertise in various real estate sectors are awarded by NAR and each affiliated group upon completion of required courses.
    ///
    /// [BuyerAgentDesignation](https://ddwiki.reso.org/display/DDW17/BuyerAgentDesignation+Field)
    #[serde(
        rename = "BuyerAgentDesignation",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub buyer_agent_designation: Option<Vec<crate::BuyerAgentDesignation>>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentDirectPhone](https://ddwiki.reso.org/display/DDW17/BuyerAgentDirectPhone+Field)
    #[serde(
        rename = "BuyerAgentDirectPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_direct_phone: Option<String>,

    /// The email address of the Buyer's Agent.
    ///
    /// [BuyerAgentEmail](https://ddwiki.reso.org/display/DDW17/BuyerAgentEmail+Field)
    #[serde(rename = "BuyerAgentEmail", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentFax](https://ddwiki.reso.org/display/DDW17/BuyerAgentFax+Field)
    #[serde(rename = "BuyerAgentFax", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_fax: Option<String>,

    /// The first name of the buyer's agent.
    ///
    /// [BuyerAgentFirstName](https://ddwiki.reso.org/display/DDW17/BuyerAgentFirstName+Field)
    #[serde(
        rename = "BuyerAgentFirstName",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_first_name: Option<String>,

    /// The full name of the buyer's agent. (First Middle Last)
    ///
    /// [BuyerAgentFullName](https://ddwiki.reso.org/display/DDW17/BuyerAgentFullName+Field)
    #[serde(rename = "BuyerAgentFullName", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_full_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentHomePhone](https://ddwiki.reso.org/display/DDW17/BuyerAgentHomePhone+Field)
    #[serde(
        rename = "BuyerAgentHomePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_home_phone: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [BuyerAgentKey](https://ddwiki.reso.org/display/DDW17/BuyerAgentKey+Field)
    #[serde(rename = "BuyerAgentKey", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the BuyerAgentKey field.
    ///
    /// [BuyerAgentKeyNumeric](https://ddwiki.reso.org/display/DDW17/BuyerAgentKeyNumeric+Field)
    #[serde(
        rename = "BuyerAgentKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_key_numeric: Option<f64>,

    /// The last name of the buyer's agent.
    ///
    /// [BuyerAgentLastName](https://ddwiki.reso.org/display/DDW17/BuyerAgentLastName+Field)
    #[serde(rename = "BuyerAgentLastName", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_last_name: Option<String>,

    /// The middle name of the buyer's agent.
    ///
    /// [BuyerAgentMiddleName](https://ddwiki.reso.org/display/DDW17/BuyerAgentMiddleName+Field)
    #[serde(
        rename = "BuyerAgentMiddleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_middle_name: Option<String>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [BuyerAgentMlsId](https://ddwiki.reso.org/display/DDW17/BuyerAgentMlsId+Field)
    #[serde(rename = "BuyerAgentMlsId", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_mls_id: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentMobilePhone](https://ddwiki.reso.org/display/DDW17/BuyerAgentMobilePhone+Field)
    #[serde(
        rename = "BuyerAgentMobilePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_mobile_phone: Option<String>,

    /// Prefix to the name (e.g. Dr. Mr. Ms. etc.)
    ///
    /// [BuyerAgentNamePrefix](https://ddwiki.reso.org/display/DDW17/BuyerAgentNamePrefix+Field)
    #[serde(
        rename = "BuyerAgentNamePrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_name_prefix: Option<String>,

    /// Suffix to the BuyerAgentLastName (e.g. Esq., Jr., III etc.)
    ///
    /// [BuyerAgentNameSuffix](https://ddwiki.reso.org/display/DDW17/BuyerAgentNameSuffix+Field)
    #[serde(
        rename = "BuyerAgentNameSuffix",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_name_suffix: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentOfficePhone](https://ddwiki.reso.org/display/DDW17/BuyerAgentOfficePhone+Field)
    #[serde(
        rename = "BuyerAgentOfficePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [BuyerAgentOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/BuyerAgentOfficePhoneExt+Field)
    #[serde(
        rename = "BuyerAgentOfficePhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_office_phone_ext: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentPager](https://ddwiki.reso.org/display/DDW17/BuyerAgentPager+Field)
    #[serde(rename = "BuyerAgentPager", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_pager: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentPreferredPhone](https://ddwiki.reso.org/display/DDW17/BuyerAgentPreferredPhone+Field)
    #[serde(
        rename = "BuyerAgentPreferredPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_preferred_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [BuyerAgentPreferredPhoneExt](https://ddwiki.reso.org/display/DDW17/BuyerAgentPreferredPhoneExt+Field)
    #[serde(
        rename = "BuyerAgentPreferredPhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_preferred_phone_ext: Option<String>,

    /// The license of the buyers agent. Separate multiple licenses with a comma and space.
    ///
    /// [BuyerAgentStateLicense](https://ddwiki.reso.org/display/DDW17/BuyerAgentStateLicense+Field)
    #[serde(
        rename = "BuyerAgentStateLicense",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_state_license: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentTollFreePhone](https://ddwiki.reso.org/display/DDW17/BuyerAgentTollFreePhone+Field)
    #[serde(
        rename = "BuyerAgentTollFreePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_toll_free_phone: Option<String>,

    /// The website URI of the buyers agent.
    ///
    /// [BuyerAgentURL](https://ddwiki.reso.org/display/DDW17/BuyerAgentURL+Field)
    #[serde(rename = "BuyerAgentURL", skip_serializing_if = "Option::is_none")]
    pub buyer_agent_url: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerAgentVoiceMail](https://ddwiki.reso.org/display/DDW17/BuyerAgentVoiceMail+Field)
    #[serde(
        rename = "BuyerAgentVoiceMail",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_voice_mail: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [BuyerAgentVoiceMailExt](https://ddwiki.reso.org/display/DDW17/BuyerAgentVoiceMailExt+Field)
    #[serde(
        rename = "BuyerAgentVoiceMailExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_agent_voice_mail_ext: Option<String>,

    /// A list of options that describe the type of financing used.  This field is used when setting a listing to Closed.  i.e. cash, FHA loan, etc.
    ///
    /// [BuyerFinancing](https://ddwiki.reso.org/display/DDW17/BuyerFinancing+Field)
    #[serde(rename = "BuyerFinancing", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub buyer_financing: Option<Vec<crate::BuyerFinancing>>,

    /// The Buyer's Office's Board or Association of REALTORS.
    ///
    /// [BuyerOfficeAOR](https://ddwiki.reso.org/display/DDW17/BuyerOfficeAOR+Field)
    #[serde(rename = "BuyerOfficeAOR", skip_serializing_if = "Option::is_none")]
    pub buyer_office_aor: Option<String>,

    /// The email address of the Buyer's Office.
    ///
    /// [BuyerOfficeEmail](https://ddwiki.reso.org/display/DDW17/BuyerOfficeEmail+Field)
    #[serde(rename = "BuyerOfficeEmail", skip_serializing_if = "Option::is_none")]
    pub buyer_office_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerOfficeFax](https://ddwiki.reso.org/display/DDW17/BuyerOfficeFax+Field)
    #[serde(rename = "BuyerOfficeFax", skip_serializing_if = "Option::is_none")]
    pub buyer_office_fax: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Office resource's OfficeKey.
    ///
    /// [BuyerOfficeKey](https://ddwiki.reso.org/display/DDW17/BuyerOfficeKey+Field)
    #[serde(rename = "BuyerOfficeKey", skip_serializing_if = "Option::is_none")]
    pub buyer_office_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Office resource's OfficeKey.  This is the numeric only key and used as an alternative to the BuyerOfficeKey field.
    ///
    /// [BuyerOfficeKeyNumeric](https://ddwiki.reso.org/display/DDW17/BuyerOfficeKeyNumeric+Field)
    #[serde(
        rename = "BuyerOfficeKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_office_key_numeric: Option<f64>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [BuyerOfficeMlsId](https://ddwiki.reso.org/display/DDW17/BuyerOfficeMlsId+Field)
    #[serde(rename = "BuyerOfficeMlsId", skip_serializing_if = "Option::is_none")]
    pub buyer_office_mls_id: Option<String>,

    /// The legal name of the brokerage representing the buyer.
    ///
    /// [BuyerOfficeName](https://ddwiki.reso.org/display/DDW17/BuyerOfficeName+Field)
    #[serde(rename = "BuyerOfficeName", skip_serializing_if = "Option::is_none")]
    pub buyer_office_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [BuyerOfficePhone](https://ddwiki.reso.org/display/DDW17/BuyerOfficePhone+Field)
    #[serde(rename = "BuyerOfficePhone", skip_serializing_if = "Option::is_none")]
    pub buyer_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [BuyerOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/BuyerOfficePhoneExt+Field)
    #[serde(
        rename = "BuyerOfficePhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_office_phone_ext: Option<String>,

    /// The website URI for the buyers office.
    ///
    /// [BuyerOfficeURL](https://ddwiki.reso.org/display/DDW17/BuyerOfficeURL+Field)
    #[serde(rename = "BuyerOfficeURL", skip_serializing_if = "Option::is_none")]
    pub buyer_office_url: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Teams resource's TeamKey.
    ///
    /// [BuyerTeamKey](https://ddwiki.reso.org/display/DDW17/BuyerTeamKey+Field)
    #[serde(rename = "BuyerTeamKey", skip_serializing_if = "Option::is_none")]
    pub buyer_team_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Teams resource's TeamKey.  This is the numeric only key and used as an alternative to the BuyerTeamKey field.
    ///
    /// [BuyerTeamKeyNumeric](https://ddwiki.reso.org/display/DDW17/BuyerTeamKeyNumeric+Field)
    #[serde(
        rename = "BuyerTeamKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_team_key_numeric: Option<f64>,

    /// The name of the team representing the buyer.
    ///
    /// [BuyerTeamName](https://ddwiki.reso.org/display/DDW17/BuyerTeamName+Field)
    #[serde(rename = "BuyerTeamName", skip_serializing_if = "Option::is_none")]
    pub buyer_team_name: Option<String>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [CableTvExpense](https://ddwiki.reso.org/display/DDW17/CableTvExpense+Field)
    #[serde(rename = "CableTvExpense", skip_serializing_if = "Option::is_none")]
    pub cable_tv_expense: Option<f64>,

    /// Date the listing contract between the seller and listing agent was cancelled. This is the date entered by the agent reflecting when the change occurred contractually, not a timestamp of when the change was made in the MLS.
    ///
    /// [CancellationDate](https://ddwiki.reso.org/display/DDW17/CancellationDate+Field)
    #[serde(rename = "CancellationDate", skip_serializing_if = "Option::is_none")]
    pub cancellation_date: Option<chrono::NaiveDate>,

    /// Cap Rate is equivalent to the return on investment you would receive if you pay cash for a property. The ratio between the net operating income produced by an asset and its capital cost (the original price paid to buy the asset) or alternatively its current market value.
    ///
    /// [CapRate](https://ddwiki.reso.org/display/DDW17/CapRate+Field)
    #[serde(rename = "CapRate", skip_serializing_if = "Option::is_none")]
    pub cap_rate: Option<f64>,

    /// The number of carport spaces included in the sale.
    ///
    /// [CarportSpaces](https://ddwiki.reso.org/display/DDW17/CarportSpaces+Field)
    #[serde(rename = "CarportSpaces", skip_serializing_if = "Option::is_none")]
    pub carport_spaces: Option<f64>,

    /// A flag indicating that the listing has a car port. This flag may be T/F, Y/N or other true, false or unknown indicator. As with all flags, the field may be null.
    ///
    /// [CarportYN](https://ddwiki.reso.org/display/DDW17/CarportYN+Field)
    #[serde(rename = "CarportYN", skip_serializing_if = "Option::is_none")]
    pub carport_yn: Option<bool>,

    /// The group of addresses to which the USPS assigns the same code to aid in mail delivery. For the USPS, these codes are 9 digits: 5 numbers for the ZIP Code, one letter for the carrier route type, and 3 numbers for the carrier route number.
    ///
    /// [CarrierRoute](https://ddwiki.reso.org/display/DDW17/CarrierRoute+Field)
    #[serde(rename = "CarrierRoute", skip_serializing_if = "Option::is_none")]
    pub carrier_route: Option<String>,

    /// The city in listing address.
    ///
    /// [City](https://ddwiki.reso.org/display/DDW17/City+Field)
    #[serde(rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// A sub-section or area of a defined city.  Examples would be SOHO in New York, NY, Ironbound in Newark, NJ or Inside the Beltway.
    ///
    /// [CityRegion](https://ddwiki.reso.org/display/DDW17/CityRegion+Field)
    #[serde(rename = "CityRegion", skip_serializing_if = "Option::is_none")]
    pub city_region: Option<String>,

    /// With for-sale listings, the date the purchase agreement was fulfilled. With lease listings, the date the requirements were fulfilled, such as contract and/or deposit.  This is the date entered by the agent reflecting when the change occurred contractually, not a timestamp of when the change was made in the MLS.
    ///
    /// [CloseDate](https://ddwiki.reso.org/display/DDW17/CloseDate+Field)
    #[serde(rename = "CloseDate", skip_serializing_if = "Option::is_none")]
    pub close_date: Option<chrono::NaiveDate>,

    /// The amount of money paid by the purchaser to the seller for the property under the agreement.
    ///
    /// [ClosePrice](https://ddwiki.reso.org/display/DDW17/ClosePrice+Field)
    #[serde(rename = "ClosePrice", skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,

    /// The Co Buyer's Agent's Board or Association of REALTORS.
    ///
    /// [CoBuyerAgentAOR](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentAOR+Field)
    #[serde(rename = "CoBuyerAgentAOR", skip_serializing_if = "Option::is_none")]
    pub co_buyer_agent_aor: Option<String>,

    /// Designations and certifications acknowledging experience and expertise in various real estate sectors are awarded by NAR and each affiliated group upon completion of required courses.
    ///
    /// [CoBuyerAgentDesignation](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentDesignation+Field)
    #[serde(
        rename = "CoBuyerAgentDesignation",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub co_buyer_agent_designation: Option<Vec<crate::CoBuyerAgentDesignation>>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentDirectPhone](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentDirectPhone+Field)
    #[serde(
        rename = "CoBuyerAgentDirectPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_direct_phone: Option<String>,

    /// The email address of the Buyer's Co Agent.
    ///
    /// [CoBuyerAgentEmail](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentEmail+Field)
    #[serde(rename = "CoBuyerAgentEmail", skip_serializing_if = "Option::is_none")]
    pub co_buyer_agent_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentFax](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentFax+Field)
    #[serde(rename = "CoBuyerAgentFax", skip_serializing_if = "Option::is_none")]
    pub co_buyer_agent_fax: Option<String>,

    /// The first name of the buyer's co-agent.
    ///
    /// [CoBuyerAgentFirstName](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentFirstName+Field)
    #[serde(
        rename = "CoBuyerAgentFirstName",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_first_name: Option<String>,

    /// The full name of the buyer's co-agent. (First Middle Last)
    ///
    /// [CoBuyerAgentFullName](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentFullName+Field)
    #[serde(
        rename = "CoBuyerAgentFullName",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_full_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentHomePhone](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentHomePhone+Field)
    #[serde(
        rename = "CoBuyerAgentHomePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_home_phone: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [CoBuyerAgentKey](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentKey+Field)
    #[serde(rename = "CoBuyerAgentKey", skip_serializing_if = "Option::is_none")]
    pub co_buyer_agent_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the CoBuyerAgentKey field.
    ///
    /// [CoBuyerAgentKeyNumeric](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentKeyNumeric+Field)
    #[serde(
        rename = "CoBuyerAgentKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_key_numeric: Option<f64>,

    /// The last name of the buyer's co-agent.
    ///
    /// [CoBuyerAgentLastName](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentLastName+Field)
    #[serde(
        rename = "CoBuyerAgentLastName",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_last_name: Option<String>,

    /// The middle name of the buyer's co-agent.
    ///
    /// [CoBuyerAgentMiddleName](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentMiddleName+Field)
    #[serde(
        rename = "CoBuyerAgentMiddleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_middle_name: Option<String>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [CoBuyerAgentMlsId](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentMlsId+Field)
    #[serde(rename = "CoBuyerAgentMlsId", skip_serializing_if = "Option::is_none")]
    pub co_buyer_agent_mls_id: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentMobilePhone](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentMobilePhone+Field)
    #[serde(
        rename = "CoBuyerAgentMobilePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_mobile_phone: Option<String>,

    /// Prefix to the name (e.g. Dr. Mr. Ms. etc.)
    ///
    /// [CoBuyerAgentNamePrefix](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentNamePrefix+Field)
    #[serde(
        rename = "CoBuyerAgentNamePrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_name_prefix: Option<String>,

    /// Suffix to the CoBuyerAgentLastName (e.g. Esq., Jr., III etc.)
    ///
    /// [CoBuyerAgentNameSuffix](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentNameSuffix+Field)
    #[serde(
        rename = "CoBuyerAgentNameSuffix",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_name_suffix: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentOfficePhone](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentOfficePhone+Field)
    #[serde(
        rename = "CoBuyerAgentOfficePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [CoBuyerAgentOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentOfficePhoneExt+Field)
    #[serde(
        rename = "CoBuyerAgentOfficePhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_office_phone_ext: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentPager](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentPager+Field)
    #[serde(rename = "CoBuyerAgentPager", skip_serializing_if = "Option::is_none")]
    pub co_buyer_agent_pager: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentPreferredPhone](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentPreferredPhone+Field)
    #[serde(
        rename = "CoBuyerAgentPreferredPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_preferred_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [CoBuyerAgentPreferredPhoneExt](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentPreferredPhoneExt+Field)
    #[serde(
        rename = "CoBuyerAgentPreferredPhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_preferred_phone_ext: Option<String>,

    /// The license of the co-buyers agent. Separate multiple licenses with a comma and space.
    ///
    /// [CoBuyerAgentStateLicense](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentStateLicense+Field)
    #[serde(
        rename = "CoBuyerAgentStateLicense",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_state_license: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentTollFreePhone](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentTollFreePhone+Field)
    #[serde(
        rename = "CoBuyerAgentTollFreePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_toll_free_phone: Option<String>,

    /// The website URI of the co-buyers agent.
    ///
    /// [CoBuyerAgentURL](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentURL+Field)
    #[serde(rename = "CoBuyerAgentURL", skip_serializing_if = "Option::is_none")]
    pub co_buyer_agent_url: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerAgentVoiceMail](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentVoiceMail+Field)
    #[serde(
        rename = "CoBuyerAgentVoiceMail",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_voice_mail: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [CoBuyerAgentVoiceMailExt](https://ddwiki.reso.org/display/DDW17/CoBuyerAgentVoiceMailExt+Field)
    #[serde(
        rename = "CoBuyerAgentVoiceMailExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_agent_voice_mail_ext: Option<String>,

    /// The Co Buyer's Office's Board or Association of REALTORS.
    ///
    /// [CoBuyerOfficeAOR](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficeAOR+Field)
    #[serde(rename = "CoBuyerOfficeAOR", skip_serializing_if = "Option::is_none")]
    pub co_buyer_office_aor: Option<String>,

    /// The email address of the Buyer's Co Office.
    ///
    /// [CoBuyerOfficeEmail](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficeEmail+Field)
    #[serde(rename = "CoBuyerOfficeEmail", skip_serializing_if = "Option::is_none")]
    pub co_buyer_office_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerOfficeFax](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficeFax+Field)
    #[serde(rename = "CoBuyerOfficeFax", skip_serializing_if = "Option::is_none")]
    pub co_buyer_office_fax: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Office resource's OfficeKey.
    ///
    /// [CoBuyerOfficeKey](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficeKey+Field)
    #[serde(rename = "CoBuyerOfficeKey", skip_serializing_if = "Option::is_none")]
    pub co_buyer_office_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Office resource's OfficeKey.  This is the numeric only key and used as an alternative to the CoBuyerOfficeKey field.
    ///
    /// [CoBuyerOfficeKeyNumeric](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficeKeyNumeric+Field)
    #[serde(
        rename = "CoBuyerOfficeKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_office_key_numeric: Option<f64>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [CoBuyerOfficeMlsId](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficeMlsId+Field)
    #[serde(rename = "CoBuyerOfficeMlsId", skip_serializing_if = "Option::is_none")]
    pub co_buyer_office_mls_id: Option<String>,

    /// The legal name of the brokerage co-representing the buyer.
    ///
    /// [CoBuyerOfficeName](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficeName+Field)
    #[serde(rename = "CoBuyerOfficeName", skip_serializing_if = "Option::is_none")]
    pub co_buyer_office_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [CoBuyerOfficePhone](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficePhone+Field)
    #[serde(rename = "CoBuyerOfficePhone", skip_serializing_if = "Option::is_none")]
    pub co_buyer_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [CoBuyerOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficePhoneExt+Field)
    #[serde(
        rename = "CoBuyerOfficePhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_buyer_office_phone_ext: Option<String>,

    /// The website URI for the co-buyers office.
    ///
    /// [CoBuyerOfficeURL](https://ddwiki.reso.org/display/DDW17/CoBuyerOfficeURL+Field)
    #[serde(rename = "CoBuyerOfficeURL", skip_serializing_if = "Option::is_none")]
    pub co_buyer_office_url: Option<String>,

    /// The Co Listing Agent's Board or Association of REALTORS.
    ///
    /// [CoListAgentAOR](https://ddwiki.reso.org/display/DDW17/CoListAgentAOR+Field)
    #[serde(rename = "CoListAgentAOR", skip_serializing_if = "Option::is_none")]
    pub co_list_agent_aor: Option<String>,

    /// Designations and certifications acknowledging experience and expertise in various real estate sectors are awarded by NAR and each affiliated group upon completion of required courses.
    ///
    /// [CoListAgentDesignation](https://ddwiki.reso.org/display/DDW17/CoListAgentDesignation+Field)
    #[serde(
        rename = "CoListAgentDesignation",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub co_list_agent_designation: Option<Vec<crate::CoListAgentDesignation>>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentDirectPhone](https://ddwiki.reso.org/display/DDW17/CoListAgentDirectPhone+Field)
    #[serde(
        rename = "CoListAgentDirectPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_direct_phone: Option<String>,

    /// The email address of the Co Listing Agent.
    ///
    /// [CoListAgentEmail](https://ddwiki.reso.org/display/DDW17/CoListAgentEmail+Field)
    #[serde(rename = "CoListAgentEmail", skip_serializing_if = "Option::is_none")]
    pub co_list_agent_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentFax](https://ddwiki.reso.org/display/DDW17/CoListAgentFax+Field)
    #[serde(rename = "CoListAgentFax", skip_serializing_if = "Option::is_none")]
    pub co_list_agent_fax: Option<String>,

    /// The first name of the co-listing agent.
    ///
    /// [CoListAgentFirstName](https://ddwiki.reso.org/display/DDW17/CoListAgentFirstName+Field)
    #[serde(
        rename = "CoListAgentFirstName",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_first_name: Option<String>,

    /// The full name of the co-listing agent. (First Middle Last)
    ///
    /// [CoListAgentFullName](https://ddwiki.reso.org/display/DDW17/CoListAgentFullName+Field)
    #[serde(
        rename = "CoListAgentFullName",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_full_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentHomePhone](https://ddwiki.reso.org/display/DDW17/CoListAgentHomePhone+Field)
    #[serde(
        rename = "CoListAgentHomePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_home_phone: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [CoListAgentKey](https://ddwiki.reso.org/display/DDW17/CoListAgentKey+Field)
    #[serde(rename = "CoListAgentKey", skip_serializing_if = "Option::is_none")]
    pub co_list_agent_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the CoListAgentKey field.
    ///
    /// [CoListAgentKeyNumeric](https://ddwiki.reso.org/display/DDW17/CoListAgentKeyNumeric+Field)
    #[serde(
        rename = "CoListAgentKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_key_numeric: Option<f64>,

    /// The last name of the co-listing agent.
    ///
    /// [CoListAgentLastName](https://ddwiki.reso.org/display/DDW17/CoListAgentLastName+Field)
    #[serde(
        rename = "CoListAgentLastName",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_last_name: Option<String>,

    /// The middle name of the co-listing agent.
    ///
    /// [CoListAgentMiddleName](https://ddwiki.reso.org/display/DDW17/CoListAgentMiddleName+Field)
    #[serde(
        rename = "CoListAgentMiddleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_middle_name: Option<String>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [CoListAgentMlsId](https://ddwiki.reso.org/display/DDW17/CoListAgentMlsId+Field)
    #[serde(rename = "CoListAgentMlsId", skip_serializing_if = "Option::is_none")]
    pub co_list_agent_mls_id: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentMobilePhone](https://ddwiki.reso.org/display/DDW17/CoListAgentMobilePhone+Field)
    #[serde(
        rename = "CoListAgentMobilePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_mobile_phone: Option<String>,

    /// Prefix to the name (e.g. Dr. Mr. Ms. etc.)
    ///
    /// [CoListAgentNamePrefix](https://ddwiki.reso.org/display/DDW17/CoListAgentNamePrefix+Field)
    #[serde(
        rename = "CoListAgentNamePrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_name_prefix: Option<String>,

    /// Suffix to the CoListAgentLastName (e.g. Esq., Jr., III etc.)
    ///
    /// [CoListAgentNameSuffix](https://ddwiki.reso.org/display/DDW17/CoListAgentNameSuffix+Field)
    #[serde(
        rename = "CoListAgentNameSuffix",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_name_suffix: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentOfficePhone](https://ddwiki.reso.org/display/DDW17/CoListAgentOfficePhone+Field)
    #[serde(
        rename = "CoListAgentOfficePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [CoListAgentOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/CoListAgentOfficePhoneExt+Field)
    #[serde(
        rename = "CoListAgentOfficePhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_office_phone_ext: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentPager](https://ddwiki.reso.org/display/DDW17/CoListAgentPager+Field)
    #[serde(rename = "CoListAgentPager", skip_serializing_if = "Option::is_none")]
    pub co_list_agent_pager: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentPreferredPhone](https://ddwiki.reso.org/display/DDW17/CoListAgentPreferredPhone+Field)
    #[serde(
        rename = "CoListAgentPreferredPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_preferred_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [CoListAgentPreferredPhoneExt](https://ddwiki.reso.org/display/DDW17/CoListAgentPreferredPhoneExt+Field)
    #[serde(
        rename = "CoListAgentPreferredPhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_preferred_phone_ext: Option<String>,

    /// The license of the co-listing agent. Separate multiple licenses with a comma and space.
    ///
    /// [CoListAgentStateLicense](https://ddwiki.reso.org/display/DDW17/CoListAgentStateLicense+Field)
    #[serde(
        rename = "CoListAgentStateLicense",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_state_license: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentTollFreePhone](https://ddwiki.reso.org/display/DDW17/CoListAgentTollFreePhone+Field)
    #[serde(
        rename = "CoListAgentTollFreePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_toll_free_phone: Option<String>,

    /// The website URI of the co-listing agent.
    ///
    /// [CoListAgentURL](https://ddwiki.reso.org/display/DDW17/CoListAgentURL+Field)
    #[serde(rename = "CoListAgentURL", skip_serializing_if = "Option::is_none")]
    pub co_list_agent_url: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [CoListAgentVoiceMail](https://ddwiki.reso.org/display/DDW17/CoListAgentVoiceMail+Field)
    #[serde(
        rename = "CoListAgentVoiceMail",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_voice_mail: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [CoListAgentVoiceMailExt](https://ddwiki.reso.org/display/DDW17/CoListAgentVoiceMailExt+Field)
    #[serde(
        rename = "CoListAgentVoiceMailExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_agent_voice_mail_ext: Option<String>,

    /// The Co Listing Office's Board or Association of REALTORS.
    ///
    /// [CoListOfficeAOR](https://ddwiki.reso.org/display/DDW17/CoListOfficeAOR+Field)
    #[serde(rename = "CoListOfficeAOR", skip_serializing_if = "Option::is_none")]
    pub co_list_office_aor: Option<String>,

    /// The email address of the Co Listing Office.
    ///
    /// [CoListOfficeEmail](https://ddwiki.reso.org/display/DDW17/CoListOfficeEmail+Field)
    #[serde(rename = "CoListOfficeEmail", skip_serializing_if = "Option::is_none")]
    pub co_list_office_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [CoListOfficeFax](https://ddwiki.reso.org/display/DDW17/CoListOfficeFax+Field)
    #[serde(rename = "CoListOfficeFax", skip_serializing_if = "Option::is_none")]
    pub co_list_office_fax: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Office resource's OfficeKey.
    ///
    /// [CoListOfficeKey](https://ddwiki.reso.org/display/DDW17/CoListOfficeKey+Field)
    #[serde(rename = "CoListOfficeKey", skip_serializing_if = "Option::is_none")]
    pub co_list_office_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Office resource's OfficeKey.  This is the numeric only key and used as an alternative to the CoListOfficeKey field.
    ///
    /// [CoListOfficeKeyNumeric](https://ddwiki.reso.org/display/DDW17/CoListOfficeKeyNumeric+Field)
    #[serde(
        rename = "CoListOfficeKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_office_key_numeric: Option<f64>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [CoListOfficeMlsId](https://ddwiki.reso.org/display/DDW17/CoListOfficeMlsId+Field)
    #[serde(rename = "CoListOfficeMlsId", skip_serializing_if = "Option::is_none")]
    pub co_list_office_mls_id: Option<String>,

    /// The legal name of the brokerage co-representing the seller.
    ///
    /// [CoListOfficeName](https://ddwiki.reso.org/display/DDW17/CoListOfficeName+Field)
    #[serde(rename = "CoListOfficeName", skip_serializing_if = "Option::is_none")]
    pub co_list_office_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [CoListOfficePhone](https://ddwiki.reso.org/display/DDW17/CoListOfficePhone+Field)
    #[serde(rename = "CoListOfficePhone", skip_serializing_if = "Option::is_none")]
    pub co_list_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [CoListOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/CoListOfficePhoneExt+Field)
    #[serde(
        rename = "CoListOfficePhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub co_list_office_phone_ext: Option<String>,

    /// The website URI for the co-listing office.
    ///
    /// [CoListOfficeURL](https://ddwiki.reso.org/display/DDW17/CoListOfficeURL+Field)
    #[serde(rename = "CoListOfficeURL", skip_serializing_if = "Option::is_none")]
    pub co_list_office_url: Option<String>,

    /// Common Interest is a type of ownership in a property that is composed of an individual lot or  unit and a share of the ownership or use of common areas. A Common Interest Development (CID) is usually governed by a recorded set of Covenants, Conditions & Restrictions (CC&Rs).
    ///
    /// [CommonInterest](https://ddwiki.reso.org/display/DDW17/CommonInterest+Field)
    #[serde(rename = "CommonInterest", skip_serializing_if = "Option::is_none")]
    pub common_interest: Option<crate::CommonInterest>,

    /// A multi select list with options like 1 Common Wall, 2 Common Walls, No Common Walls, No One Above, No One Below. Implementation should include rules preventing illogical selection combinations and to ensure consistency with the Property Attached Y/N field.
    ///
    /// [CommonWalls](https://ddwiki.reso.org/display/DDW17/CommonWalls+Field)
    #[serde(rename = "CommonWalls", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub common_walls: Option<Vec<crate::CommonWalls>>,

    /// A list of features related to, or available within, the community.
    ///
    /// [CommunityFeatures](https://ddwiki.reso.org/display/DDW17/CommunityFeatures+Field)
    #[serde(rename = "CommunityFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub community_features: Option<Vec<crate::CommunityFeatures>>,

    /// Are there concessions included in the sales agreement? Yes, No or Call Listing Agent
    ///
    /// [Concessions](https://ddwiki.reso.org/display/DDW17/Concessions+Field)
    #[serde(rename = "Concessions", skip_serializing_if = "Option::is_none")]
    pub concessions: Option<crate::Concessions>,

    /// The dollar amount of the concessions.  If the concessions are made by the seller, some may subtract this value from the sales price as a means of calculating their own true price.  If concessions are made by the buyer, some may add this amount to the sale price to create their own true price.  Concessions made by both buyer and seller should be subtracted from each other providing a net value.  Details of this calculation should be added to the Concessions Comments field.
    ///
    /// [ConcessionsAmount](https://ddwiki.reso.org/display/DDW17/ConcessionsAmount+Field)
    #[serde(rename = "ConcessionsAmount", skip_serializing_if = "Option::is_none")]
    pub concessions_amount: Option<f64>,

    /// Comments describing the concessions made by the buyer or the seller.
    ///
    /// [ConcessionsComments](https://ddwiki.reso.org/display/DDW17/ConcessionsComments+Field)
    #[serde(
        rename = "ConcessionsComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub concessions_comments: Option<String>,

    /// A list of the materials that were used in the construction of the property.
    ///
    /// [ConstructionMaterials](https://ddwiki.reso.org/display/DDW17/ConstructionMaterials+Field)
    #[serde(
        rename = "ConstructionMaterials",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub construction_materials: Option<Vec<crate::ConstructionMaterials>>,

    /// A sub-section or area of a continent.  Examples would be Southern Europe or Scandinavia.
    ///
    /// [ContinentRegion](https://ddwiki.reso.org/display/DDW17/ContinentRegion+Field)
    #[serde(rename = "ContinentRegion", skip_serializing_if = "Option::is_none")]
    pub continent_region: Option<String>,

    /// A list of contingencies that must be satisfied in order to complete the transaction.
    ///
    /// [Contingency](https://ddwiki.reso.org/display/DDW17/Contingency+Field)
    #[serde(rename = "Contingency", skip_serializing_if = "Option::is_none")]
    pub contingency: Option<String>,

    /// The date an offer was made with a contingency. The Listing remains On Market.  This is the date entered by the agent reflecting when the change occurred contractually, not a timestamp of when the change was made in the MLS.
    ///
    /// [ContingentDate](https://ddwiki.reso.org/display/DDW17/ContingentDate+Field)
    #[serde(rename = "ContingentDate", skip_serializing_if = "Option::is_none")]
    pub contingent_date: Option<chrono::NaiveDate>,

    /// The date of the listings contractual status change. This is not necessarily the time the agent made the change in the MLS system, but rather the date of the contractual change.
    ///
    /// [ContractStatusChangeDate](https://ddwiki.reso.org/display/DDW17/ContractStatusChangeDate+Field)
    #[serde(
        rename = "ContractStatusChangeDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_status_change_date: Option<chrono::NaiveDate>,

    /// A list describing the cooling or air conditioning features of the property.
    ///
    /// [Cooling](https://ddwiki.reso.org/display/DDW17/Cooling+Field)
    #[serde(rename = "Cooling", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub cooling: Option<Vec<crate::Cooling>>,

    /// The property has cooling or Air Conditioning.
    ///
    /// [CoolingYN](https://ddwiki.reso.org/display/DDW17/CoolingYN+Field)
    #[serde(rename = "CoolingYN", skip_serializing_if = "Option::is_none")]
    pub cooling_yn: Option<bool>,

    /// Notice of the legal rights of the owner of the information or data.
    ///
    /// [CopyrightNotice](https://ddwiki.reso.org/display/DDW17/CopyrightNotice+Field)
    #[serde(rename = "CopyrightNotice", skip_serializing_if = "Option::is_none")]
    pub copyright_notice: Option<String>,

    /// The country abbreviation in a postal address.
    ///
    /// [Country](https://ddwiki.reso.org/display/DDW17/Country+Field)
    #[serde(rename = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<crate::Country>,

    /// A sub-section or area of a defined country.  Examples would be Napa Valley in the US, or the Amalfi Coast in Italy.
    ///
    /// [CountryRegion](https://ddwiki.reso.org/display/DDW17/CountryRegion+Field)
    #[serde(rename = "CountryRegion", skip_serializing_if = "Option::is_none")]
    pub country_region: Option<String>,

    /// The County, Parish or other regional authority
    ///
    /// [CountyOrParish](https://ddwiki.reso.org/display/DDW17/CountyOrParish+Field)
    #[serde(rename = "CountyOrParish", skip_serializing_if = "Option::is_none")]
    pub county_or_parish: Option<String>,

    /// The total number of garage and carport spaces.
    ///
    /// [CoveredSpaces](https://ddwiki.reso.org/display/DDW17/CoveredSpaces+Field)
    #[serde(rename = "CoveredSpaces", skip_serializing_if = "Option::is_none")]
    pub covered_spaces: Option<f64>,

    /// Are crops included in the sale of the property.
    ///
    /// [CropsIncludedYN](https://ddwiki.reso.org/display/DDW17/CropsIncludedYN+Field)
    #[serde(rename = "CropsIncludedYN", skip_serializing_if = "Option::is_none")]
    pub crops_included_yn: Option<bool>,

    /// Nearest cross streets to the property. This field is in addition to, and independent of, the driving directions field.
    ///
    /// [CrossStreet](https://ddwiki.reso.org/display/DDW17/CrossStreet+Field)
    #[serde(rename = "CrossStreet", skip_serializing_if = "Option::is_none")]
    pub cross_street: Option<String>,

    /// Measurement or percentage of the property that has been cultivated.
    ///
    /// [CultivatedArea](https://ddwiki.reso.org/display/DDW17/CultivatedArea+Field)
    #[serde(rename = "CultivatedArea", skip_serializing_if = "Option::is_none")]
    pub cultivated_area: Option<f64>,

    /// The number of days the property is on market, as defined by the MLS business rules.
    ///
    /// [CumulativeDaysOnMarket](https://ddwiki.reso.org/display/DDW17/CumulativeDaysOnMarket+Field)
    #[serde(
        rename = "CumulativeDaysOnMarket",
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_days_on_market: Option<f64>,

    /// A list of options that describe the type of financing that the seller currently has in place for the property being sold.  i.e. cash, assumable, FHA loan, etc.
    ///
    /// [CurrentFinancing](https://ddwiki.reso.org/display/DDW17/CurrentFinancing+Field)
    #[serde(rename = "CurrentFinancing", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub current_financing: Option<Vec<crate::CurrentFinancing>>,

    /// A list of the type(s) of current use of the property. The current use of the property is an important factor in understanding the overall condition of the land and determining it's appropriateness for intended use.
    ///
    /// [CurrentUse](https://ddwiki.reso.org/display/DDW17/CurrentUse+Field)
    #[serde(rename = "CurrentUse", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub current_use: Option<Vec<crate::CurrentUse>>,

    /// Department of Housing decal number for the mobile or manufactured home.  For the first or only unit/section use DOH 1 over DOH 2 or 3.
    ///
    /// [DOH1](https://ddwiki.reso.org/display/DDW17/DOH1+Field)
    #[serde(rename = "DOH1", skip_serializing_if = "Option::is_none")]
    pub doh1: Option<String>,

    /// Department of Housing decal number for the mobile or manufactured home.  For two units/sections use DOH 1 and 2 over DOH 3.
    ///
    /// [DOH2](https://ddwiki.reso.org/display/DDW17/DOH2+Field)
    #[serde(rename = "DOH2", skip_serializing_if = "Option::is_none")]
    pub doh2: Option<String>,

    /// Department of Housing decal number for the mobile or manufactured home.  For two units/sections use DOH 1 and 2 over DOH 3.
    ///
    /// [DOH3](https://ddwiki.reso.org/display/DDW17/DOH3+Field)
    #[serde(rename = "DOH3", skip_serializing_if = "Option::is_none")]
    pub doh3: Option<String>,

    /// The number of days the listing is on market, as defined by the MLS business rules.
    ///
    /// [DaysOnMarket](https://ddwiki.reso.org/display/DDW17/DaysOnMarket+Field)
    #[serde(rename = "DaysOnMarket", skip_serializing_if = "Option::is_none")]
    pub days_on_market: Option<f64>,

    /// A list of the Development Status of the property. The developmental status of land is an important factor in selling, purchasing and developing of land properties.
    ///
    /// [DevelopmentStatus](https://ddwiki.reso.org/display/DDW17/DevelopmentStatus+Field)
    #[serde(rename = "DevelopmentStatus", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub development_status: Option<Vec<crate::DevelopmentStatus>>,

    /// The compass direction that the main entrance to the building faces. For example, North, South, East, West, South-West, etc. It may also be known as the building exposure.
    ///
    /// [DirectionFaces](https://ddwiki.reso.org/display/DDW17/DirectionFaces+Field)
    #[serde(rename = "DirectionFaces", skip_serializing_if = "Option::is_none")]
    pub direction_faces: Option<crate::DirectionFaces>,

    /// Driving directions to the property.
    ///
    /// [Directions](https://ddwiki.reso.org/display/DDW17/Directions+Field)
    #[serde(rename = "Directions", skip_serializing_if = "Option::is_none")]
    pub directions: Option<String>,

    /// Text that serves as the negation or limitation of the rights under a warranty given by a seller to a buyer.
    ///
    /// [Disclaimer](https://ddwiki.reso.org/display/DDW17/Disclaimer+Field)
    #[serde(rename = "Disclaimer", skip_serializing_if = "Option::is_none")]
    pub disclaimer: Option<String>,

    /// Legal or pertinent information that should be disclosed to potential buyer's agents.
    ///
    /// [Disclosures](https://ddwiki.reso.org/display/DDW17/Disclosures+Field)
    #[serde(rename = "Disclosures", skip_serializing_if = "Option::is_none")]
    pub disclosures: Option<String>,

    /// A textual description of the distance to local bus stops.
    ///
    /// [DistanceToBusComments](https://ddwiki.reso.org/display/DDW17/DistanceToBusComments+Field)
    #[serde(
        rename = "DistanceToBusComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_bus_comments: Option<String>,

    /// Numeric distance from the property to the nearest bus stop.
    ///
    /// [DistanceToBusNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToBusNumeric+Field)
    #[serde(
        rename = "DistanceToBusNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_bus_numeric: Option<f64>,

    /// A pick list of the unit linear measurement. i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToBusUnits](https://ddwiki.reso.org/display/DDW17/DistanceToBusUnits+Field)
    #[serde(rename = "DistanceToBusUnits", skip_serializing_if = "Option::is_none")]
    pub distance_to_bus_units: Option<crate::LinearUnits>,

    /// If the property does not currently have electrical utility, is service available and if so, what is the distance.
    ///
    /// [DistanceToElectricComments](https://ddwiki.reso.org/display/DDW17/DistanceToElectricComments+Field)
    #[serde(
        rename = "DistanceToElectricComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_electric_comments: Option<String>,

    /// Numeric distance from the property to the electrical utility.
    ///
    /// [DistanceToElectricNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToElectricNumeric+Field)
    #[serde(
        rename = "DistanceToElectricNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_electric_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToElectricUnits](https://ddwiki.reso.org/display/DDW17/DistanceToElectricUnits+Field)
    #[serde(
        rename = "DistanceToElectricUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_electric_units: Option<crate::LinearUnits>,

    /// A textual description of the distance to freeways.
    ///
    /// [DistanceToFreewayComments](https://ddwiki.reso.org/display/DDW17/DistanceToFreewayComments+Field)
    #[serde(
        rename = "DistanceToFreewayComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_freeway_comments: Option<String>,

    /// Numeric distance from the property to the nearest freeway.
    ///
    /// [DistanceToFreewayNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToFreewayNumeric+Field)
    #[serde(
        rename = "DistanceToFreewayNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_freeway_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToFreewayUnits](https://ddwiki.reso.org/display/DDW17/DistanceToFreewayUnits+Field)
    #[serde(
        rename = "DistanceToFreewayUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_freeway_units: Option<crate::LinearUnits>,

    /// If the property does not currently have natural gas utility, is service available and if so, what is the distance.
    ///
    /// [DistanceToGasComments](https://ddwiki.reso.org/display/DDW17/DistanceToGasComments+Field)
    #[serde(
        rename = "DistanceToGasComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_gas_comments: Option<String>,

    /// Numeric distance from the property to the gas utility.
    ///
    /// [DistanceToGasNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToGasNumeric+Field)
    #[serde(
        rename = "DistanceToGasNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_gas_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToGasUnits](https://ddwiki.reso.org/display/DDW17/DistanceToGasUnits+Field)
    #[serde(rename = "DistanceToGasUnits", skip_serializing_if = "Option::is_none")]
    pub distance_to_gas_units: Option<crate::LinearUnits>,

    /// If the property does not currently have phone service, is service available and if so, what is the distance.
    ///
    /// [DistanceToPhoneServiceComments](https://ddwiki.reso.org/display/DDW17/DistanceToPhoneServiceComments+Field)
    #[serde(
        rename = "DistanceToPhoneServiceComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_phone_service_comments: Option<String>,

    /// Numeric distance from the property to the phone utility.
    ///
    /// [DistanceToPhoneServiceNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToPhoneServiceNumeric+Field)
    #[serde(
        rename = "DistanceToPhoneServiceNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_phone_service_numeric: Option<f64>,

    /// A pick list of the unit linear measurement. i.e. feet, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToPhoneServiceUnits](https://ddwiki.reso.org/display/DDW17/DistanceToPhoneServiceUnits+Field)
    #[serde(
        rename = "DistanceToPhoneServiceUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_phone_service_units: Option<crate::LinearUnits>,

    /// A textual description of the distance to local places of worship.
    ///
    /// [DistanceToPlaceofWorshipComments](https://ddwiki.reso.org/display/DDW17/DistanceToPlaceofWorshipComments+Field)
    #[serde(
        rename = "DistanceToPlaceofWorshipComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_placeof_worship_comments: Option<String>,

    /// Numeric distance from the property to the nearest place of worship.
    ///
    /// [DistanceToPlaceofWorshipNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToPlaceofWorshipNumeric+Field)
    #[serde(
        rename = "DistanceToPlaceofWorshipNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_placeof_worship_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToPlaceofWorshipUnits](https://ddwiki.reso.org/display/DDW17/DistanceToPlaceofWorshipUnits+Field)
    #[serde(
        rename = "DistanceToPlaceofWorshipUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_placeof_worship_units: Option<crate::LinearUnits>,

    /// Distance from the property to the nearest school bus pickup point.
    ///
    /// [DistanceToSchoolBusComments](https://ddwiki.reso.org/display/DDW17/DistanceToSchoolBusComments+Field)
    #[serde(
        rename = "DistanceToSchoolBusComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_school_bus_comments: Option<String>,

    /// Numeric distance from the property to the nearest school bus pickup point.
    ///
    /// [DistanceToSchoolBusNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToSchoolBusNumeric+Field)
    #[serde(
        rename = "DistanceToSchoolBusNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_school_bus_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToSchoolBusUnits](https://ddwiki.reso.org/display/DDW17/DistanceToSchoolBusUnits+Field)
    #[serde(
        rename = "DistanceToSchoolBusUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_school_bus_units: Option<crate::LinearUnits>,

    /// A textual description of the distance to local schools.
    ///
    /// [DistanceToSchoolsComments](https://ddwiki.reso.org/display/DDW17/DistanceToSchoolsComments+Field)
    #[serde(
        rename = "DistanceToSchoolsComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_schools_comments: Option<String>,

    /// Numeric distance from the property to the nearest school.
    ///
    /// [DistanceToSchoolsNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToSchoolsNumeric+Field)
    #[serde(
        rename = "DistanceToSchoolsNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_schools_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToSchoolsUnits](https://ddwiki.reso.org/display/DDW17/DistanceToSchoolsUnits+Field)
    #[serde(
        rename = "DistanceToSchoolsUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_schools_units: Option<crate::LinearUnits>,

    /// If the property does not currently have sewer or septic, is sewer service available and if so, what is the distance.
    ///
    /// [DistanceToSewerComments](https://ddwiki.reso.org/display/DDW17/DistanceToSewerComments+Field)
    #[serde(
        rename = "DistanceToSewerComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_sewer_comments: Option<String>,

    /// Numeric distance from the property to the sewer utility.
    ///
    /// [DistanceToSewerNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToSewerNumeric+Field)
    #[serde(
        rename = "DistanceToSewerNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_sewer_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToSewerUnits](https://ddwiki.reso.org/display/DDW17/DistanceToSewerUnits+Field)
    #[serde(
        rename = "DistanceToSewerUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_sewer_units: Option<crate::LinearUnits>,

    /// A description of the distance to primary shopping sources such as groceries, gasoline, clothing or department stores.
    ///
    /// [DistanceToShoppingComments](https://ddwiki.reso.org/display/DDW17/DistanceToShoppingComments+Field)
    #[serde(
        rename = "DistanceToShoppingComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_shopping_comments: Option<String>,

    /// Numeric distance from the property to the nearest shopping.
    ///
    /// [DistanceToShoppingNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToShoppingNumeric+Field)
    #[serde(
        rename = "DistanceToShoppingNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_shopping_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToShoppingUnits](https://ddwiki.reso.org/display/DDW17/DistanceToShoppingUnits+Field)
    #[serde(
        rename = "DistanceToShoppingUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_shopping_units: Option<crate::LinearUnits>,

    /// If the property does not have a maintained road or street adjacent to the lot, what are the conditions of access and distance to a maintained road.
    ///
    /// [DistanceToStreetComments](https://ddwiki.reso.org/display/DDW17/DistanceToStreetComments+Field)
    #[serde(
        rename = "DistanceToStreetComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_street_comments: Option<String>,

    /// Numeric distance from the property to the street.
    ///
    /// [DistanceToStreetNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToStreetNumeric+Field)
    #[serde(
        rename = "DistanceToStreetNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_street_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToStreetUnits](https://ddwiki.reso.org/display/DDW17/DistanceToStreetUnits+Field)
    #[serde(
        rename = "DistanceToStreetUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_street_units: Option<crate::LinearUnits>,

    /// If the property does not currently have water utility, is service available and if so, what is the distance.
    ///
    /// [DistanceToWaterComments](https://ddwiki.reso.org/display/DDW17/DistanceToWaterComments+Field)
    #[serde(
        rename = "DistanceToWaterComments",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_water_comments: Option<String>,

    /// Numeric distance from the property to the water utility.
    ///
    /// [DistanceToWaterNumeric](https://ddwiki.reso.org/display/DDW17/DistanceToWaterNumeric+Field)
    #[serde(
        rename = "DistanceToWaterNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_water_numeric: Option<f64>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [DistanceToWaterUnits](https://ddwiki.reso.org/display/DDW17/DistanceToWaterUnits+Field)
    #[serde(
        rename = "DistanceToWaterUnits",
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_to_water_units: Option<crate::LinearUnits>,

    /// A list of the Documents available for the property.  Knowing what documents are available for the property is valuable information.
    ///
    /// [DocumentsAvailable](https://ddwiki.reso.org/display/DDW17/DocumentsAvailable+Field)
    #[serde(rename = "DocumentsAvailable", skip_serializing_if = "Option::is_none")]
    pub documents_available: Option<String>,

    /// System generated timestamp of when the last update or change to the documents for this listing was made.
    ///
    /// [DocumentsChangeTimestamp](https://ddwiki.reso.org/display/DDW17/DocumentsChangeTimestamp+Field)
    #[serde(
        rename = "DocumentsChangeTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub documents_change_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The total number of documents or supplements included with the listings.
    ///
    /// [DocumentsCount](https://ddwiki.reso.org/display/DDW17/DocumentsCount+Field)
    #[serde(rename = "DocumentsCount", skip_serializing_if = "Option::is_none")]
    pub documents_count: Option<f64>,

    /// A list of features or description of the doors included in the sale/lease.
    ///
    /// [DoorFeatures](https://ddwiki.reso.org/display/DDW17/DoorFeatures+Field)
    #[serde(rename = "DoorFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub door_features: Option<Vec<crate::DoorFeatures>>,

    /// A commission arrangement in which the seller agrees to pay a specified commission to the listing broker if the property is sold through the efforts of a cooperating broker, but the seller pays the Listing broker a different commission amount if the sale occurs if:1) there is no cooperating broker involved or 2) due to the efforts of the seller directly.
    ///
    /// [DualVariableCompensationYN](https://ddwiki.reso.org/display/DDW17/DualVariableCompensationYN+Field)
    #[serde(
        rename = "DualVariableCompensationYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub dual_variable_compensation_yn: Option<bool>,

    /// A list of electric-service related features of the property (e.g. 110 Volt, 3 Phase, 220 Volt, RV Hookup). Note: the previous "Electric" field was renamed to DistanceToElectricComments
    ///
    /// [Electric](https://ddwiki.reso.org/display/DDW17/Electric+Field)
    #[serde(rename = "Electric", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub electric: Option<Vec<crate::Electric>>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [ElectricExpense](https://ddwiki.reso.org/display/DDW17/ElectricExpense+Field)
    #[serde(rename = "ElectricExpense", skip_serializing_if = "Option::is_none")]
    pub electric_expense: Option<f64>,

    /// Does the property currently have electrical utility available on the property.
    ///
    /// [ElectricOnPropertyYN](https://ddwiki.reso.org/display/DDW17/ElectricOnPropertyYN+Field)
    #[serde(
        rename = "ElectricOnPropertyYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub electric_on_property_yn: Option<bool>,

    /// The name of the primary school having a catchment area that includes the associated property.
    ///
    /// [ElementarySchool](https://ddwiki.reso.org/display/DDW17/ElementarySchool+Field)
    #[serde(rename = "ElementarySchool", skip_serializing_if = "Option::is_none")]
    pub elementary_school: Option<String>,

    /// The name of the elementary school district having a catchment area that includes the associated property.
    ///
    /// [ElementarySchoolDistrict](https://ddwiki.reso.org/display/DDW17/ElementarySchoolDistrict+Field)
    #[serde(
        rename = "ElementarySchoolDistrict",
        skip_serializing_if = "Option::is_none"
    )]
    pub elementary_school_district: Option<String>,

    /// The elevation of the property in relation to sea level.  Use the Elevation Units field to communicate the unit of measurement.  i.e. Feet or Meters.
    ///
    /// [Elevation](https://ddwiki.reso.org/display/DDW17/Elevation+Field)
    #[serde(rename = "Elevation", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    /// A pick list of the unit of measurement used in the Elevation field.  i.e. Feet, Meters.
    ///
    /// [ElevationUnits](https://ddwiki.reso.org/display/DDW17/ElevationUnits+Field)
    #[serde(rename = "ElevationUnits", skip_serializing_if = "Option::is_none")]
    pub elevation_units: Option<crate::LinearUnits>,

    /// A numeric field that describes the level within the structure, SFR or a unit in a building, where the main entry to the dwelling is located. When a unit has one floor it is implicit that this is also the level of the unit itself.
    ///
    /// [EntryLevel](https://ddwiki.reso.org/display/DDW17/EntryLevel+Field)
    #[serde(rename = "EntryLevel", skip_serializing_if = "Option::is_none")]
    pub entry_level: Option<f64>,

    /// A description of the main entry way to the property. i.e. Elevator, Ground Level w/ Steps, Ground Level w/o Steps, Mid Level, Top Level, etc.
    ///
    /// [EntryLocation](https://ddwiki.reso.org/display/DDW17/EntryLocation+Field)
    #[serde(rename = "EntryLocation", skip_serializing_if = "Option::is_none")]
    pub entry_location: Option<String>,

    /// Elements of the property that will not be included in the sale.  i.e. Chandeliers will be removed prior to close.
    ///
    /// [Exclusions](https://ddwiki.reso.org/display/DDW17/Exclusions+Field)
    #[serde(rename = "Exclusions", skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<String>,

    /// Information about the status of the existing lease on the property.  i.e. Net, NNN, NN, Gross, Absolute Net, Escalation Clause, Ground Lease, etc.
    ///
    /// [ExistingLeaseType](https://ddwiki.reso.org/display/DDW17/ExistingLeaseType+Field)
    #[serde(rename = "ExistingLeaseType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub existing_lease_type: Option<Vec<crate::ExistingLeaseType>>,

    /// The date when the listing agreement will expire.  This is the date entered by the agent reflecting when the change occurred, or will occur, contractually, not a timestamp of when the change was made in the MLS.  The expiration date of listings, prior to their expiration, cancellation, sale or lease, is confidential information and should be restricted to the agent and their managers, partners or broker.
    ///
    /// [ExpirationDate](https://ddwiki.reso.org/display/DDW17/ExpirationDate+Field)
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<chrono::NaiveDate>,

    /// A list of features or description of the exterior of the property included in the sale/lease.
    ///
    /// [ExteriorFeatures](https://ddwiki.reso.org/display/DDW17/ExteriorFeatures+Field)
    #[serde(rename = "ExteriorFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub exterior_features: Option<Vec<crate::ExteriorFeatures>>,

    /// Specifies whether or not Farm Credit Service shares are included in the price of the property.
    ///
    /// [FarmCreditServiceInclYN](https://ddwiki.reso.org/display/DDW17/FarmCreditServiceInclYN+Field)
    #[serde(
        rename = "FarmCreditServiceInclYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub farm_credit_service_incl_yn: Option<bool>,

    /// The source of the measurements. This may be a pick list of options showing the source of the measurement. i.e. Agent, Assessor, Estimate, etc. This field applies to all farm area fields (Cultivated, Pasture, Range, Wooded)
    ///
    /// [FarmLandAreaSource](https://ddwiki.reso.org/display/DDW17/FarmLandAreaSource+Field)
    #[serde(rename = "FarmLandAreaSource", skip_serializing_if = "Option::is_none")]
    pub farm_land_area_source: Option<crate::AreaSource>,

    /// A pick list of the unit of measurement for the area.  i.e. Square Feet, Square Meters, Acres, etc.  This field applies to all farm area fields (Cultivated, Pasture, Range, Wooded)
    ///
    /// [FarmLandAreaUnits](https://ddwiki.reso.org/display/DDW17/FarmLandAreaUnits+Field)
    #[serde(rename = "FarmLandAreaUnits", skip_serializing_if = "Option::is_none")]
    pub farm_land_area_units: Option<crate::AreaUnits>,

    /// A list of types of fencing found at the property being sold.
    ///
    /// [Fencing](https://ddwiki.reso.org/display/DDW17/Fencing+Field)
    #[serde(rename = "Fencing", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub fencing: Option<Vec<crate::Fencing>>,

    /// The source of the Rental information. For example Accountant, Owner, etc.
    ///
    /// [FinancialDataSource](https://ddwiki.reso.org/display/DDW17/FinancialDataSource+Field)
    #[serde(
        rename = "FinancialDataSource",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub financial_data_source: Option<Vec<crate::FinancialDataSource>>,

    /// A list of features or description of the fireplace(s) included in the sale/lease.
    ///
    /// [FireplaceFeatures](https://ddwiki.reso.org/display/DDW17/FireplaceFeatures+Field)
    #[serde(rename = "FireplaceFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub fireplace_features: Option<Vec<crate::FireplaceFeatures>>,

    /// Does the property include a fireplace.
    ///
    /// [FireplaceYN](https://ddwiki.reso.org/display/DDW17/FireplaceYN+Field)
    #[serde(rename = "FireplaceYN", skip_serializing_if = "Option::is_none")]
    pub fireplace_yn: Option<bool>,

    /// The total number of fireplaces included in the property.
    ///
    /// [FireplacesTotal](https://ddwiki.reso.org/display/DDW17/FireplacesTotal+Field)
    #[serde(rename = "FireplacesTotal", skip_serializing_if = "Option::is_none")]
    pub fireplaces_total: Option<f64>,

    /// A list of the type(s) of flooring found within the property.
    ///
    /// [Flooring](https://ddwiki.reso.org/display/DDW17/Flooring+Field)
    #[serde(rename = "Flooring", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub flooring: Option<Vec<crate::Flooring>>,

    /// The area or dimensions of the footprint of the structure on the lot.
    ///
    /// [FoundationArea](https://ddwiki.reso.org/display/DDW17/FoundationArea+Field)
    #[serde(rename = "FoundationArea", skip_serializing_if = "Option::is_none")]
    pub foundation_area: Option<f64>,

    /// A list of the type(s) of foundation on which the property sits.
    ///
    /// [FoundationDetails](https://ddwiki.reso.org/display/DDW17/FoundationDetails+Field)
    #[serde(rename = "FoundationDetails", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub foundation_details: Option<Vec<crate::FoundationDetails>>,

    /// Textual description of the length of the frontages selected in the Frontage Type field.
    ///
    /// [FrontageLength](https://ddwiki.reso.org/display/DDW17/FrontageLength+Field)
    #[serde(rename = "FrontageLength", skip_serializing_if = "Option::is_none")]
    pub frontage_length: Option<String>,

    /// Pick list of types of frontage. i.e. Oceanfront, Lakefront, Golf course, etc. Information about roads or road frontage should be located in the Road Frontage Type and Road Surface Type fields.
    ///
    /// [FrontageType](https://ddwiki.reso.org/display/DDW17/FrontageType+Field)
    #[serde(rename = "FrontageType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub frontage_type: Option<Vec<crate::FrontageType>>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [FuelExpense](https://ddwiki.reso.org/display/DDW17/FuelExpense+Field)
    #[serde(rename = "FuelExpense", skip_serializing_if = "Option::is_none")]
    pub fuel_expense: Option<f64>,

    /// The property being leased is furnished, unfurnished or partially furnished.
    ///
    /// [Furnished](https://ddwiki.reso.org/display/DDW17/Furnished+Field)
    #[serde(rename = "Furnished", skip_serializing_if = "Option::is_none")]
    pub furnished: Option<crate::Furnished>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [FurnitureReplacementExpense](https://ddwiki.reso.org/display/DDW17/FurnitureReplacementExpense+Field)
    #[serde(
        rename = "FurnitureReplacementExpense",
        skip_serializing_if = "Option::is_none"
    )]
    pub furniture_replacement_expense: Option<f64>,

    /// The number of spaces in the garage(s).
    ///
    /// [GarageSpaces](https://ddwiki.reso.org/display/DDW17/GarageSpaces+Field)
    #[serde(rename = "GarageSpaces", skip_serializing_if = "Option::is_none")]
    pub garage_spaces: Option<f64>,

    /// A flag indicating that the listing has a garage. This flag may be T/F, Y/N or other true, false or unknown indicator. As with all flags, the field may be null.
    ///
    /// [GarageYN](https://ddwiki.reso.org/display/DDW17/GarageYN+Field)
    #[serde(rename = "GarageYN", skip_serializing_if = "Option::is_none")]
    pub garage_yn: Option<bool>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [GardenerExpense](https://ddwiki.reso.org/display/DDW17/GardenerExpense+Field)
    #[serde(rename = "GardenerExpense", skip_serializing_if = "Option::is_none")]
    pub gardener_expense: Option<f64>,

    /// Specifies whether or not the property owner has grazing permits from the Bureau of Land Management.
    ///
    /// [GrazingPermitsBlmYN](https://ddwiki.reso.org/display/DDW17/GrazingPermitsBlmYN+Field)
    #[serde(
        rename = "GrazingPermitsBlmYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub grazing_permits_blm_yn: Option<bool>,

    /// Specifies whether or not the property owner has grazing permits from the Forestry Service.
    ///
    /// [GrazingPermitsForestServiceYN](https://ddwiki.reso.org/display/DDW17/GrazingPermitsForestServiceYN+Field)
    #[serde(
        rename = "GrazingPermitsForestServiceYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub grazing_permits_forest_service_yn: Option<bool>,

    /// Specifies whether or not the property owner has private grazing permits.
    ///
    /// [GrazingPermitsPrivateYN](https://ddwiki.reso.org/display/DDW17/GrazingPermitsPrivateYN+Field)
    #[serde(
        rename = "GrazingPermitsPrivateYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub grazing_permits_private_yn: Option<bool>,

    /// A collection of verifications or certifications awarded to a new or pre-existing residential or commercial structure. For example: LEED, Energy Star, ICC-700. The collection includes information about the type, year, rating and other details about the awarded performance verification.
    ///
    /// [GreenBuildingVerification](https://ddwiki.reso.org/display/DDW17/GreenBuildingVerification+Field)
    #[serde(
        rename = "GreenBuildingVerification",
        skip_serializing_if = "Option::is_none"
    )]
    pub green_building_verification: Option<String>,

    /// The name of the verification or certification awarded to a new or pre-existing residential or commercial structure. For example: LEED, Energy Star, ICC-700.  In cases where more than one certification have been awarded, leverage multiple iterations of the green verification fields via the repeating element method.
    ///
    /// [GreenBuildingVerificationType](https://ddwiki.reso.org/display/DDW17/GreenBuildingVerificationType+Field)
    #[serde(
        rename = "GreenBuildingVerificationType",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub green_building_verification_type: Option<Vec<crate::GreenBuildingVerificationType>>,

    /// Pick list of general green attributes such as energy efficient doors, or appliances without naming specific elements with ratings that may wane over time.
    ///
    /// [GreenEnergyEfficient](https://ddwiki.reso.org/display/DDW17/GreenEnergyEfficient+Field)
    #[serde(
        rename = "GreenEnergyEfficient",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub green_energy_efficient: Option<Vec<crate::GreenEnergyEfficient>>,

    /// Methods of generating power that are included in the sale or lease.
    ///
    /// [GreenEnergyGeneration](https://ddwiki.reso.org/display/DDW17/GreenEnergyGeneration+Field)
    #[serde(
        rename = "GreenEnergyGeneration",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub green_energy_generation: Option<Vec<crate::GreenEnergyGeneration>>,

    /// Pick list of indoor air quality measures without naming specific elements with ratings that may wane over time.
    ///
    /// [GreenIndoorAirQuality](https://ddwiki.reso.org/display/DDW17/GreenIndoorAirQuality+Field)
    #[serde(
        rename = "GreenIndoorAirQuality",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub green_indoor_air_quality: Option<Vec<crate::GreenIndoorAirQuality>>,

    /// Pick list describing efficiencies involved with the property's location such as walkability or transportation proximity without naming specific elements with ratings that may wane over time.
    ///
    /// [GreenLocation](https://ddwiki.reso.org/display/DDW17/GreenLocation+Field)
    #[serde(rename = "GreenLocation", skip_serializing_if = "Option::is_none")]
    pub green_location: Option<String>,

    /// Pick list of sustainable elements used in the construction of the structure without naming specific elements with ratings that may wane over time.
    ///
    /// [GreenSustainability](https://ddwiki.reso.org/display/DDW17/GreenSustainability+Field)
    #[serde(
        rename = "GreenSustainability",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub green_sustainability: Option<Vec<crate::GreenSustainability>>,

    /// Pick list of general water conserving attributes of the property such as landscaping or reclamation without naming specific elements with ratings that may wane over time.
    ///
    /// [GreenWaterConservation](https://ddwiki.reso.org/display/DDW17/GreenWaterConservation+Field)
    #[serde(
        rename = "GreenWaterConservation",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub green_water_conservation: Option<Vec<crate::GreenWaterConservation>>,

    /// The actual current income from rent and all other revenue generating sources.
    ///
    /// [GrossIncome](https://ddwiki.reso.org/display/DDW17/GrossIncome+Field)
    #[serde(rename = "GrossIncome", skip_serializing_if = "Option::is_none")]
    pub gross_income: Option<f64>,

    /// The maximum amount of annual rent collected if the property were 100% occupied all year and all tenants paid their rent.
    ///
    /// [GrossScheduledIncome](https://ddwiki.reso.org/display/DDW17/GrossScheduledIncome+Field)
    #[serde(
        rename = "GrossScheduledIncome",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_scheduled_income: Option<f64>,

    /// Does the property include a structure that can be lived in.
    ///
    /// [HabitableResidenceYN](https://ddwiki.reso.org/display/DDW17/HabitableResidenceYN+Field)
    #[serde(
        rename = "HabitableResidenceYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub habitable_residence_yn: Option<bool>,

    /// A list describing the heating features of the property.
    ///
    /// [Heating](https://ddwiki.reso.org/display/DDW17/Heating+Field)
    #[serde(rename = "Heating", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub heating: Option<Vec<crate::Heating>>,

    /// The property has heating.
    ///
    /// [HeatingYN](https://ddwiki.reso.org/display/DDW17/HeatingYN+Field)
    #[serde(rename = "HeatingYN", skip_serializing_if = "Option::is_none")]
    pub heating_yn: Option<bool>,

    /// The name of the high school having a catchment area that includes the associated property.
    ///
    /// [HighSchool](https://ddwiki.reso.org/display/DDW17/HighSchool+Field)
    #[serde(rename = "HighSchool", skip_serializing_if = "Option::is_none")]
    pub high_school: Option<String>,

    /// The name of the high school district having a catchment area that includes the associated property.  When only one school district is used, this field should be used over the Junior or Elementary Districts.
    ///
    /// [HighSchoolDistrict](https://ddwiki.reso.org/display/DDW17/HighSchoolDistrict+Field)
    #[serde(rename = "HighSchoolDistrict", skip_serializing_if = "Option::is_none")]
    pub high_school_district: Option<String>,

    /// Is a home warranty included in the sale of the property? Single select.
    ///
    /// [HomeWarrantyYN](https://ddwiki.reso.org/display/DDW17/HomeWarrantyYN+Field)
    #[serde(rename = "HomeWarrantyYN", skip_serializing_if = "Option::is_none")]
    pub home_warranty_yn: Option<bool>,

    /// A list of horse amenities on the lot or in the community.
    ///
    /// [HorseAmenities](https://ddwiki.reso.org/display/DDW17/HorseAmenities+Field)
    #[serde(rename = "HorseAmenities", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub horse_amenities: Option<Vec<crate::HorseAmenities>>,

    /// The Property is allowed to raise horses.
    ///
    /// [HorseYN](https://ddwiki.reso.org/display/DDW17/HorseYN+Field)
    #[serde(rename = "HorseYN", skip_serializing_if = "Option::is_none")]
    pub horse_yn: Option<bool>,

    /// A simplified enumerated list of the days and hours of operation of the business being sold. i.e. Open 24 Hours or Open 7 Days. For more detailed descriptions use the HoursDaysofOperationDescription field.
    ///
    /// [HoursDaysOfOperation](https://ddwiki.reso.org/display/DDW17/HoursDaysOfOperation+Field)
    #[serde(
        rename = "HoursDaysOfOperation",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub hours_days_of_operation: Option<Vec<crate::HoursDaysOfOperation>>,

    /// A detailed description of the hours and days the business being sold is open for business. For a specific list of simplified times the business is open, use the HoursDaysOfOperation Field (enumerated).
    ///
    /// [HoursDaysOfOperationDescription](https://ddwiki.reso.org/display/DDW17/HoursDaysOfOperationDescription+Field)
    #[serde(
        rename = "HoursDaysOfOperationDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub hours_days_of_operation_description: Option<String>,

    /// Portable elements of the property that will be included in the sale.
    ///
    /// [Inclusions](https://ddwiki.reso.org/display/DDW17/Inclusions+Field)
    #[serde(rename = "Inclusions", skip_serializing_if = "Option::is_none")]
    pub inclusions: Option<String>,

    /// A list of income sources included in the GrossScheduledIncome and GrossIncome.  i.e. Laundry, Parking, Recreation, Storage, etc.
    ///
    /// [IncomeIncludes](https://ddwiki.reso.org/display/DDW17/IncomeIncludes+Field)
    #[serde(rename = "IncomeIncludes", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub income_includes: Option<Vec<crate::IncomeIncludes>>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [InsuranceExpense](https://ddwiki.reso.org/display/DDW17/InsuranceExpense+Field)
    #[serde(rename = "InsuranceExpense", skip_serializing_if = "Option::is_none")]
    pub insurance_expense: Option<f64>,

    /// A list of features or description of the interior of the property included in the sale/lease.
    ///
    /// [InteriorFeatures](https://ddwiki.reso.org/display/DDW17/InteriorFeatures+Field)
    #[serde(rename = "InteriorFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub interior_features: Option<Vec<crate::InteriorOrRoomFeatures>>,

    /// A yes/no field that states the seller has allowed the listing address to be displayed on Internet sites.
    ///
    /// [InternetAddressDisplayYN](https://ddwiki.reso.org/display/DDW17/InternetAddressDisplayYN+Field)
    #[serde(
        rename = "InternetAddressDisplayYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_address_display_yn: Option<bool>,

    /// A yes/no field that states the seller allows the listing can be displayed with an AVM on Internet sites.
    ///
    /// [InternetAutomatedValuationDisplayYN](https://ddwiki.reso.org/display/DDW17/InternetAutomatedValuationDisplayYN+Field)
    #[serde(
        rename = "InternetAutomatedValuationDisplayYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_automated_valuation_display_yn: Option<bool>,

    /// A yes/no field that states the seller allows a comment or blog system to be attached to the listing on Internet sites.
    ///
    /// [InternetConsumerCommentYN](https://ddwiki.reso.org/display/DDW17/InternetConsumerCommentYN+Field)
    #[serde(
        rename = "InternetConsumerCommentYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_consumer_comment_yn: Option<bool>,

    /// A yes/no field that states the seller has allowed the listing to be displayed on Internet sites.
    ///
    /// [InternetEntireListingDisplayYN](https://ddwiki.reso.org/display/DDW17/InternetEntireListingDisplayYN+Field)
    #[serde(
        rename = "InternetEntireListingDisplayYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_entire_listing_display_yn: Option<bool>,

    /// The source which the property receives its water for irrigation.
    ///
    /// [IrrigationSource](https://ddwiki.reso.org/display/DDW17/IrrigationSource+Field)
    #[serde(rename = "IrrigationSource", skip_serializing_if = "Option::is_none")]
    pub irrigation_source: Option<String>,

    /// The number of acres allowed under the property's water rights.
    ///
    /// [IrrigationWaterRightsAcres](https://ddwiki.reso.org/display/DDW17/IrrigationWaterRightsAcres+Field)
    #[serde(
        rename = "IrrigationWaterRightsAcres",
        skip_serializing_if = "Option::is_none"
    )]
    pub irrigation_water_rights_acres: Option<f64>,

    /// Does the property include water rights for irrigation?  A Boolean or Yes / No field.
    ///
    /// [IrrigationWaterRightsYN](https://ddwiki.reso.org/display/DDW17/IrrigationWaterRightsYN+Field)
    #[serde(
        rename = "IrrigationWaterRightsYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub irrigation_water_rights_yn: Option<bool>,

    /// Information about labor laws that are applicable to the business being sold. i.e. Union, Non-Union, Employee License Required.
    ///
    /// [LaborInformation](https://ddwiki.reso.org/display/DDW17/LaborInformation+Field)
    #[serde(rename = "LaborInformation", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub labor_information: Option<Vec<crate::LaborInformation>>,

    /// When the land is not included in the sale, but is leased, the amount of the lease.  This is the Space Rent for Mobile homes in a Park.
    ///
    /// [LandLeaseAmount](https://ddwiki.reso.org/display/DDW17/LandLeaseAmount+Field)
    #[serde(rename = "LandLeaseAmount", skip_serializing_if = "Option::is_none")]
    pub land_lease_amount: Option<f64>,

    /// When the land is not included in the sale, but is leased, the frequency the Land Lease Fee is paid.
    ///
    /// [LandLeaseAmountFrequency](https://ddwiki.reso.org/display/DDW17/LandLeaseAmountFrequency+Field)
    #[serde(
        rename = "LandLeaseAmountFrequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub land_lease_amount_frequency: Option<crate::FeeFrequency>,

    /// When the land is not included in the sale, but is leased, the expiration date of the Land Lease.
    ///
    /// [LandLeaseExpirationDate](https://ddwiki.reso.org/display/DDW17/LandLeaseExpirationDate+Field)
    #[serde(
        rename = "LandLeaseExpirationDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub land_lease_expiration_date: Option<chrono::NaiveDate>,

    /// The land is not included in the sale and a lease exists.
    ///
    /// [LandLeaseYN](https://ddwiki.reso.org/display/DDW17/LandLeaseYN+Field)
    #[serde(rename = "LandLeaseYN", skip_serializing_if = "Option::is_none")]
    pub land_lease_yn: Option<bool>,

    /// The geographic latitude of some reference point on the property, specified in degrees and decimal parts.  Positive numbers must not include the plus symbol.
    ///
    /// [Latitude](https://ddwiki.reso.org/display/DDW17/Latitude+Field)
    #[serde(rename = "Latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    /// Add this pick list of features and locations where the laundry is located in the property being sold. i.e. Gas Dryer Hookup, In Kitchen, In Garage, etc.
    ///
    /// [LaundryFeatures](https://ddwiki.reso.org/display/DDW17/LaundryFeatures+Field)
    #[serde(rename = "LaundryFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub laundry_features: Option<Vec<crate::LaundryFeatures>>,

    /// The area that may be leased within the commercial property.
    ///
    /// [LeasableArea](https://ddwiki.reso.org/display/DDW17/LeasableArea+Field)
    #[serde(rename = "LeasableArea", skip_serializing_if = "Option::is_none")]
    pub leasable_area: Option<f64>,

    /// A pick list of the unit of measurement for the area.  i.e. Square Feet, Square Meters, Acres, etc.
    ///
    /// [LeasableAreaUnits](https://ddwiki.reso.org/display/DDW17/LeasableAreaUnits+Field)
    #[serde(rename = "LeasableAreaUnits", skip_serializing_if = "Option::is_none")]
    pub leasable_area_units: Option<crate::AreaUnits>,

    /// The amount of any lease the business pays for it's current location.
    ///
    /// [LeaseAmount](https://ddwiki.reso.org/display/DDW17/LeaseAmount+Field)
    #[serde(rename = "LeaseAmount", skip_serializing_if = "Option::is_none")]
    pub lease_amount: Option<f64>,

    /// The frequency of the LeaseAmount is paid.  Monthly, weekly, annual, etc.
    ///
    /// [LeaseAmountFrequency](https://ddwiki.reso.org/display/DDW17/LeaseAmountFrequency+Field)
    #[serde(
        rename = "LeaseAmountFrequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub lease_amount_frequency: Option<crate::FeeFrequency>,

    /// Can the lease at the business' current location be assigned to another party.
    ///
    /// [LeaseAssignableYN](https://ddwiki.reso.org/display/DDW17/LeaseAssignableYN+Field)
    #[serde(rename = "LeaseAssignableYN", skip_serializing_if = "Option::is_none")]
    pub lease_assignable_yn: Option<bool>,

    /// Will the seller consider leasing the property instead of selling?  Single select.
    ///
    /// [LeaseConsideredYN](https://ddwiki.reso.org/display/DDW17/LeaseConsideredYN+Field)
    #[serde(rename = "LeaseConsideredYN", skip_serializing_if = "Option::is_none")]
    pub lease_considered_yn: Option<bool>,

    /// The expiration date of the lease for the business' current location.
    ///
    /// [LeaseExpiration](https://ddwiki.reso.org/display/DDW17/LeaseExpiration+Field)
    #[serde(rename = "LeaseExpiration", skip_serializing_if = "Option::is_none")]
    pub lease_expiration: Option<chrono::NaiveDate>,

    /// A list of compensations other than the original Selling Office Compensation.  i.e. Compensation Paid on Renewal, Compensation Paid on Tennant Purchase, No Renewal Commission, Call Listing Office, etc.
    ///
    /// [LeaseRenewalCompensation](https://ddwiki.reso.org/display/DDW17/LeaseRenewalCompensation+Field)
    #[serde(
        rename = "LeaseRenewalCompensation",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub lease_renewal_compensation: Option<Vec<crate::LeaseRenewalCompensation>>,

    /// Is there an option to renew the lease at the business' current location.
    ///
    /// [LeaseRenewalOptionYN](https://ddwiki.reso.org/display/DDW17/LeaseRenewalOptionYN+Field)
    #[serde(
        rename = "LeaseRenewalOptionYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub lease_renewal_option_yn: Option<bool>,

    /// A pick list of lengths that represent the length of the lease.  i.e. Weekly, Month to Month, 6 Month Lease, 12 Month Lease, 24 Month Lease.
    ///
    /// [LeaseTerm](https://ddwiki.reso.org/display/DDW17/LeaseTerm+Field)
    #[serde(rename = "LeaseTerm", skip_serializing_if = "Option::is_none")]
    pub lease_term: Option<crate::LeaseTerm>,

    /// The number of levels in the property being sold. For example, One Level, Two Levels, Three or More Levels, <a href="http://ddwiki.reso.org/pages/viewpage.action?pageId=9941240">Multi/Split</a>, Loft. A discreet horizontal plane of interior living space (excluding basements).
    ///
    /// [Levels](https://ddwiki.reso.org/display/DDW17/Levels+Field)
    #[serde(rename = "Levels", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub levels: Option<Vec<crate::Levels>>,

    /// License number of the mobile or manufactured home.  Also known as the Department of Housing label/insignia number. For the first or only unit/section use License 1 over License 2 or 3.
    ///
    /// [License1](https://ddwiki.reso.org/display/DDW17/License1+Field)
    #[serde(rename = "License1", skip_serializing_if = "Option::is_none")]
    pub license1: Option<String>,

    /// License number of the mobile or manufactured home.  Also known as the Department of Housing label/insignia number. For two units/sections use License 1 and 2 over License 3.
    ///
    /// [License2](https://ddwiki.reso.org/display/DDW17/License2+Field)
    #[serde(rename = "License2", skip_serializing_if = "Option::is_none")]
    pub license2: Option<String>,

    /// License number of the mobile or manufactured home.  Also known as the Department of Housing label/insignia number. For two units/sections use License 1 and 2 over License 3.
    ///
    /// [License3](https://ddwiki.reso.org/display/DDW17/License3+Field)
    #[serde(rename = "License3", skip_serializing_if = "Option::is_none")]
    pub license3: Option<String>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [LicensesExpense](https://ddwiki.reso.org/display/DDW17/LicensesExpense+Field)
    #[serde(rename = "LicensesExpense", skip_serializing_if = "Option::is_none")]
    pub licenses_expense: Option<f64>,

    /// The responsible Board or Association of REALTORS for this listing.
    ///
    /// [ListAOR](https://ddwiki.reso.org/display/DDW17/ListAOR+Field)
    #[serde(rename = "ListAOR", skip_serializing_if = "Option::is_none")]
    pub list_aor: Option<String>,

    /// The Listing Agent's Board or Association of REALTORS.
    ///
    /// [ListAgentAOR](https://ddwiki.reso.org/display/DDW17/ListAgentAOR+Field)
    #[serde(rename = "ListAgentAOR", skip_serializing_if = "Option::is_none")]
    pub list_agent_aor: Option<String>,

    /// Designations and certifications acknowledging experience and expertise in various real estate sectors are awarded by NAR and each affiliated group upon completion of required courses.
    ///
    /// [ListAgentDesignation](https://ddwiki.reso.org/display/DDW17/ListAgentDesignation+Field)
    #[serde(
        rename = "ListAgentDesignation",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub list_agent_designation: Option<Vec<crate::ListAgentDesignation>>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentDirectPhone](https://ddwiki.reso.org/display/DDW17/ListAgentDirectPhone+Field)
    #[serde(
        rename = "ListAgentDirectPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_direct_phone: Option<String>,

    /// The email address of the Listing Agent.
    ///
    /// [ListAgentEmail](https://ddwiki.reso.org/display/DDW17/ListAgentEmail+Field)
    #[serde(rename = "ListAgentEmail", skip_serializing_if = "Option::is_none")]
    pub list_agent_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentFax](https://ddwiki.reso.org/display/DDW17/ListAgentFax+Field)
    #[serde(rename = "ListAgentFax", skip_serializing_if = "Option::is_none")]
    pub list_agent_fax: Option<String>,

    /// The first name of the listing agent.
    ///
    /// [ListAgentFirstName](https://ddwiki.reso.org/display/DDW17/ListAgentFirstName+Field)
    #[serde(rename = "ListAgentFirstName", skip_serializing_if = "Option::is_none")]
    pub list_agent_first_name: Option<String>,

    /// The full name of the listing agent. (First Middle Last)
    ///
    /// [ListAgentFullName](https://ddwiki.reso.org/display/DDW17/ListAgentFullName+Field)
    #[serde(rename = "ListAgentFullName", skip_serializing_if = "Option::is_none")]
    pub list_agent_full_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentHomePhone](https://ddwiki.reso.org/display/DDW17/ListAgentHomePhone+Field)
    #[serde(rename = "ListAgentHomePhone", skip_serializing_if = "Option::is_none")]
    pub list_agent_home_phone: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the ListAgentKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId. This is a foreign key relating to the Member resource's MemberKey.
    ///
    /// [ListAgentKey](https://ddwiki.reso.org/display/DDW17/ListAgentKey+Field)
    #[serde(rename = "ListAgentKey", skip_serializing_if = "Option::is_none")]
    pub list_agent_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the ListAgentKey is the system unique identifier from the system that the record was retrieved. This may be identical to the related xxxId. This is a foreign key relating to the Member resource's MemberKey. This is the numeric only key and used as an alternative to the ListAgentKey field.
    ///
    /// [ListAgentKeyNumeric](https://ddwiki.reso.org/display/DDW17/ListAgentKeyNumeric+Field)
    #[serde(
        rename = "ListAgentKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_key_numeric: Option<f64>,

    /// The last name of the listing agent.
    ///
    /// [ListAgentLastName](https://ddwiki.reso.org/display/DDW17/ListAgentLastName+Field)
    #[serde(rename = "ListAgentLastName", skip_serializing_if = "Option::is_none")]
    pub list_agent_last_name: Option<String>,

    /// The middle name of the listing agent.
    ///
    /// [ListAgentMiddleName](https://ddwiki.reso.org/display/DDW17/ListAgentMiddleName+Field)
    #[serde(
        rename = "ListAgentMiddleName",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_middle_name: Option<String>,

    /// The local, well-known identifier for the member. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [ListAgentMlsId](https://ddwiki.reso.org/display/DDW17/ListAgentMlsId+Field)
    #[serde(rename = "ListAgentMlsId", skip_serializing_if = "Option::is_none")]
    pub list_agent_mls_id: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentMobilePhone](https://ddwiki.reso.org/display/DDW17/ListAgentMobilePhone+Field)
    #[serde(
        rename = "ListAgentMobilePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_mobile_phone: Option<String>,

    /// Prefix to the name (e.g. Dr. Mr. Ms. etc.)
    ///
    /// [ListAgentNamePrefix](https://ddwiki.reso.org/display/DDW17/ListAgentNamePrefix+Field)
    #[serde(
        rename = "ListAgentNamePrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_name_prefix: Option<String>,

    /// Suffix to the ListAgentLastName (e.g. Esq., Jr., III etc.)
    ///
    /// [ListAgentNameSuffix](https://ddwiki.reso.org/display/DDW17/ListAgentNameSuffix+Field)
    #[serde(
        rename = "ListAgentNameSuffix",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_name_suffix: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentOfficePhone](https://ddwiki.reso.org/display/DDW17/ListAgentOfficePhone+Field)
    #[serde(
        rename = "ListAgentOfficePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [ListAgentOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/ListAgentOfficePhoneExt+Field)
    #[serde(
        rename = "ListAgentOfficePhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_office_phone_ext: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentPager](https://ddwiki.reso.org/display/DDW17/ListAgentPager+Field)
    #[serde(rename = "ListAgentPager", skip_serializing_if = "Option::is_none")]
    pub list_agent_pager: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentPreferredPhone](https://ddwiki.reso.org/display/DDW17/ListAgentPreferredPhone+Field)
    #[serde(
        rename = "ListAgentPreferredPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_preferred_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [ListAgentPreferredPhoneExt](https://ddwiki.reso.org/display/DDW17/ListAgentPreferredPhoneExt+Field)
    #[serde(
        rename = "ListAgentPreferredPhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_preferred_phone_ext: Option<String>,

    /// The license of the listing agent. Separate multiple licenses with a comma and space.
    ///
    /// [ListAgentStateLicense](https://ddwiki.reso.org/display/DDW17/ListAgentStateLicense+Field)
    #[serde(
        rename = "ListAgentStateLicense",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_state_license: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentTollFreePhone](https://ddwiki.reso.org/display/DDW17/ListAgentTollFreePhone+Field)
    #[serde(
        rename = "ListAgentTollFreePhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_toll_free_phone: Option<String>,

    /// The website URI of the listing agent.
    ///
    /// [ListAgentURL](https://ddwiki.reso.org/display/DDW17/ListAgentURL+Field)
    #[serde(rename = "ListAgentURL", skip_serializing_if = "Option::is_none")]
    pub list_agent_url: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens). Other conventions should use the common local standard. International numbers should be preceded by a plus symbol.
    ///
    /// [ListAgentVoiceMail](https://ddwiki.reso.org/display/DDW17/ListAgentVoiceMail+Field)
    #[serde(rename = "ListAgentVoiceMail", skip_serializing_if = "Option::is_none")]
    pub list_agent_voice_mail: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [ListAgentVoiceMailExt](https://ddwiki.reso.org/display/DDW17/ListAgentVoiceMailExt+Field)
    #[serde(
        rename = "ListAgentVoiceMailExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_agent_voice_mail_ext: Option<String>,

    /// The Listing Office's Board or Association of REALTORS.
    ///
    /// [ListOfficeAOR](https://ddwiki.reso.org/display/DDW17/ListOfficeAOR+Field)
    #[serde(rename = "ListOfficeAOR", skip_serializing_if = "Option::is_none")]
    pub list_office_aor: Option<String>,

    /// The email address of the Listing Office.
    ///
    /// [ListOfficeEmail](https://ddwiki.reso.org/display/DDW17/ListOfficeEmail+Field)
    #[serde(rename = "ListOfficeEmail", skip_serializing_if = "Option::is_none")]
    pub list_office_email: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [ListOfficeFax](https://ddwiki.reso.org/display/DDW17/ListOfficeFax+Field)
    #[serde(rename = "ListOfficeFax", skip_serializing_if = "Option::is_none")]
    pub list_office_fax: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Office resource's OfficeKey.
    ///
    /// [ListOfficeKey](https://ddwiki.reso.org/display/DDW17/ListOfficeKey+Field)
    #[serde(rename = "ListOfficeKey", skip_serializing_if = "Option::is_none")]
    pub list_office_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Office resource's OfficeKey.  This is the numeric only key and used as an alternative to the ListOfficeKey field.
    ///
    /// [ListOfficeKeyNumeric](https://ddwiki.reso.org/display/DDW17/ListOfficeKeyNumeric+Field)
    #[serde(
        rename = "ListOfficeKeyNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub list_office_key_numeric: Option<f64>,

    /// The local, well-known identifier. This value may not be unique, specifically in the case of aggregation systems, this value should be the identifier from the original system.
    ///
    /// [ListOfficeMlsId](https://ddwiki.reso.org/display/DDW17/ListOfficeMlsId+Field)
    #[serde(rename = "ListOfficeMlsId", skip_serializing_if = "Option::is_none")]
    pub list_office_mls_id: Option<String>,

    /// The legal name of the brokerage representing the seller.
    ///
    /// [ListOfficeName](https://ddwiki.reso.org/display/DDW17/ListOfficeName+Field)
    #[serde(rename = "ListOfficeName", skip_serializing_if = "Option::is_none")]
    pub list_office_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [ListOfficePhone](https://ddwiki.reso.org/display/DDW17/ListOfficePhone+Field)
    #[serde(rename = "ListOfficePhone", skip_serializing_if = "Option::is_none")]
    pub list_office_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [ListOfficePhoneExt](https://ddwiki.reso.org/display/DDW17/ListOfficePhoneExt+Field)
    #[serde(rename = "ListOfficePhoneExt", skip_serializing_if = "Option::is_none")]
    pub list_office_phone_ext: Option<String>,

    /// The website URI for the listing office.
    ///
    /// [ListOfficeURL](https://ddwiki.reso.org/display/DDW17/ListOfficeURL+Field)
    #[serde(rename = "ListOfficeURL", skip_serializing_if = "Option::is_none")]
    pub list_office_url: Option<String>,

    /// The current price of the property as determined by the seller and the seller's broker.  For auctions this is the minimum or reserve price.
    ///
    /// [ListPrice](https://ddwiki.reso.org/display/DDW17/ListPrice+Field)
    #[serde(rename = "ListPrice", skip_serializing_if = "Option::is_none")]
    pub list_price: Option<f64>,

    /// The lower price used for Value Range Pricing.  The List Price must be greater than or equal to the ListPriceLow.
    ///
    /// [ListPriceLow](https://ddwiki.reso.org/display/DDW17/ListPriceLow+Field)
    #[serde(rename = "ListPriceLow", skip_serializing_if = "Option::is_none")]
    pub list_price_low: Option<f64>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Teams resource's TeamKey.
    ///
    /// [ListTeamKey](https://ddwiki.reso.org/display/DDW17/ListTeamKey+Field)
    #[serde(rename = "ListTeamKey", skip_serializing_if = "Option::is_none")]
    pub list_team_key: Option<String>,

    /// A system unique identifier. Specifically, in aggregation systems, the Key is the system unique identifier from the system that the record was just retrieved. This may be identical to the related xxxId identifier, but the key is guaranteed unique for this record set.  This is a foreign key relating to the Teams resource's TeamKey.  This is the numeric only key and used as an alternative to the ListTeamKey field.
    ///
    /// [ListTeamKeyNumeric](https://ddwiki.reso.org/display/DDW17/ListTeamKeyNumeric+Field)
    #[serde(rename = "ListTeamKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub list_team_key_numeric: Option<f64>,

    /// The name of the team representing the seller.
    ///
    /// [ListTeamName](https://ddwiki.reso.org/display/DDW17/ListTeamName+Field)
    #[serde(rename = "ListTeamName", skip_serializing_if = "Option::is_none")]
    pub list_team_name: Option<String>,

    /// The nature of the agreement between the seller and the listing agent. Examples are Exclusive Agency, Open Listing, etc.
    ///
    /// [ListingAgreement](https://ddwiki.reso.org/display/DDW17/ListingAgreement+Field)
    #[serde(rename = "ListingAgreement", skip_serializing_if = "Option::is_none")]
    pub listing_agreement: Option<crate::ListingAgreement>,

    /// The effective date of the agreement between the seller and the seller's broker. This is the date entered by the agent reflecting when the change occurred contractually, not a timestamp of when the change was made in the MLS.
    ///
    /// [ListingContractDate](https://ddwiki.reso.org/display/DDW17/ListingContractDate+Field)
    #[serde(
        rename = "ListingContractDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_contract_date: Option<chrono::NaiveDate>,

    /// The well known identifier for the listing. The value may be identical to that of the Listing Key, but the Listing ID is intended to be the value used by a human to retrieve the information about a specific listing. In a multiple originating system or a merged system, this value may not be unique and may require the use of the provider system to create a synthetic unique value.
    ///
    /// [ListingId](https://ddwiki.reso.org/display/DDW17/ListingId+Field)
    #[serde(rename = "ListingId", skip_serializing_if = "Option::is_none")]
    pub listing_id: Option<String>,

    /// A unique identifier for this record from the immediate source. This is a string that can include URI or other forms.  Alternatively use the ListingKeyNumeric for a numeric only key field.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey.
    ///
    /// [ListingKey](https://ddwiki.reso.org/display/DDW17/ListingKey+Field)
    #[serde(rename = "ListingKey", skip_serializing_if = "Option::is_none")]
    pub listing_key: Option<String>,

    /// A unique identifier for this record from the immediate source. This is the numeric only key and used as an alternative to the ListingKey fields.  This is the local key of the system.  When records are received from other systems, a local key is commonly applied.  If conveying the original keys from the source or originating systems, see SourceSystemKey and OriginatingSystemKey.
    ///
    /// [ListingKeyNumeric](https://ddwiki.reso.org/display/DDW17/ListingKeyNumeric+Field)
    #[serde(rename = "ListingKeyNumeric", skip_serializing_if = "Option::is_none")]
    pub listing_key_numeric: Option<f64>,

    /// Defines the type or level of service the listing member will be providing to the selling home owner.  This will typically be a single selection.  Examples include Full Service, Limited Service or Entry Only.
    ///
    /// [ListingService](https://ddwiki.reso.org/display/DDW17/ListingService+Field)
    #[serde(rename = "ListingService", skip_serializing_if = "Option::is_none")]
    pub listing_service: Option<crate::ListingService>,

    /// Terms of the listing such as Lien Release, Subject to Court Approval or Owner Will Carry. Also may include options that describe the financing terms that are acceptable to the seller, i.e. cash, assumable, FHA loan, etc.
    ///
    /// [ListingTerms](https://ddwiki.reso.org/display/DDW17/ListingTerms+Field)
    #[serde(rename = "ListingTerms", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub listing_terms: Option<Vec<crate::ListingTerms>>,

    /// The total livable area within the structure.
    ///
    /// [LivingArea](https://ddwiki.reso.org/display/DDW17/LivingArea+Field)
    #[serde(rename = "LivingArea", skip_serializing_if = "Option::is_none")]
    pub living_area: Option<f64>,

    /// The source of the measurements. This is a pick list of options showing the source of the measurement. i.e. Agent, Assessor, Estimate, etc.
    ///
    /// [LivingAreaSource](https://ddwiki.reso.org/display/DDW17/LivingAreaSource+Field)
    #[serde(rename = "LivingAreaSource", skip_serializing_if = "Option::is_none")]
    pub living_area_source: Option<crate::AreaSource>,

    /// A pick list of the unit of measurement for the area.  i.e. Square Feet, Square Meters, Acres, etc.
    ///
    /// [LivingAreaUnits](https://ddwiki.reso.org/display/DDW17/LivingAreaUnits+Field)
    #[serde(rename = "LivingAreaUnits", skip_serializing_if = "Option::is_none")]
    pub living_area_units: Option<crate::AreaUnits>,

    /// A field describing the location of the lock box.
    ///
    /// [LockBoxLocation](https://ddwiki.reso.org/display/DDW17/LockBoxLocation+Field)
    #[serde(rename = "LockBoxLocation", skip_serializing_if = "Option::is_none")]
    pub lock_box_location: Option<String>,

    /// The serial number of the lockbox placed on the property.
    ///
    /// [LockBoxSerialNumber](https://ddwiki.reso.org/display/DDW17/LockBoxSerialNumber+Field)
    #[serde(
        rename = "LockBoxSerialNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub lock_box_serial_number: Option<String>,

    /// A field describing the type of lock box.
    ///
    /// [LockBoxType](https://ddwiki.reso.org/display/DDW17/LockBoxType+Field)
    #[serde(rename = "LockBoxType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub lock_box_type: Option<Vec<crate::LockBoxType>>,

    /// The geographic longitude of some reference point on the property, specified in degrees and decimal parts. Positive numbers must not include the plus symbol.
    ///
    /// [Longitude](https://ddwiki.reso.org/display/DDW17/Longitude+Field)
    #[serde(rename = "Longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,

    /// The source of the measurements. This may be a pick list of options showing the source of the measurement. i.e. Agent, Assessor, Estimate, etc.
    ///
    /// [LotDimensionsSource](https://ddwiki.reso.org/display/DDW17/LotDimensionsSource+Field)
    #[serde(
        rename = "LotDimensionsSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub lot_dimensions_source: Option<crate::LotDimensionsSource>,

    /// A list of features or description of the lot included in the sale/lease.
    ///
    /// [LotFeatures](https://ddwiki.reso.org/display/DDW17/LotFeatures+Field)
    #[serde(rename = "LotFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub lot_features: Option<Vec<crate::LotFeatures>>,

    /// The total Acres of the lot.  This field is related to the Lot Size Area and Lot Size Units and must be in sync with the values represented in those fields.  Lot Size Source also applies to this field when used.
    ///
    /// [LotSizeAcres](https://ddwiki.reso.org/display/DDW17/LotSizeAcres+Field)
    #[serde(rename = "LotSizeAcres", skip_serializing_if = "Option::is_none")]
    pub lot_size_acres: Option<f64>,

    /// The total area of the lot.  See Lot Size Units for the units of measurement (Square Feet, Square Meters, Acres, etc.).
    ///
    /// [LotSizeArea](https://ddwiki.reso.org/display/DDW17/LotSizeArea+Field)
    #[serde(rename = "LotSizeArea", skip_serializing_if = "Option::is_none")]
    pub lot_size_area: Option<f64>,

    /// The dimensions of the lot minimally represented as length and width (i.e. 250 x 180) or a measurement of all sides of the polygon representing the property lines of the property.  i.e. 30 x 50 x 120 x 60 x 22.
    ///
    /// [LotSizeDimensions](https://ddwiki.reso.org/display/DDW17/LotSizeDimensions+Field)
    #[serde(rename = "LotSizeDimensions", skip_serializing_if = "Option::is_none")]
    pub lot_size_dimensions: Option<String>,

    /// The source of the measurements. This may be a pick list of options showing the source of the measurement. i.e. Agent, Assessor, Estimate, etc.
    ///
    /// [LotSizeSource](https://ddwiki.reso.org/display/DDW17/LotSizeSource+Field)
    #[serde(rename = "LotSizeSource", skip_serializing_if = "Option::is_none")]
    pub lot_size_source: Option<crate::LotSizeSource>,

    /// The total square footage of the lot.  This field is related to the Lot Size Area and Lot Size Units and must be in sync with the values represented in those fields.  Lot Size Source also applies to this field when used.
    ///
    /// [LotSizeSquareFeet](https://ddwiki.reso.org/display/DDW17/LotSizeSquareFeet+Field)
    #[serde(rename = "LotSizeSquareFeet", skip_serializing_if = "Option::is_none")]
    pub lot_size_square_feet: Option<f64>,

    /// A pick list of the unit of measurement for the area.  i.e. Square Feet, Square Meters, Acres, etc.
    ///
    /// [LotSizeUnits](https://ddwiki.reso.org/display/DDW17/LotSizeUnits+Field)
    #[serde(rename = "LotSizeUnits", skip_serializing_if = "Option::is_none")]
    pub lot_size_units: Option<crate::LotSizeUnits>,

    /// The major marketing area name, as defined by the MLS or other non-governmental organization.  If there is only one MLS Area in use, it must be the MLSAreaMajor.
    ///
    /// [MLSAreaMajor](https://ddwiki.reso.org/display/DDW17/MLSAreaMajor+Field)
    #[serde(rename = "MLSAreaMajor", skip_serializing_if = "Option::is_none")]
    pub mlsarea_major: Option<String>,

    /// The minor/sub marketing area name, as defined by the MLS or other non-governmental organization.  If there is only one MLS Area in use, it must be the MLSAreaMajor.
    ///
    /// [MLSAreaMinor](https://ddwiki.reso.org/display/DDW17/MLSAreaMinor+Field)
    #[serde(rename = "MLSAreaMinor", skip_serializing_if = "Option::is_none")]
    pub mlsarea_minor: Option<String>,

    /// The number of bathrooms located on the main or entry level of the property.
    ///
    /// [MainLevelBathrooms](https://ddwiki.reso.org/display/DDW17/MainLevelBathrooms+Field)
    #[serde(rename = "MainLevelBathrooms", skip_serializing_if = "Option::is_none")]
    pub main_level_bathrooms: Option<f64>,

    /// The number of bedrooms located on the main or entry level of the property.
    ///
    /// [MainLevelBedrooms](https://ddwiki.reso.org/display/DDW17/MainLevelBedrooms+Field)
    #[serde(rename = "MainLevelBedrooms", skip_serializing_if = "Option::is_none")]
    pub main_level_bedrooms: Option<f64>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [MaintenanceExpense](https://ddwiki.reso.org/display/DDW17/MaintenanceExpense+Field)
    #[serde(rename = "MaintenanceExpense", skip_serializing_if = "Option::is_none")]
    pub maintenance_expense: Option<f64>,

    /// Timestamp of the last major change on the listing (see also MajorChangeType).
    ///
    /// [MajorChangeTimestamp](https://ddwiki.reso.org/display/DDW17/MajorChangeTimestamp+Field)
    #[serde(
        rename = "MajorChangeTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub major_change_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Description of the last major change on the listing, i.e. “price reduction”, “back on market”, etc.  May be used to display on a summary view of listing results to quickly identify listings that have had major changes recently.
    ///
    /// [MajorChangeType](https://ddwiki.reso.org/display/DDW17/MajorChangeType+Field)
    #[serde(rename = "MajorChangeType", skip_serializing_if = "Option::is_none")]
    pub major_change_type: Option<crate::ChangeType>,

    /// Make of the mobile or manufactured home.
    ///
    /// [Make](https://ddwiki.reso.org/display/DDW17/Make+Field)
    #[serde(rename = "Make", skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.  This is for an individual manager.  Use ProfessionalManagementExpense for a management company.
    ///
    /// [ManagerExpense](https://ddwiki.reso.org/display/DDW17/ManagerExpense+Field)
    #[serde(rename = "ManagerExpense", skip_serializing_if = "Option::is_none")]
    pub manager_expense: Option<f64>,

    /// A map coordinate for the property, as determined by local custom. This is not necessarily the same as the geographic coordinate but may depend on the coordinate system used by whatever mapping service is customarily used by the listing service.
    ///
    /// [MapCoordinate](https://ddwiki.reso.org/display/DDW17/MapCoordinate+Field)
    #[serde(rename = "MapCoordinate", skip_serializing_if = "Option::is_none")]
    pub map_coordinate: Option<String>,

    /// Name of the map or map book publisher.
    ///
    /// [MapCoordinateSource](https://ddwiki.reso.org/display/DDW17/MapCoordinateSource+Field)
    #[serde(
        rename = "MapCoordinateSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub map_coordinate_source: Option<String>,

    /// URI to a map of the property.
    ///
    /// [MapURL](https://ddwiki.reso.org/display/DDW17/MapURL+Field)
    #[serde(rename = "MapURL", skip_serializing_if = "Option::is_none")]
    pub map_url: Option<String>,

    /// The name of the junior or middle school having a catchment area that includes the associated property.
    ///
    /// [MiddleOrJuniorSchool](https://ddwiki.reso.org/display/DDW17/MiddleOrJuniorSchool+Field)
    #[serde(
        rename = "MiddleOrJuniorSchool",
        skip_serializing_if = "Option::is_none"
    )]
    pub middle_or_junior_school: Option<String>,

    /// The name of the junior or middle school district having a catchment area that includes the associated property.
    ///
    /// [MiddleOrJuniorSchoolDistrict](https://ddwiki.reso.org/display/DDW17/MiddleOrJuniorSchoolDistrict+Field)
    #[serde(
        rename = "MiddleOrJuniorSchoolDistrict",
        skip_serializing_if = "Option::is_none"
    )]
    pub middle_or_junior_school_district: Option<String>,

    /// Local or regional status that are well known by business users. Each MlsStatus must map to a single StandardStatus. Multiple MlsStatus may map to a single StandardStatus.
    ///
    /// [MlsStatus](https://ddwiki.reso.org/display/DDW17/MlsStatus+Field)
    #[serde(rename = "MlsStatus", skip_serializing_if = "Option::is_none")]
    pub mls_status: Option<String>,

    /// A pick list of the unit linear measurement.  i.e. feed, meters, yards, kilometers, miles, etc.
    ///
    /// [MobileDimUnits](https://ddwiki.reso.org/display/DDW17/MobileDimUnits+Field)
    #[serde(rename = "MobileDimUnits", skip_serializing_if = "Option::is_none")]
    pub mobile_dim_units: Option<crate::LinearUnits>,

    /// Is the mobile home to remain and be included in the sale of the property.
    ///
    /// [MobileHomeRemainsYN](https://ddwiki.reso.org/display/DDW17/MobileHomeRemainsYN+Field)
    #[serde(
        rename = "MobileHomeRemainsYN",
        skip_serializing_if = "Option::is_none"
    )]
    pub mobile_home_remains_yn: Option<bool>,

    /// Length of the mobile/manufactured home.
    ///
    /// [MobileLength](https://ddwiki.reso.org/display/DDW17/MobileLength+Field)
    #[serde(rename = "MobileLength", skip_serializing_if = "Option::is_none")]
    pub mobile_length: Option<f64>,

    /// Width of the mobile/manufactured home.
    ///
    /// [MobileWidth](https://ddwiki.reso.org/display/DDW17/MobileWidth+Field)
    #[serde(rename = "MobileWidth", skip_serializing_if = "Option::is_none")]
    pub mobile_width: Option<f64>,

    /// Model of the mobile or manufactured home.
    ///
    /// [Model](https://ddwiki.reso.org/display/DDW17/Model+Field)
    #[serde(rename = "Model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the listing was last modified.
    ///
    /// [ModificationTimestamp](https://ddwiki.reso.org/display/DDW17/ModificationTimestamp+Field)
    #[serde(
        rename = "ModificationTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Net operating income is the revenue from a property after operating expenses have been deducted, but before deducting income taxes and financing expenses (interest and Principal Payments).   For example, Gross Income - Operating Expenses = Net Operating Income (NOI).
    ///
    /// [NetOperatingIncome](https://ddwiki.reso.org/display/DDW17/NetOperatingIncome+Field)
    #[serde(rename = "NetOperatingIncome", skip_serializing_if = "Option::is_none")]
    pub net_operating_income: Option<f64>,

    /// Is the property newly constructed and has not been previously occupied?
    ///
    /// [NewConstructionYN](https://ddwiki.reso.org/display/DDW17/NewConstructionYN+Field)
    #[serde(rename = "NewConstructionYN", skip_serializing_if = "Option::is_none")]
    pub new_construction_yn: Option<bool>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [NewTaxesExpense](https://ddwiki.reso.org/display/DDW17/NewTaxesExpense+Field)
    #[serde(rename = "NewTaxesExpense", skip_serializing_if = "Option::is_none")]
    pub new_taxes_expense: Option<f64>,

    /// Total number of separate buildings included in the income property.
    ///
    /// [NumberOfBuildings](https://ddwiki.reso.org/display/DDW17/NumberOfBuildings+Field)
    #[serde(rename = "NumberOfBuildings", skip_serializing_if = "Option::is_none")]
    pub number_of_buildings: Option<f64>,

    /// The current number of individuals employed by the business on a full-time basis.
    ///
    /// [NumberOfFullTimeEmployees](https://ddwiki.reso.org/display/DDW17/NumberOfFullTimeEmployees+Field)
    #[serde(
        rename = "NumberOfFullTimeEmployees",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_full_time_employees: Option<f64>,

    /// Total number of lots on the property or included in the sale. Land properties are often sold with multiple lots. It is important to be able to describe how many lots are in the property and not in all cases do lots have separate Parcel IDs.
    ///
    /// [NumberOfLots](https://ddwiki.reso.org/display/DDW17/NumberOfLots+Field)
    #[serde(rename = "NumberOfLots", skip_serializing_if = "Option::is_none")]
    pub number_of_lots: Option<f64>,

    /// The number of pads or spaces in the mobile home park.
    ///
    /// [NumberOfPads](https://ddwiki.reso.org/display/DDW17/NumberOfPads+Field)
    #[serde(rename = "NumberOfPads", skip_serializing_if = "Option::is_none")]
    pub number_of_pads: Option<f64>,

    /// The current number of individuals employed by the business on a part-time basis.
    ///
    /// [NumberOfPartTimeEmployees](https://ddwiki.reso.org/display/DDW17/NumberOfPartTimeEmployees+Field)
    #[serde(
        rename = "NumberOfPartTimeEmployees",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_part_time_employees: Option<f64>,

    /// Total number of separate meters on the property.
    ///
    /// [NumberOfSeparateElectricMeters](https://ddwiki.reso.org/display/DDW17/NumberOfSeparateElectricMeters+Field)
    #[serde(
        rename = "NumberOfSeparateElectricMeters",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_separate_electric_meters: Option<f64>,

    /// Total number of separate meters on the property.
    ///
    /// [NumberOfSeparateGasMeters](https://ddwiki.reso.org/display/DDW17/NumberOfSeparateGasMeters+Field)
    #[serde(
        rename = "NumberOfSeparateGasMeters",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_separate_gas_meters: Option<f64>,

    /// Total number of separate meters on the property.
    ///
    /// [NumberOfSeparateWaterMeters](https://ddwiki.reso.org/display/DDW17/NumberOfSeparateWaterMeters+Field)
    #[serde(
        rename = "NumberOfSeparateWaterMeters",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_separate_water_meters: Option<f64>,

    /// The total number of units in the building, complex or community.  This is not the number of units being sold, but rather the size of the community in which the dwelling being sold is located.
    ///
    /// [NumberOfUnitsInCommunity](https://ddwiki.reso.org/display/DDW17/NumberOfUnitsInCommunity+Field)
    #[serde(
        rename = "NumberOfUnitsInCommunity",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_units_in_community: Option<f64>,

    /// Total number of units currently under a lease agreement.
    ///
    /// [NumberOfUnitsLeased](https://ddwiki.reso.org/display/DDW17/NumberOfUnitsLeased+Field)
    #[serde(
        rename = "NumberOfUnitsLeased",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_units_leased: Option<f64>,

    /// The total number of units leasable month to month.
    ///
    /// [NumberOfUnitsMoMo](https://ddwiki.reso.org/display/DDW17/NumberOfUnitsMoMo+Field)
    #[serde(rename = "NumberOfUnitsMoMo", skip_serializing_if = "Option::is_none")]
    pub number_of_units_mo_mo: Option<f64>,

    /// Total number of units included in the income property, occupied or unoccupied.
    ///
    /// [NumberOfUnitsTotal](https://ddwiki.reso.org/display/DDW17/NumberOfUnitsTotal+Field)
    #[serde(rename = "NumberOfUnitsTotal", skip_serializing_if = "Option::is_none")]
    pub number_of_units_total: Option<f64>,

    /// The number of units currently vacant.
    ///
    /// [NumberOfUnitsVacant](https://ddwiki.reso.org/display/DDW17/NumberOfUnitsVacant+Field)
    #[serde(
        rename = "NumberOfUnitsVacant",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_units_vacant: Option<f64>,

    /// Name of the current occupant, if any, of the property being sold.
    ///
    /// [OccupantName](https://ddwiki.reso.org/display/DDW17/OccupantName+Field)
    #[serde(rename = "OccupantName", skip_serializing_if = "Option::is_none")]
    pub occupant_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [OccupantPhone](https://ddwiki.reso.org/display/DDW17/OccupantPhone+Field)
    #[serde(rename = "OccupantPhone", skip_serializing_if = "Option::is_none")]
    pub occupant_phone: Option<String>,

    /// A field that describes the type of occupant, i.e. Owner, Tenant, Vacant.
    ///
    /// [OccupantType](https://ddwiki.reso.org/display/DDW17/OccupantType+Field)
    #[serde(rename = "OccupantType", skip_serializing_if = "Option::is_none")]
    pub occupant_type: Option<crate::OccupantType>,

    /// The date the listing was taken off market. Where possible, this date is reflective of the date entered by the agent reflecting when the change occurred contractually, not a timestamp of when the change was made in the MLS.
    ///
    /// [OffMarketDate](https://ddwiki.reso.org/display/DDW17/OffMarketDate+Field)
    #[serde(rename = "OffMarketDate", skip_serializing_if = "Option::is_none")]
    pub off_market_date: Option<chrono::NaiveDate>,

    /// The transactional timestamp automatically recorded by the MLS system representing the most recent date/time the listing's status was set to and off market status (not Active or Backup)
    ///
    /// [OffMarketTimestamp](https://ddwiki.reso.org/display/DDW17/OffMarketTimestamp+Field)
    #[serde(rename = "OffMarketTimestamp", skip_serializing_if = "Option::is_none")]
    pub off_market_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The date the listing was placed on market. Where possible, this date is reflective of the date entered by the agent reflecting when the change occurred contractually, not a timestamp of when the change was made in the MLS.
    ///
    /// [OnMarketDate](https://ddwiki.reso.org/display/DDW17/OnMarketDate+Field)
    #[serde(rename = "OnMarketDate", skip_serializing_if = "Option::is_none")]
    pub on_market_date: Option<chrono::NaiveDate>,

    /// The transactional timestamp automatically recorded by the MLS system representing the most recent date/time the listing's status was set to Active or Backup.  This also includes initial input of the listing to Active/Backup or from a draft or approval status to Active/Backup.
    ///
    /// [OnMarketTimestamp](https://ddwiki.reso.org/display/DDW17/OnMarketTimestamp+Field)
    #[serde(rename = "OnMarketTimestamp", skip_serializing_if = "Option::is_none")]
    pub on_market_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The number of open or uncovered parking spaces included in the sale.
    ///
    /// [OpenParkingSpaces](https://ddwiki.reso.org/display/DDW17/OpenParkingSpaces+Field)
    #[serde(rename = "OpenParkingSpaces", skip_serializing_if = "Option::is_none")]
    pub open_parking_spaces: Option<f64>,

    /// A flag indicating that any parking spaces associated with the property are not covered by a roof.
    ///
    /// [OpenParkingYN](https://ddwiki.reso.org/display/DDW17/OpenParkingYN+Field)
    #[serde(rename = "OpenParkingYN", skip_serializing_if = "Option::is_none")]
    pub open_parking_yn: Option<bool>,

    /// The costs associated with the operation and maintenance of an income-producing property.
    ///
    /// [OperatingExpense](https://ddwiki.reso.org/display/DDW17/OperatingExpense+Field)
    #[serde(rename = "OperatingExpense", skip_serializing_if = "Option::is_none")]
    pub operating_expense: Option<f64>,

    /// When individual expense fields are not used and only a total is entered, this lists the expenses that are included in the OperatingExpense field.
    ///
    /// [OperatingExpenseIncludes](https://ddwiki.reso.org/display/DDW17/OperatingExpenseIncludes+Field)
    #[serde(
        rename = "OperatingExpenseIncludes",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub operating_expense_includes: Option<Vec<crate::OperatingExpenseIncludes>>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the listing was entered and made visible to members of the MLS.
    ///
    /// [OriginalEntryTimestamp](https://ddwiki.reso.org/display/DDW17/OriginalEntryTimestamp+Field)
    #[serde(
        rename = "OriginalEntryTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_entry_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The original price of the property on the initial agreement between the seller and the seller's broker.
    ///
    /// [OriginalListPrice](https://ddwiki.reso.org/display/DDW17/OriginalListPrice+Field)
    #[serde(rename = "OriginalListPrice", skip_serializing_if = "Option::is_none")]
    pub original_list_price: Option<f64>,

    /// The RESO OUID's OrganizationUniqueId of the Originating record provider.  The Originating system is the system with authoritative control over the record.  For example; the name of the MLS where the listing was input.  In cases where the Originating system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [OriginatingSystemID](https://ddwiki.reso.org/display/DDW17/OriginatingSystemID+Field)
    #[serde(
        rename = "OriginatingSystemID",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Originating system.  The Originating system is the system with authoritative control over the record.  For example, the Multiple Listing Service where the listing was input.  There may be cases where the Source System (how you received the record) is not the Originating System.  See Source System Key for more information.
    ///
    /// [OriginatingSystemKey](https://ddwiki.reso.org/display/DDW17/OriginatingSystemKey+Field)
    #[serde(
        rename = "OriginatingSystemKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_key: Option<String>,

    /// The name of the Originating record provider.  Most commonly the name of the MLS. The place where the listing is originally input by the member.  The legal name of the company.
    ///
    /// [OriginatingSystemName](https://ddwiki.reso.org/display/DDW17/OriginatingSystemName+Field)
    #[serde(
        rename = "OriginatingSystemName",
        skip_serializing_if = "Option::is_none"
    )]
    pub originating_system_name: Option<String>,

    /// A list of other equipment that will be included in the sale of the property.
    ///
    /// [OtherEquipment](https://ddwiki.reso.org/display/DDW17/OtherEquipment+Field)
    #[serde(rename = "OtherEquipment", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub other_equipment: Option<Vec<crate::OtherEquipment>>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [OtherExpense](https://ddwiki.reso.org/display/DDW17/OtherExpense+Field)
    #[serde(rename = "OtherExpense", skip_serializing_if = "Option::is_none")]
    pub other_expense: Option<f64>,

    /// Other types of parking available to, or part of, the property.
    ///
    /// [OtherParking](https://ddwiki.reso.org/display/DDW17/OtherParking+Field)
    #[serde(rename = "OtherParking", skip_serializing_if = "Option::is_none")]
    pub other_parking: Option<String>,

    /// A list of structures other than the main dwelling. For example, Guest House, Barn, Shed, etc.
    ///
    /// [OtherStructures](https://ddwiki.reso.org/display/DDW17/OtherStructures+Field)
    #[serde(rename = "OtherStructures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub other_structures: Option<Vec<crate::OtherStructures>>,

    /// Name of the owner of the property being sold.
    ///
    /// [OwnerName](https://ddwiki.reso.org/display/DDW17/OwnerName+Field)
    #[serde(rename = "OwnerName", skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,

    /// A list of expenses for the property paid for by the owner as opposed to the tenant (e.g. Water, Trash, Electric).
    ///
    /// [OwnerPays](https://ddwiki.reso.org/display/DDW17/OwnerPays+Field)
    #[serde(rename = "OwnerPays", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub owner_pays: Option<Vec<crate::OwnerPays>>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [OwnerPhone](https://ddwiki.reso.org/display/DDW17/OwnerPhone+Field)
    #[serde(rename = "OwnerPhone", skip_serializing_if = "Option::is_none")]
    pub owner_phone: Option<String>,

    /// A text description of the manner in which title to a property is held.  Trust, Corporation, Joint Tennant, Individual.
    ///
    /// [Ownership](https://ddwiki.reso.org/display/DDW17/Ownership+Field)
    #[serde(rename = "Ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<String>,

    /// Current type of ownership of the business being sold.  i.e. Corporation, LLC, Sole P, Partnership, etc.,
    ///
    /// [OwnershipType](https://ddwiki.reso.org/display/DDW17/OwnershipType+Field)
    #[serde(rename = "OwnershipType", skip_serializing_if = "Option::is_none")]
    pub ownership_type: Option<crate::OwnershipType>,

    /// A number used to uniquely identify a parcel or lot.  This number is typically issued by the county or county assessor.  The AP number format varies from county to county.  It is recommended that all Parcel Numbers be transmitted without dashes or hyphens.
    ///
    /// [ParcelNumber](https://ddwiki.reso.org/display/DDW17/ParcelNumber+Field)
    #[serde(rename = "ParcelNumber", skip_serializing_if = "Option::is_none")]
    pub parcel_number: Option<String>,

    /// Name of the manager of the mobile home park.
    ///
    /// [ParkManagerName](https://ddwiki.reso.org/display/DDW17/ParkManagerName+Field)
    #[serde(rename = "ParkManagerName", skip_serializing_if = "Option::is_none")]
    pub park_manager_name: Option<String>,

    /// North American 10 digit phone numbers should be in the format of ###-###-#### (separated by hyphens).  Other conventions should use the common local standard.  International numbers should be preceded by a plus symbol.
    ///
    /// [ParkManagerPhone](https://ddwiki.reso.org/display/DDW17/ParkManagerPhone+Field)
    #[serde(rename = "ParkManagerPhone", skip_serializing_if = "Option::is_none")]
    pub park_manager_phone: Option<String>,

    /// Name of the mobile home park or corporate/commercial park.
    ///
    /// [ParkName](https://ddwiki.reso.org/display/DDW17/ParkName+Field)
    #[serde(rename = "ParkName", skip_serializing_if = "Option::is_none")]
    pub park_name: Option<String>,

    /// A list of features or description of the parking included in the sale/lease.
    ///
    /// [ParkingFeatures](https://ddwiki.reso.org/display/DDW17/ParkingFeatures+Field)
    #[serde(rename = "ParkingFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub parking_features: Option<Vec<crate::ParkingFeatures>>,

    /// The total number of parking spaces included in the sale.
    ///
    /// [ParkingTotal](https://ddwiki.reso.org/display/DDW17/ParkingTotal+Field)
    #[serde(rename = "ParkingTotal", skip_serializing_if = "Option::is_none")]
    pub parking_total: Option<f64>,

    /// Measurement or percentage of the property that has been allocated as pasture or grazing area.
    ///
    /// [PastureArea](https://ddwiki.reso.org/display/DDW17/PastureArea+Field)
    #[serde(rename = "PastureArea", skip_serializing_if = "Option::is_none")]
    pub pasture_area: Option<f64>,

    /// A list of features or description of the patio or porch included in the sale/lease.
    ///
    /// [PatioAndPorchFeatures](https://ddwiki.reso.org/display/DDW17/PatioAndPorchFeatures+Field)
    #[serde(
        rename = "PatioAndPorchFeatures",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub patio_and_porch_features: Option<Vec<crate::PatioAndPorchFeatures>>,

    /// The transactional timestamp automatically recorded by the MLS system representing the most recent date/time the listing's status was set to Pending.
    ///
    /// [PendingTimestamp](https://ddwiki.reso.org/display/DDW17/PendingTimestamp+Field)
    #[serde(rename = "PendingTimestamp", skip_serializing_if = "Option::is_none")]
    pub pending_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [PestControlExpense](https://ddwiki.reso.org/display/DDW17/PestControlExpense+Field)
    #[serde(rename = "PestControlExpense", skip_serializing_if = "Option::is_none")]
    pub pest_control_expense: Option<f64>,

    /// Are pets allowed at the property being leased?  A list of yes, no and more detailed restrictions/allowances.
    ///
    /// [PetsAllowed](https://ddwiki.reso.org/display/DDW17/PetsAllowed+Field)
    #[serde(rename = "PetsAllowed", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub pets_allowed: Option<Vec<crate::PetsAllowed>>,

    /// System generated timestamp of when the last update or change to the photos for this listing was made.
    ///
    /// [PhotosChangeTimestamp](https://ddwiki.reso.org/display/DDW17/PhotosChangeTimestamp+Field)
    #[serde(
        rename = "PhotosChangeTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub photos_change_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The total number of pictures or photos included with the listing.
    ///
    /// [PhotosCount](https://ddwiki.reso.org/display/DDW17/PhotosCount+Field)
    #[serde(rename = "PhotosCount", skip_serializing_if = "Option::is_none")]
    pub photos_count: Option<f64>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [PoolExpense](https://ddwiki.reso.org/display/DDW17/PoolExpense+Field)
    #[serde(rename = "PoolExpense", skip_serializing_if = "Option::is_none")]
    pub pool_expense: Option<f64>,

    /// A list of features or description of the pool included in the sale/lease.
    ///
    /// [PoolFeatures](https://ddwiki.reso.org/display/DDW17/PoolFeatures+Field)
    #[serde(rename = "PoolFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub pool_features: Option<Vec<crate::PoolFeatures>>,

    /// The property has a privately owned pool that is included in the sale/lease.
    ///
    /// [PoolPrivateYN](https://ddwiki.reso.org/display/DDW17/PoolPrivateYN+Field)
    #[serde(rename = "PoolPrivateYN", skip_serializing_if = "Option::is_none")]
    pub pool_private_yn: Option<bool>,

    /// A list defining when possession will occur.  i.e. COE, COE+1, etc.
    ///
    /// [Possession](https://ddwiki.reso.org/display/DDW17/Possession+Field)
    #[serde(rename = "Possession", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub possession: Option<Vec<crate::Possession>>,

    /// A list of the type(s) of possible or best uses of the property. Probable use gives a good indication of what the best use or potential use of the property could be. i.e. Primary, Vacation, Investment, Rental, Retirement
    ///
    /// [PossibleUse](https://ddwiki.reso.org/display/DDW17/PossibleUse+Field)
    #[serde(rename = "PossibleUse", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub possible_use: Option<Vec<crate::PossibleUse>>,

    /// The official city per the USPS.  May be different from the "City".
    ///
    /// [PostalCity](https://ddwiki.reso.org/display/DDW17/PostalCity+Field)
    #[serde(rename = "PostalCity", skip_serializing_if = "Option::is_none")]
    pub postal_city: Option<String>,

    /// The postal code portion of a street or mailing address.
    ///
    /// [PostalCode](https://ddwiki.reso.org/display/DDW17/PostalCode+Field)
    #[serde(rename = "PostalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// The postal code +4 portion of a street or mailing address.
    ///
    /// [PostalCodePlus4](https://ddwiki.reso.org/display/DDW17/PostalCodePlus4+Field)
    #[serde(rename = "PostalCodePlus4", skip_serializing_if = "Option::is_none")]
    pub postal_code_plus4: Option<String>,

    /// A collection of the types of power production system(s) available on the property. The collection includes the type of system and other details about the power produced and age of the system.
    ///
    /// [PowerProduction](https://ddwiki.reso.org/display/DDW17/PowerProduction+Field)
    #[serde(rename = "PowerProduction", skip_serializing_if = "Option::is_none")]
    pub power_production: Option<String>,

    /// This field is a list of the types of power production system(s) available on the property. The key characteristics of the system are expected to appear as the "[type]" in the related power production fields in a flattened implementation (RETS 1.x only) of the power production fields.  A relational implementation of power production must omit the type from the field name and use PowerProductionType to create a vertical representation of the various types of power production available.  **Note that PV Solar is the only type of power production currently justified in multiple markets and thus shown. Up and coming renewables that could be added in the future depending on uptake: Wind, Geothermal, Thin Film Solar.
    ///
    /// [PowerProductionType](https://ddwiki.reso.org/display/DDW17/PowerProductionType+Field)
    #[serde(
        rename = "PowerProductionType",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub power_production_type: Option<Vec<crate::PowerProductionType>>,

    /// The most recent previous ListPrice of the listing.
    ///
    /// [PreviousListPrice](https://ddwiki.reso.org/display/DDW17/PreviousListPrice+Field)
    #[serde(rename = "PreviousListPrice", skip_serializing_if = "Option::is_none")]
    pub previous_list_price: Option<f64>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the listing's price was last changed.
    ///
    /// [PriceChangeTimestamp](https://ddwiki.reso.org/display/DDW17/PriceChangeTimestamp+Field)
    #[serde(
        rename = "PriceChangeTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_change_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// A remarks field that is only visible to members of the same offices as the listing agent.
    ///
    /// [PrivateOfficeRemarks](https://ddwiki.reso.org/display/DDW17/PrivateOfficeRemarks+Field)
    #[serde(
        rename = "PrivateOfficeRemarks",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_office_remarks: Option<String>,

    /// Remarks that may contain security or proprietary information and should be restricted from public view.
    ///
    /// [PrivateRemarks](https://ddwiki.reso.org/display/DDW17/PrivateRemarks+Field)
    #[serde(rename = "PrivateRemarks", skip_serializing_if = "Option::is_none")]
    pub private_remarks: Option<String>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.  This is for a management company.  Use ManagerExpense for a individual manager.
    ///
    /// [ProfessionalManagementExpense](https://ddwiki.reso.org/display/DDW17/ProfessionalManagementExpense+Field)
    #[serde(
        rename = "ProfessionalManagementExpense",
        skip_serializing_if = "Option::is_none"
    )]
    pub professional_management_expense: Option<f64>,

    /// A flag indicating that the primary structure is attached to another structure that is not included in the sale. i.e. one unit of a duplex. This flag may be T/F, Y/N or a list of attached or detached. As with all flags, the field may be null. In some systems this information may be part of the Property Sub Type.
    ///
    /// [PropertyAttachedYN](https://ddwiki.reso.org/display/DDW17/PropertyAttachedYN+Field)
    #[serde(rename = "PropertyAttachedYN", skip_serializing_if = "Option::is_none")]
    pub property_attached_yn: Option<bool>,

    /// A list describing the condition of the property and any structures included in the sale.
    ///
    /// [PropertyCondition](https://ddwiki.reso.org/display/DDW17/PropertyCondition+Field)
    #[serde(rename = "PropertyCondition", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub property_condition: Option<Vec<crate::PropertyCondition>>,

    /// A list of types of residential and residential lease properties, i.e. SFR, Condo, etc. Or a list of Sub Types for Mobile, such as Expando, Manufactured, Modular, etc.
    ///
    /// [PropertySubType](https://ddwiki.reso.org/display/DDW17/PropertySubType+Field)
    #[serde(rename = "PropertySubType", skip_serializing_if = "Option::is_none")]
    pub property_sub_type: Option<crate::PropertySubType>,

    /// A list of types of properties such as Residential, Lease, Income, Land, Mobile, Commercial Sale, etc...
    ///
    /// [PropertyType](https://ddwiki.reso.org/display/DDW17/PropertyType+Field)
    #[serde(rename = "PropertyType", skip_serializing_if = "Option::is_none")]
    pub property_type: Option<crate::PropertyType>,

    /// Text remarks that may be displayed to the public. In an MLS, it is the field where information is entered for the public. This information is intended to be visible on-line. This is typically information that describes the selling points of the building and/or land for sale. Local conditions and rules will determine what such content can contain. Generally, the following information is excluded: any information pertaining to entry to the property, the seller and/or tenant, listing member contact information. In other systems, these remarks will be determined by local business rules.
    ///
    /// [PublicRemarks](https://ddwiki.reso.org/display/DDW17/PublicRemarks+Field)
    #[serde(rename = "PublicRemarks", skip_serializing_if = "Option::is_none")]
    pub public_remarks: Option<String>,

    /// This field specifically identifies the Range identified by the Public Land Survey System (PLSS).
    ///
    /// [PublicSurveyRange](https://ddwiki.reso.org/display/DDW17/PublicSurveyRange+Field)
    #[serde(rename = "PublicSurveyRange", skip_serializing_if = "Option::is_none")]
    pub public_survey_range: Option<String>,

    /// This field specifically identifies the Section identified by the Public Land Survey System (PLSS).
    ///
    /// [PublicSurveySection](https://ddwiki.reso.org/display/DDW17/PublicSurveySection+Field)
    #[serde(
        rename = "PublicSurveySection",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_survey_section: Option<String>,

    /// This field specifically identifies the Township identified by the Public Land Survey System (PLSS).
    ///
    /// [PublicSurveyTownship](https://ddwiki.reso.org/display/DDW17/PublicSurveyTownship+Field)
    #[serde(
        rename = "PublicSurveyTownship",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_survey_township: Option<String>,

    /// With for-sale listings, the date an offer was accepted and the listing was no longer on market.  This is the date entered by the agent reflecting when the change occurred contractually, not a timestamp of when the change was made in the MLS.  With lease listings this may represent a meeting of the minds to lease, but some contractual requirements are yet to be fulfilled, such as contract signing or receipt of the deposit.
    ///
    /// [PurchaseContractDate](https://ddwiki.reso.org/display/DDW17/PurchaseContractDate+Field)
    #[serde(
        rename = "PurchaseContractDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub purchase_contract_date: Option<chrono::NaiveDate>,

    /// The dimensions of the RV parking area minimally represented as length and width (i.e. 25 x 18) or a measurement of all sides of the polygon representing the usable RV parking space. i.e. 33 x 15 x 12 x 60.
    ///
    /// [RVParkingDimensions](https://ddwiki.reso.org/display/DDW17/RVParkingDimensions+Field)
    #[serde(
        rename = "RVParkingDimensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub rvparking_dimensions: Option<String>,

    /// Measurement or percentage of the property that has been allocated as range.
    ///
    /// [RangeArea](https://ddwiki.reso.org/display/DDW17/RangeArea+Field)
    #[serde(rename = "RangeArea", skip_serializing_if = "Option::is_none")]
    pub range_area: Option<f64>,

    /// Is the property in a rent control area?
    ///
    /// [RentControlYN](https://ddwiki.reso.org/display/DDW17/RentControlYN+Field)
    #[serde(rename = "RentControlYN", skip_serializing_if = "Option::is_none")]
    pub rent_control_yn: Option<bool>,

    /// A list of services or items that the tenant is not responsible to pay.
    ///
    /// [RentIncludes](https://ddwiki.reso.org/display/DDW17/RentIncludes+Field)
    #[serde(rename = "RentIncludes", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub rent_includes: Option<Vec<crate::RentIncludes>>,

    /// Pick list of types of Road frontage. i.e. Freeway frontage, No Road Frontage, etc. The road frontage of the property is an important factor in determining value of the property and it’s appropriateness for intended use.
    ///
    /// [RoadFrontageType](https://ddwiki.reso.org/display/DDW17/RoadFrontageType+Field)
    #[serde(rename = "RoadFrontageType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub road_frontage_type: Option<Vec<crate::RoadFrontageType>>,

    /// The person or entity responsible for road maintenance (e.g., City, County, Private).
    ///
    /// [RoadResponsibility](https://ddwiki.reso.org/display/DDW17/RoadResponsibility+Field)
    #[serde(rename = "RoadResponsibility", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub road_responsibility: Option<Vec<crate::RoadResponsibility>>,

    /// Pick list of types of surface of the Road to access the property. The surface of the road(s) for access to the property is an important factor in determining value of the property and it’s appropriateness for intended use.
    ///
    /// [RoadSurfaceType](https://ddwiki.reso.org/display/DDW17/RoadSurfaceType+Field)
    #[serde(rename = "RoadSurfaceType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub road_surface_type: Option<Vec<crate::RoadSurfaceType>>,

    /// A list describing the type or style of roof.  For example Spanish Tile, Composite, Shake, etc.
    ///
    /// [Roof](https://ddwiki.reso.org/display/DDW17/Roof+Field)
    #[serde(rename = "Roof", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub roof: Option<Vec<crate::Roof>>,

    /// This field is a list of the types used in the rooms repeating elements. The Type is a list of possible room types. i.e. Bedroom, Bathroom, Living Room, Workshop, etc. Each selected are expected to appear as the "[type]" in the related rooms fields in a flattened implementation (RETS 1.x only) of the room fields. A relational implementation of rooms must omit the type from the field name and use RoomType to create a vertical representation of the various rooms. **Note that Garage or Basement should not be added as a room type and are represented by the ParkingFeatures and Basement fields respectively.
    ///
    /// [RoomType](https://ddwiki.reso.org/display/DDW17/RoomType+Field)
    #[serde(rename = "RoomType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub room_type: Option<Vec<crate::RoomType>>,

    /// A collection of types of rooms and details/features about the given room.
    ///
    /// [Rooms](https://ddwiki.reso.org/display/DDW17/Rooms+Field)
    #[serde(rename = "Rooms", skip_serializing_if = "Option::is_none")]
    pub rooms: Option<String>,

    /// The number of rooms in the dwelling.
    ///
    /// [RoomsTotal](https://ddwiki.reso.org/display/DDW17/RoomsTotal+Field)
    #[serde(rename = "RoomsTotal", skip_serializing_if = "Option::is_none")]
    pub rooms_total: Option<f64>,

    /// The seating capacity of the business being sold.
    ///
    /// [SeatingCapacity](https://ddwiki.reso.org/display/DDW17/SeatingCapacity+Field)
    #[serde(rename = "SeatingCapacity", skip_serializing_if = "Option::is_none")]
    pub seating_capacity: Option<f64>,

    /// A list describing the security features included in the sale/lease.
    ///
    /// [SecurityFeatures](https://ddwiki.reso.org/display/DDW17/SecurityFeatures+Field)
    #[serde(rename = "SecurityFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub security_features: Option<Vec<crate::SecurityFeatures>>,

    /// The community is a senior community.
    ///
    /// [SeniorCommunityYN](https://ddwiki.reso.org/display/DDW17/SeniorCommunityYN+Field)
    #[serde(rename = "SeniorCommunityYN", skip_serializing_if = "Option::is_none")]
    pub senior_community_yn: Option<bool>,

    /// Serial number of the mobile or manufactured home.  For the first or only unit/section use Serial U over Serial X or Serial XX.
    ///
    /// [SerialU](https://ddwiki.reso.org/display/DDW17/SerialU+Field)
    #[serde(rename = "SerialU", skip_serializing_if = "Option::is_none")]
    pub serial_u: Option<String>,

    /// Serial number of the mobile or manufactured home.  For two units/sections, Serial U should be used first, Serial X second over or Serial XX.
    ///
    /// [SerialX](https://ddwiki.reso.org/display/DDW17/SerialX+Field)
    #[serde(rename = "SerialX", skip_serializing_if = "Option::is_none")]
    pub serial_x: Option<String>,

    /// Serial number of the mobile or manufactured home.  For two units/sections, Serial U should be used first, Serial X second over or Serial XX.
    ///
    /// [SerialXX](https://ddwiki.reso.org/display/DDW17/SerialXX+Field)
    #[serde(rename = "SerialXX", skip_serializing_if = "Option::is_none")]
    pub serial_xx: Option<String>,

    /// A list describing the sewer or septic features of the property.
    ///
    /// [Sewer](https://ddwiki.reso.org/display/DDW17/Sewer+Field)
    #[serde(rename = "Sewer", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub sewer: Option<Vec<crate::Sewer>>,

    /// The hours of advance notice required to schedule a showing.
    ///
    /// [ShowingAdvanceNotice](https://ddwiki.reso.org/display/DDW17/ShowingAdvanceNotice+Field)
    #[serde(
        rename = "ShowingAdvanceNotice",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_advance_notice: Option<f64>,

    /// Does this home require an attended showing?  i.e. Yes = licensed agent representing the seller must be present during showing.
    ///
    /// [ShowingAttendedYN](https://ddwiki.reso.org/display/DDW17/ShowingAttendedYN+Field)
    #[serde(rename = "ShowingAttendedYN", skip_serializing_if = "Option::is_none")]
    pub showing_attended_yn: Option<bool>,

    /// The name of the contact for the showing of the listed property.
    ///
    /// [ShowingContactName](https://ddwiki.reso.org/display/DDW17/ShowingContactName+Field)
    #[serde(rename = "ShowingContactName", skip_serializing_if = "Option::is_none")]
    pub showing_contact_name: Option<String>,

    /// A telephone number that should be called to arrange showing the property.
    ///
    /// [ShowingContactPhone](https://ddwiki.reso.org/display/DDW17/ShowingContactPhone+Field)
    #[serde(
        rename = "ShowingContactPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_contact_phone: Option<String>,

    /// The extension of the given phone number (if applicable).
    ///
    /// [ShowingContactPhoneExt](https://ddwiki.reso.org/display/DDW17/ShowingContactPhoneExt+Field)
    #[serde(
        rename = "ShowingContactPhoneExt",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_contact_phone_ext: Option<String>,

    /// The type of contact for the showing.  i.e. Agent, Broker, Seller.
    ///
    /// [ShowingContactType](https://ddwiki.reso.org/display/DDW17/ShowingContactType+Field)
    #[serde(rename = "ShowingContactType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub showing_contact_type: Option<Vec<crate::ShowingContactType>>,

    /// The days of the week that the property is available for showing.  i.e. Sundays, Mondays, Tuesdays, Wednesdays, Thursdays, Fridays, Saturdays
    ///
    /// [ShowingDays](https://ddwiki.reso.org/display/DDW17/ShowingDays+Field)
    #[serde(rename = "ShowingDays", skip_serializing_if = "Option::is_none")]
    pub showing_days: Option<String>,

    /// From the days selected in the ShowingDays field, the end time that the property is available for showing.
    ///
    /// [ShowingEndTime](https://ddwiki.reso.org/display/DDW17/ShowingEndTime+Field)
    #[serde(rename = "ShowingEndTime", skip_serializing_if = "Option::is_none")]
    pub showing_end_time: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Remarks that detail the seller's instructions for showing the subject property. Showing instructions may include: contact information, showing times, notice required or other information. These remarks are privileged and are not for public viewing.
    ///
    /// [ShowingInstructions](https://ddwiki.reso.org/display/DDW17/ShowingInstructions+Field)
    #[serde(
        rename = "ShowingInstructions",
        skip_serializing_if = "Option::is_none"
    )]
    pub showing_instructions: Option<String>,

    /// A pick list of types of notice required to see the home.  i.e. Appointment Required, Courtesy Call Only, Go Direct, etc.
    ///
    /// [ShowingRequirements](https://ddwiki.reso.org/display/DDW17/ShowingRequirements+Field)
    #[serde(
        rename = "ShowingRequirements",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub showing_requirements: Option<Vec<crate::ShowingRequirements>>,

    /// From the days selected in the ShowingDays field, the start time that the property is available for showing.
    ///
    /// [ShowingStartTime](https://ddwiki.reso.org/display/DDW17/ShowingStartTime+Field)
    #[serde(rename = "ShowingStartTime", skip_serializing_if = "Option::is_none")]
    pub showing_start_time: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// Is there a sign on the property.
    ///
    /// [SignOnPropertyYN](https://ddwiki.reso.org/display/DDW17/SignOnPropertyYN+Field)
    #[serde(rename = "SignOnPropertyYN", skip_serializing_if = "Option::is_none")]
    pub sign_on_property_yn: Option<bool>,

    /// A list of types of mobile home skirting.
    ///
    /// [Skirt](https://ddwiki.reso.org/display/DDW17/Skirt+Field)
    #[serde(rename = "Skirt", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub skirt: Option<Vec<crate::Skirt>>,

    /// The RESO OUID's OrganizationUniqueId of the Source record provider. The source system is the system from which the record was directly received. In cases where the source system was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemID](https://ddwiki.reso.org/display/DDW17/SourceSystemID+Field)
    #[serde(rename = "SourceSystemID", skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,

    /// The system key, a unique record identifier, from the Source System. The Source System is the system from which the record was directly received. In cases where the Source System was not where the record originated (the authoritative system), see the Originating System fields.
    ///
    /// [SourceSystemKey](https://ddwiki.reso.org/display/DDW17/SourceSystemKey+Field)
    #[serde(rename = "SourceSystemKey", skip_serializing_if = "Option::is_none")]
    pub source_system_key: Option<String>,

    /// The name of the immediate record provider. The system from which the record was directly received. The legal name of the company.
    ///
    /// [SourceSystemName](https://ddwiki.reso.org/display/DDW17/SourceSystemName+Field)
    #[serde(rename = "SourceSystemName", skip_serializing_if = "Option::is_none")]
    pub source_system_name: Option<String>,

    /// A list of features or description of the spa included in the sale/lease.
    ///
    /// [SpaFeatures](https://ddwiki.reso.org/display/DDW17/SpaFeatures+Field)
    #[serde(rename = "SpaFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub spa_features: Option<Vec<crate::SpaFeatures>>,

    /// The property has a spa.
    ///
    /// [SpaYN](https://ddwiki.reso.org/display/DDW17/SpaYN+Field)
    #[serde(rename = "SpaYN", skip_serializing_if = "Option::is_none")]
    pub spa_yn: Option<bool>,

    /// Special licenses required/used by the business being sold.  i.e. Beer/Wine, Class H, Professional, Gambling, None.
    ///
    /// [SpecialLicenses](https://ddwiki.reso.org/display/DDW17/SpecialLicenses+Field)
    #[serde(rename = "SpecialLicenses", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub special_licenses: Option<Vec<crate::SpecialLicenses>>,

    /// A list of options that describe the type of sale.  i.e. Standard, REO, Short Sale, Probate, Auction, NOD, etc., at the time of listing.
    ///
    /// [SpecialListingConditions](https://ddwiki.reso.org/display/DDW17/SpecialListingConditions+Field)
    #[serde(
        rename = "SpecialListingConditions",
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(default, with = "crate::comma_delimited")]
    pub special_listing_conditions: Option<Vec<crate::SpecialListingConditions>>,

    /// The status of the listing as it reflects the state of the contract between the listing agent and seller or an agreement with a buyer (Active, Active Under Contract, Canceled, Closed, Expired, Pending, Withdrawn).  This is a Single Select field.
    ///
    /// [StandardStatus](https://ddwiki.reso.org/display/DDW17/StandardStatus+Field)
    #[serde(rename = "StandardStatus", skip_serializing_if = "Option::is_none")]
    pub standard_status: Option<crate::StandardStatus>,

    /// Text field containing the accepted postal abbreviation for the state or province.
    ///
    /// [StateOrProvince](https://ddwiki.reso.org/display/DDW17/StateOrProvince+Field)
    #[serde(rename = "StateOrProvince", skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<crate::StateOrProvince>,

    /// A sub-section or area of a defined state or province.  Examples would be the Keys in FL or Hudson Valley in NY.
    ///
    /// [StateRegion](https://ddwiki.reso.org/display/DDW17/StateRegion+Field)
    #[serde(rename = "StateRegion", skip_serializing_if = "Option::is_none")]
    pub state_region: Option<String>,

    /// The transactional timestamp automatically recorded by the MLS system representing the date/time the listing's status was last changed.
    ///
    /// [StatusChangeTimestamp](https://ddwiki.reso.org/display/DDW17/StatusChangeTimestamp+Field)
    #[serde(
        rename = "StatusChangeTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_change_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The number of floors in the property being sold.
    ///
    /// [Stories](https://ddwiki.reso.org/display/DDW17/Stories+Field)
    #[serde(rename = "Stories", skip_serializing_if = "Option::is_none")]
    pub stories: Option<f64>,

    /// The total number of floors in the building. In the case of multi-dwelling structures, this is the entire structure and not the individual dwelling being sold.
    ///
    /// [StoriesTotal](https://ddwiki.reso.org/display/DDW17/StoriesTotal+Field)
    #[serde(rename = "StoriesTotal", skip_serializing_if = "Option::is_none")]
    pub stories_total: Option<f64>,

    /// Information other than a prefix or suffix for the street portion of a postal address.
    ///
    /// [StreetAdditionalInfo](https://ddwiki.reso.org/display/DDW17/StreetAdditionalInfo+Field)
    #[serde(
        rename = "StreetAdditionalInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub street_additional_info: Option<String>,

    /// The direction indicator that precedes the listed property's street name.
    ///
    /// [StreetDirPrefix](https://ddwiki.reso.org/display/DDW17/StreetDirPrefix+Field)
    #[serde(rename = "StreetDirPrefix", skip_serializing_if = "Option::is_none")]
    pub street_dir_prefix: Option<crate::StreetDirection>,

    /// The direction indicator that follows a listed property's street address.
    ///
    /// [StreetDirSuffix](https://ddwiki.reso.org/display/DDW17/StreetDirSuffix+Field)
    #[serde(rename = "StreetDirSuffix", skip_serializing_if = "Option::is_none")]
    pub street_dir_suffix: Option<crate::StreetDirection>,

    /// The street name portion of a listed property's street address.
    ///
    /// [StreetName](https://ddwiki.reso.org/display/DDW17/StreetName+Field)
    #[serde(rename = "StreetName", skip_serializing_if = "Option::is_none")]
    pub street_name: Option<String>,

    /// The street number portion of a listed property's street address.  In some areas the street number may contain non-numeric characters.  This field can also contain extensions and modifiers to the street number, such as "1/2" or "-B".  This street number field should not include Prefixes, Direction or Suffixes.
    ///
    /// [StreetNumber](https://ddwiki.reso.org/display/DDW17/StreetNumber+Field)
    #[serde(rename = "StreetNumber", skip_serializing_if = "Option::is_none")]
    pub street_number: Option<String>,

    /// The integer portion of the street number.
    ///
    /// [StreetNumberNumeric](https://ddwiki.reso.org/display/DDW17/StreetNumberNumeric+Field)
    #[serde(
        rename = "StreetNumberNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub street_number_numeric: Option<f64>,

    /// The suffix portion of a listed property's street address.
    ///
    /// [StreetSuffix](https://ddwiki.reso.org/display/DDW17/StreetSuffix+Field)
    #[serde(rename = "StreetSuffix", skip_serializing_if = "Option::is_none")]
    pub street_suffix: Option<String>,

    /// The Street Suffix Modifier allows the member to enter a unique Street Suffix that was not found in the Street Suffix pick list or to extend or prefix the suffix.
    ///
    /// [StreetSuffixModifier](https://ddwiki.reso.org/display/DDW17/StreetSuffixModifier+Field)
    #[serde(
        rename = "StreetSuffixModifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub street_suffix_modifier: Option<String>,

    /// The type of structure that the property completely or partially encompasses.  For example, House or Cabin are the overall structure and typically sold or leased as a whole.  Multi Family and Docks may be sold in whole, but are often sold or leased by unit/slip.  This field is the type of structure as opposed to style, which is under the Architectural Style field.
    ///
    /// [StructureType](https://ddwiki.reso.org/display/DDW17/StructureType+Field)
    #[serde(rename = "StructureType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub structure_type: Option<Vec<crate::StructureType>>,

    /// The total commission to be paid to the Sub Agency, expressed as either a percentage or a constant currency amount.
    ///
    /// [SubAgencyCompensation](https://ddwiki.reso.org/display/DDW17/SubAgencyCompensation+Field)
    #[serde(
        rename = "SubAgencyCompensation",
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_agency_compensation: Option<String>,

    /// A list of types to clarify the value entered in the SubAgencyCompensation field.  For example $, % or some other clarification of the SubAgencyCompensation.
    ///
    /// [SubAgencyCompensationType](https://ddwiki.reso.org/display/DDW17/SubAgencyCompensationType+Field)
    #[serde(
        rename = "SubAgencyCompensationType",
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_agency_compensation_type: Option<crate::CompensationType>,

    /// A neighborhood, community, complex or builder tract.
    ///
    /// [SubdivisionName](https://ddwiki.reso.org/display/DDW17/SubdivisionName+Field)
    #[serde(rename = "SubdivisionName", skip_serializing_if = "Option::is_none")]
    pub subdivision_name: Option<String>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [SuppliesExpense](https://ddwiki.reso.org/display/DDW17/SuppliesExpense+Field)
    #[serde(rename = "SuppliesExpense", skip_serializing_if = "Option::is_none")]
    pub supplies_expense: Option<f64>,

    /// When permitted by the broker, the options made by the agent on behalf of the seller, where they would like their listings syndicated. i.e. Zillow, Trulia, Homes.com, etc.
    ///
    /// [SyndicateTo](https://ddwiki.reso.org/display/DDW17/SyndicateTo+Field)
    #[serde(rename = "SyndicateTo", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub syndicate_to: Option<Vec<crate::SyndicateTo>>,

    /// Becoming more common in the industry, MLS's are hosting a separate "Public Remarks" for syndication purposes.  This field should be defaulted to containing the Public Remarks, but upon broker decision, modified to include contact and other information denied by IDX rules, but allowed under local and national regulations.
    ///
    /// [SyndicationRemarks](https://ddwiki.reso.org/display/DDW17/SyndicationRemarks+Field)
    #[serde(rename = "SyndicationRemarks", skip_serializing_if = "Option::is_none")]
    pub syndication_remarks: Option<String>,

    /// The annual property tax amount as of the last assessment made by the taxing authority.
    ///
    /// [TaxAnnualAmount](https://ddwiki.reso.org/display/DDW17/TaxAnnualAmount+Field)
    #[serde(rename = "TaxAnnualAmount", skip_serializing_if = "Option::is_none")]
    pub tax_annual_amount: Option<f64>,

    /// The property value as of the last assessment made by the taxing authority.
    ///
    /// [TaxAssessedValue](https://ddwiki.reso.org/display/DDW17/TaxAssessedValue+Field)
    #[serde(rename = "TaxAssessedValue", skip_serializing_if = "Option::is_none")]
    pub tax_assessed_value: Option<f64>,

    /// A type of legal description for land in developed areas where streets or other rights-of-ways delineate large parcels of land referred to as divided into lots on which homes or other types of developments are built.  An example would read "Lot 12 of Block 45 of Tract 3002 of the City of San Dunes, Desert County." Such a description would also reference an official plat filed with the clerk or recorder for that area which shows the location of the block and often the dimensions of the lots therein.
    ///
    /// [TaxBlock](https://ddwiki.reso.org/display/DDW17/TaxBlock+Field)
    #[serde(rename = "TaxBlock", skip_serializing_if = "Option::is_none")]
    pub tax_block: Option<String>,

    /// Some systems of parcel identification incorporate a method which utilizes a county identifier, a tax book number, a tax map number and a parcel identification number.
    ///
    /// [TaxBookNumber](https://ddwiki.reso.org/display/DDW17/TaxBookNumber+Field)
    #[serde(rename = "TaxBookNumber", skip_serializing_if = "Option::is_none")]
    pub tax_book_number: Option<String>,

    /// A type of legal description for land in developed areas where streets or other rights-of-ways delineate large parcels of land referred to as divided into lots on which homes or other types of developments are built.  An example would read "Lot 12 of Block 45 of Tract 3002 of the City of San Dunes, Desert County." Such a description would also reference an official plat filed with the clerk or recorder for that area which shows the location of the block and often the dimensions of the lots therein. The text here is also an index into the property as described by the County Recorder.
    ///
    /// [TaxLegalDescription](https://ddwiki.reso.org/display/DDW17/TaxLegalDescription+Field)
    #[serde(
        rename = "TaxLegalDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_legal_description: Option<String>,

    /// A type of legal description for land in developed areas where streets or other rights-of-ways delineate large parcels of land referred to as divided into lots on which homes or other types of developments are built.  An example would read "Lot 12 of Block 45 of Tract 3002 of the City of San Dunes, Desert County." Such a description would also reference an official plat filed with the clerk or recorder for that area which shows the location of the block and often the dimensions of the lots therein.
    ///
    /// [TaxLot](https://ddwiki.reso.org/display/DDW17/TaxLot+Field)
    #[serde(rename = "TaxLot", skip_serializing_if = "Option::is_none")]
    pub tax_lot: Option<String>,

    /// Some systems of parcel identification incorporate a method which utilizes a county identifier, a tax book number, a tax map number and a parcel identification number.
    ///
    /// [TaxMapNumber](https://ddwiki.reso.org/display/DDW17/TaxMapNumber+Field)
    #[serde(rename = "TaxMapNumber", skip_serializing_if = "Option::is_none")]
    pub tax_map_number: Option<String>,

    /// Any other annual taxes, not including the tax reported in the TaxAmount field, as of the last assessment made by the taxing authority.
    ///
    /// [TaxOtherAnnualAssessmentAmount](https://ddwiki.reso.org/display/DDW17/TaxOtherAnnualAssessmentAmount+Field)
    #[serde(
        rename = "TaxOtherAnnualAssessmentAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_other_annual_assessment_amount: Option<f64>,

    /// Some systems of parcel identification incorporate a method which utilizes a county identifier, a tax book number, a tax map number and a parcel identification number.
    ///
    /// [TaxParcelLetter](https://ddwiki.reso.org/display/DDW17/TaxParcelLetter+Field)
    #[serde(rename = "TaxParcelLetter", skip_serializing_if = "Option::is_none")]
    pub tax_parcel_letter: Option<String>,

    /// The current tax status of the mobile home in cases where the land or space is included in the sale.
    ///
    /// [TaxStatusCurrent](https://ddwiki.reso.org/display/DDW17/TaxStatusCurrent+Field)
    #[serde(rename = "TaxStatusCurrent", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub tax_status_current: Option<Vec<crate::TaxStatusCurrent>>,

    /// A type of legal description for land in developed areas where streets or other rights-of-ways delineate large parcels of land referred to as divided into lots on which homes or other types of developments are built.  An example would read "Lot 12 of Block 45 of Tract 3002 of the City of San Dunes, Desert County." Such a description would also reference an official plat filed with the clerk or recorder for that area which shows the location of the block and often the dimensions of the lots therein.
    ///
    /// [TaxTract](https://ddwiki.reso.org/display/DDW17/TaxTract+Field)
    #[serde(rename = "TaxTract", skip_serializing_if = "Option::is_none")]
    pub tax_tract: Option<String>,

    /// The year in with the last assessment of the property value/tax was made.
    ///
    /// [TaxYear](https://ddwiki.reso.org/display/DDW17/TaxYear+Field)
    #[serde(rename = "TaxYear", skip_serializing_if = "Option::is_none")]
    pub tax_year: Option<f64>,

    /// A list of services or items that the tenant is responsible to pay.
    ///
    /// [TenantPays](https://ddwiki.reso.org/display/DDW17/TenantPays+Field)
    #[serde(rename = "TenantPays", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub tenant_pays: Option<Vec<crate::TenantPays>>,

    /// The state of the surface of the land included with the property.  i.e. flat, rolling, etc.
    ///
    /// [Topography](https://ddwiki.reso.org/display/DDW17/Topography+Field)
    #[serde(rename = "Topography", skip_serializing_if = "Option::is_none")]
    pub topography: Option<String>,

    /// Total actual rent currently being collected from tenants of the income property.
    ///
    /// [TotalActualRent](https://ddwiki.reso.org/display/DDW17/TotalActualRent+Field)
    #[serde(rename = "TotalActualRent", skip_serializing_if = "Option::is_none")]
    pub total_actual_rent: Option<f64>,

    /// A subdivision of the county.
    ///
    /// [Township](https://ddwiki.reso.org/display/DDW17/Township+Field)
    #[serde(rename = "Township", skip_serializing_if = "Option::is_none")]
    pub township: Option<String>,

    /// The total commission to be paid to the transaction facilitator, expressed as either a percentage or a constant currency amount.
    ///
    /// [TransactionBrokerCompensation](https://ddwiki.reso.org/display/DDW17/TransactionBrokerCompensation+Field)
    #[serde(
        rename = "TransactionBrokerCompensation",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_broker_compensation: Option<String>,

    /// A list of types to clarify the value entered in the TransactionBrokerCompensation field.  For example $, % or some other clarification of the TransactionBrokerCompensation.
    ///
    /// [TransactionBrokerCompensationType](https://ddwiki.reso.org/display/DDW17/TransactionBrokerCompensationType+Field)
    #[serde(
        rename = "TransactionBrokerCompensationType",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_broker_compensation_type: Option<crate::CompensationType>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [TrashExpense](https://ddwiki.reso.org/display/DDW17/TrashExpense+Field)
    #[serde(rename = "TrashExpense", skip_serializing_if = "Option::is_none")]
    pub trash_expense: Option<f64>,

    /// Text field containing the number or portion of a larger building or complex. Unit Number should appear following the street suffix or, if it exists, the street suffix direction, in the street address. Examples are: "APT G", "55", etc.
    ///
    /// [UnitNumber](https://ddwiki.reso.org/display/DDW17/UnitNumber+Field)
    #[serde(rename = "UnitNumber", skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<String>,

    /// This field is a list of the types used in the Unit Type repeating elements. The Type is a list of possible Unit Types. i.e. 1, 2, 3 or 2 Bed, Studio, Special Loft, etc. Each selected are expected to appear as the "[type]" in the related UnitType fields in a flattened implementation (RETS 1.x only) of the room fields. A relational implementation of UnitTypes must omit the type from the field name and use UnitTypeType to create a vertical representation of the various unit types. The fact that the field repeats the word "type" is intentional.
    ///
    /// [UnitTypeType](https://ddwiki.reso.org/display/DDW17/UnitTypeType+Field)
    #[serde(rename = "UnitTypeType", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub unit_type_type: Option<Vec<crate::UnitTypeType>>,

    /// A collection of types of units included in the income (multi-family) property. The collection includes a type, beds, baths and other aspects of the type of unit behind described.
    ///
    /// [UnitTypes](https://ddwiki.reso.org/display/DDW17/UnitTypes+Field)
    #[serde(rename = "UnitTypes", skip_serializing_if = "Option::is_none")]
    pub unit_types: Option<String>,

    /// Are the units furnished? i.e. All Units, Varies By Unit, None.
    ///
    /// [UnitsFurnished](https://ddwiki.reso.org/display/DDW17/UnitsFurnished+Field)
    #[serde(rename = "UnitsFurnished", skip_serializing_if = "Option::is_none")]
    pub units_furnished: Option<crate::UnitsFurnished>,

    /// The Universal Property Identifier is a unique identifier for all real property in the US and Canada.  It is based on country and local identification methods and is limited to real property.  For cases such as shares of real property, units, and other more granular cases, please utilize the UniversalPropertySubId.
    ///
    /// [UniversalPropertyId](https://ddwiki.reso.org/display/DDW17/UniversalPropertyId+Field)
    #[serde(
        rename = "UniversalPropertyId",
        skip_serializing_if = "Option::is_none"
    )]
    pub universal_property_id: Option<String>,

    /// The Universal Property Sub Identifier is a unique identifier for all sub sets or shares of real property in the US and Canada.  This may include Stock Cooperatives, Community Apartment, Units for Rent, etc.  Informally abbreviated as "UPSI", It is based on country and local identification methods just as the UPI, but is limited to sub sets or shares of real property.  For cases of complete real property, please utilize the UniversalPropertyId field.
    ///
    /// [UniversalPropertySubId](https://ddwiki.reso.org/display/DDW17/UniversalPropertySubId+Field)
    #[serde(
        rename = "UniversalPropertySubId",
        skip_serializing_if = "Option::is_none"
    )]
    pub universal_property_sub_id: Option<String>,

    /// The UnparsedAddress is a text representation of the address with the full civic location as a single entity. It may optionally include any of City, StateOrProvince, PostalCode and Country.
    ///
    /// [UnparsedAddress](https://ddwiki.reso.org/display/DDW17/UnparsedAddress+Field)
    #[serde(rename = "UnparsedAddress", skip_serializing_if = "Option::is_none")]
    pub unparsed_address: Option<String>,

    /// A list of the utilities for the property being sold/leased.
    ///
    /// [Utilities](https://ddwiki.reso.org/display/DDW17/Utilities+Field)
    #[serde(rename = "Utilities", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub utilities: Option<Vec<crate::Utilities>>,

    /// An estimate of the amount of rent that may be foregone because of unoccupied units.
    ///
    /// [VacancyAllowance](https://ddwiki.reso.org/display/DDW17/VacancyAllowance+Field)
    #[serde(rename = "VacancyAllowance", skip_serializing_if = "Option::is_none")]
    pub vacancy_allowance: Option<f64>,

    /// An estimate of the percent of rent that may be foregone because of unoccupied units.
    ///
    /// [VacancyAllowanceRate](https://ddwiki.reso.org/display/DDW17/VacancyAllowanceRate+Field)
    #[serde(
        rename = "VacancyAllowanceRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub vacancy_allowance_rate: Option<f64>,

    /// A list of the type(s) of vegetation on the property. Note that this is not for farm crops, but more residential type vegetation.
    ///
    /// [Vegetation](https://ddwiki.reso.org/display/DDW17/Vegetation+Field)
    #[serde(rename = "Vegetation", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub vegetation: Option<Vec<crate::Vegetation>>,

    /// System generated timestamp of when the last update or change to the videos for this listing was made.
    ///
    /// [VideosChangeTimestamp](https://ddwiki.reso.org/display/DDW17/VideosChangeTimestamp+Field)
    #[serde(
        rename = "VideosChangeTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub videos_change_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    /// The total number of videos or virtual tours included with the listing.
    ///
    /// [VideosCount](https://ddwiki.reso.org/display/DDW17/VideosCount+Field)
    #[serde(rename = "VideosCount", skip_serializing_if = "Option::is_none")]
    pub videos_count: Option<f64>,

    /// A view as seen from the listed property.
    ///
    /// [View](https://ddwiki.reso.org/display/DDW17/View+Field)
    #[serde(rename = "View", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub view: Option<Vec<crate::View>>,

    /// The property has a view.
    ///
    /// [ViewYN](https://ddwiki.reso.org/display/DDW17/ViewYN+Field)
    #[serde(rename = "ViewYN", skip_serializing_if = "Option::is_none")]
    pub view_yn: Option<bool>,

    /// A text field that holds the URL for a branded virtual tour of the property.
    ///
    /// [VirtualTourURLBranded](https://ddwiki.reso.org/display/DDW17/VirtualTourURLBranded+Field)
    #[serde(
        rename = "VirtualTourURLBranded",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_tour_urlbranded: Option<String>,

    /// A text field that holds the URL for an unbranded virtual tour of the property.
    ///
    /// [VirtualTourURLUnbranded](https://ddwiki.reso.org/display/DDW17/VirtualTourURLUnbranded+Field)
    #[serde(
        rename = "VirtualTourURLUnbranded",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_tour_urlunbranded: Option<String>,

    /// A walkability index based on the time to walk from a property to near by essentials such as grocery stores, schools, churches, etc.  See www.walkscore.com for more information and requirements for using WalkScore.
    ///
    /// [WalkScore](https://ddwiki.reso.org/display/DDW17/WalkScore+Field)
    #[serde(rename = "WalkScore", skip_serializing_if = "Option::is_none")]
    pub walk_score: Option<f64>,

    /// The name, if known, of the body of water on which the property is located. (E.g., lake name, river name, ocean name, sea name, canal name).
    ///
    /// [WaterBodyName](https://ddwiki.reso.org/display/DDW17/WaterBodyName+Field)
    #[serde(rename = "WaterBodyName", skip_serializing_if = "Option::is_none")]
    pub water_body_name: Option<String>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [WaterSewerExpense](https://ddwiki.reso.org/display/DDW17/WaterSewerExpense+Field)
    #[serde(rename = "WaterSewerExpense", skip_serializing_if = "Option::is_none")]
    pub water_sewer_expense: Option<f64>,

    /// A list of the source(s) of water for the property
    ///
    /// [WaterSource](https://ddwiki.reso.org/display/DDW17/WaterSource+Field)
    #[serde(rename = "WaterSource", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub water_source: Option<Vec<crate::WaterSource>>,

    /// Features of the waterfront on which the property is located.
    ///
    /// [WaterfrontFeatures](https://ddwiki.reso.org/display/DDW17/WaterfrontFeatures+Field)
    #[serde(rename = "WaterfrontFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub waterfront_features: Option<Vec<crate::WaterfrontFeatures>>,

    /// The property is on the waterfront.
    ///
    /// [WaterfrontYN](https://ddwiki.reso.org/display/DDW17/WaterfrontYN+Field)
    #[serde(rename = "WaterfrontYN", skip_serializing_if = "Option::is_none")]
    pub waterfront_yn: Option<bool>,

    /// A list of features or description of the windows included in the sale/lease.
    ///
    /// [WindowFeatures](https://ddwiki.reso.org/display/DDW17/WindowFeatures+Field)
    #[serde(rename = "WindowFeatures", skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::comma_delimited")]
    pub window_features: Option<Vec<crate::WindowFeatures>>,

    /// Date the listing was withdrawn from the market.  This is not when a listing contact was cancelled or closed, but a withdrawal from the market while the contract between the seller and listing agent is still in effect and an offer has not been accepted. This is the date entered by the agent reflecting when the change occurred contractually, not a timestamp of when the change was made in the MLS.
    ///
    /// [WithdrawnDate](https://ddwiki.reso.org/display/DDW17/WithdrawnDate+Field)
    #[serde(rename = "WithdrawnDate", skip_serializing_if = "Option::is_none")]
    pub withdrawn_date: Option<chrono::NaiveDate>,

    /// Measurement or percentage of the property that is wooded or forest.
    ///
    /// [WoodedArea](https://ddwiki.reso.org/display/DDW17/WoodedArea+Field)
    #[serde(rename = "WoodedArea", skip_serializing_if = "Option::is_none")]
    pub wooded_area: Option<f64>,

    /// The annual expense that is not paid directly by the tenant and is included in the Operating Expense calculations.
    ///
    /// [WorkmansCompensationExpense](https://ddwiki.reso.org/display/DDW17/WorkmansCompensationExpense+Field)
    #[serde(
        rename = "WorkmansCompensationExpense",
        skip_serializing_if = "Option::is_none"
    )]
    pub workmans_compensation_expense: Option<f64>,

    /// The year that an occupancy permit is first granted for the house or other local measure of initial habitability of the build. The type definition permits an empty value with an attribute noting that it is an unknown date or that the building is new construction. While constraints have not been applied, convention at the time of adoption has this as a four (4) digit year value.
    ///
    /// [YearBuilt](https://ddwiki.reso.org/display/DDW17/YearBuilt+Field)
    #[serde(rename = "YearBuilt", skip_serializing_if = "Option::is_none")]
    pub year_built: Option<f64>,

    /// A description of the details behind the year the structure was built.
    ///
    /// [YearBuiltDetails](https://ddwiki.reso.org/display/DDW17/YearBuiltDetails+Field)
    #[serde(rename = "YearBuiltDetails", skip_serializing_if = "Option::is_none")]
    pub year_built_details: Option<String>,

    /// The year a major rebuild/renovated of the structure occurred.
    ///
    /// [YearBuiltEffective](https://ddwiki.reso.org/display/DDW17/YearBuiltEffective+Field)
    #[serde(rename = "YearBuiltEffective", skip_serializing_if = "Option::is_none")]
    pub year_built_effective: Option<f64>,

    /// Add a list of sources of the year built.  i.e. Appraiser, Assessor, Builder, Estimated, etc.,
    ///
    /// [YearBuiltSource](https://ddwiki.reso.org/display/DDW17/YearBuiltSource+Field)
    #[serde(rename = "YearBuiltSource", skip_serializing_if = "Option::is_none")]
    pub year_built_source: Option<crate::YearBuiltSource>,

    /// The year the business being sold was established.
    ///
    /// [YearEstablished](https://ddwiki.reso.org/display/DDW17/YearEstablished+Field)
    #[serde(rename = "YearEstablished", skip_serializing_if = "Option::is_none")]
    pub year_established: Option<f64>,

    /// The number of years the current owner has had possession of the business.
    ///
    /// [YearsCurrentOwner](https://ddwiki.reso.org/display/DDW17/YearsCurrentOwner+Field)
    #[serde(rename = "YearsCurrentOwner", skip_serializing_if = "Option::is_none")]
    pub years_current_owner: Option<f64>,

    /// A division of the city or county into areas of different permissible land uses. This Zone field should be used for the short code that is commonly used.  For full textual descriptions please use the ZoningDescription field.
    ///
    /// [Zoning](https://ddwiki.reso.org/display/DDW17/Zoning+Field)
    #[serde(rename = "Zoning", skip_serializing_if = "Option::is_none")]
    pub zoning: Option<String>,

    /// A list of descriptions of the zoning of the property. The zoning codes are often non-descriptive and variant. Zoning Description is a more descriptive form of the zoning for the property, i.e. Agricultural, Residential, Rezone Possible, etc. Specific zone codes must be added to the Zoning field.
    ///
    /// [ZoningDescription](https://ddwiki.reso.org/display/DDW17/ZoningDescription+Field)
    #[serde(rename = "ZoningDescription", skip_serializing_if = "Option::is_none")]
    pub zoning_description: Option<String>,
}
