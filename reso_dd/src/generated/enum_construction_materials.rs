// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ConstructionMaterials Lookups](https://ddwiki.reso.org/display/DDW17/ConstructionMaterials+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConstructionMaterials {
    /// "[Adobe](https://ddwiki.reso.org/display/DDW17/Adobe)": The structure was made wholly or partly with adobe.
    Adobe,

    /// "[Aluminum Siding](https://ddwiki.reso.org/display/DDW17/Aluminum+Siding)": The structure was made wholly or partly with aluminum siding.
    AluminumSiding,

    /// "[Asbestos](https://ddwiki.reso.org/display/DDW17/Asbestos)": The structure was made wholly or partly with asbestos.
    Asbestos,

    /// "[Asphalt](https://ddwiki.reso.org/display/DDW17/Asphalt)": The structure was made wholly or partly with asphalt.
    Asphalt,

    /// "[Attic/Crawl Hatchway(s) Insulated](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244076)": When not insulated, a home’s attic hatch or crawlspace hatch creates one of the biggest gaps in the building envelope, increasing heat loss in winter and heat gain in summer, and making indoor living areas uncomfortable. Insulation standards are clearly defined: <a href="http://www.ornl.gov/sci/roofs+walls/insulation/fact">http://www.ornl.gov/sci/roofs+walls/insulation/fact</a>
    AtticCrawlHatchwaysInsulated,

    /// "[Batts Insulation](https://ddwiki.reso.org/display/DDW17/Batts+Insulation)": Rolls and batts, or blankets, are flexible products made from mineral fibers such as fiberglass and rock wool. Can also be made of cotton and wool. They are available in widths suited to standard spacing of wall studs and attic or floor joists.
    BattsInsulation,

    /// "[Block](https://ddwiki.reso.org/display/DDW17/Block)": The structure was made wholly or partly with block.
    Block,

    /// "[Blown-In Insulation](https://ddwiki.reso.org/display/DDW17/Blown-In+Insulation)": Blown-in or loose-fill insulation is usually made of fiberglass, rock wool, or cellulose in the form of loose fibers or fiber pellets installed using special pneumatic equipment. The blown-in material conforms readily to odd-sized building cavities and attics with wires, ducts, and pipes, making it well suited for places where it is difficult to effectively install other types of insulation.
    BlownInInsulation,

    /// "[Board & Batten Siding](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244080)": The structure was made wholly or partly with board & batten siding.
    BoardBattenSiding,

    /// "[Brick](https://ddwiki.reso.org/display/DDW17/Brick)": The structure was made wholly or partly with brick.
    Brick,

    /// "[Brick Veneer](https://ddwiki.reso.org/display/DDW17/Brick+Veneer)": The structure was made wholly or partly with brick veneer.
    BrickVeneer,

    /// "[Cedar](https://ddwiki.reso.org/display/DDW17/Cedar)": The structure was made wholly or partly with cedar.
    Cedar,

    /// "[Cement Siding](https://ddwiki.reso.org/display/DDW17/Cement+Siding)": The structure was made wholly or partly with cement siding.
    CementSiding,

    /// "[Clapboard](https://ddwiki.reso.org/display/DDW17/Clapboard)": The structure was made wholly or partly with clapboard.
    Clapboard,

    /// "[Concrete](https://ddwiki.reso.org/display/DDW17/Concrete)": The structure was made wholly or partly with concrete.
    Concrete,

    /// "[Ducts Professionally Air-Sealed](https://ddwiki.reso.org/display/DDW17/Ducts+Professionally+Air-Sealed)": The structure was made wholly or partly with ducts professionally air-sealed.
    DuctsProfessionallyAirSealed,

    /// "[Exterior Duct-Work is Insulated](https://ddwiki.reso.org/display/DDW17/Exterior+Duct-Work+is+Insulated)": DOE estimates that heating and cooling ducts located in unconditioned spaces such as attics and garages can underperform by 60–75 percent. Exterior ducts that are properly insulated can save energy and reduce overall equipment sizing. Insulation standards are clearly defined: <a name="m_-4439061441316115113__MailEndCompose"></a><a href="https://www.energy.gov/energysaver/insulation">https://www.energy.gov/energysaver/insulation</a>
    ExteriorDuctWorkisInsulated,

    /// "[Fiber Cement](https://ddwiki.reso.org/display/DDW17/Fiber+Cement)": The structure was made wholly or partly with fiber cement.
    FiberCement,

    /// "[Fiberglass Siding](https://ddwiki.reso.org/display/DDW17/Fiberglass+Siding)": The structure was made wholly or partly with fiberglass siding.
    FiberglassSiding,

    /// "[Foam Insulation](https://ddwiki.reso.org/display/DDW17/Foam+Insulation)": Spray foam or foam-in-place insulation can be sprayed into walls, on attic surfaces, or under floors to insulate and reduce air leakage. There are two types of foam-in-place insulation: closed-cell and open-cell. Both are typically made with polyurethane. Closed-cell foam has a greater insulation value and provides stronger resistance against moisture and air leakage. Open-cell foam is lighter and less expensive but should not be used below ground level where it could absorb water.
    FoamInsulation,

    /// "[Frame](https://ddwiki.reso.org/display/DDW17/Frame)": The structure was made wholly or partly with frame.
    Frame,

    /// "[Glass](https://ddwiki.reso.org/display/DDW17/Glass)": The structure was made wholly or partly with glass.
    Glass,

    /// "[HardiPlank Type](https://ddwiki.reso.org/display/DDW17/HardiPlank+Type)": The structure was made wholly or partly with HardiPlank type.
    HardiPlankType,

    /// "[ICAT Recessed Lighting](https://ddwiki.reso.org/display/DDW17/ICAT+Recessed+Lighting)": ICAT recessed light fixtures are rated both to safely come in contact with insulation and are better airsealed. ICAT is an acronym for Insulation Contact/AirTight. They can be installed safely with insulation and air sealing. These lights are different from IC (Insulation Contact) fixtures, which are not very airtight. They can also be identified by the wording “Washington State Approved.” Documentation on the installation is recommended because ICAT rating often requires multiple components be used as specified by the manufacturer. Substitutions of components can negate the rating. See: <a href="http://energy.gov/energysaver/articles/tips-lighting">http://energy.gov/energysaver/articles/tips-lighting</a> and <a href="http://energy.gov/energysaver/articles/tips-insulation">http://energy.gov/energysaver/articles/tips-insulation</a>
    ICATRecessedLighting,

    /// "[ICFs (Insulated Concrete Forms)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244096)": The structure was made wholly or partly with insulated concrete forms.
    ICFsInsulatedConcreteForms,

    /// "[Lap Siding](https://ddwiki.reso.org/display/DDW17/Lap+Siding)": The structure was made wholly or partly with lap siding.
    LapSiding,

    /// "[Log](https://ddwiki.reso.org/display/DDW17/Log)": The structure was made wholly or partly with log.
    Log,

    /// "[Log Siding](https://ddwiki.reso.org/display/DDW17/Log+Siding)": The structure was made wholly or partly with log siding.
    LogSiding,

    /// "[Low VOC Insulation](https://ddwiki.reso.org/display/DDW17/Low+VOC+Insulation)": Volatile organic compounds (VOCs) are emitted as gases from certain solids or liquids. Different types of insulation can be certified for having low VOC content by third-party verifiers such as GreenGuard.
    LowVOCInsulation,

    /// "[Masonite](https://ddwiki.reso.org/display/DDW17/Masonite)": The structure was made wholly or partly with Masonite.
    Masonite,

    /// "[Metal Siding](https://ddwiki.reso.org/display/DDW17/Metal+Siding)": The structure was made wholly or partly with metal siding.
    MetalSiding,

    /// "[Natural Building](https://ddwiki.reso.org/display/DDW17/Natural+Building)": The structure was made wholly or partly with natural building.
    NaturalBuilding,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244104)": The structure was made wholly or partly with other.
    Other,

    /// "[Plaster](https://ddwiki.reso.org/display/DDW17/Plaster)": The structure was made wholly or partly with plaster.
    Plaster,

    /// "[Radiant Barrier](https://ddwiki.reso.org/display/DDW17/Radiant+Barrier)": The structure was made wholly or partly with radiant barrier.
    RadiantBarrier,

    /// "[Rammed Earth](https://ddwiki.reso.org/display/DDW17/Rammed+Earth)": The structure was made wholly or partly with rammed earth.
    RammedEarth,

    /// "[Recycled/Bio-Based Insulation](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244108)": Insulation can be made from natural or recycled materials ranging from paper to soy to denim, using sustainable materials to improve energy efficiency.
    RecycledBioBasedInsulation,

    /// "[Redwood Siding](https://ddwiki.reso.org/display/DDW17/Redwood+Siding)": The structure was made wholly or partly with redwood siding.
    RedwoodSiding,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244110)": The structure was made wholly or partly with see remarks.
    SeeRemarks,

    /// "[Shake Siding](https://ddwiki.reso.org/display/DDW17/Shake+Siding)": The structure was made wholly or partly with shake siding.
    ShakeSiding,

    /// "[Shingle Siding](https://ddwiki.reso.org/display/DDW17/Shingle+Siding)": The structure was made wholly or partly with shingle siding.
    ShingleSiding,

    /// "[Slump Block](https://ddwiki.reso.org/display/DDW17/Slump+Block)": The structure was made wholly or partly with slump block.
    SlumpBlock,

    /// "[Spray Foam Insulation](https://ddwiki.reso.org/display/DDW17/Spray+Foam+Insulation)": The structure was made wholly or partly with spray foam insulation.
    SprayFoamInsulation,

    /// "[Steel Siding](https://ddwiki.reso.org/display/DDW17/Steel+Siding)": The structure was made wholly or partly with steel siding.
    SteelSiding,

    /// "[Stone](https://ddwiki.reso.org/display/DDW17/Stone)": The structure was made wholly or partly with stone.
    Stone,

    /// "[Stone Veneer](https://ddwiki.reso.org/display/DDW17/Stone+Veneer)": The structure was made wholly or partly with stone veneer.
    StoneVeneer,

    /// "[Straw](https://ddwiki.reso.org/display/DDW17/Straw)": The structure was made wholly or partly with straw.
    Straw,

    /// "[Stucco](https://ddwiki.reso.org/display/DDW17/Stucco)": The structure was made wholly or partly with stucco.
    Stucco,

    /// "[Synthetic Stucco](https://ddwiki.reso.org/display/DDW17/Synthetic+Stucco)": The structure was made wholly or partly with synthetic stucco.
    SyntheticStucco,

    /// "[Unknown](https://ddwiki.reso.org/display/DDW17/Unknown)": The structure was made wholly or partly with unknown.
    Unknown,

    /// "[Vertical Siding](https://ddwiki.reso.org/display/DDW17/Vertical+Siding)": The structure was made wholly or partly with vertical siding.
    VerticalSiding,

    /// "[Vinyl Siding](https://ddwiki.reso.org/display/DDW17/Vinyl+Siding)": The structure was made wholly or partly with vinyl siding.
    VinylSiding,

    /// "[Wood Siding](https://ddwiki.reso.org/display/DDW17/Wood+Siding)": The structure was made wholly or partly with wood siding.
    WoodSiding,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ConstructionMaterials {
    fn from_str(s: &str) -> ConstructionMaterials {
        match s {
            "Adobe" => ConstructionMaterials::Adobe,

            "Aluminum Siding" => ConstructionMaterials::AluminumSiding,

            "Asbestos" => ConstructionMaterials::Asbestos,

            "Asphalt" => ConstructionMaterials::Asphalt,

            "Attic/Crawl Hatchway(s) Insulated" => {
                ConstructionMaterials::AtticCrawlHatchwaysInsulated
            }

            "Batts Insulation" => ConstructionMaterials::BattsInsulation,

            "Block" => ConstructionMaterials::Block,

            "Blown-In Insulation" => ConstructionMaterials::BlownInInsulation,

            "Board & Batten Siding" => ConstructionMaterials::BoardBattenSiding,

            "Brick" => ConstructionMaterials::Brick,

            "Brick Veneer" => ConstructionMaterials::BrickVeneer,

            "Cedar" => ConstructionMaterials::Cedar,

            "Cement Siding" => ConstructionMaterials::CementSiding,

            "Clapboard" => ConstructionMaterials::Clapboard,

            "Concrete" => ConstructionMaterials::Concrete,

            "Ducts Professionally Air-Sealed" => {
                ConstructionMaterials::DuctsProfessionallyAirSealed
            }

            "Exterior Duct-Work is Insulated" => ConstructionMaterials::ExteriorDuctWorkisInsulated,

            "Fiber Cement" => ConstructionMaterials::FiberCement,

            "Fiberglass Siding" => ConstructionMaterials::FiberglassSiding,

            "Foam Insulation" => ConstructionMaterials::FoamInsulation,

            "Frame" => ConstructionMaterials::Frame,

            "Glass" => ConstructionMaterials::Glass,

            "HardiPlank Type" => ConstructionMaterials::HardiPlankType,

            "ICAT Recessed Lighting" => ConstructionMaterials::ICATRecessedLighting,

            "ICFs (Insulated Concrete Forms)" => ConstructionMaterials::ICFsInsulatedConcreteForms,

            "Lap Siding" => ConstructionMaterials::LapSiding,

            "Log" => ConstructionMaterials::Log,

            "Log Siding" => ConstructionMaterials::LogSiding,

            "Low VOC Insulation" => ConstructionMaterials::LowVOCInsulation,

            "Masonite" => ConstructionMaterials::Masonite,

            "Metal Siding" => ConstructionMaterials::MetalSiding,

            "Natural Building" => ConstructionMaterials::NaturalBuilding,

            "Other" => ConstructionMaterials::Other,

            "Plaster" => ConstructionMaterials::Plaster,

            "Radiant Barrier" => ConstructionMaterials::RadiantBarrier,

            "Rammed Earth" => ConstructionMaterials::RammedEarth,

            "Recycled/Bio-Based Insulation" => ConstructionMaterials::RecycledBioBasedInsulation,

            "Redwood Siding" => ConstructionMaterials::RedwoodSiding,

            "See Remarks" => ConstructionMaterials::SeeRemarks,

            "Shake Siding" => ConstructionMaterials::ShakeSiding,

            "Shingle Siding" => ConstructionMaterials::ShingleSiding,

            "Slump Block" => ConstructionMaterials::SlumpBlock,

            "Spray Foam Insulation" => ConstructionMaterials::SprayFoamInsulation,

            "Steel Siding" => ConstructionMaterials::SteelSiding,

            "Stone" => ConstructionMaterials::Stone,

            "Stone Veneer" => ConstructionMaterials::StoneVeneer,

            "Straw" => ConstructionMaterials::Straw,

            "Stucco" => ConstructionMaterials::Stucco,

            "Synthetic Stucco" => ConstructionMaterials::SyntheticStucco,

            "Unknown" => ConstructionMaterials::Unknown,

            "Vertical Siding" => ConstructionMaterials::VerticalSiding,

            "Vinyl Siding" => ConstructionMaterials::VinylSiding,

            "Wood Siding" => ConstructionMaterials::WoodSiding,

            _ => ConstructionMaterials::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ConstructionMaterials {
        match s.as_ref() {
            "Adobe" => ConstructionMaterials::Adobe,

            "Aluminum Siding" => ConstructionMaterials::AluminumSiding,

            "Asbestos" => ConstructionMaterials::Asbestos,

            "Asphalt" => ConstructionMaterials::Asphalt,

            "Attic/Crawl Hatchway(s) Insulated" => {
                ConstructionMaterials::AtticCrawlHatchwaysInsulated
            }

            "Batts Insulation" => ConstructionMaterials::BattsInsulation,

            "Block" => ConstructionMaterials::Block,

            "Blown-In Insulation" => ConstructionMaterials::BlownInInsulation,

            "Board & Batten Siding" => ConstructionMaterials::BoardBattenSiding,

            "Brick" => ConstructionMaterials::Brick,

            "Brick Veneer" => ConstructionMaterials::BrickVeneer,

            "Cedar" => ConstructionMaterials::Cedar,

            "Cement Siding" => ConstructionMaterials::CementSiding,

            "Clapboard" => ConstructionMaterials::Clapboard,

            "Concrete" => ConstructionMaterials::Concrete,

            "Ducts Professionally Air-Sealed" => {
                ConstructionMaterials::DuctsProfessionallyAirSealed
            }

            "Exterior Duct-Work is Insulated" => ConstructionMaterials::ExteriorDuctWorkisInsulated,

            "Fiber Cement" => ConstructionMaterials::FiberCement,

            "Fiberglass Siding" => ConstructionMaterials::FiberglassSiding,

            "Foam Insulation" => ConstructionMaterials::FoamInsulation,

            "Frame" => ConstructionMaterials::Frame,

            "Glass" => ConstructionMaterials::Glass,

            "HardiPlank Type" => ConstructionMaterials::HardiPlankType,

            "ICAT Recessed Lighting" => ConstructionMaterials::ICATRecessedLighting,

            "ICFs (Insulated Concrete Forms)" => ConstructionMaterials::ICFsInsulatedConcreteForms,

            "Lap Siding" => ConstructionMaterials::LapSiding,

            "Log" => ConstructionMaterials::Log,

            "Log Siding" => ConstructionMaterials::LogSiding,

            "Low VOC Insulation" => ConstructionMaterials::LowVOCInsulation,

            "Masonite" => ConstructionMaterials::Masonite,

            "Metal Siding" => ConstructionMaterials::MetalSiding,

            "Natural Building" => ConstructionMaterials::NaturalBuilding,

            "Other" => ConstructionMaterials::Other,

            "Plaster" => ConstructionMaterials::Plaster,

            "Radiant Barrier" => ConstructionMaterials::RadiantBarrier,

            "Rammed Earth" => ConstructionMaterials::RammedEarth,

            "Recycled/Bio-Based Insulation" => ConstructionMaterials::RecycledBioBasedInsulation,

            "Redwood Siding" => ConstructionMaterials::RedwoodSiding,

            "See Remarks" => ConstructionMaterials::SeeRemarks,

            "Shake Siding" => ConstructionMaterials::ShakeSiding,

            "Shingle Siding" => ConstructionMaterials::ShingleSiding,

            "Slump Block" => ConstructionMaterials::SlumpBlock,

            "Spray Foam Insulation" => ConstructionMaterials::SprayFoamInsulation,

            "Steel Siding" => ConstructionMaterials::SteelSiding,

            "Stone" => ConstructionMaterials::Stone,

            "Stone Veneer" => ConstructionMaterials::StoneVeneer,

            "Straw" => ConstructionMaterials::Straw,

            "Stucco" => ConstructionMaterials::Stucco,

            "Synthetic Stucco" => ConstructionMaterials::SyntheticStucco,

            "Unknown" => ConstructionMaterials::Unknown,

            "Vertical Siding" => ConstructionMaterials::VerticalSiding,

            "Vinyl Siding" => ConstructionMaterials::VinylSiding,

            "Wood Siding" => ConstructionMaterials::WoodSiding,

            _ => ConstructionMaterials::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ConstructionMaterials::Adobe => "Adobe",

            ConstructionMaterials::AluminumSiding => "Aluminum Siding",

            ConstructionMaterials::Asbestos => "Asbestos",

            ConstructionMaterials::Asphalt => "Asphalt",

            ConstructionMaterials::AtticCrawlHatchwaysInsulated => {
                "Attic/Crawl Hatchway(s) Insulated"
            }

            ConstructionMaterials::BattsInsulation => "Batts Insulation",

            ConstructionMaterials::Block => "Block",

            ConstructionMaterials::BlownInInsulation => "Blown-In Insulation",

            ConstructionMaterials::BoardBattenSiding => "Board & Batten Siding",

            ConstructionMaterials::Brick => "Brick",

            ConstructionMaterials::BrickVeneer => "Brick Veneer",

            ConstructionMaterials::Cedar => "Cedar",

            ConstructionMaterials::CementSiding => "Cement Siding",

            ConstructionMaterials::Clapboard => "Clapboard",

            ConstructionMaterials::Concrete => "Concrete",

            ConstructionMaterials::DuctsProfessionallyAirSealed => {
                "Ducts Professionally Air-Sealed"
            }

            ConstructionMaterials::ExteriorDuctWorkisInsulated => "Exterior Duct-Work is Insulated",

            ConstructionMaterials::FiberCement => "Fiber Cement",

            ConstructionMaterials::FiberglassSiding => "Fiberglass Siding",

            ConstructionMaterials::FoamInsulation => "Foam Insulation",

            ConstructionMaterials::Frame => "Frame",

            ConstructionMaterials::Glass => "Glass",

            ConstructionMaterials::HardiPlankType => "HardiPlank Type",

            ConstructionMaterials::ICATRecessedLighting => "ICAT Recessed Lighting",

            ConstructionMaterials::ICFsInsulatedConcreteForms => "ICFs (Insulated Concrete Forms)",

            ConstructionMaterials::LapSiding => "Lap Siding",

            ConstructionMaterials::Log => "Log",

            ConstructionMaterials::LogSiding => "Log Siding",

            ConstructionMaterials::LowVOCInsulation => "Low VOC Insulation",

            ConstructionMaterials::Masonite => "Masonite",

            ConstructionMaterials::MetalSiding => "Metal Siding",

            ConstructionMaterials::NaturalBuilding => "Natural Building",

            ConstructionMaterials::Other => "Other",

            ConstructionMaterials::Plaster => "Plaster",

            ConstructionMaterials::RadiantBarrier => "Radiant Barrier",

            ConstructionMaterials::RammedEarth => "Rammed Earth",

            ConstructionMaterials::RecycledBioBasedInsulation => "Recycled/Bio-Based Insulation",

            ConstructionMaterials::RedwoodSiding => "Redwood Siding",

            ConstructionMaterials::SeeRemarks => "See Remarks",

            ConstructionMaterials::ShakeSiding => "Shake Siding",

            ConstructionMaterials::ShingleSiding => "Shingle Siding",

            ConstructionMaterials::SlumpBlock => "Slump Block",

            ConstructionMaterials::SprayFoamInsulation => "Spray Foam Insulation",

            ConstructionMaterials::SteelSiding => "Steel Siding",

            ConstructionMaterials::Stone => "Stone",

            ConstructionMaterials::StoneVeneer => "Stone Veneer",

            ConstructionMaterials::Straw => "Straw",

            ConstructionMaterials::Stucco => "Stucco",

            ConstructionMaterials::SyntheticStucco => "Synthetic Stucco",

            ConstructionMaterials::Unknown => "Unknown",

            ConstructionMaterials::VerticalSiding => "Vertical Siding",

            ConstructionMaterials::VinylSiding => "Vinyl Siding",

            ConstructionMaterials::WoodSiding => "Wood Siding",

            ConstructionMaterials::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ConstructionMaterials::Adobe => "Adobe".into(),

            ConstructionMaterials::AluminumSiding => "Aluminum Siding".into(),

            ConstructionMaterials::Asbestos => "Asbestos".into(),

            ConstructionMaterials::Asphalt => "Asphalt".into(),

            ConstructionMaterials::AtticCrawlHatchwaysInsulated => {
                "Attic/Crawl Hatchway(s) Insulated".into()
            }

            ConstructionMaterials::BattsInsulation => "Batts Insulation".into(),

            ConstructionMaterials::Block => "Block".into(),

            ConstructionMaterials::BlownInInsulation => "Blown-In Insulation".into(),

            ConstructionMaterials::BoardBattenSiding => "Board & Batten Siding".into(),

            ConstructionMaterials::Brick => "Brick".into(),

            ConstructionMaterials::BrickVeneer => "Brick Veneer".into(),

            ConstructionMaterials::Cedar => "Cedar".into(),

            ConstructionMaterials::CementSiding => "Cement Siding".into(),

            ConstructionMaterials::Clapboard => "Clapboard".into(),

            ConstructionMaterials::Concrete => "Concrete".into(),

            ConstructionMaterials::DuctsProfessionallyAirSealed => {
                "Ducts Professionally Air-Sealed".into()
            }

            ConstructionMaterials::ExteriorDuctWorkisInsulated => {
                "Exterior Duct-Work is Insulated".into()
            }

            ConstructionMaterials::FiberCement => "Fiber Cement".into(),

            ConstructionMaterials::FiberglassSiding => "Fiberglass Siding".into(),

            ConstructionMaterials::FoamInsulation => "Foam Insulation".into(),

            ConstructionMaterials::Frame => "Frame".into(),

            ConstructionMaterials::Glass => "Glass".into(),

            ConstructionMaterials::HardiPlankType => "HardiPlank Type".into(),

            ConstructionMaterials::ICATRecessedLighting => "ICAT Recessed Lighting".into(),

            ConstructionMaterials::ICFsInsulatedConcreteForms => {
                "ICFs (Insulated Concrete Forms)".into()
            }

            ConstructionMaterials::LapSiding => "Lap Siding".into(),

            ConstructionMaterials::Log => "Log".into(),

            ConstructionMaterials::LogSiding => "Log Siding".into(),

            ConstructionMaterials::LowVOCInsulation => "Low VOC Insulation".into(),

            ConstructionMaterials::Masonite => "Masonite".into(),

            ConstructionMaterials::MetalSiding => "Metal Siding".into(),

            ConstructionMaterials::NaturalBuilding => "Natural Building".into(),

            ConstructionMaterials::Other => "Other".into(),

            ConstructionMaterials::Plaster => "Plaster".into(),

            ConstructionMaterials::RadiantBarrier => "Radiant Barrier".into(),

            ConstructionMaterials::RammedEarth => "Rammed Earth".into(),

            ConstructionMaterials::RecycledBioBasedInsulation => {
                "Recycled/Bio-Based Insulation".into()
            }

            ConstructionMaterials::RedwoodSiding => "Redwood Siding".into(),

            ConstructionMaterials::SeeRemarks => "See Remarks".into(),

            ConstructionMaterials::ShakeSiding => "Shake Siding".into(),

            ConstructionMaterials::ShingleSiding => "Shingle Siding".into(),

            ConstructionMaterials::SlumpBlock => "Slump Block".into(),

            ConstructionMaterials::SprayFoamInsulation => "Spray Foam Insulation".into(),

            ConstructionMaterials::SteelSiding => "Steel Siding".into(),

            ConstructionMaterials::Stone => "Stone".into(),

            ConstructionMaterials::StoneVeneer => "Stone Veneer".into(),

            ConstructionMaterials::Straw => "Straw".into(),

            ConstructionMaterials::Stucco => "Stucco".into(),

            ConstructionMaterials::SyntheticStucco => "Synthetic Stucco".into(),

            ConstructionMaterials::Unknown => "Unknown".into(),

            ConstructionMaterials::VerticalSiding => "Vertical Siding".into(),

            ConstructionMaterials::VinylSiding => "Vinyl Siding".into(),

            ConstructionMaterials::WoodSiding => "Wood Siding".into(),

            ConstructionMaterials::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ConstructionMaterials::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ConstructionMaterials {
    fn from(s: String) -> ConstructionMaterials {
        match s.as_ref() {
            "Adobe" => ConstructionMaterials::Adobe,

            "Aluminum Siding" => ConstructionMaterials::AluminumSiding,

            "Asbestos" => ConstructionMaterials::Asbestos,

            "Asphalt" => ConstructionMaterials::Asphalt,

            "Attic/Crawl Hatchway(s) Insulated" => {
                ConstructionMaterials::AtticCrawlHatchwaysInsulated
            }

            "Batts Insulation" => ConstructionMaterials::BattsInsulation,

            "Block" => ConstructionMaterials::Block,

            "Blown-In Insulation" => ConstructionMaterials::BlownInInsulation,

            "Board & Batten Siding" => ConstructionMaterials::BoardBattenSiding,

            "Brick" => ConstructionMaterials::Brick,

            "Brick Veneer" => ConstructionMaterials::BrickVeneer,

            "Cedar" => ConstructionMaterials::Cedar,

            "Cement Siding" => ConstructionMaterials::CementSiding,

            "Clapboard" => ConstructionMaterials::Clapboard,

            "Concrete" => ConstructionMaterials::Concrete,

            "Ducts Professionally Air-Sealed" => {
                ConstructionMaterials::DuctsProfessionallyAirSealed
            }

            "Exterior Duct-Work is Insulated" => ConstructionMaterials::ExteriorDuctWorkisInsulated,

            "Fiber Cement" => ConstructionMaterials::FiberCement,

            "Fiberglass Siding" => ConstructionMaterials::FiberglassSiding,

            "Foam Insulation" => ConstructionMaterials::FoamInsulation,

            "Frame" => ConstructionMaterials::Frame,

            "Glass" => ConstructionMaterials::Glass,

            "HardiPlank Type" => ConstructionMaterials::HardiPlankType,

            "ICAT Recessed Lighting" => ConstructionMaterials::ICATRecessedLighting,

            "ICFs (Insulated Concrete Forms)" => ConstructionMaterials::ICFsInsulatedConcreteForms,

            "Lap Siding" => ConstructionMaterials::LapSiding,

            "Log" => ConstructionMaterials::Log,

            "Log Siding" => ConstructionMaterials::LogSiding,

            "Low VOC Insulation" => ConstructionMaterials::LowVOCInsulation,

            "Masonite" => ConstructionMaterials::Masonite,

            "Metal Siding" => ConstructionMaterials::MetalSiding,

            "Natural Building" => ConstructionMaterials::NaturalBuilding,

            "Other" => ConstructionMaterials::Other,

            "Plaster" => ConstructionMaterials::Plaster,

            "Radiant Barrier" => ConstructionMaterials::RadiantBarrier,

            "Rammed Earth" => ConstructionMaterials::RammedEarth,

            "Recycled/Bio-Based Insulation" => ConstructionMaterials::RecycledBioBasedInsulation,

            "Redwood Siding" => ConstructionMaterials::RedwoodSiding,

            "See Remarks" => ConstructionMaterials::SeeRemarks,

            "Shake Siding" => ConstructionMaterials::ShakeSiding,

            "Shingle Siding" => ConstructionMaterials::ShingleSiding,

            "Slump Block" => ConstructionMaterials::SlumpBlock,

            "Spray Foam Insulation" => ConstructionMaterials::SprayFoamInsulation,

            "Steel Siding" => ConstructionMaterials::SteelSiding,

            "Stone" => ConstructionMaterials::Stone,

            "Stone Veneer" => ConstructionMaterials::StoneVeneer,

            "Straw" => ConstructionMaterials::Straw,

            "Stucco" => ConstructionMaterials::Stucco,

            "Synthetic Stucco" => ConstructionMaterials::SyntheticStucco,

            "Unknown" => ConstructionMaterials::Unknown,

            "Vertical Siding" => ConstructionMaterials::VerticalSiding,

            "Vinyl Siding" => ConstructionMaterials::VinylSiding,

            "Wood Siding" => ConstructionMaterials::WoodSiding,

            _ => ConstructionMaterials::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ConstructionMaterials {
    fn from(s: &str) -> ConstructionMaterials {
        match s {
            "Adobe" => ConstructionMaterials::Adobe,

            "Aluminum Siding" => ConstructionMaterials::AluminumSiding,

            "Asbestos" => ConstructionMaterials::Asbestos,

            "Asphalt" => ConstructionMaterials::Asphalt,

            "Attic/Crawl Hatchway(s) Insulated" => {
                ConstructionMaterials::AtticCrawlHatchwaysInsulated
            }

            "Batts Insulation" => ConstructionMaterials::BattsInsulation,

            "Block" => ConstructionMaterials::Block,

            "Blown-In Insulation" => ConstructionMaterials::BlownInInsulation,

            "Board & Batten Siding" => ConstructionMaterials::BoardBattenSiding,

            "Brick" => ConstructionMaterials::Brick,

            "Brick Veneer" => ConstructionMaterials::BrickVeneer,

            "Cedar" => ConstructionMaterials::Cedar,

            "Cement Siding" => ConstructionMaterials::CementSiding,

            "Clapboard" => ConstructionMaterials::Clapboard,

            "Concrete" => ConstructionMaterials::Concrete,

            "Ducts Professionally Air-Sealed" => {
                ConstructionMaterials::DuctsProfessionallyAirSealed
            }

            "Exterior Duct-Work is Insulated" => ConstructionMaterials::ExteriorDuctWorkisInsulated,

            "Fiber Cement" => ConstructionMaterials::FiberCement,

            "Fiberglass Siding" => ConstructionMaterials::FiberglassSiding,

            "Foam Insulation" => ConstructionMaterials::FoamInsulation,

            "Frame" => ConstructionMaterials::Frame,

            "Glass" => ConstructionMaterials::Glass,

            "HardiPlank Type" => ConstructionMaterials::HardiPlankType,

            "ICAT Recessed Lighting" => ConstructionMaterials::ICATRecessedLighting,

            "ICFs (Insulated Concrete Forms)" => ConstructionMaterials::ICFsInsulatedConcreteForms,

            "Lap Siding" => ConstructionMaterials::LapSiding,

            "Log" => ConstructionMaterials::Log,

            "Log Siding" => ConstructionMaterials::LogSiding,

            "Low VOC Insulation" => ConstructionMaterials::LowVOCInsulation,

            "Masonite" => ConstructionMaterials::Masonite,

            "Metal Siding" => ConstructionMaterials::MetalSiding,

            "Natural Building" => ConstructionMaterials::NaturalBuilding,

            "Other" => ConstructionMaterials::Other,

            "Plaster" => ConstructionMaterials::Plaster,

            "Radiant Barrier" => ConstructionMaterials::RadiantBarrier,

            "Rammed Earth" => ConstructionMaterials::RammedEarth,

            "Recycled/Bio-Based Insulation" => ConstructionMaterials::RecycledBioBasedInsulation,

            "Redwood Siding" => ConstructionMaterials::RedwoodSiding,

            "See Remarks" => ConstructionMaterials::SeeRemarks,

            "Shake Siding" => ConstructionMaterials::ShakeSiding,

            "Shingle Siding" => ConstructionMaterials::ShingleSiding,

            "Slump Block" => ConstructionMaterials::SlumpBlock,

            "Spray Foam Insulation" => ConstructionMaterials::SprayFoamInsulation,

            "Steel Siding" => ConstructionMaterials::SteelSiding,

            "Stone" => ConstructionMaterials::Stone,

            "Stone Veneer" => ConstructionMaterials::StoneVeneer,

            "Straw" => ConstructionMaterials::Straw,

            "Stucco" => ConstructionMaterials::Stucco,

            "Synthetic Stucco" => ConstructionMaterials::SyntheticStucco,

            "Unknown" => ConstructionMaterials::Unknown,

            "Vertical Siding" => ConstructionMaterials::VerticalSiding,

            "Vinyl Siding" => ConstructionMaterials::VinylSiding,

            "Wood Siding" => ConstructionMaterials::WoodSiding,

            _ => ConstructionMaterials::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ConstructionMaterials> for &'a str {
    fn from(s: &'a ConstructionMaterials) -> &'a str {
        match s {
            ConstructionMaterials::Adobe => "Adobe",

            ConstructionMaterials::AluminumSiding => "Aluminum Siding",

            ConstructionMaterials::Asbestos => "Asbestos",

            ConstructionMaterials::Asphalt => "Asphalt",

            ConstructionMaterials::AtticCrawlHatchwaysInsulated => {
                "Attic/Crawl Hatchway(s) Insulated"
            }

            ConstructionMaterials::BattsInsulation => "Batts Insulation",

            ConstructionMaterials::Block => "Block",

            ConstructionMaterials::BlownInInsulation => "Blown-In Insulation",

            ConstructionMaterials::BoardBattenSiding => "Board & Batten Siding",

            ConstructionMaterials::Brick => "Brick",

            ConstructionMaterials::BrickVeneer => "Brick Veneer",

            ConstructionMaterials::Cedar => "Cedar",

            ConstructionMaterials::CementSiding => "Cement Siding",

            ConstructionMaterials::Clapboard => "Clapboard",

            ConstructionMaterials::Concrete => "Concrete",

            ConstructionMaterials::DuctsProfessionallyAirSealed => {
                "Ducts Professionally Air-Sealed"
            }

            ConstructionMaterials::ExteriorDuctWorkisInsulated => "Exterior Duct-Work is Insulated",

            ConstructionMaterials::FiberCement => "Fiber Cement",

            ConstructionMaterials::FiberglassSiding => "Fiberglass Siding",

            ConstructionMaterials::FoamInsulation => "Foam Insulation",

            ConstructionMaterials::Frame => "Frame",

            ConstructionMaterials::Glass => "Glass",

            ConstructionMaterials::HardiPlankType => "HardiPlank Type",

            ConstructionMaterials::ICATRecessedLighting => "ICAT Recessed Lighting",

            ConstructionMaterials::ICFsInsulatedConcreteForms => "ICFs (Insulated Concrete Forms)",

            ConstructionMaterials::LapSiding => "Lap Siding",

            ConstructionMaterials::Log => "Log",

            ConstructionMaterials::LogSiding => "Log Siding",

            ConstructionMaterials::LowVOCInsulation => "Low VOC Insulation",

            ConstructionMaterials::Masonite => "Masonite",

            ConstructionMaterials::MetalSiding => "Metal Siding",

            ConstructionMaterials::NaturalBuilding => "Natural Building",

            ConstructionMaterials::Other => "Other",

            ConstructionMaterials::Plaster => "Plaster",

            ConstructionMaterials::RadiantBarrier => "Radiant Barrier",

            ConstructionMaterials::RammedEarth => "Rammed Earth",

            ConstructionMaterials::RecycledBioBasedInsulation => "Recycled/Bio-Based Insulation",

            ConstructionMaterials::RedwoodSiding => "Redwood Siding",

            ConstructionMaterials::SeeRemarks => "See Remarks",

            ConstructionMaterials::ShakeSiding => "Shake Siding",

            ConstructionMaterials::ShingleSiding => "Shingle Siding",

            ConstructionMaterials::SlumpBlock => "Slump Block",

            ConstructionMaterials::SprayFoamInsulation => "Spray Foam Insulation",

            ConstructionMaterials::SteelSiding => "Steel Siding",

            ConstructionMaterials::Stone => "Stone",

            ConstructionMaterials::StoneVeneer => "Stone Veneer",

            ConstructionMaterials::Straw => "Straw",

            ConstructionMaterials::Stucco => "Stucco",

            ConstructionMaterials::SyntheticStucco => "Synthetic Stucco",

            ConstructionMaterials::Unknown => "Unknown",

            ConstructionMaterials::VerticalSiding => "Vertical Siding",

            ConstructionMaterials::VinylSiding => "Vinyl Siding",

            ConstructionMaterials::WoodSiding => "Wood Siding",

            ConstructionMaterials::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ConstructionMaterials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ConstructionMaterials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
