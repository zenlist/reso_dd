//! Structures and Enumerations that implement the [Real Estate Standards Organization (RESO) Data
//! Dictionary][reso].
//!
//! The structures defined here can be serialized and deserialized using [serde][serde].
//!
//! ```
//! use serde_json;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let reso = r#"{
//!   "StandardStatus": "Active"
//! }"#;
//!
//! let property: reso_dd::Property = serde_json::from_str(reso)?;
//! assert_eq!(property.standard_status, Some(reso_dd::StandardStatus::Active));
//!
//! println!("{}", serde_json::to_string(&property)?);
//! # Ok(())
//! # }
//! ```
//!
//! [reso]: https://ddwiki.reso.org/display/DDW17/RESO+Data+Dictionary+Wiki+v1.7
//! [serde]: https://serde.rs/

#![deny(missing_docs)]

mod generated;
pub use generated::*;
mod reso_enumeration;
pub use reso_enumeration::ResoEnumeration;
pub mod comma_delimited;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let data = r###"
        {
            "LotSizeAcres": 0.1846,
            "RoomType": "Recreation Room",
            "SpecialListingConditions": "REO/Lender Owned",
            "InternetAddressDisplayYN": true,
            "Cooling": "Central Air",
            "InternetConsumerCommentYN": false,
            "MLSAreaMajor": "Oak Lawn",
            "LivingArea": 2101,
            "LivingAreaSource": "Assessor",
            "YearBuilt": 1965,
            "Basement": "Partial",
            "BedroomsTotal": 3,
            "BathroomsTotalInteger": 1.1,
            "BuyerAgencyCompensation": "3.0%",
            "City": "Oak Lawn",
            "CountyOrParish": "Cook",
            "StreetDirPrefix": "W",
            "LotSizeDimensions": "60 X 134",
            "Directions": "[anonymized]",
            "Electric": "Circuit Breakers,200+ Amp Service",
            "AssociationFeeFrequency": "Not Applicable",
            "BathroomsFull": 1,
            "ElementarySchoolDistrict": "123",
            "BathroomsHalf": 1,
            "Heating": "Natural Gas,Forced Air",
            "HighSchoolDistrict": "218",
            "StreetNumber": "1234",
            "Sewer": "Public Sewer",
            "WaterSource": "Lake Michigan",
            "InternetAutomatedValuationDisplayYN": true,
            "InternetEntireListingDisplayYN": true,
            "MiddleOrJuniorSchoolDistrict": "123",
            "ListAgentMobilePhone": "555-123-4321",
            "ListAgentEmail": "janedoe@example.com",
            "ListAgentFax": "(555) 123-4568",
            "ListAgentFirstName": "Jane",
            "ListAgentFullName": "Jane Doe",
            "ListAgentMlsId": "123456",
            "ListAgentKey": "123456",
            "ListAgentLastName": "Doe",
            "ListAgentOfficePhone": "555-123-4567",
            "ListingContractDate": "2019-10-08",
            "OriginalEntryTimestamp": "2019-10-08T21:00:33.000Z",
            "LotFeatures": "Fenced Yard",
            "LandLeaseYN": false,
            "ListingAgreement": "Exclusive Right To Sell",
            "PreviousListPrice": 202000,
            "DaysOnMarket": 64,
            "ListingKey": "11111",
            "ListingId": "11111",
            "ListOfficeFax": "(555) 123-4568",
            "ListOfficeMlsId": "12345",
            "ListOfficeKey": "12345",
            "ListOfficeName": "Best Realty",
            "ListOfficePhone": "555-123-4567",
            "ListPrice": 195000,
            "AssociationFeeIncludes": "None",
            "Model": "SPLIT WITH SUB BASEMENT",
            "AdditionalParcelsYN": false,
            "CumulativeDaysOnMarket": 101,
            "NewConstructionYN": false,
            "GarageSpaces": 2,
            "OriginalListPrice": 202000,
            "OffMarketDate": "2019-12-10",
            "Ownership": "Fee Simple",
            "OwnerName": "Bill",
            "PhotosCount": 23,
            "PhotosChangeTimestamp": "2019-12-06T01:07:02.640Z",
            "ParcelNumber": "24151160240000",
            "Possession": "Closing",
            "OriginatingSystemModificationTimestamp": "2019-12-11T00:01:18.000Z",
            "PublicRemarks": "Classic Oak Lawn Split Level!  This home has lots to offer a new family.  3 Bedrooms, 1.5 baths,  updated kitchen and baths, updated HVAC,  some newer windows, Hardwood Floors, 6 Panel Doors, Finished Sub Basement, Full fenced in yard with a shed!  Nice block with close proximity to school, shopping.  Make an appointment to this before it's gone!  Property was built prior to 1978 and lead based paint may potentially exist.  This property may qualify for Seller Financing (Vendee).",
            "PropertyType": "Residential Income",
            "RoomsTotal": 8,
            "ShowingInstructions": "Use MLS Showing Assist to schedule an appointment",
            "MlsStatus": "Active",
            "StandardStatus": "Active",
            "StateOrProvince": "CA",
            "StatusChangeTimestamp": "2019-12-11T00:01:18.000Z",
            "StreetName": "1st",
            "StreetSuffix": "Place",
            "ArchitecturalStyle": "Tri-Level",
            "TaxAnnualAmount": 8295.87,
            "Township": "Everytown",
            "TaxYear": 2018,
            "WaterfrontYN": false,
            "PostalCode": "01234",
            "MlgCanView": true,
            "ModificationTimestamp": "2019-12-11T00:02:03.715Z",
            "OriginatingSystemName": "mred",
            "CloseDate": null,
            "ExpirationDate": null,
            "PurchaseContractDate": null
        }"###;

        let result = serde_json::from_str::<Property>(data);

        assert!(result.is_ok());
        let property = result.unwrap();
        assert_eq!(
            property.heating,
            Some(vec![Heating::NaturalGas, Heating::ForcedAir])
        );
        assert_eq!(
            property.association_fee_frequency,
            Some(FeeFrequency::OpenEnumeration("Not Applicable".into()))
        );
        assert_eq!(property.purchase_contract_date, None);
    }
}
