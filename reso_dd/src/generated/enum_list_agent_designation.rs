// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ListAgentDesignation Lookups](https://ddwiki.reso.org/display/DDW17/ListAgentDesignation+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ListAgentDesignation {
    /// "[Accredited Buyer's Representative / ABR](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245436)": The Accredited Buyer’s Representative (ABR®) designation is designed for real estate buyer agents who focus on working directly with buyer-clients.  http://www.rebac.net/abr
    AccreditedBuyersRepresentativeABR,

    /// "[Accredited Land Consultant / ALC](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245437)": Accredited Land Consultants (ALCs) are the most trusted, knowledgeable, experienced, and highest-producing experts in all segments of land. Conferred by the REALTORS® Land Institute, the designation requires successful completion of a rigorous LANDU education program, a specific, high-volume and experience level, and adherence to an honorable Code of Conduct. https://www.nar.realtor/designations-and-certifications/alc
    AccreditedLandConsultantALC,

    /// "[At Home With Diversity / AHWD](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245438)": Learn to work effectively with – and within – today’s diverse real estate market. The At Home With Diversity certification teaches you how to conduct your business with sensitivity to all client profiles and build a business plan to successfully serve them.  https://www.nar.realtor/designations-and-certifications/ahwd
    AtHomeWithDiversityAHWD,

    /// "[Certified Commercial Investment Member / CCIM](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245439)": The Certified Commercial Investment Member (CCIM) designation is commercial real estate’s global standard for professional achievement, earned through an extensive curriculum of 200 classroom hours and professional experiential requirements. https://www.nar.realtor/designations-and-certifications/ccim
    CertifiedCommercialInvestmentMemberCCIM,

    /// "[Certified Distressed Property Expert / CDPE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245440)": A Certified Distressed Property Expert® (CDPE) has a thorough understanding of complex issues in today’s turbulent real estate industry and knowledge of foreclosure avoidance options available to homeowners. CDPEs can provide solutions, specifically short sales, for homeowners facing market hardships.  http://www.cdpe.com/
    CertifiedDistressedPropertyExpertCDPE,

    /// "[Certified International Property Specialist / CIPS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245441)": The CIPS designation is for REALTORS® from the United States and abroad, as well as association staff and volunteer leaders who wish to develop or grow their international real estate business. It will provide you with the knowledge, research, network, and tools to globalize your business.                        https://www.nar.realtor/designations-and-certifications/cips-designation
    CertifiedInternationalPropertySpecialistCIPS,

    /// "[Certified Property Manager / CPM](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245442)": Certified Property Managers® (CPM®) are recognized as experts in real estate management, and they are at the top of the profession. 70% of those who hold the CPM® designation hold the highest management positions in their offices (owner/partner/officer/director). https://www.nar.realtor/designations-and-certifications/cpm
    CertifiedPropertyManagerCPM,

    /// "[Certified Real Estate Brokerage Manager / CRB](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245443)": The Certified Real Estate Brokerage Manager (CRB) Designation raises professional standards, strengthens individual and office performance, and indicates expertise in brokerage management. This designation represents the highest level of professional achievement in real estate brokerage management. You do not need a broker's license to earn the CRB Designation.  https://www.nar.realtor/designations-and-certifications/crb
    CertifiedRealEstateBrokerageManagerCRB,

    /// "[Certified Real Estate Team Specialist / C-RETS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245444)": The Certified Real Estate Team Specialist certification is designed to improve team development, individual leadership skills, and financial performance. The courses provide the tools, strategies, and knowledge that are required of today’s real estate professionals who are either considering or currently operating in a team environment. It is for team leaders, team members, those looking to start a team, and those who simply want to sharpen their management skills.  https://www.nar.realtor/designations-and-certifications/c-rets
    CertifiedRealEstateTeamSpecialistCRETS,

    /// "[Certified Residential Specialist / CRS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245445)": Certified Residential Specialist (CRS) is the highest credential awarded to residential sales agents, managers and brokers.  https://www.nar.realtor/designations-and-certifications/crs
    CertifiedResidentialSpecialistCRS,

    /// "[Counselor of Real Estate / CRE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245446)": The Counselors of Real Estate®  is an international group of recognized professionals who provide seasoned, expert, objective advice on real property and land-related matters. Only 1,100 practitioners throughout the world carry the CRE® designation. Membership is by invitation only.  https://www.nar.realtor/designations-and-certifications/cre
    CounselorofRealEstateCRE,

    /// "[e-PRO](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245447)": NAR's e-PRO® certification  teaches you to use cutting-edge technologies and digital initiatives to link up with today's savvy real estate consumer.  https://www.nar.realtor/designations-and-certifications/e-pro
    EPRO,

    /// "[General Accredited Appraiser / GAA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245448)": For general appraisers, this designation is awarded to those whose education and experience exceed state appraisal certification requirements and is supported by the National Association of REALTORS®. https://www.nar.realtor/designations-and-certifications/gaa
    GeneralAccreditedAppraiserGAA,

    /// "[Graduate, REALTOR Institute / GRI](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245449)": REALTORS® with the GRI designation have in-depth training in legal and regulatory issues, technology, professional standards, and the sales process. Earning the designation is a way to stand out to prospective buyers and sellers as a professional with expertise in these areas.  https://www.nar.realtor/designations-and-certifications/gri
    GraduateREALTORInstituteGRI,

    /// "[Military Relocation Professional / MRP](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245450)": NAR's Military Relocation Professional certification focuses on educating real estate professionals about working with current and former military service members to find housing solutions that best suit their needs and take full advantage of military benefits and support.  https://www.nar.realtor/designations-and-certifications/mrp
    MilitaryRelocationProfessionalMRP,

    /// "[NAR's Green Designation / GREEN](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245451)": Through NAR's Green Designation, the Green Resource Council provides ongoing education, resources and tools to help real estate practitioners find, understand, and market properties with green features.  https://www.nar.realtor/designations-and-certifications/green
    NARsGreenDesignationGREEN,

    /// "[Performance Management Network / PMN](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245452)": This designation is unique to the REALTOR® family designations, emphasizing that in order to enhance your business, you must enhance yourself. It focuses on negotiating strategies and tactics, networking and referrals, business planning and systems, personal performance management and  leadership development.  https://www.nar.realtor/designations-and-certifications/pmn
    PerformanceManagementNetworkPMN,

    /// "[Pricing Strategy Advisor / PSA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245453)": Enhance your skills in pricing properties, creating CMAs, working with appraisers, and guiding clients through the anxieties and misperceptions they often have about home values with NAR’s PSA (Pricing Strategy Advisor) certification. https://www.nar.realtor/designations-and-certifications/psa
    PricingStrategyAdvisorPSA,

    /// "[Real Estate Negotiation Expert / RENE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245454)": This certification is for real estate professionals who want to sharpen their negotiation skills. The RENE certification program gives REALTORS® the tips and tools they need to be skillful advocates for their clients.  https://www.nar.realtor/designations-and-certifications/rene
    RealEstateNegotiationExpertRENE,

    /// "[REALTOR Association Certified Executive / RCE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245455)": RCE is the only professional designation designed specifically for REALTOR® association executives. RCE designees exemplify goal-oriented AEs with drive, experience and commitment to professional growth.  https://www.nar.realtor/designations-and-certifications/rce
    REALTORAssociationCertifiedExecutiveRCE,

    /// "[Residential Accredited Appraiser / RAA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245456)": For residential appraisers, this designation is awarded to those whose education and experience exceed state appraisal certification requirements and is supported by the National Association of REALTORS®.  https://www.nar.realtor/designations-and-certifications/raa
    ResidentialAccreditedAppraiserRAA,

    /// "[Resort & Second-Home Property Specialist / RSPS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245457)": This certification is designed for REALTORS® who facilitate the buying, selling, or management of properties for investment, development, retirement, or second homes in a resort, recreational and/or vacation destination are involved in this market niche.  https://www.nar.realtor/designations-and-certifications/rsps
    ResortSecondHomePropertySpecialistRSPS,

    /// "[Seller Representative Specialist / SRS](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245458)": The Seller Representative Specialist (SRS) designation is the premier credential in seller representation. It is designed to elevate professional standards and enhance personal performance. The designation is awarded to real estate practitioners by the Real Estate Business Institute (REBI) who meet specific educational and practical experience criteria.  https://www.nar.realtor/designations-and-certifications/seller-representative-specialist-srs
    SellerRepresentativeSpecialistSRS,

    /// "[Seniors Real Estate Specialist / SRES](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245459)": The SRES® Designation program educates REALTORS® on how to profitably and ethically serve the real estate needs of the fastest growing market in real estate, clients age 50+. By earning the SRES® designation, you gain access to valuable member benefits, useful resources, and networking opportunities across the U.S. and Canada to help you in your business.  https://www.nar.realtor/designations-and-certifications/sres
    SeniorsRealEstateSpecialistSRES,

    /// "[Short Sales & Foreclosure Resource / SFR](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245460)": The SFR® certification  teaches real estate professionals to work with distressed sellers and the finance, tax, and legal professionals who can help them, qualify sellers for short sales, develop a short sale package, negotiate with lenders, safeguard your commission, limit risk, and protect buyers.  https://www.nar.realtor/designations-and-certifications/sfr
    ShortSalesForeclosureResourceSFR,

    /// "[Society of Industrial and Office REALTORS / SIOR](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245461)": The SIOR designation is held by only the most knowledgeable, experienced, and successful commercial real estate brokerage specialists. To earn it, designees must meet standards of experience, production, education, ethics, and provide recommendations.  https://www.nar.realtor/designations-and-certifications/sior
    SocietyofIndustrialandOfficeREALTORSSIOR,

    /// "[Transnational Referral Certification / TRC](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245462)": Real estate professionals who have taken the Transnational Referral Certified (TRC) training, have completed special training on  making and receiving client referrals from professionals in other countries. https://worldproperties.com/about-us/international-referrals-and-trc/
    TransnationalReferralCertificationTRC,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for ListAgentDesignation {
    fn from(s: String) -> ListAgentDesignation {
        match s.as_ref() {
            "Accredited Buyer's Representative / ABR" => {
                ListAgentDesignation::AccreditedBuyersRepresentativeABR
            }

            "Accredited Land Consultant / ALC" => ListAgentDesignation::AccreditedLandConsultantALC,

            "At Home With Diversity / AHWD" => ListAgentDesignation::AtHomeWithDiversityAHWD,

            "Certified Commercial Investment Member / CCIM" => {
                ListAgentDesignation::CertifiedCommercialInvestmentMemberCCIM
            }

            "Certified Distressed Property Expert / CDPE" => {
                ListAgentDesignation::CertifiedDistressedPropertyExpertCDPE
            }

            "Certified International Property Specialist / CIPS" => {
                ListAgentDesignation::CertifiedInternationalPropertySpecialistCIPS
            }

            "Certified Property Manager / CPM" => ListAgentDesignation::CertifiedPropertyManagerCPM,

            "Certified Real Estate Brokerage Manager / CRB" => {
                ListAgentDesignation::CertifiedRealEstateBrokerageManagerCRB
            }

            "Certified Real Estate Team Specialist / C-RETS" => {
                ListAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS
            }

            "Certified Residential Specialist / CRS" => {
                ListAgentDesignation::CertifiedResidentialSpecialistCRS
            }

            "Counselor of Real Estate / CRE" => ListAgentDesignation::CounselorofRealEstateCRE,

            "e-PRO" => ListAgentDesignation::EPRO,

            "General Accredited Appraiser / GAA" => {
                ListAgentDesignation::GeneralAccreditedAppraiserGAA
            }

            "Graduate, REALTOR Institute / GRI" => {
                ListAgentDesignation::GraduateREALTORInstituteGRI
            }

            "Military Relocation Professional / MRP" => {
                ListAgentDesignation::MilitaryRelocationProfessionalMRP
            }

            "NAR's Green Designation / GREEN" => ListAgentDesignation::NARsGreenDesignationGREEN,

            "Performance Management Network / PMN" => {
                ListAgentDesignation::PerformanceManagementNetworkPMN
            }

            "Pricing Strategy Advisor / PSA" => ListAgentDesignation::PricingStrategyAdvisorPSA,

            "Real Estate Negotiation Expert / RENE" => {
                ListAgentDesignation::RealEstateNegotiationExpertRENE
            }

            "REALTOR Association Certified Executive / RCE" => {
                ListAgentDesignation::REALTORAssociationCertifiedExecutiveRCE
            }

            "Residential Accredited Appraiser / RAA" => {
                ListAgentDesignation::ResidentialAccreditedAppraiserRAA
            }

            "Resort & Second-Home Property Specialist / RSPS" => {
                ListAgentDesignation::ResortSecondHomePropertySpecialistRSPS
            }

            "Seller Representative Specialist / SRS" => {
                ListAgentDesignation::SellerRepresentativeSpecialistSRS
            }

            "Seniors Real Estate Specialist / SRES" => {
                ListAgentDesignation::SeniorsRealEstateSpecialistSRES
            }

            "Short Sales & Foreclosure Resource / SFR" => {
                ListAgentDesignation::ShortSalesForeclosureResourceSFR
            }

            "Society of Industrial and Office REALTORS / SIOR" => {
                ListAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR
            }

            "Transnational Referral Certification / TRC" => {
                ListAgentDesignation::TransnationalReferralCertificationTRC
            }

            _ => ListAgentDesignation::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ListAgentDesignation {
    fn from(s: &str) -> ListAgentDesignation {
        match s {
            "Accredited Buyer's Representative / ABR" => {
                ListAgentDesignation::AccreditedBuyersRepresentativeABR
            }

            "Accredited Land Consultant / ALC" => ListAgentDesignation::AccreditedLandConsultantALC,

            "At Home With Diversity / AHWD" => ListAgentDesignation::AtHomeWithDiversityAHWD,

            "Certified Commercial Investment Member / CCIM" => {
                ListAgentDesignation::CertifiedCommercialInvestmentMemberCCIM
            }

            "Certified Distressed Property Expert / CDPE" => {
                ListAgentDesignation::CertifiedDistressedPropertyExpertCDPE
            }

            "Certified International Property Specialist / CIPS" => {
                ListAgentDesignation::CertifiedInternationalPropertySpecialistCIPS
            }

            "Certified Property Manager / CPM" => ListAgentDesignation::CertifiedPropertyManagerCPM,

            "Certified Real Estate Brokerage Manager / CRB" => {
                ListAgentDesignation::CertifiedRealEstateBrokerageManagerCRB
            }

            "Certified Real Estate Team Specialist / C-RETS" => {
                ListAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS
            }

            "Certified Residential Specialist / CRS" => {
                ListAgentDesignation::CertifiedResidentialSpecialistCRS
            }

            "Counselor of Real Estate / CRE" => ListAgentDesignation::CounselorofRealEstateCRE,

            "e-PRO" => ListAgentDesignation::EPRO,

            "General Accredited Appraiser / GAA" => {
                ListAgentDesignation::GeneralAccreditedAppraiserGAA
            }

            "Graduate, REALTOR Institute / GRI" => {
                ListAgentDesignation::GraduateREALTORInstituteGRI
            }

            "Military Relocation Professional / MRP" => {
                ListAgentDesignation::MilitaryRelocationProfessionalMRP
            }

            "NAR's Green Designation / GREEN" => ListAgentDesignation::NARsGreenDesignationGREEN,

            "Performance Management Network / PMN" => {
                ListAgentDesignation::PerformanceManagementNetworkPMN
            }

            "Pricing Strategy Advisor / PSA" => ListAgentDesignation::PricingStrategyAdvisorPSA,

            "Real Estate Negotiation Expert / RENE" => {
                ListAgentDesignation::RealEstateNegotiationExpertRENE
            }

            "REALTOR Association Certified Executive / RCE" => {
                ListAgentDesignation::REALTORAssociationCertifiedExecutiveRCE
            }

            "Residential Accredited Appraiser / RAA" => {
                ListAgentDesignation::ResidentialAccreditedAppraiserRAA
            }

            "Resort & Second-Home Property Specialist / RSPS" => {
                ListAgentDesignation::ResortSecondHomePropertySpecialistRSPS
            }

            "Seller Representative Specialist / SRS" => {
                ListAgentDesignation::SellerRepresentativeSpecialistSRS
            }

            "Seniors Real Estate Specialist / SRES" => {
                ListAgentDesignation::SeniorsRealEstateSpecialistSRES
            }

            "Short Sales & Foreclosure Resource / SFR" => {
                ListAgentDesignation::ShortSalesForeclosureResourceSFR
            }

            "Society of Industrial and Office REALTORS / SIOR" => {
                ListAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR
            }

            "Transnational Referral Certification / TRC" => {
                ListAgentDesignation::TransnationalReferralCertificationTRC
            }

            _ => ListAgentDesignation::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ListAgentDesignation> for &'a str {
    fn from(s: &'a ListAgentDesignation) -> &'a str {
        match s {
            ListAgentDesignation::AccreditedBuyersRepresentativeABR => {
                "Accredited Buyer's Representative / ABR"
            }

            ListAgentDesignation::AccreditedLandConsultantALC => "Accredited Land Consultant / ALC",

            ListAgentDesignation::AtHomeWithDiversityAHWD => "At Home With Diversity / AHWD",

            ListAgentDesignation::CertifiedCommercialInvestmentMemberCCIM => {
                "Certified Commercial Investment Member / CCIM"
            }

            ListAgentDesignation::CertifiedDistressedPropertyExpertCDPE => {
                "Certified Distressed Property Expert / CDPE"
            }

            ListAgentDesignation::CertifiedInternationalPropertySpecialistCIPS => {
                "Certified International Property Specialist / CIPS"
            }

            ListAgentDesignation::CertifiedPropertyManagerCPM => "Certified Property Manager / CPM",

            ListAgentDesignation::CertifiedRealEstateBrokerageManagerCRB => {
                "Certified Real Estate Brokerage Manager / CRB"
            }

            ListAgentDesignation::CertifiedRealEstateTeamSpecialistCRETS => {
                "Certified Real Estate Team Specialist / C-RETS"
            }

            ListAgentDesignation::CertifiedResidentialSpecialistCRS => {
                "Certified Residential Specialist / CRS"
            }

            ListAgentDesignation::CounselorofRealEstateCRE => "Counselor of Real Estate / CRE",

            ListAgentDesignation::EPRO => "e-PRO",

            ListAgentDesignation::GeneralAccreditedAppraiserGAA => {
                "General Accredited Appraiser / GAA"
            }

            ListAgentDesignation::GraduateREALTORInstituteGRI => {
                "Graduate, REALTOR Institute / GRI"
            }

            ListAgentDesignation::MilitaryRelocationProfessionalMRP => {
                "Military Relocation Professional / MRP"
            }

            ListAgentDesignation::NARsGreenDesignationGREEN => "NAR's Green Designation / GREEN",

            ListAgentDesignation::PerformanceManagementNetworkPMN => {
                "Performance Management Network / PMN"
            }

            ListAgentDesignation::PricingStrategyAdvisorPSA => "Pricing Strategy Advisor / PSA",

            ListAgentDesignation::RealEstateNegotiationExpertRENE => {
                "Real Estate Negotiation Expert / RENE"
            }

            ListAgentDesignation::REALTORAssociationCertifiedExecutiveRCE => {
                "REALTOR Association Certified Executive / RCE"
            }

            ListAgentDesignation::ResidentialAccreditedAppraiserRAA => {
                "Residential Accredited Appraiser / RAA"
            }

            ListAgentDesignation::ResortSecondHomePropertySpecialistRSPS => {
                "Resort & Second-Home Property Specialist / RSPS"
            }

            ListAgentDesignation::SellerRepresentativeSpecialistSRS => {
                "Seller Representative Specialist / SRS"
            }

            ListAgentDesignation::SeniorsRealEstateSpecialistSRES => {
                "Seniors Real Estate Specialist / SRES"
            }

            ListAgentDesignation::ShortSalesForeclosureResourceSFR => {
                "Short Sales & Foreclosure Resource / SFR"
            }

            ListAgentDesignation::SocietyofIndustrialandOfficeREALTORSSIOR => {
                "Society of Industrial and Office REALTORS / SIOR"
            }

            ListAgentDesignation::TransnationalReferralCertificationTRC => {
                "Transnational Referral Certification / TRC"
            }

            ListAgentDesignation::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ListAgentDesignation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ListAgentDesignation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_list_agent_designation_format {
    use super::ListAgentDesignation;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ListAgentDesignation>>,
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
    ) -> Result<Option<Vec<ListAgentDesignation>>, D::Error>
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
