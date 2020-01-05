// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [BuyerAgentDesignation Lookups](https://ddwiki.reso.org/display/DDW17/BuyerAgentDesignation+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BuyerAgentDesignation {
    /// "[Accredited Buyer's Representative / ABR](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243974)": The Accredited Buyer’s Representative (ABR®) designation is designed for real estate buyer agents who focus on working directly with buyer-clients.  http://www.rebac.net/abr
    AccreditedBuyersRepresentativeABR,

    /// "[Accredited Land Consultant / ALC](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243975)": Accredited Land Consultants (ALCs) are the most trusted, knowledgeable, experienced, and highest-producing experts in all segments of land. Conferred by the REALTORS® Land Institute, the designation requires successful completion of a rigorous LANDU education program, a specific, high-volume and experience level, and adherence to an honorable Code of Conduct. https://www.nar.realtor/designations-and-certifications/alc
    AccreditedLandConsultantALC,

    /// "[At Home With Diversity / AHWD](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243976)": Learn to work effectively with – and within – today’s diverse real estate market. The At Home With Diversity certification teaches you how to conduct your business with sensitivity to all client profiles and build a business plan to successfully serve them.  https://www.nar.realtor/designations-and-certifications/ahwd
    AtHomeWithDiversityAHWD,

    /// "[Certified Commercial Investment Member / CCIM](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243977)": The Certified Commercial Investment Member (CCIM) designation is commercial real estate’s global standard for professional achievement, earned through an extensive curriculum of 200 classroom hours and professional experiential requirements. https://www.nar.realtor/designations-and-certifications/ccim
    CertifiedCommercialInvestmentMemberCCIM,

    /// "[Certified Distressed Property Expert / CDPE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243978)": A Certified Distressed Property Expert® (CDPE) has a thorough understanding of complex issues in today’s turbulent real estate industry and knowledge of foreclosure avoidance options available to homeowners. CDPEs can provide solutions, specifically short sales, for homeowners facing market hardships.  http://www.cdpe.com/
    CertifiedDistressedPropertyExpertCDPE,

    /// "[Certified International Property Specialist / CIPS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243979)": The CIPS designation is for REALTORS® from the United States and abroad, as well as association staff and volunteer leaders who wish to develop or grow their international real estate business. It will provide you with the knowledge, research, network, and tools to globalize your business.                        https://www.nar.realtor/designations-and-certifications/cips-designation
    CertifiedInternationalPropertySpecialistCIPS,

    /// "[Certified Property Manager / CPM](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243980)": Certified Property Managers® (CPM®) are recognized as experts in real estate management, and they are at the top of the profession. 70% of those who hold the CPM® designation hold the highest management positions in their offices (owner/partner/officer/director). https://www.nar.realtor/designations-and-certifications/cpm
    CertifiedPropertyManagerCPM,

    /// "[Certified Real Estate Brokerage Manager / CRB](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243981)": The Certified Real Estate Brokerage Manager (CRB) Designation raises professional standards, strengthens individual and office performance, and indicates expertise in brokerage management. This designation represents the highest level of professional achievement in real estate brokerage management. You do not need a broker's license to earn the CRB Designation.  https://www.nar.realtor/designations-and-certifications/crb
    CertifiedRealEstateBrokerageManagerCRB,

    /// "[Certified Real Estate Team Specialist / C-RETS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243982)": The Certified Real Estate Team Specialist certification is designed to improve team development, individual leadership skills, and financial performance. The courses provide the tools, strategies, and knowledge that are required of today’s real estate professionals who are either considering or currently operating in a team environment. It is for team leaders, team members, those looking to start a team, and those who simply want to sharpen their management skills.  https://www.nar.realtor/designations-and-certifications/c-rets
    CertifiedRealEstateTeamSpecialistCRETS,

    /// "[Certified Residential Specialist / CRS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243983)": Certified Residential Specialist (CRS) is the highest credential awarded to residential sales agents, managers and brokers.  https://www.nar.realtor/designations-and-certifications/crs
    CertifiedResidentialSpecialistCRS,

    /// "[Counselor of Real Estate / CRE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243984)": The Counselors of Real Estate®  is an international group of recognized professionals who provide seasoned, expert, objective advice on real property and land-related matters. Only 1,100 practitioners throughout the world carry the CRE® designation. Membership is by invitation only.  https://www.nar.realtor/designations-and-certifications/cre
    CounselorofRealEstateCRE,

    /// "[e-PRO](https://ddwiki.reso.org/display/DDW17/e-PRO)": NAR's e-PRO® certification  teaches you to use cutting-edge technologies and digital initiatives to link up with today's savvy real estate consumer.  https://www.nar.realtor/designations-and-certifications/e-pro
    EPRO,

    /// "[General Accredited Appraiser / GAA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243986)": For general appraisers, this designation is awarded to those whose education and experience exceed state appraisal certification requirements and is supported by the National Association of REALTORS®. https://www.nar.realtor/designations-and-certifications/gaa
    GeneralAccreditedAppraiserGAA,

    /// "[Graduate, REALTOR Institute / GRI](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243987)": REALTORS® with the GRI designation have in-depth training in legal and regulatory issues, technology, professional standards, and the sales process. Earning the designation is a way to stand out to prospective buyers and sellers as a professional with expertise in these areas.  https://www.nar.realtor/designations-and-certifications/gri
    GraduateREALTORInstituteGRI,

    /// "[Military Relocation Professional / MRP](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243988)": NAR's Military Relocation Professional certification focuses on educating real estate professionals about working with current and former military service members to find housing solutions that best suit their needs and take full advantage of military benefits and support.  https://www.nar.realtor/designations-and-certifications/mrp
    MilitaryRelocationProfessionalMRP,

    /// "[NAR's Green Designation / GREEN](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243989)": Through NAR's Green Designation, the Green Resource Council provides ongoing education, resources and tools to help real estate practitioners find, understand, and market properties with green features.  https://www.nar.realtor/designations-and-certifications/green
    NARsGreenDesignationGREEN,

    /// "[Performance Management Network / PMN](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243990)": This designation is unique to the REALTOR® family designations, emphasizing that in order to enhance your business, you must enhance yourself. It focuses on negotiating strategies and tactics, networking and referrals, business planning and systems, personal performance management and  leadership development.  https://www.nar.realtor/designations-and-certifications/pmn
    PerformanceManagementNetworkPMN,

    /// "[Pricing Strategy Advisor / PSA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243991)": Enhance your skills in pricing properties, creating CMAs, working with appraisers, and guiding clients through the anxieties and misperceptions they often have about home values with NAR’s PSA (Pricing Strategy Advisor) certification. https://www.nar.realtor/designations-and-certifications/psa
    PricingStrategyAdvisorPSA,

    /// "[Real Estate Negotiation Expert / RENE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243992)": This certification is for real estate professionals who want to sharpen their negotiation skills. The RENE certification program gives REALTORS® the tips and tools they need to be skillful advocates for their clients.  https://www.nar.realtor/designations-and-certifications/rene
    RealEstateNegotiationExpertRENE,

    /// "[REALTOR Association Certified Executive / RCE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243993)": RCE is the only professional designation designed specifically for REALTOR® association executives. RCE designees exemplify goal-oriented AEs with drive, experience and commitment to professional growth.  https://www.nar.realtor/designations-and-certifications/rce
    REALTORAssociationCertifiedExecutiveRCE,

    /// "[Residential Accredited Appraiser / RAA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243994)": For residential appraisers, this designation is awarded to those whose education and experience exceed state appraisal certification requirements and is supported by the National Association of REALTORS®.  https://www.nar.realtor/designations-and-certifications/raa
    ResidentialAccreditedAppraiserRAA,

    /// "[Resort & Second-Home Property Specialist / RSPS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243995)": This certification is designed for REALTORS® who facilitate the buying, selling, or management of properties for investment, development, retirement, or second homes in a resort, recreational and/or vacation destination are involved in this market niche.  https://www.nar.realtor/designations-and-certifications/rsps
    ResortSecondHomePropertySpecialistRSPS,

    /// "[Seller Representative Specialist / SRS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243996)": The Seller Representative Specialist (SRS) designation is the premier credential in seller representation. It is designed to elevate professional standards and enhance personal performance. The designation is awarded to real estate practitioners by the Real Estate Business Institute (REBI) who meet specific educational and practical experience criteria.  https://www.nar.realtor/designations-and-certifications/seller-representative-specialist-srs
    SellerRepresentativeSpecialistSRS,

    /// "[Seniors Real Estate Specialist / SRES](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243997)": The SRES® Designation program educates REALTORS® on how to profitably and ethically serve the real estate needs of the fastest growing market in real estate, clients age 50+. By earning the SRES® designation, you gain access to valuable member benefits, useful resources, and networking opportunities across the U.S. and Canada to help you in your business.  https://www.nar.realtor/designations-and-certifications/sres
    SeniorsRealEstateSpecialistSRES,

    /// "[Short Sales & Foreclosure Resource / SFR](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243998)": The SFR® certification  teaches real estate professionals to work with distressed sellers and the finance, tax, and legal professionals who can help them, qualify sellers for short sales, develop a short sale package, negotiate with lenders, safeguard your commission, limit risk, and protect buyers.  https://www.nar.realtor/designations-and-certifications/sfr
    ShortSalesForeclosureResourceSFR,

    /// "[Society of Industrial and Office REALTORS / SIOR](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243999)": The SIOR designation is held by only the most knowledgeable, experienced, and successful commercial real estate brokerage specialists. To earn it, designees must meet standards of experience, production, education, ethics, and provide recommendations.  https://www.nar.realtor/designations-and-certifications/sior
    SocietyofIndustrialandOfficeREALTORSSIOR,

    /// "[Transnational Referral Certification / TRC](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244000)": Real estate professionals who have taken the Transnational Referral Certified (TRC) training, have completed special training on  making and receiving client referrals from professionals in other countries. https://worldproperties.com/about-us/international-referrals-and-trc/
    TransnationalReferralCertificationTRC,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for BuyerAgentDesignation {
    fn from_str(s: &str) -> BuyerAgentDesignation {
        match s {
            "Accredited Buyer's Representative / ABR" => {
                BuyerAgentDesignation::AccreditedBuyersRepresentativeABR
            }

            "Accredited Land Consultant / ALC" => {
                BuyerAgentDesignation::AccreditedLandConsultantALC
            }

            "At Home With Diversity / AHWD" => BuyerAgentDesignation::AtHomeWithDiversityAHWD,

            "Certified Commercial Investment Member / CCIM" => {
                BuyerAgentDesignation::CertifiedCommercialInvestmentMemberCCIM
            }

            "Certified Distressed Property Expert / CDPE" => {
                BuyerAgentDesignation::CertifiedDistressedPropertyExpertCDPE
            }

            "Certified International Property Specialist / CIPS" => {
                BuyerAgentDesignation::CertifiedInternationalPropertySpecialistCIPS
            }

            "Certified Property Manager / CPM" => {
                BuyerAgentDesignation::CertifiedPropertyManagerCPM
            }

            "Certified Real Estate Brokerage Manager / CRB" => {
                BuyerAgentDesignation::CertifiedRealEstateBrokerageManagerCRB
            }

            "Certified Real Estate Team Specialist / C-RETS" => {
                BuyerAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS
            }

            "Certified Residential Specialist / CRS" => {
                BuyerAgentDesignation::CertifiedResidentialSpecialistCRS
            }

            "Counselor of Real Estate / CRE" => BuyerAgentDesignation::CounselorofRealEstateCRE,

            "e-PRO" => BuyerAgentDesignation::EPRO,

            "General Accredited Appraiser / GAA" => {
                BuyerAgentDesignation::GeneralAccreditedAppraiserGAA
            }

            "Graduate, REALTOR Institute / GRI" => {
                BuyerAgentDesignation::GraduateREALTORInstituteGRI
            }

            "Military Relocation Professional / MRP" => {
                BuyerAgentDesignation::MilitaryRelocationProfessionalMRP
            }

            "NAR's Green Designation / GREEN" => BuyerAgentDesignation::NARsGreenDesignationGREEN,

            "Performance Management Network / PMN" => {
                BuyerAgentDesignation::PerformanceManagementNetworkPMN
            }

            "Pricing Strategy Advisor / PSA" => BuyerAgentDesignation::PricingStrategyAdvisorPSA,

            "Real Estate Negotiation Expert / RENE" => {
                BuyerAgentDesignation::RealEstateNegotiationExpertRENE
            }

            "REALTOR Association Certified Executive / RCE" => {
                BuyerAgentDesignation::REALTORAssociationCertifiedExecutiveRCE
            }

            "Residential Accredited Appraiser / RAA" => {
                BuyerAgentDesignation::ResidentialAccreditedAppraiserRAA
            }

            "Resort & Second-Home Property Specialist / RSPS" => {
                BuyerAgentDesignation::ResortSecondHomePropertySpecialistRSPS
            }

            "Seller Representative Specialist / SRS" => {
                BuyerAgentDesignation::SellerRepresentativeSpecialistSRS
            }

            "Seniors Real Estate Specialist / SRES" => {
                BuyerAgentDesignation::SeniorsRealEstateSpecialistSRES
            }

            "Short Sales & Foreclosure Resource / SFR" => {
                BuyerAgentDesignation::ShortSalesForeclosureResourceSFR
            }

            "Society of Industrial and Office REALTORS / SIOR" => {
                BuyerAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR
            }

            "Transnational Referral Certification / TRC" => {
                BuyerAgentDesignation::TransnationalReferralCertificationTRC
            }

            _ => BuyerAgentDesignation::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> BuyerAgentDesignation {
        match s.as_ref() {
            "Accredited Buyer's Representative / ABR" => {
                BuyerAgentDesignation::AccreditedBuyersRepresentativeABR
            }

            "Accredited Land Consultant / ALC" => {
                BuyerAgentDesignation::AccreditedLandConsultantALC
            }

            "At Home With Diversity / AHWD" => BuyerAgentDesignation::AtHomeWithDiversityAHWD,

            "Certified Commercial Investment Member / CCIM" => {
                BuyerAgentDesignation::CertifiedCommercialInvestmentMemberCCIM
            }

            "Certified Distressed Property Expert / CDPE" => {
                BuyerAgentDesignation::CertifiedDistressedPropertyExpertCDPE
            }

            "Certified International Property Specialist / CIPS" => {
                BuyerAgentDesignation::CertifiedInternationalPropertySpecialistCIPS
            }

            "Certified Property Manager / CPM" => {
                BuyerAgentDesignation::CertifiedPropertyManagerCPM
            }

            "Certified Real Estate Brokerage Manager / CRB" => {
                BuyerAgentDesignation::CertifiedRealEstateBrokerageManagerCRB
            }

            "Certified Real Estate Team Specialist / C-RETS" => {
                BuyerAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS
            }

            "Certified Residential Specialist / CRS" => {
                BuyerAgentDesignation::CertifiedResidentialSpecialistCRS
            }

            "Counselor of Real Estate / CRE" => BuyerAgentDesignation::CounselorofRealEstateCRE,

            "e-PRO" => BuyerAgentDesignation::EPRO,

            "General Accredited Appraiser / GAA" => {
                BuyerAgentDesignation::GeneralAccreditedAppraiserGAA
            }

            "Graduate, REALTOR Institute / GRI" => {
                BuyerAgentDesignation::GraduateREALTORInstituteGRI
            }

            "Military Relocation Professional / MRP" => {
                BuyerAgentDesignation::MilitaryRelocationProfessionalMRP
            }

            "NAR's Green Designation / GREEN" => BuyerAgentDesignation::NARsGreenDesignationGREEN,

            "Performance Management Network / PMN" => {
                BuyerAgentDesignation::PerformanceManagementNetworkPMN
            }

            "Pricing Strategy Advisor / PSA" => BuyerAgentDesignation::PricingStrategyAdvisorPSA,

            "Real Estate Negotiation Expert / RENE" => {
                BuyerAgentDesignation::RealEstateNegotiationExpertRENE
            }

            "REALTOR Association Certified Executive / RCE" => {
                BuyerAgentDesignation::REALTORAssociationCertifiedExecutiveRCE
            }

            "Residential Accredited Appraiser / RAA" => {
                BuyerAgentDesignation::ResidentialAccreditedAppraiserRAA
            }

            "Resort & Second-Home Property Specialist / RSPS" => {
                BuyerAgentDesignation::ResortSecondHomePropertySpecialistRSPS
            }

            "Seller Representative Specialist / SRS" => {
                BuyerAgentDesignation::SellerRepresentativeSpecialistSRS
            }

            "Seniors Real Estate Specialist / SRES" => {
                BuyerAgentDesignation::SeniorsRealEstateSpecialistSRES
            }

            "Short Sales & Foreclosure Resource / SFR" => {
                BuyerAgentDesignation::ShortSalesForeclosureResourceSFR
            }

            "Society of Industrial and Office REALTORS / SIOR" => {
                BuyerAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR
            }

            "Transnational Referral Certification / TRC" => {
                BuyerAgentDesignation::TransnationalReferralCertificationTRC
            }

            _ => BuyerAgentDesignation::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            BuyerAgentDesignation::AccreditedBuyersRepresentativeABR => {
                "Accredited Buyer's Representative / ABR"
            }

            BuyerAgentDesignation::AccreditedLandConsultantALC => {
                "Accredited Land Consultant / ALC"
            }

            BuyerAgentDesignation::AtHomeWithDiversityAHWD => "At Home With Diversity / AHWD",

            BuyerAgentDesignation::CertifiedCommercialInvestmentMemberCCIM => {
                "Certified Commercial Investment Member / CCIM"
            }

            BuyerAgentDesignation::CertifiedDistressedPropertyExpertCDPE => {
                "Certified Distressed Property Expert / CDPE"
            }

            BuyerAgentDesignation::CertifiedInternationalPropertySpecialistCIPS => {
                "Certified International Property Specialist / CIPS"
            }

            BuyerAgentDesignation::CertifiedPropertyManagerCPM => {
                "Certified Property Manager / CPM"
            }

            BuyerAgentDesignation::CertifiedRealEstateBrokerageManagerCRB => {
                "Certified Real Estate Brokerage Manager / CRB"
            }

            BuyerAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS => {
                "Certified Real Estate Team Specialist / C-RETS"
            }

            BuyerAgentDesignation::CertifiedResidentialSpecialistCRS => {
                "Certified Residential Specialist / CRS"
            }

            BuyerAgentDesignation::CounselorofRealEstateCRE => "Counselor of Real Estate / CRE",

            BuyerAgentDesignation::EPRO => "e-PRO",

            BuyerAgentDesignation::GeneralAccreditedAppraiserGAA => {
                "General Accredited Appraiser / GAA"
            }

            BuyerAgentDesignation::GraduateREALTORInstituteGRI => {
                "Graduate, REALTOR Institute / GRI"
            }

            BuyerAgentDesignation::MilitaryRelocationProfessionalMRP => {
                "Military Relocation Professional / MRP"
            }

            BuyerAgentDesignation::NARsGreenDesignationGREEN => "NAR's Green Designation / GREEN",

            BuyerAgentDesignation::PerformanceManagementNetworkPMN => {
                "Performance Management Network / PMN"
            }

            BuyerAgentDesignation::PricingStrategyAdvisorPSA => "Pricing Strategy Advisor / PSA",

            BuyerAgentDesignation::RealEstateNegotiationExpertRENE => {
                "Real Estate Negotiation Expert / RENE"
            }

            BuyerAgentDesignation::REALTORAssociationCertifiedExecutiveRCE => {
                "REALTOR Association Certified Executive / RCE"
            }

            BuyerAgentDesignation::ResidentialAccreditedAppraiserRAA => {
                "Residential Accredited Appraiser / RAA"
            }

            BuyerAgentDesignation::ResortSecondHomePropertySpecialistRSPS => {
                "Resort & Second-Home Property Specialist / RSPS"
            }

            BuyerAgentDesignation::SellerRepresentativeSpecialistSRS => {
                "Seller Representative Specialist / SRS"
            }

            BuyerAgentDesignation::SeniorsRealEstateSpecialistSRES => {
                "Seniors Real Estate Specialist / SRES"
            }

            BuyerAgentDesignation::ShortSalesForeclosureResourceSFR => {
                "Short Sales & Foreclosure Resource / SFR"
            }

            BuyerAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR => {
                "Society of Industrial and Office REALTORS / SIOR"
            }

            BuyerAgentDesignation::TransnationalReferralCertificationTRC => {
                "Transnational Referral Certification / TRC"
            }

            BuyerAgentDesignation::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            BuyerAgentDesignation::AccreditedBuyersRepresentativeABR => {
                "Accredited Buyer's Representative / ABR".into()
            }

            BuyerAgentDesignation::AccreditedLandConsultantALC => {
                "Accredited Land Consultant / ALC".into()
            }

            BuyerAgentDesignation::AtHomeWithDiversityAHWD => {
                "At Home With Diversity / AHWD".into()
            }

            BuyerAgentDesignation::CertifiedCommercialInvestmentMemberCCIM => {
                "Certified Commercial Investment Member / CCIM".into()
            }

            BuyerAgentDesignation::CertifiedDistressedPropertyExpertCDPE => {
                "Certified Distressed Property Expert / CDPE".into()
            }

            BuyerAgentDesignation::CertifiedInternationalPropertySpecialistCIPS => {
                "Certified International Property Specialist / CIPS".into()
            }

            BuyerAgentDesignation::CertifiedPropertyManagerCPM => {
                "Certified Property Manager / CPM".into()
            }

            BuyerAgentDesignation::CertifiedRealEstateBrokerageManagerCRB => {
                "Certified Real Estate Brokerage Manager / CRB".into()
            }

            BuyerAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS => {
                "Certified Real Estate Team Specialist / C-RETS".into()
            }

            BuyerAgentDesignation::CertifiedResidentialSpecialistCRS => {
                "Certified Residential Specialist / CRS".into()
            }

            BuyerAgentDesignation::CounselorofRealEstateCRE => {
                "Counselor of Real Estate / CRE".into()
            }

            BuyerAgentDesignation::EPRO => "e-PRO".into(),

            BuyerAgentDesignation::GeneralAccreditedAppraiserGAA => {
                "General Accredited Appraiser / GAA".into()
            }

            BuyerAgentDesignation::GraduateREALTORInstituteGRI => {
                "Graduate, REALTOR Institute / GRI".into()
            }

            BuyerAgentDesignation::MilitaryRelocationProfessionalMRP => {
                "Military Relocation Professional / MRP".into()
            }

            BuyerAgentDesignation::NARsGreenDesignationGREEN => {
                "NAR's Green Designation / GREEN".into()
            }

            BuyerAgentDesignation::PerformanceManagementNetworkPMN => {
                "Performance Management Network / PMN".into()
            }

            BuyerAgentDesignation::PricingStrategyAdvisorPSA => {
                "Pricing Strategy Advisor / PSA".into()
            }

            BuyerAgentDesignation::RealEstateNegotiationExpertRENE => {
                "Real Estate Negotiation Expert / RENE".into()
            }

            BuyerAgentDesignation::REALTORAssociationCertifiedExecutiveRCE => {
                "REALTOR Association Certified Executive / RCE".into()
            }

            BuyerAgentDesignation::ResidentialAccreditedAppraiserRAA => {
                "Residential Accredited Appraiser / RAA".into()
            }

            BuyerAgentDesignation::ResortSecondHomePropertySpecialistRSPS => {
                "Resort & Second-Home Property Specialist / RSPS".into()
            }

            BuyerAgentDesignation::SellerRepresentativeSpecialistSRS => {
                "Seller Representative Specialist / SRS".into()
            }

            BuyerAgentDesignation::SeniorsRealEstateSpecialistSRES => {
                "Seniors Real Estate Specialist / SRES".into()
            }

            BuyerAgentDesignation::ShortSalesForeclosureResourceSFR => {
                "Short Sales & Foreclosure Resource / SFR".into()
            }

            BuyerAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR => {
                "Society of Industrial and Office REALTORS / SIOR".into()
            }

            BuyerAgentDesignation::TransnationalReferralCertificationTRC => {
                "Transnational Referral Certification / TRC".into()
            }

            BuyerAgentDesignation::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            BuyerAgentDesignation::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for BuyerAgentDesignation {
    fn from(s: String) -> BuyerAgentDesignation {
        match s.as_ref() {
            "Accredited Buyer's Representative / ABR" => {
                BuyerAgentDesignation::AccreditedBuyersRepresentativeABR
            }

            "Accredited Land Consultant / ALC" => {
                BuyerAgentDesignation::AccreditedLandConsultantALC
            }

            "At Home With Diversity / AHWD" => BuyerAgentDesignation::AtHomeWithDiversityAHWD,

            "Certified Commercial Investment Member / CCIM" => {
                BuyerAgentDesignation::CertifiedCommercialInvestmentMemberCCIM
            }

            "Certified Distressed Property Expert / CDPE" => {
                BuyerAgentDesignation::CertifiedDistressedPropertyExpertCDPE
            }

            "Certified International Property Specialist / CIPS" => {
                BuyerAgentDesignation::CertifiedInternationalPropertySpecialistCIPS
            }

            "Certified Property Manager / CPM" => {
                BuyerAgentDesignation::CertifiedPropertyManagerCPM
            }

            "Certified Real Estate Brokerage Manager / CRB" => {
                BuyerAgentDesignation::CertifiedRealEstateBrokerageManagerCRB
            }

            "Certified Real Estate Team Specialist / C-RETS" => {
                BuyerAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS
            }

            "Certified Residential Specialist / CRS" => {
                BuyerAgentDesignation::CertifiedResidentialSpecialistCRS
            }

            "Counselor of Real Estate / CRE" => BuyerAgentDesignation::CounselorofRealEstateCRE,

            "e-PRO" => BuyerAgentDesignation::EPRO,

            "General Accredited Appraiser / GAA" => {
                BuyerAgentDesignation::GeneralAccreditedAppraiserGAA
            }

            "Graduate, REALTOR Institute / GRI" => {
                BuyerAgentDesignation::GraduateREALTORInstituteGRI
            }

            "Military Relocation Professional / MRP" => {
                BuyerAgentDesignation::MilitaryRelocationProfessionalMRP
            }

            "NAR's Green Designation / GREEN" => BuyerAgentDesignation::NARsGreenDesignationGREEN,

            "Performance Management Network / PMN" => {
                BuyerAgentDesignation::PerformanceManagementNetworkPMN
            }

            "Pricing Strategy Advisor / PSA" => BuyerAgentDesignation::PricingStrategyAdvisorPSA,

            "Real Estate Negotiation Expert / RENE" => {
                BuyerAgentDesignation::RealEstateNegotiationExpertRENE
            }

            "REALTOR Association Certified Executive / RCE" => {
                BuyerAgentDesignation::REALTORAssociationCertifiedExecutiveRCE
            }

            "Residential Accredited Appraiser / RAA" => {
                BuyerAgentDesignation::ResidentialAccreditedAppraiserRAA
            }

            "Resort & Second-Home Property Specialist / RSPS" => {
                BuyerAgentDesignation::ResortSecondHomePropertySpecialistRSPS
            }

            "Seller Representative Specialist / SRS" => {
                BuyerAgentDesignation::SellerRepresentativeSpecialistSRS
            }

            "Seniors Real Estate Specialist / SRES" => {
                BuyerAgentDesignation::SeniorsRealEstateSpecialistSRES
            }

            "Short Sales & Foreclosure Resource / SFR" => {
                BuyerAgentDesignation::ShortSalesForeclosureResourceSFR
            }

            "Society of Industrial and Office REALTORS / SIOR" => {
                BuyerAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR
            }

            "Transnational Referral Certification / TRC" => {
                BuyerAgentDesignation::TransnationalReferralCertificationTRC
            }

            _ => BuyerAgentDesignation::OpenEnumeration(s),
        }
    }
}

impl From<&str> for BuyerAgentDesignation {
    fn from(s: &str) -> BuyerAgentDesignation {
        match s {
            "Accredited Buyer's Representative / ABR" => {
                BuyerAgentDesignation::AccreditedBuyersRepresentativeABR
            }

            "Accredited Land Consultant / ALC" => {
                BuyerAgentDesignation::AccreditedLandConsultantALC
            }

            "At Home With Diversity / AHWD" => BuyerAgentDesignation::AtHomeWithDiversityAHWD,

            "Certified Commercial Investment Member / CCIM" => {
                BuyerAgentDesignation::CertifiedCommercialInvestmentMemberCCIM
            }

            "Certified Distressed Property Expert / CDPE" => {
                BuyerAgentDesignation::CertifiedDistressedPropertyExpertCDPE
            }

            "Certified International Property Specialist / CIPS" => {
                BuyerAgentDesignation::CertifiedInternationalPropertySpecialistCIPS
            }

            "Certified Property Manager / CPM" => {
                BuyerAgentDesignation::CertifiedPropertyManagerCPM
            }

            "Certified Real Estate Brokerage Manager / CRB" => {
                BuyerAgentDesignation::CertifiedRealEstateBrokerageManagerCRB
            }

            "Certified Real Estate Team Specialist / C-RETS" => {
                BuyerAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS
            }

            "Certified Residential Specialist / CRS" => {
                BuyerAgentDesignation::CertifiedResidentialSpecialistCRS
            }

            "Counselor of Real Estate / CRE" => BuyerAgentDesignation::CounselorofRealEstateCRE,

            "e-PRO" => BuyerAgentDesignation::EPRO,

            "General Accredited Appraiser / GAA" => {
                BuyerAgentDesignation::GeneralAccreditedAppraiserGAA
            }

            "Graduate, REALTOR Institute / GRI" => {
                BuyerAgentDesignation::GraduateREALTORInstituteGRI
            }

            "Military Relocation Professional / MRP" => {
                BuyerAgentDesignation::MilitaryRelocationProfessionalMRP
            }

            "NAR's Green Designation / GREEN" => BuyerAgentDesignation::NARsGreenDesignationGREEN,

            "Performance Management Network / PMN" => {
                BuyerAgentDesignation::PerformanceManagementNetworkPMN
            }

            "Pricing Strategy Advisor / PSA" => BuyerAgentDesignation::PricingStrategyAdvisorPSA,

            "Real Estate Negotiation Expert / RENE" => {
                BuyerAgentDesignation::RealEstateNegotiationExpertRENE
            }

            "REALTOR Association Certified Executive / RCE" => {
                BuyerAgentDesignation::REALTORAssociationCertifiedExecutiveRCE
            }

            "Residential Accredited Appraiser / RAA" => {
                BuyerAgentDesignation::ResidentialAccreditedAppraiserRAA
            }

            "Resort & Second-Home Property Specialist / RSPS" => {
                BuyerAgentDesignation::ResortSecondHomePropertySpecialistRSPS
            }

            "Seller Representative Specialist / SRS" => {
                BuyerAgentDesignation::SellerRepresentativeSpecialistSRS
            }

            "Seniors Real Estate Specialist / SRES" => {
                BuyerAgentDesignation::SeniorsRealEstateSpecialistSRES
            }

            "Short Sales & Foreclosure Resource / SFR" => {
                BuyerAgentDesignation::ShortSalesForeclosureResourceSFR
            }

            "Society of Industrial and Office REALTORS / SIOR" => {
                BuyerAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR
            }

            "Transnational Referral Certification / TRC" => {
                BuyerAgentDesignation::TransnationalReferralCertificationTRC
            }

            _ => BuyerAgentDesignation::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a BuyerAgentDesignation> for &'a str {
    fn from(s: &'a BuyerAgentDesignation) -> &'a str {
        match s {
            BuyerAgentDesignation::AccreditedBuyersRepresentativeABR => {
                "Accredited Buyer's Representative / ABR"
            }

            BuyerAgentDesignation::AccreditedLandConsultantALC => {
                "Accredited Land Consultant / ALC"
            }

            BuyerAgentDesignation::AtHomeWithDiversityAHWD => "At Home With Diversity / AHWD",

            BuyerAgentDesignation::CertifiedCommercialInvestmentMemberCCIM => {
                "Certified Commercial Investment Member / CCIM"
            }

            BuyerAgentDesignation::CertifiedDistressedPropertyExpertCDPE => {
                "Certified Distressed Property Expert / CDPE"
            }

            BuyerAgentDesignation::CertifiedInternationalPropertySpecialistCIPS => {
                "Certified International Property Specialist / CIPS"
            }

            BuyerAgentDesignation::CertifiedPropertyManagerCPM => {
                "Certified Property Manager / CPM"
            }

            BuyerAgentDesignation::CertifiedRealEstateBrokerageManagerCRB => {
                "Certified Real Estate Brokerage Manager / CRB"
            }

            BuyerAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS => {
                "Certified Real Estate Team Specialist / C-RETS"
            }

            BuyerAgentDesignation::CertifiedResidentialSpecialistCRS => {
                "Certified Residential Specialist / CRS"
            }

            BuyerAgentDesignation::CounselorofRealEstateCRE => "Counselor of Real Estate / CRE",

            BuyerAgentDesignation::EPRO => "e-PRO",

            BuyerAgentDesignation::GeneralAccreditedAppraiserGAA => {
                "General Accredited Appraiser / GAA"
            }

            BuyerAgentDesignation::GraduateREALTORInstituteGRI => {
                "Graduate, REALTOR Institute / GRI"
            }

            BuyerAgentDesignation::MilitaryRelocationProfessionalMRP => {
                "Military Relocation Professional / MRP"
            }

            BuyerAgentDesignation::NARsGreenDesignationGREEN => "NAR's Green Designation / GREEN",

            BuyerAgentDesignation::PerformanceManagementNetworkPMN => {
                "Performance Management Network / PMN"
            }

            BuyerAgentDesignation::PricingStrategyAdvisorPSA => "Pricing Strategy Advisor / PSA",

            BuyerAgentDesignation::RealEstateNegotiationExpertRENE => {
                "Real Estate Negotiation Expert / RENE"
            }

            BuyerAgentDesignation::REALTORAssociationCertifiedExecutiveRCE => {
                "REALTOR Association Certified Executive / RCE"
            }

            BuyerAgentDesignation::ResidentialAccreditedAppraiserRAA => {
                "Residential Accredited Appraiser / RAA"
            }

            BuyerAgentDesignation::ResortSecondHomePropertySpecialistRSPS => {
                "Resort & Second-Home Property Specialist / RSPS"
            }

            BuyerAgentDesignation::SellerRepresentativeSpecialistSRS => {
                "Seller Representative Specialist / SRS"
            }

            BuyerAgentDesignation::SeniorsRealEstateSpecialistSRES => {
                "Seniors Real Estate Specialist / SRES"
            }

            BuyerAgentDesignation::ShortSalesForeclosureResourceSFR => {
                "Short Sales & Foreclosure Resource / SFR"
            }

            BuyerAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR => {
                "Society of Industrial and Office REALTORS / SIOR"
            }

            BuyerAgentDesignation::TransnationalReferralCertificationTRC => {
                "Transnational Referral Certification / TRC"
            }

            BuyerAgentDesignation::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for BuyerAgentDesignation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for BuyerAgentDesignation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
