// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [BusinessType Lookups](https://ddwiki.reso.org/display/DDW17/BusinessType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BusinessType {
    /// "[Accounting](https://ddwiki.reso.org/display/DDW17/Accounting)": The listing is for an accounting business.
    Accounting,

    /// "[Administrative and Support](https://ddwiki.reso.org/display/DDW17/Administrative+and+Support)": The listing is for an administrative and support business.
    AdministrativeandSupport,

    /// "[Advertising](https://ddwiki.reso.org/display/DDW17/Advertising)": The listing is for an advertising business.
    Advertising,

    /// "[Agriculture](https://ddwiki.reso.org/display/DDW17/Agriculture)": The listing is for an agriculture business.
    Agriculture,

    /// "[Animal Grooming](https://ddwiki.reso.org/display/DDW17/Animal+Grooming)": The listing is for an animal grooming business.
    AnimalGrooming,

    /// "[Appliances](https://ddwiki.reso.org/display/DDW17/Appliances)": The listing is for an appliances business.
    Appliances,

    /// "[Aquarium Supplies](https://ddwiki.reso.org/display/DDW17/Aquarium+Supplies)": The listing is for an aquarium supplies business.
    AquariumSupplies,

    /// "[Arts and Entertainment](https://ddwiki.reso.org/display/DDW17/Arts+and+Entertainment)": The listing is for an arts and entertainment business.
    ArtsandEntertainment,

    /// "[Athletic](https://ddwiki.reso.org/display/DDW17/Athletic)": The listing is for an athletic business.
    Athletic,

    /// "[Auto Body](https://ddwiki.reso.org/display/DDW17/Auto+Body)": The listing is for an Auto Body business.
    AutoBody,

    /// "[Auto Dealer](https://ddwiki.reso.org/display/DDW17/Auto+Dealer)": The listing is for an auto dealer business.
    AutoDealer,

    /// "[Auto Glass](https://ddwiki.reso.org/display/DDW17/Auto+Glass)": The listing is for an Auto Glass business.
    AutoGlass,

    /// "[Auto Parts](https://ddwiki.reso.org/display/DDW17/Auto+Parts)": The listing is for an Auto Parts business.
    AutoParts,

    /// "[Auto Rent/Lease](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243862)": The listing is for an Auto Rent/Lease business.
    AutoRentLease,

    /// "[Auto Repair-Specialty](https://ddwiki.reso.org/display/DDW17/Auto+Repair-Specialty)": The listing is for an Auto Repair-Specialty business.
    AutoRepairSpecialty,

    /// "[Auto Service](https://ddwiki.reso.org/display/DDW17/Auto+Service)": The listing is for an auto service business.
    AutoService,

    /// "[Auto Stereo/Alarm](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243865)": The listing is for an Auto Stereo/Alarm business.
    AutoStereoAlarm,

    /// "[Auto Tires](https://ddwiki.reso.org/display/DDW17/Auto+Tires)": The listing is for an Auto Tires business.
    AutoTires,

    /// "[Auto Wrecking](https://ddwiki.reso.org/display/DDW17/Auto+Wrecking)": The listing is for an Auto Wrecking business.
    AutoWrecking,

    /// "[Bakery](https://ddwiki.reso.org/display/DDW17/Bakery)": The listing is for a bakery business.
    Bakery,

    /// "[Bar/Tavern/Lounge](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243869)": The listing is for a bar/tavern/lounge business.
    BarTavernLounge,

    /// "[Barber/Beauty](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243870)": The listing is for a barber/beauty business.
    BarberBeauty,

    /// "[Bed & Breakfast](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243871)": The listing is for a bed & breakfast business.
    BedBreakfast,

    /// "[Books/Cards/Stationary](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243872)": The listing is for a Books/Cards/Stationary business.
    BooksCardsStationary,

    /// "[Butcher](https://ddwiki.reso.org/display/DDW17/Butcher)": The listing is for a butcher business.
    Butcher,

    /// "[Cabinets](https://ddwiki.reso.org/display/DDW17/Cabinets)": The listing is for a Cabinets business.
    Cabinets,

    /// "[Candy/Cookie](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243875)": The listing is for a Candy/Cookie business.
    CandyCookie,

    /// "[Car Wash](https://ddwiki.reso.org/display/DDW17/Car+Wash)": The listing is for a car wash business.
    CarWash,

    /// "[Carpet/Tile](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243877)": The listing is for a Carpet/Tile business.
    CarpetTile,

    /// "[Child Care](https://ddwiki.reso.org/display/DDW17/Child+Care)": The listing is for a child care business.
    ChildCare,

    /// "[Church](https://ddwiki.reso.org/display/DDW17/Church)": The listing is for a church business.
    Church,

    /// "[Clothing](https://ddwiki.reso.org/display/DDW17/Clothing)": The listing is for a clothing business.
    Clothing,

    /// "[Commercial](https://ddwiki.reso.org/display/DDW17/Commercial)": The listing is for a commercial business.
    Commercial,

    /// "[Computer](https://ddwiki.reso.org/display/DDW17/Computer)": The listing is for a computer business.
    Computer,

    /// "[Construction/Contractor](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243883)": The listing is for a construction/contractor business.
    ConstructionContractor,

    /// "[Convalescent](https://ddwiki.reso.org/display/DDW17/Convalescent)": The listing is for a Convalescent business.
    Convalescent,

    /// "[Convenience Store](https://ddwiki.reso.org/display/DDW17/Convenience+Store)": The listing is for a convenience store business.
    ConvenienceStore,

    /// "[Dance Studio](https://ddwiki.reso.org/display/DDW17/Dance+Studio)": The listing is for a Dance Studio business.
    DanceStudio,

    /// "[Decorator](https://ddwiki.reso.org/display/DDW17/Decorator)": The listing is for a Decorator business.
    Decorator,

    /// "[Deli/Catering](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243888)": The listing is for a Deli/Catering business.
    DeliCatering,

    /// "[Dental](https://ddwiki.reso.org/display/DDW17/Dental)": The listing is for a dental business.
    Dental,

    /// "[Distribution](https://ddwiki.reso.org/display/DDW17/Distribution)": The listing is for a distribution business.
    Distribution,

    /// "[Doughnut](https://ddwiki.reso.org/display/DDW17/Doughnut)": The listing is for a doughnut business.
    Doughnut,

    /// "[Drugstore](https://ddwiki.reso.org/display/DDW17/Drugstore)": The listing is for a Drugstore business.
    Drugstore,

    /// "[Dry Cleaner](https://ddwiki.reso.org/display/DDW17/Dry+Cleaner)": The listing is for a dry cleaner business.
    DryCleaner,

    /// "[Education/School](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243894)": The listing is for an education/school business.
    EducationSchool,

    /// "[Electronics](https://ddwiki.reso.org/display/DDW17/Electronics)": The listing is for an Electronics business.
    Electronics,

    /// "[Employment](https://ddwiki.reso.org/display/DDW17/Employment)": The listing is for an Employment business.
    Employment,

    /// "[Farm](https://ddwiki.reso.org/display/DDW17/Farm)": The listing is for a farm business.
    Farm,

    /// "[Fast Food](https://ddwiki.reso.org/display/DDW17/Fast+Food)": The listing is for a fast food business.
    FastFood,

    /// "[Financial](https://ddwiki.reso.org/display/DDW17/Financial)": The listing is for a financial business.
    Financial,

    /// "[Fitness](https://ddwiki.reso.org/display/DDW17/Fitness)": The listing is for a fitness business.
    Fitness,

    /// "[Florist/Nursery](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243901)": The listing is for a florist/nursery business.
    FloristNursery,

    /// "[Food & Beverage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243902)": The listing is for a food & beverage business.
    FoodBeverage,

    /// "[Forest Reserve](https://ddwiki.reso.org/display/DDW17/Forest+Reserve)": The listing is for a forest reserve business.
    ForestReserve,

    /// "[Franchise](https://ddwiki.reso.org/display/DDW17/Franchise)": The listing is for a franchise business.
    Franchise,

    /// "[Furniture](https://ddwiki.reso.org/display/DDW17/Furniture)": The listing is for a Furniture business.
    Furniture,

    /// "[Gas Station](https://ddwiki.reso.org/display/DDW17/Gas+Station)": The listing is for a gas station business.
    GasStation,

    /// "[Gift Shop](https://ddwiki.reso.org/display/DDW17/Gift+Shop)": The listing is for a gift shop business.
    GiftShop,

    /// "[Grocery](https://ddwiki.reso.org/display/DDW17/Grocery)": The listing is for a grocery business.
    Grocery,

    /// "[Hardware](https://ddwiki.reso.org/display/DDW17/Hardware)": The listing is for a hardware business.
    Hardware,

    /// "[Health Food](https://ddwiki.reso.org/display/DDW17/Health+Food)": The listing is for a Health Food business.
    HealthFood,

    /// "[Health Services](https://ddwiki.reso.org/display/DDW17/Health+Services)": The listing is for a health services business.
    HealthServices,

    /// "[Hobby](https://ddwiki.reso.org/display/DDW17/Hobby)": The listing is for a Hobby business.
    Hobby,

    /// "[Home Cleaner](https://ddwiki.reso.org/display/DDW17/Home+Cleaner)": The listing is for a Home Cleaner business.
    HomeCleaner,

    /// "[Hospitality](https://ddwiki.reso.org/display/DDW17/Hospitality)": The listing is for a hospitality business.
    Hospitality,

    /// "[Hotel/Motel](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243915)": The listing is for a hotel/motel business.
    HotelMotel,

    /// "[Ice Cream/Frozen Yogurt](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243916)": The listing is for an ice cream/frozen yogurt business.
    IceCreamFrozenYogurt,

    /// "[Industrial](https://ddwiki.reso.org/display/DDW17/Industrial)": The listing is for an industrial business.
    Industrial,

    /// "[Jewelry](https://ddwiki.reso.org/display/DDW17/Jewelry)": The listing is for a Jewelry business.
    Jewelry,

    /// "[Landscaping](https://ddwiki.reso.org/display/DDW17/Landscaping)": The listing is for a Landscaping business.
    Landscaping,

    /// "[Laundromat](https://ddwiki.reso.org/display/DDW17/Laundromat)": The listing is for a laundromat business.
    Laundromat,

    /// "[Liquor Store](https://ddwiki.reso.org/display/DDW17/Liquor+Store)": The listing is for a liquor store business.
    LiquorStore,

    /// "[Locksmith](https://ddwiki.reso.org/display/DDW17/Locksmith)": The listing is for a Locksmith business.
    Locksmith,

    /// "[Manufacturing](https://ddwiki.reso.org/display/DDW17/Manufacturing)": The listing is for a manufacturing business.
    Manufacturing,

    /// "[Medical](https://ddwiki.reso.org/display/DDW17/Medical)": The listing is for a medical business.
    Medical,

    /// "[Mixed](https://ddwiki.reso.org/display/DDW17/Mixed)": The listing is for a mixed business.
    Mixed,

    /// "[Mobile/Trailer Park](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243927)": The listing is for a mobile/trailer park business.
    MobileTrailerPark,

    /// "[Music](https://ddwiki.reso.org/display/DDW17/Music)": The listing is for a Music business.
    Music,

    /// "[Nursing Home](https://ddwiki.reso.org/display/DDW17/Nursing+Home)": The listing is for a nursing home business.
    NursingHome,

    /// "[Office Supply](https://ddwiki.reso.org/display/DDW17/Office+Supply)": The listing is for an Office Supply business.
    OfficeSupply,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243931)": The listing is for an other business.
    Other,

    /// "[Paints](https://ddwiki.reso.org/display/DDW17/Paints)": The listing is for a Paints business.
    Paints,

    /// "[Parking](https://ddwiki.reso.org/display/DDW17/Parking)": The listing is for a parking business.
    Parking,

    /// "[Pet Store](https://ddwiki.reso.org/display/DDW17/Pet+Store)": The listing is for a pet store business.
    PetStore,

    /// "[Photographer](https://ddwiki.reso.org/display/DDW17/Photographer)": The listing is for a Photographer business.
    Photographer,

    /// "[Pizza](https://ddwiki.reso.org/display/DDW17/Pizza)": The listing is for a Pizza business.
    Pizza,

    /// "[Printing](https://ddwiki.reso.org/display/DDW17/Printing)": The listing is for a printing business.
    Printing,

    /// "[Professional Service](https://ddwiki.reso.org/display/DDW17/Professional+Service)": The listing is for a professional service business.
    ProfessionalService,

    /// "[Professional/Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243939)": The listing is for a professional/office business.
    ProfessionalOffice,

    /// "[Real Estate](https://ddwiki.reso.org/display/DDW17/Real+Estate)": The listing is for a Real Estate business.
    RealEstate,

    /// "[Recreation](https://ddwiki.reso.org/display/DDW17/Recreation)": The listing is for a recreation business.
    Recreation,

    /// "[Rental](https://ddwiki.reso.org/display/DDW17/Rental)": The listing is for a Rental business.
    Rental,

    /// "[Residential](https://ddwiki.reso.org/display/DDW17/Residential)": The listing is for a residential business.
    Residential,

    /// "[Restaurant](https://ddwiki.reso.org/display/DDW17/Restaurant)": The listing is for a restaurant business.
    Restaurant,

    /// "[Retail](https://ddwiki.reso.org/display/DDW17/Retail)": The listing is for a retail business.
    Retail,

    /// "[Saddlery/Harness](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243946)": The listing is for a Saddlery/Harness business.
    SaddleryHarness,

    /// "[Sporting Goods](https://ddwiki.reso.org/display/DDW17/Sporting+Goods)": The listing is for a sporting goods business.
    SportingGoods,

    /// "[Storage](https://ddwiki.reso.org/display/DDW17/Storage)": The listing is for a storage business.
    Storage,

    /// "[Toys](https://ddwiki.reso.org/display/DDW17/Toys)": The listing is for a Toys business.
    Toys,

    /// "[Transportation](https://ddwiki.reso.org/display/DDW17/Transportation)": The listing is for a transportation business.
    Transportation,

    /// "[Travel](https://ddwiki.reso.org/display/DDW17/Travel)": The listing is for a Travel business.
    Travel,

    /// "[Upholstery](https://ddwiki.reso.org/display/DDW17/Upholstery)": The listing is for an Upholstery business.
    Upholstery,

    /// "[Utility](https://ddwiki.reso.org/display/DDW17/Utility)": The listing is for a Utility business.
    Utility,

    /// "[Variety](https://ddwiki.reso.org/display/DDW17/Variety)": The listing is for a Variety business.
    Variety,

    /// "[Video](https://ddwiki.reso.org/display/DDW17/Video)": The listing is for a Video business.
    Video,

    /// "[Wallpaper](https://ddwiki.reso.org/display/DDW17/Wallpaper)": The listing is for a Wallpaper business.
    Wallpaper,

    /// "[Warehouse](https://ddwiki.reso.org/display/DDW17/Warehouse)": The listing is for a warehouse business.
    Warehouse,

    /// "[Wholesale](https://ddwiki.reso.org/display/DDW17/Wholesale)": The listing is for a wholesale business.
    Wholesale,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for BusinessType {
    fn from(s: String) -> BusinessType {
        match s.as_ref() {
            "Accounting" => BusinessType::Accounting,

            "Administrative and Support" => BusinessType::AdministrativeandSupport,

            "Advertising" => BusinessType::Advertising,

            "Agriculture" => BusinessType::Agriculture,

            "Animal Grooming" => BusinessType::AnimalGrooming,

            "Appliances" => BusinessType::Appliances,

            "Aquarium Supplies" => BusinessType::AquariumSupplies,

            "Arts and Entertainment" => BusinessType::ArtsandEntertainment,

            "Athletic" => BusinessType::Athletic,

            "Auto Body" => BusinessType::AutoBody,

            "Auto Dealer" => BusinessType::AutoDealer,

            "Auto Glass" => BusinessType::AutoGlass,

            "Auto Parts" => BusinessType::AutoParts,

            "Auto Rent/Lease" => BusinessType::AutoRentLease,

            "Auto Repair-Specialty" => BusinessType::AutoRepairSpecialty,

            "Auto Service" => BusinessType::AutoService,

            "Auto Stereo/Alarm" => BusinessType::AutoStereoAlarm,

            "Auto Tires" => BusinessType::AutoTires,

            "Auto Wrecking" => BusinessType::AutoWrecking,

            "Bakery" => BusinessType::Bakery,

            "Bar/Tavern/Lounge" => BusinessType::BarTavernLounge,

            "Barber/Beauty" => BusinessType::BarberBeauty,

            "Bed & Breakfast" => BusinessType::BedBreakfast,

            "Books/Cards/Stationary" => BusinessType::BooksCardsStationary,

            "Butcher" => BusinessType::Butcher,

            "Cabinets" => BusinessType::Cabinets,

            "Candy/Cookie" => BusinessType::CandyCookie,

            "Car Wash" => BusinessType::CarWash,

            "Carpet/Tile" => BusinessType::CarpetTile,

            "Child Care" => BusinessType::ChildCare,

            "Church" => BusinessType::Church,

            "Clothing" => BusinessType::Clothing,

            "Commercial" => BusinessType::Commercial,

            "Computer" => BusinessType::Computer,

            "Construction/Contractor" => BusinessType::ConstructionContractor,

            "Convalescent" => BusinessType::Convalescent,

            "Convenience Store" => BusinessType::ConvenienceStore,

            "Dance Studio" => BusinessType::DanceStudio,

            "Decorator" => BusinessType::Decorator,

            "Deli/Catering" => BusinessType::DeliCatering,

            "Dental" => BusinessType::Dental,

            "Distribution" => BusinessType::Distribution,

            "Doughnut" => BusinessType::Doughnut,

            "Drugstore" => BusinessType::Drugstore,

            "Dry Cleaner" => BusinessType::DryCleaner,

            "Education/School" => BusinessType::EducationSchool,

            "Electronics" => BusinessType::Electronics,

            "Employment" => BusinessType::Employment,

            "Farm" => BusinessType::Farm,

            "Fast Food" => BusinessType::FastFood,

            "Financial" => BusinessType::Financial,

            "Fitness" => BusinessType::Fitness,

            "Florist/Nursery" => BusinessType::FloristNursery,

            "Food & Beverage" => BusinessType::FoodBeverage,

            "Forest Reserve" => BusinessType::ForestReserve,

            "Franchise" => BusinessType::Franchise,

            "Furniture" => BusinessType::Furniture,

            "Gas Station" => BusinessType::GasStation,

            "Gift Shop" => BusinessType::GiftShop,

            "Grocery" => BusinessType::Grocery,

            "Hardware" => BusinessType::Hardware,

            "Health Food" => BusinessType::HealthFood,

            "Health Services" => BusinessType::HealthServices,

            "Hobby" => BusinessType::Hobby,

            "Home Cleaner" => BusinessType::HomeCleaner,

            "Hospitality" => BusinessType::Hospitality,

            "Hotel/Motel" => BusinessType::HotelMotel,

            "Ice Cream/Frozen Yogurt" => BusinessType::IceCreamFrozenYogurt,

            "Industrial" => BusinessType::Industrial,

            "Jewelry" => BusinessType::Jewelry,

            "Landscaping" => BusinessType::Landscaping,

            "Laundromat" => BusinessType::Laundromat,

            "Liquor Store" => BusinessType::LiquorStore,

            "Locksmith" => BusinessType::Locksmith,

            "Manufacturing" => BusinessType::Manufacturing,

            "Medical" => BusinessType::Medical,

            "Mixed" => BusinessType::Mixed,

            "Mobile/Trailer Park" => BusinessType::MobileTrailerPark,

            "Music" => BusinessType::Music,

            "Nursing Home" => BusinessType::NursingHome,

            "Office Supply" => BusinessType::OfficeSupply,

            "Other" => BusinessType::Other,

            "Paints" => BusinessType::Paints,

            "Parking" => BusinessType::Parking,

            "Pet Store" => BusinessType::PetStore,

            "Photographer" => BusinessType::Photographer,

            "Pizza" => BusinessType::Pizza,

            "Printing" => BusinessType::Printing,

            "Professional Service" => BusinessType::ProfessionalService,

            "Professional/Office" => BusinessType::ProfessionalOffice,

            "Real Estate" => BusinessType::RealEstate,

            "Recreation" => BusinessType::Recreation,

            "Rental" => BusinessType::Rental,

            "Residential" => BusinessType::Residential,

            "Restaurant" => BusinessType::Restaurant,

            "Retail" => BusinessType::Retail,

            "Saddlery/Harness" => BusinessType::SaddleryHarness,

            "Sporting Goods" => BusinessType::SportingGoods,

            "Storage" => BusinessType::Storage,

            "Toys" => BusinessType::Toys,

            "Transportation" => BusinessType::Transportation,

            "Travel" => BusinessType::Travel,

            "Upholstery" => BusinessType::Upholstery,

            "Utility" => BusinessType::Utility,

            "Variety" => BusinessType::Variety,

            "Video" => BusinessType::Video,

            "Wallpaper" => BusinessType::Wallpaper,

            "Warehouse" => BusinessType::Warehouse,

            "Wholesale" => BusinessType::Wholesale,

            _ => BusinessType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for BusinessType {
    fn from(s: &str) -> BusinessType {
        match s {
            "Accounting" => BusinessType::Accounting,

            "Administrative and Support" => BusinessType::AdministrativeandSupport,

            "Advertising" => BusinessType::Advertising,

            "Agriculture" => BusinessType::Agriculture,

            "Animal Grooming" => BusinessType::AnimalGrooming,

            "Appliances" => BusinessType::Appliances,

            "Aquarium Supplies" => BusinessType::AquariumSupplies,

            "Arts and Entertainment" => BusinessType::ArtsandEntertainment,

            "Athletic" => BusinessType::Athletic,

            "Auto Body" => BusinessType::AutoBody,

            "Auto Dealer" => BusinessType::AutoDealer,

            "Auto Glass" => BusinessType::AutoGlass,

            "Auto Parts" => BusinessType::AutoParts,

            "Auto Rent/Lease" => BusinessType::AutoRentLease,

            "Auto Repair-Specialty" => BusinessType::AutoRepairSpecialty,

            "Auto Service" => BusinessType::AutoService,

            "Auto Stereo/Alarm" => BusinessType::AutoStereoAlarm,

            "Auto Tires" => BusinessType::AutoTires,

            "Auto Wrecking" => BusinessType::AutoWrecking,

            "Bakery" => BusinessType::Bakery,

            "Bar/Tavern/Lounge" => BusinessType::BarTavernLounge,

            "Barber/Beauty" => BusinessType::BarberBeauty,

            "Bed & Breakfast" => BusinessType::BedBreakfast,

            "Books/Cards/Stationary" => BusinessType::BooksCardsStationary,

            "Butcher" => BusinessType::Butcher,

            "Cabinets" => BusinessType::Cabinets,

            "Candy/Cookie" => BusinessType::CandyCookie,

            "Car Wash" => BusinessType::CarWash,

            "Carpet/Tile" => BusinessType::CarpetTile,

            "Child Care" => BusinessType::ChildCare,

            "Church" => BusinessType::Church,

            "Clothing" => BusinessType::Clothing,

            "Commercial" => BusinessType::Commercial,

            "Computer" => BusinessType::Computer,

            "Construction/Contractor" => BusinessType::ConstructionContractor,

            "Convalescent" => BusinessType::Convalescent,

            "Convenience Store" => BusinessType::ConvenienceStore,

            "Dance Studio" => BusinessType::DanceStudio,

            "Decorator" => BusinessType::Decorator,

            "Deli/Catering" => BusinessType::DeliCatering,

            "Dental" => BusinessType::Dental,

            "Distribution" => BusinessType::Distribution,

            "Doughnut" => BusinessType::Doughnut,

            "Drugstore" => BusinessType::Drugstore,

            "Dry Cleaner" => BusinessType::DryCleaner,

            "Education/School" => BusinessType::EducationSchool,

            "Electronics" => BusinessType::Electronics,

            "Employment" => BusinessType::Employment,

            "Farm" => BusinessType::Farm,

            "Fast Food" => BusinessType::FastFood,

            "Financial" => BusinessType::Financial,

            "Fitness" => BusinessType::Fitness,

            "Florist/Nursery" => BusinessType::FloristNursery,

            "Food & Beverage" => BusinessType::FoodBeverage,

            "Forest Reserve" => BusinessType::ForestReserve,

            "Franchise" => BusinessType::Franchise,

            "Furniture" => BusinessType::Furniture,

            "Gas Station" => BusinessType::GasStation,

            "Gift Shop" => BusinessType::GiftShop,

            "Grocery" => BusinessType::Grocery,

            "Hardware" => BusinessType::Hardware,

            "Health Food" => BusinessType::HealthFood,

            "Health Services" => BusinessType::HealthServices,

            "Hobby" => BusinessType::Hobby,

            "Home Cleaner" => BusinessType::HomeCleaner,

            "Hospitality" => BusinessType::Hospitality,

            "Hotel/Motel" => BusinessType::HotelMotel,

            "Ice Cream/Frozen Yogurt" => BusinessType::IceCreamFrozenYogurt,

            "Industrial" => BusinessType::Industrial,

            "Jewelry" => BusinessType::Jewelry,

            "Landscaping" => BusinessType::Landscaping,

            "Laundromat" => BusinessType::Laundromat,

            "Liquor Store" => BusinessType::LiquorStore,

            "Locksmith" => BusinessType::Locksmith,

            "Manufacturing" => BusinessType::Manufacturing,

            "Medical" => BusinessType::Medical,

            "Mixed" => BusinessType::Mixed,

            "Mobile/Trailer Park" => BusinessType::MobileTrailerPark,

            "Music" => BusinessType::Music,

            "Nursing Home" => BusinessType::NursingHome,

            "Office Supply" => BusinessType::OfficeSupply,

            "Other" => BusinessType::Other,

            "Paints" => BusinessType::Paints,

            "Parking" => BusinessType::Parking,

            "Pet Store" => BusinessType::PetStore,

            "Photographer" => BusinessType::Photographer,

            "Pizza" => BusinessType::Pizza,

            "Printing" => BusinessType::Printing,

            "Professional Service" => BusinessType::ProfessionalService,

            "Professional/Office" => BusinessType::ProfessionalOffice,

            "Real Estate" => BusinessType::RealEstate,

            "Recreation" => BusinessType::Recreation,

            "Rental" => BusinessType::Rental,

            "Residential" => BusinessType::Residential,

            "Restaurant" => BusinessType::Restaurant,

            "Retail" => BusinessType::Retail,

            "Saddlery/Harness" => BusinessType::SaddleryHarness,

            "Sporting Goods" => BusinessType::SportingGoods,

            "Storage" => BusinessType::Storage,

            "Toys" => BusinessType::Toys,

            "Transportation" => BusinessType::Transportation,

            "Travel" => BusinessType::Travel,

            "Upholstery" => BusinessType::Upholstery,

            "Utility" => BusinessType::Utility,

            "Variety" => BusinessType::Variety,

            "Video" => BusinessType::Video,

            "Wallpaper" => BusinessType::Wallpaper,

            "Warehouse" => BusinessType::Warehouse,

            "Wholesale" => BusinessType::Wholesale,

            _ => BusinessType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a BusinessType> for &'a str {
    fn from(s: &'a BusinessType) -> &'a str {
        match s {
            BusinessType::Accounting => "Accounting",

            BusinessType::AdministrativeandSupport => "Administrative and Support",

            BusinessType::Advertising => "Advertising",

            BusinessType::Agriculture => "Agriculture",

            BusinessType::AnimalGrooming => "Animal Grooming",

            BusinessType::Appliances => "Appliances",

            BusinessType::AquariumSupplies => "Aquarium Supplies",

            BusinessType::ArtsandEntertainment => "Arts and Entertainment",

            BusinessType::Athletic => "Athletic",

            BusinessType::AutoBody => "Auto Body",

            BusinessType::AutoDealer => "Auto Dealer",

            BusinessType::AutoGlass => "Auto Glass",

            BusinessType::AutoParts => "Auto Parts",

            BusinessType::AutoRentLease => "Auto Rent/Lease",

            BusinessType::AutoRepairSpecialty => "Auto Repair-Specialty",

            BusinessType::AutoService => "Auto Service",

            BusinessType::AutoStereoAlarm => "Auto Stereo/Alarm",

            BusinessType::AutoTires => "Auto Tires",

            BusinessType::AutoWrecking => "Auto Wrecking",

            BusinessType::Bakery => "Bakery",

            BusinessType::BarTavernLounge => "Bar/Tavern/Lounge",

            BusinessType::BarberBeauty => "Barber/Beauty",

            BusinessType::BedBreakfast => "Bed & Breakfast",

            BusinessType::BooksCardsStationary => "Books/Cards/Stationary",

            BusinessType::Butcher => "Butcher",

            BusinessType::Cabinets => "Cabinets",

            BusinessType::CandyCookie => "Candy/Cookie",

            BusinessType::CarWash => "Car Wash",

            BusinessType::CarpetTile => "Carpet/Tile",

            BusinessType::ChildCare => "Child Care",

            BusinessType::Church => "Church",

            BusinessType::Clothing => "Clothing",

            BusinessType::Commercial => "Commercial",

            BusinessType::Computer => "Computer",

            BusinessType::ConstructionContractor => "Construction/Contractor",

            BusinessType::Convalescent => "Convalescent",

            BusinessType::ConvenienceStore => "Convenience Store",

            BusinessType::DanceStudio => "Dance Studio",

            BusinessType::Decorator => "Decorator",

            BusinessType::DeliCatering => "Deli/Catering",

            BusinessType::Dental => "Dental",

            BusinessType::Distribution => "Distribution",

            BusinessType::Doughnut => "Doughnut",

            BusinessType::Drugstore => "Drugstore",

            BusinessType::DryCleaner => "Dry Cleaner",

            BusinessType::EducationSchool => "Education/School",

            BusinessType::Electronics => "Electronics",

            BusinessType::Employment => "Employment",

            BusinessType::Farm => "Farm",

            BusinessType::FastFood => "Fast Food",

            BusinessType::Financial => "Financial",

            BusinessType::Fitness => "Fitness",

            BusinessType::FloristNursery => "Florist/Nursery",

            BusinessType::FoodBeverage => "Food & Beverage",

            BusinessType::ForestReserve => "Forest Reserve",

            BusinessType::Franchise => "Franchise",

            BusinessType::Furniture => "Furniture",

            BusinessType::GasStation => "Gas Station",

            BusinessType::GiftShop => "Gift Shop",

            BusinessType::Grocery => "Grocery",

            BusinessType::Hardware => "Hardware",

            BusinessType::HealthFood => "Health Food",

            BusinessType::HealthServices => "Health Services",

            BusinessType::Hobby => "Hobby",

            BusinessType::HomeCleaner => "Home Cleaner",

            BusinessType::Hospitality => "Hospitality",

            BusinessType::HotelMotel => "Hotel/Motel",

            BusinessType::IceCreamFrozenYogurt => "Ice Cream/Frozen Yogurt",

            BusinessType::Industrial => "Industrial",

            BusinessType::Jewelry => "Jewelry",

            BusinessType::Landscaping => "Landscaping",

            BusinessType::Laundromat => "Laundromat",

            BusinessType::LiquorStore => "Liquor Store",

            BusinessType::Locksmith => "Locksmith",

            BusinessType::Manufacturing => "Manufacturing",

            BusinessType::Medical => "Medical",

            BusinessType::Mixed => "Mixed",

            BusinessType::MobileTrailerPark => "Mobile/Trailer Park",

            BusinessType::Music => "Music",

            BusinessType::NursingHome => "Nursing Home",

            BusinessType::OfficeSupply => "Office Supply",

            BusinessType::Other => "Other",

            BusinessType::Paints => "Paints",

            BusinessType::Parking => "Parking",

            BusinessType::PetStore => "Pet Store",

            BusinessType::Photographer => "Photographer",

            BusinessType::Pizza => "Pizza",

            BusinessType::Printing => "Printing",

            BusinessType::ProfessionalService => "Professional Service",

            BusinessType::ProfessionalOffice => "Professional/Office",

            BusinessType::RealEstate => "Real Estate",

            BusinessType::Recreation => "Recreation",

            BusinessType::Rental => "Rental",

            BusinessType::Residential => "Residential",

            BusinessType::Restaurant => "Restaurant",

            BusinessType::Retail => "Retail",

            BusinessType::SaddleryHarness => "Saddlery/Harness",

            BusinessType::SportingGoods => "Sporting Goods",

            BusinessType::Storage => "Storage",

            BusinessType::Toys => "Toys",

            BusinessType::Transportation => "Transportation",

            BusinessType::Travel => "Travel",

            BusinessType::Upholstery => "Upholstery",

            BusinessType::Utility => "Utility",

            BusinessType::Variety => "Variety",

            BusinessType::Video => "Video",

            BusinessType::Wallpaper => "Wallpaper",

            BusinessType::Warehouse => "Warehouse",

            BusinessType::Wholesale => "Wholesale",

            BusinessType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for BusinessType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for BusinessType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_business_type_format {
    use super::BusinessType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<BusinessType>>,
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
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Vec<BusinessType>>, D::Error>
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
