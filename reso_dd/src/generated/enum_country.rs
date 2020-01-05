// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Country Lookups](https://ddwiki.reso.org/display/DDW17/Country+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Country {
    /// "[AD](https://ddwiki.reso.org/display/DDW17/AD)": Andorra is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AD,

    /// "[AE](https://ddwiki.reso.org/display/DDW17/AE)": United Arab Emirates is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AE,

    /// "[AF](https://ddwiki.reso.org/display/DDW17/AF)": Afghanistan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AF,

    /// "[AG](https://ddwiki.reso.org/display/DDW17/AG)": Antigua Barbuda is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AG,

    /// "[AI](https://ddwiki.reso.org/display/DDW17/AI)": Anguilla is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AI,

    /// "[AL](https://ddwiki.reso.org/display/DDW17/AL)": Albania is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AL,

    /// "[AM](https://ddwiki.reso.org/display/DDW17/AM)": Armenia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AM,

    /// "[AN](https://ddwiki.reso.org/display/DDW17/AN)": Netherlands Antilles is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AN,

    /// "[AO](https://ddwiki.reso.org/display/DDW17/AO)": Angola is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AO,

    /// "[AQ](https://ddwiki.reso.org/display/DDW17/AQ)": Antarctica is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AQ,

    /// "[AR](https://ddwiki.reso.org/display/DDW17/AR)": Argentina is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AR,

    /// "[AS](https://ddwiki.reso.org/display/DDW17/AS)": American Samoa is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AS,

    /// "[AT](https://ddwiki.reso.org/display/DDW17/AT)": Austria is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AT,

    /// "[AU](https://ddwiki.reso.org/display/DDW17/AU)": Australia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AU,

    /// "[AW](https://ddwiki.reso.org/display/DDW17/AW)": Aruba is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AW,

    /// "[AX](https://ddwiki.reso.org/display/DDW17/AX)": Land Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AX,

    /// "[AZ](https://ddwiki.reso.org/display/DDW17/AZ)": Azerbaijan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    AZ,

    /// "[BA](https://ddwiki.reso.org/display/DDW17/BA)": Bosnia Herzegovina is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BA,

    /// "[BB](https://ddwiki.reso.org/display/DDW17/BB)": Barbados is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BB,

    /// "[BD](https://ddwiki.reso.org/display/DDW17/BD)": Bangladesh is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BD,

    /// "[BE](https://ddwiki.reso.org/display/DDW17/BE)": Belgium is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BE,

    /// "[BF](https://ddwiki.reso.org/display/DDW17/BF)": Burkina Faso is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BF,

    /// "[BG](https://ddwiki.reso.org/display/DDW17/BG)": Bulgaria is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BG,

    /// "[BH](https://ddwiki.reso.org/display/DDW17/BH)": Bahrain is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BH,

    /// "[BI](https://ddwiki.reso.org/display/DDW17/BI)": Burundi is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BI,

    /// "[BJ](https://ddwiki.reso.org/display/DDW17/BJ)": Benin is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BJ,

    /// "[BL](https://ddwiki.reso.org/display/DDW17/BL)": Saint Barthelemy is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BL,

    /// "[BM](https://ddwiki.reso.org/display/DDW17/BM)": Bermuda is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BM,

    /// "[BN](https://ddwiki.reso.org/display/DDW17/BN)": Brunei Darussalam is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BN,

    /// "[BO](https://ddwiki.reso.org/display/DDW17/BO)": Bolivia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BO,

    /// "[BR](https://ddwiki.reso.org/display/DDW17/BR)": Brazil is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BR,

    /// "[BS](https://ddwiki.reso.org/display/DDW17/BS)": Bahamas is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BS,

    /// "[BT](https://ddwiki.reso.org/display/DDW17/BT)": Bhutan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BT,

    /// "[BV](https://ddwiki.reso.org/display/DDW17/BV)": Bouvet Island is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BV,

    /// "[BW](https://ddwiki.reso.org/display/DDW17/BW)": Botswana is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BW,

    /// "[BY](https://ddwiki.reso.org/display/DDW17/BY)": Belarus is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BY,

    /// "[BZ](https://ddwiki.reso.org/display/DDW17/BZ)": Belize is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    BZ,

    /// "[CA](https://ddwiki.reso.org/display/DDW17/CA)": Canada is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CA,

    /// "[CC](https://ddwiki.reso.org/display/DDW17/CC)": Cocos (Keeling) Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CC,

    /// "[CD](https://ddwiki.reso.org/display/DDW17/CD)": Congo Democratic Republic is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CD,

    /// "[CF](https://ddwiki.reso.org/display/DDW17/CF)": Central African Republic is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CF,

    /// "[CG](https://ddwiki.reso.org/display/DDW17/CG)": Congo is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CG,

    /// "[CH](https://ddwiki.reso.org/display/DDW17/CH)": Switzerland is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CH,

    /// "[CI](https://ddwiki.reso.org/display/DDW17/CI)": Cote d'Ivoire is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CI,

    /// "[CK](https://ddwiki.reso.org/display/DDW17/CK)": Cook Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CK,

    /// "[CL](https://ddwiki.reso.org/display/DDW17/CL)": Chile is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CL,

    /// "[CM](https://ddwiki.reso.org/display/DDW17/CM)": Cameroon is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CM,

    /// "[CN](https://ddwiki.reso.org/display/DDW17/CN)": China is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CN,

    /// "[CO](https://ddwiki.reso.org/display/DDW17/CO)": Colombia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CO,

    /// "[CR](https://ddwiki.reso.org/display/DDW17/CR)": Costa Rica is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CR,

    /// "[CU](https://ddwiki.reso.org/display/DDW17/CU)": Cuba is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CU,

    /// "[CV](https://ddwiki.reso.org/display/DDW17/CV)": Cabo Verde is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CV,

    /// "[CX](https://ddwiki.reso.org/display/DDW17/CX)": Christmas Island is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CX,

    /// "[CY](https://ddwiki.reso.org/display/DDW17/CY)": Cyprus is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CY,

    /// "[CZ](https://ddwiki.reso.org/display/DDW17/CZ)": Czech Republic is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    CZ,

    /// "[DE](https://ddwiki.reso.org/display/DDW17/DE)": Germany is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    DE,

    /// "[DJ](https://ddwiki.reso.org/display/DDW17/DJ)": Djibouti is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    DJ,

    /// "[DK](https://ddwiki.reso.org/display/DDW17/DK)": Denmark is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    DK,

    /// "[DM](https://ddwiki.reso.org/display/DDW17/DM)": Dominica is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    DM,

    /// "[DO](https://ddwiki.reso.org/display/DDW17/DO)": Dominican Republic is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    DO,

    /// "[DZ](https://ddwiki.reso.org/display/DDW17/DZ)": Algeria is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    DZ,

    /// "[EC](https://ddwiki.reso.org/display/DDW17/EC)": Ecuador is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    EC,

    /// "[EE](https://ddwiki.reso.org/display/DDW17/EE)": Estonia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    EE,

    /// "[EG](https://ddwiki.reso.org/display/DDW17/EG)": Egypt is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    EG,

    /// "[EH](https://ddwiki.reso.org/display/DDW17/EH)": Western Sahara is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    EH,

    /// "[ER](https://ddwiki.reso.org/display/DDW17/ER)": Eritrea is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ER,

    /// "[ES](https://ddwiki.reso.org/display/DDW17/ES)": Spain is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ES,

    /// "[ET](https://ddwiki.reso.org/display/DDW17/ET)": Ethiopia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ET,

    /// "[FI](https://ddwiki.reso.org/display/DDW17/FI)": Finland is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    FI,

    /// "[FJ](https://ddwiki.reso.org/display/DDW17/FJ)": Fiji is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    FJ,

    /// "[FK](https://ddwiki.reso.org/display/DDW17/FK)": Falkland Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    FK,

    /// "[FM](https://ddwiki.reso.org/display/DDW17/FM)": Micronesia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    FM,

    /// "[FO](https://ddwiki.reso.org/display/DDW17/FO)": Faroe Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    FO,

    /// "[FR](https://ddwiki.reso.org/display/DDW17/FR)": France is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    FR,

    /// "[GA](https://ddwiki.reso.org/display/DDW17/GA)": Gabon is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GA,

    /// "[GB](https://ddwiki.reso.org/display/DDW17/GB)": United Kingdom is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GB,

    /// "[GD](https://ddwiki.reso.org/display/DDW17/GD)": Grenada is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GD,

    /// "[GE](https://ddwiki.reso.org/display/DDW17/GE)": Georgia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GE,

    /// "[GF](https://ddwiki.reso.org/display/DDW17/GF)": French Guiana is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GF,

    /// "[GG](https://ddwiki.reso.org/display/DDW17/GG)": Guernsey is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GG,

    /// "[GH](https://ddwiki.reso.org/display/DDW17/GH)": Ghana is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GH,

    /// "[GI](https://ddwiki.reso.org/display/DDW17/GI)": Gibraltar is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GI,

    /// "[GL](https://ddwiki.reso.org/display/DDW17/GL)": Greenland is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GL,

    /// "[GM](https://ddwiki.reso.org/display/DDW17/GM)": Gambia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GM,

    /// "[GN](https://ddwiki.reso.org/display/DDW17/GN)": Guinea is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GN,

    /// "[GP](https://ddwiki.reso.org/display/DDW17/GP)": Guadeloupe is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GP,

    /// "[GQ](https://ddwiki.reso.org/display/DDW17/GQ)": Equatorial Guinea is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GQ,

    /// "[GR](https://ddwiki.reso.org/display/DDW17/GR)": Greece is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GR,

    /// "[GS](https://ddwiki.reso.org/display/DDW17/GS)": South Georgia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GS,

    /// "[GT](https://ddwiki.reso.org/display/DDW17/GT)": Guatemala is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GT,

    /// "[GU](https://ddwiki.reso.org/display/DDW17/GU)": Guam is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GU,

    /// "[GW](https://ddwiki.reso.org/display/DDW17/GW)": Guinea-Bissau is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GW,

    /// "[GY](https://ddwiki.reso.org/display/DDW17/GY)": Guyana is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    GY,

    /// "[HK](https://ddwiki.reso.org/display/DDW17/HK)": Hong Kong is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    HK,

    /// "[HM](https://ddwiki.reso.org/display/DDW17/HM)": Heard And McDonald Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    HM,

    /// "[HN](https://ddwiki.reso.org/display/DDW17/HN)": Honduras is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    HN,

    /// "[HR](https://ddwiki.reso.org/display/DDW17/HR)": Croatia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    HR,

    /// "[HT](https://ddwiki.reso.org/display/DDW17/HT)": Haiti is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    HT,

    /// "[HU](https://ddwiki.reso.org/display/DDW17/HU)": Hungary is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    HU,

    /// "[ID](https://ddwiki.reso.org/display/DDW17/ID)": Indonesia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ID,

    /// "[IE](https://ddwiki.reso.org/display/DDW17/IE)": Ireland is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IE,

    /// "[IL](https://ddwiki.reso.org/display/DDW17/IL)": Israel is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IL,

    /// "[IM](https://ddwiki.reso.org/display/DDW17/IM)": Isle Of Man is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IM,

    /// "[IN](https://ddwiki.reso.org/display/DDW17/IN)": India is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IN,

    /// "[IO](https://ddwiki.reso.org/display/DDW17/IO)": British Indian Ocean Territory is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IO,

    /// "[IQ](https://ddwiki.reso.org/display/DDW17/IQ)": Iraq is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IQ,

    /// "[IR](https://ddwiki.reso.org/display/DDW17/IR)": Iran is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IR,

    /// "[IS](https://ddwiki.reso.org/display/DDW17/IS)": Iceland is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IS,

    /// "[IT](https://ddwiki.reso.org/display/DDW17/IT)": Italy is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    IT,

    /// "[JE](https://ddwiki.reso.org/display/DDW17/JE)": Jersey is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    JE,

    /// "[JM](https://ddwiki.reso.org/display/DDW17/JM)": Jamaica is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    JM,

    /// "[JO](https://ddwiki.reso.org/display/DDW17/JO)": Jordan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    JO,

    /// "[JP](https://ddwiki.reso.org/display/DDW17/JP)": Japan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    JP,

    /// "[KE](https://ddwiki.reso.org/display/DDW17/KE)": Kenya is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KE,

    /// "[KG](https://ddwiki.reso.org/display/DDW17/KG)": Kyrgyzstan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KG,

    /// "[KH](https://ddwiki.reso.org/display/DDW17/KH)": Cambodia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KH,

    /// "[KI](https://ddwiki.reso.org/display/DDW17/KI)": Kiribati is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KI,

    /// "[KM](https://ddwiki.reso.org/display/DDW17/KM)": Comoros is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KM,

    /// "[KN](https://ddwiki.reso.org/display/DDW17/KN)": Saint Kitts And Nevis is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KN,

    /// "[KP](https://ddwiki.reso.org/display/DDW17/KP)": North Korea, officially named the Democratic People's Republic of Korea, is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KP,

    /// "[KR](https://ddwiki.reso.org/display/DDW17/KR)": South Korea, officially named the Republic of Korea, is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KR,

    /// "[KW](https://ddwiki.reso.org/display/DDW17/KW)": Kuwait is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KW,

    /// "[KY](https://ddwiki.reso.org/display/DDW17/KY)": Cayman Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KY,

    /// "[KZ](https://ddwiki.reso.org/display/DDW17/KZ)": Kazakhstan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    KZ,

    /// "[LA](https://ddwiki.reso.org/display/DDW17/LA)": Lao is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LA,

    /// "[LB](https://ddwiki.reso.org/display/DDW17/LB)": Lebanon is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LB,

    /// "[LC](https://ddwiki.reso.org/display/DDW17/LC)": Saint Lucia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LC,

    /// "[LI](https://ddwiki.reso.org/display/DDW17/LI)": Liechtenstein is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LI,

    /// "[LK](https://ddwiki.reso.org/display/DDW17/LK)": Sri Lanka is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LK,

    /// "[LR](https://ddwiki.reso.org/display/DDW17/LR)": Liberia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LR,

    /// "[LS](https://ddwiki.reso.org/display/DDW17/LS)": Lesotho is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LS,

    /// "[LT](https://ddwiki.reso.org/display/DDW17/LT)": Lithuania is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LT,

    /// "[LU](https://ddwiki.reso.org/display/DDW17/LU)": Luxembourg is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LU,

    /// "[LV](https://ddwiki.reso.org/display/DDW17/LV)": Latvia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LV,

    /// "[LY](https://ddwiki.reso.org/display/DDW17/LY)": Libyan Arab Jamahiriya is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    LY,

    /// "[MA](https://ddwiki.reso.org/display/DDW17/MA)": Morocco is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MA,

    /// "[MC](https://ddwiki.reso.org/display/DDW17/MC)": Monaco is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MC,

    /// "[MD](https://ddwiki.reso.org/display/DDW17/MD)": Moldova is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MD,

    /// "[ME](https://ddwiki.reso.org/display/DDW17/ME)": Montenegro is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ME,

    /// "[MF](https://ddwiki.reso.org/display/DDW17/MF)": Saint Martin is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MF,

    /// "[MG](https://ddwiki.reso.org/display/DDW17/MG)": Madagascar is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MG,

    /// "[MH](https://ddwiki.reso.org/display/DDW17/MH)": Marshall Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MH,

    /// "[MK](https://ddwiki.reso.org/display/DDW17/MK)": Macedonia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MK,

    /// "[ML](https://ddwiki.reso.org/display/DDW17/ML)": Mali is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ML,

    /// "[MM](https://ddwiki.reso.org/display/DDW17/MM)": Myanmar is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MM,

    /// "[MN](https://ddwiki.reso.org/display/DDW17/MN)": Mongolia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MN,

    /// "[MO](https://ddwiki.reso.org/display/DDW17/MO)": Macao is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MO,

    /// "[MP](https://ddwiki.reso.org/display/DDW17/MP)": Northern Mariana Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MP,

    /// "[MQ](https://ddwiki.reso.org/display/DDW17/MQ)": Martinique is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MQ,

    /// "[MR](https://ddwiki.reso.org/display/DDW17/MR)": Mauritania is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MR,

    /// "[MS](https://ddwiki.reso.org/display/DDW17/MS)": Montserrat is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MS,

    /// "[MT](https://ddwiki.reso.org/display/DDW17/MT)": Malta is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MT,

    /// "[MU](https://ddwiki.reso.org/display/DDW17/MU)": Mauritius is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MU,

    /// "[MV](https://ddwiki.reso.org/display/DDW17/MV)": Maldives is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MV,

    /// "[MW](https://ddwiki.reso.org/display/DDW17/MW)": Malawi is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MW,

    /// "[MX](https://ddwiki.reso.org/display/DDW17/MX)": Mexico is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MX,

    /// "[MY](https://ddwiki.reso.org/display/DDW17/MY)": Malaysia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MY,

    /// "[MZ](https://ddwiki.reso.org/display/DDW17/MZ)": Mozambique is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    MZ,

    /// "[NA](https://ddwiki.reso.org/display/DDW17/NA)": Namibia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NA,

    /// "[NC](https://ddwiki.reso.org/display/DDW17/NC)": New Caledonia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NC,

    /// "[NE](https://ddwiki.reso.org/display/DDW17/NE)": Niger is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NE,

    /// "[NF](https://ddwiki.reso.org/display/DDW17/NF)": Norfolk Island is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NF,

    /// "[NG](https://ddwiki.reso.org/display/DDW17/NG)": Nigeria is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NG,

    /// "[NI](https://ddwiki.reso.org/display/DDW17/NI)": Nicaragua is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NI,

    /// "[NL](https://ddwiki.reso.org/display/DDW17/NL)": Netherlands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NL,

    /// "[NP](https://ddwiki.reso.org/display/DDW17/NP)": Nepal is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NP,

    /// "[NR](https://ddwiki.reso.org/display/DDW17/NR)": Nauru is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NR,

    /// "[NU](https://ddwiki.reso.org/display/DDW17/NU)": Niue is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NU,

    /// "[NZ](https://ddwiki.reso.org/display/DDW17/NZ)": New Zealand is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    NZ,

    /// "[OM](https://ddwiki.reso.org/display/DDW17/OM)": Oman is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    OM,

    /// "[OT](https://ddwiki.reso.org/display/DDW17/OT)": Other is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    OT,

    /// "[PA](https://ddwiki.reso.org/display/DDW17/PA)": Panama is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PA,

    /// "[PE](https://ddwiki.reso.org/display/DDW17/PE)": Peru is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PE,

    /// "[PF](https://ddwiki.reso.org/display/DDW17/PF)": French Polynesia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PF,

    /// "[PG](https://ddwiki.reso.org/display/DDW17/PG)": Papua New Guinea is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PG,

    /// "[PH](https://ddwiki.reso.org/display/DDW17/PH)": Philippines is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PH,

    /// "[PK](https://ddwiki.reso.org/display/DDW17/PK)": Pakistan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PK,

    /// "[PL](https://ddwiki.reso.org/display/DDW17/PL)": Poland is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PL,

    /// "[PM](https://ddwiki.reso.org/display/DDW17/PM)": Saint Pierre And Miquelon is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PM,

    /// "[PN](https://ddwiki.reso.org/display/DDW17/PN)": Pitcairn is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PN,

    /// "[PR](https://ddwiki.reso.org/display/DDW17/PR)": Puerto Rico is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PR,

    /// "[PS](https://ddwiki.reso.org/display/DDW17/PS)": Palestinian Territory is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PS,

    /// "[PT](https://ddwiki.reso.org/display/DDW17/PT)": Portugal is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PT,

    /// "[PW](https://ddwiki.reso.org/display/DDW17/PW)": Palau is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PW,

    /// "[PY](https://ddwiki.reso.org/display/DDW17/PY)": Paraguay is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    PY,

    /// "[QA](https://ddwiki.reso.org/display/DDW17/QA)": Qatar is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    QA,

    /// "[RE](https://ddwiki.reso.org/display/DDW17/RE)": Reunion is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    RE,

    /// "[RO](https://ddwiki.reso.org/display/DDW17/RO)": Romania is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    RO,

    /// "[RS](https://ddwiki.reso.org/display/DDW17/RS)": Serbia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    RS,

    /// "[RU](https://ddwiki.reso.org/display/DDW17/RU)": Russian Federation is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    RU,

    /// "[RW](https://ddwiki.reso.org/display/DDW17/RW)": Rwanda is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    RW,

    /// "[SA](https://ddwiki.reso.org/display/DDW17/SA)": Saudi Arabia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SA,

    /// "[SB](https://ddwiki.reso.org/display/DDW17/SB)": Solomon Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SB,

    /// "[SC](https://ddwiki.reso.org/display/DDW17/SC)": Seychelles is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SC,

    /// "[SD](https://ddwiki.reso.org/display/DDW17/SD)": Sudan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SD,

    /// "[SE](https://ddwiki.reso.org/display/DDW17/SE)": Sweden is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SE,

    /// "[SG](https://ddwiki.reso.org/display/DDW17/SG)": Singapore is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SG,

    /// "[SH](https://ddwiki.reso.org/display/DDW17/SH)": Saint Helena is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SH,

    /// "[SI](https://ddwiki.reso.org/display/DDW17/SI)": Slovenia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SI,

    /// "[SJ](https://ddwiki.reso.org/display/DDW17/SJ)": Svalbard - Jan Mayen is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SJ,

    /// "[SK](https://ddwiki.reso.org/display/DDW17/SK)": Slovakia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SK,

    /// "[SL](https://ddwiki.reso.org/display/DDW17/SL)": Sierra Leone is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SL,

    /// "[SM](https://ddwiki.reso.org/display/DDW17/SM)": San Marino is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SM,

    /// "[SN](https://ddwiki.reso.org/display/DDW17/SN)": Senegal is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SN,

    /// "[SO](https://ddwiki.reso.org/display/DDW17/SO)": Somalia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SO,

    /// "[SR](https://ddwiki.reso.org/display/DDW17/SR)": Suriname is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SR,

    /// "[ST](https://ddwiki.reso.org/display/DDW17/ST)": Sao Tome And Principe is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ST,

    /// "[SV](https://ddwiki.reso.org/display/DDW17/SV)": El Salvador is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SV,

    /// "[SY](https://ddwiki.reso.org/display/DDW17/SY)": Syrian Arab Republic is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SY,

    /// "[SZ](https://ddwiki.reso.org/display/DDW17/SZ)": Swaziland is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    SZ,

    /// "[TC](https://ddwiki.reso.org/display/DDW17/TC)": Turks - Caicos Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TC,

    /// "[TD](https://ddwiki.reso.org/display/DDW17/TD)": Chad is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TD,

    /// "[TF](https://ddwiki.reso.org/display/DDW17/TF)": French Southern Territories is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TF,

    /// "[TG](https://ddwiki.reso.org/display/DDW17/TG)": Togo is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TG,

    /// "[TH](https://ddwiki.reso.org/display/DDW17/TH)": Thailand is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TH,

    /// "[TJ](https://ddwiki.reso.org/display/DDW17/TJ)": Tajikistan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TJ,

    /// "[TK](https://ddwiki.reso.org/display/DDW17/TK)": Tokelau is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TK,

    /// "[TL](https://ddwiki.reso.org/display/DDW17/TL)": Timor-Leste is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TL,

    /// "[TM](https://ddwiki.reso.org/display/DDW17/TM)": Turkmenistan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TM,

    /// "[TN](https://ddwiki.reso.org/display/DDW17/TN)": Tunisia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TN,

    /// "[TO](https://ddwiki.reso.org/display/DDW17/TO)": Tonga is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TO,

    /// "[TR](https://ddwiki.reso.org/display/DDW17/TR)": Turkey is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TR,

    /// "[TT](https://ddwiki.reso.org/display/DDW17/TT)": Trinidad - Tobago is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TT,

    /// "[TV](https://ddwiki.reso.org/display/DDW17/TV)": Tuvalu is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TV,

    /// "[TW](https://ddwiki.reso.org/display/DDW17/TW)": Taiwan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TW,

    /// "[TZ](https://ddwiki.reso.org/display/DDW17/TZ)": Tanzania is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    TZ,

    /// "[UA](https://ddwiki.reso.org/display/DDW17/UA)": Ukraine is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    UA,

    /// "[UG](https://ddwiki.reso.org/display/DDW17/UG)": Uganda is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    UG,

    /// "[UM](https://ddwiki.reso.org/display/DDW17/UM)": United States Minor Islands is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    UM,

    /// "[US](https://ddwiki.reso.org/display/DDW17/US)": United States is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    US,

    /// "[UY](https://ddwiki.reso.org/display/DDW17/UY)": Uruguay is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    UY,

    /// "[UZ](https://ddwiki.reso.org/display/DDW17/UZ)": Uzbekistan is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    UZ,

    /// "[VA](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244365)": Holy See (Vatican City) is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    VA,

    /// "[VC](https://ddwiki.reso.org/display/DDW17/VC)": Saint Vincent - Grenadines is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    VC,

    /// "[VE](https://ddwiki.reso.org/display/DDW17/VE)": Venezuela is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    VE,

    /// "[VG](https://ddwiki.reso.org/display/DDW17/VG)": Virgin Islands British is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    VG,

    /// "[VI](https://ddwiki.reso.org/display/DDW17/VI)": Virgin Islands U.S. is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    VI,

    /// "[VN](https://ddwiki.reso.org/display/DDW17/VN)": Viet Nam is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    VN,

    /// "[VU](https://ddwiki.reso.org/display/DDW17/VU)": Vanuatu is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    VU,

    /// "[WF](https://ddwiki.reso.org/display/DDW17/WF)": Wallis And Futuna is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    WF,

    /// "[WS](https://ddwiki.reso.org/display/DDW17/WS)": Samoa is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    WS,

    /// "[YE](https://ddwiki.reso.org/display/DDW17/YE)": Yemen is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    YE,

    /// "[YT](https://ddwiki.reso.org/display/DDW17/YT)": Mayotte is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    YT,

    /// "[ZA](https://ddwiki.reso.org/display/DDW17/ZA)": South Africa is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ZA,

    /// "[ZM](https://ddwiki.reso.org/display/DDW17/ZM)": Zambia is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ZM,

    /// "[ZW](https://ddwiki.reso.org/display/DDW17/ZW)": Zimbabwe is the country in which the individual, entity or property is located. The two letter code is based on ISO standard 3166.
    ZW,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Country {
    fn from_str(s: &str) -> Country {
        match s {
            "AD" => Country::AD,

            "AE" => Country::AE,

            "AF" => Country::AF,

            "AG" => Country::AG,

            "AI" => Country::AI,

            "AL" => Country::AL,

            "AM" => Country::AM,

            "AN" => Country::AN,

            "AO" => Country::AO,

            "AQ" => Country::AQ,

            "AR" => Country::AR,

            "AS" => Country::AS,

            "AT" => Country::AT,

            "AU" => Country::AU,

            "AW" => Country::AW,

            "AX" => Country::AX,

            "AZ" => Country::AZ,

            "BA" => Country::BA,

            "BB" => Country::BB,

            "BD" => Country::BD,

            "BE" => Country::BE,

            "BF" => Country::BF,

            "BG" => Country::BG,

            "BH" => Country::BH,

            "BI" => Country::BI,

            "BJ" => Country::BJ,

            "BL" => Country::BL,

            "BM" => Country::BM,

            "BN" => Country::BN,

            "BO" => Country::BO,

            "BR" => Country::BR,

            "BS" => Country::BS,

            "BT" => Country::BT,

            "BV" => Country::BV,

            "BW" => Country::BW,

            "BY" => Country::BY,

            "BZ" => Country::BZ,

            "CA" => Country::CA,

            "CC" => Country::CC,

            "CD" => Country::CD,

            "CF" => Country::CF,

            "CG" => Country::CG,

            "CH" => Country::CH,

            "CI" => Country::CI,

            "CK" => Country::CK,

            "CL" => Country::CL,

            "CM" => Country::CM,

            "CN" => Country::CN,

            "CO" => Country::CO,

            "CR" => Country::CR,

            "CU" => Country::CU,

            "CV" => Country::CV,

            "CX" => Country::CX,

            "CY" => Country::CY,

            "CZ" => Country::CZ,

            "DE" => Country::DE,

            "DJ" => Country::DJ,

            "DK" => Country::DK,

            "DM" => Country::DM,

            "DO" => Country::DO,

            "DZ" => Country::DZ,

            "EC" => Country::EC,

            "EE" => Country::EE,

            "EG" => Country::EG,

            "EH" => Country::EH,

            "ER" => Country::ER,

            "ES" => Country::ES,

            "ET" => Country::ET,

            "FI" => Country::FI,

            "FJ" => Country::FJ,

            "FK" => Country::FK,

            "FM" => Country::FM,

            "FO" => Country::FO,

            "FR" => Country::FR,

            "GA" => Country::GA,

            "GB" => Country::GB,

            "GD" => Country::GD,

            "GE" => Country::GE,

            "GF" => Country::GF,

            "GG" => Country::GG,

            "GH" => Country::GH,

            "GI" => Country::GI,

            "GL" => Country::GL,

            "GM" => Country::GM,

            "GN" => Country::GN,

            "GP" => Country::GP,

            "GQ" => Country::GQ,

            "GR" => Country::GR,

            "GS" => Country::GS,

            "GT" => Country::GT,

            "GU" => Country::GU,

            "GW" => Country::GW,

            "GY" => Country::GY,

            "HK" => Country::HK,

            "HM" => Country::HM,

            "HN" => Country::HN,

            "HR" => Country::HR,

            "HT" => Country::HT,

            "HU" => Country::HU,

            "ID" => Country::ID,

            "IE" => Country::IE,

            "IL" => Country::IL,

            "IM" => Country::IM,

            "IN" => Country::IN,

            "IO" => Country::IO,

            "IQ" => Country::IQ,

            "IR" => Country::IR,

            "IS" => Country::IS,

            "IT" => Country::IT,

            "JE" => Country::JE,

            "JM" => Country::JM,

            "JO" => Country::JO,

            "JP" => Country::JP,

            "KE" => Country::KE,

            "KG" => Country::KG,

            "KH" => Country::KH,

            "KI" => Country::KI,

            "KM" => Country::KM,

            "KN" => Country::KN,

            "KP" => Country::KP,

            "KR" => Country::KR,

            "KW" => Country::KW,

            "KY" => Country::KY,

            "KZ" => Country::KZ,

            "LA" => Country::LA,

            "LB" => Country::LB,

            "LC" => Country::LC,

            "LI" => Country::LI,

            "LK" => Country::LK,

            "LR" => Country::LR,

            "LS" => Country::LS,

            "LT" => Country::LT,

            "LU" => Country::LU,

            "LV" => Country::LV,

            "LY" => Country::LY,

            "MA" => Country::MA,

            "MC" => Country::MC,

            "MD" => Country::MD,

            "ME" => Country::ME,

            "MF" => Country::MF,

            "MG" => Country::MG,

            "MH" => Country::MH,

            "MK" => Country::MK,

            "ML" => Country::ML,

            "MM" => Country::MM,

            "MN" => Country::MN,

            "MO" => Country::MO,

            "MP" => Country::MP,

            "MQ" => Country::MQ,

            "MR" => Country::MR,

            "MS" => Country::MS,

            "MT" => Country::MT,

            "MU" => Country::MU,

            "MV" => Country::MV,

            "MW" => Country::MW,

            "MX" => Country::MX,

            "MY" => Country::MY,

            "MZ" => Country::MZ,

            "NA" => Country::NA,

            "NC" => Country::NC,

            "NE" => Country::NE,

            "NF" => Country::NF,

            "NG" => Country::NG,

            "NI" => Country::NI,

            "NL" => Country::NL,

            "NP" => Country::NP,

            "NR" => Country::NR,

            "NU" => Country::NU,

            "NZ" => Country::NZ,

            "OM" => Country::OM,

            "OT" => Country::OT,

            "PA" => Country::PA,

            "PE" => Country::PE,

            "PF" => Country::PF,

            "PG" => Country::PG,

            "PH" => Country::PH,

            "PK" => Country::PK,

            "PL" => Country::PL,

            "PM" => Country::PM,

            "PN" => Country::PN,

            "PR" => Country::PR,

            "PS" => Country::PS,

            "PT" => Country::PT,

            "PW" => Country::PW,

            "PY" => Country::PY,

            "QA" => Country::QA,

            "RE" => Country::RE,

            "RO" => Country::RO,

            "RS" => Country::RS,

            "RU" => Country::RU,

            "RW" => Country::RW,

            "SA" => Country::SA,

            "SB" => Country::SB,

            "SC" => Country::SC,

            "SD" => Country::SD,

            "SE" => Country::SE,

            "SG" => Country::SG,

            "SH" => Country::SH,

            "SI" => Country::SI,

            "SJ" => Country::SJ,

            "SK" => Country::SK,

            "SL" => Country::SL,

            "SM" => Country::SM,

            "SN" => Country::SN,

            "SO" => Country::SO,

            "SR" => Country::SR,

            "ST" => Country::ST,

            "SV" => Country::SV,

            "SY" => Country::SY,

            "SZ" => Country::SZ,

            "TC" => Country::TC,

            "TD" => Country::TD,

            "TF" => Country::TF,

            "TG" => Country::TG,

            "TH" => Country::TH,

            "TJ" => Country::TJ,

            "TK" => Country::TK,

            "TL" => Country::TL,

            "TM" => Country::TM,

            "TN" => Country::TN,

            "TO" => Country::TO,

            "TR" => Country::TR,

            "TT" => Country::TT,

            "TV" => Country::TV,

            "TW" => Country::TW,

            "TZ" => Country::TZ,

            "UA" => Country::UA,

            "UG" => Country::UG,

            "UM" => Country::UM,

            "US" => Country::US,

            "UY" => Country::UY,

            "UZ" => Country::UZ,

            "VA" => Country::VA,

            "VC" => Country::VC,

            "VE" => Country::VE,

            "VG" => Country::VG,

            "VI" => Country::VI,

            "VN" => Country::VN,

            "VU" => Country::VU,

            "WF" => Country::WF,

            "WS" => Country::WS,

            "YE" => Country::YE,

            "YT" => Country::YT,

            "ZA" => Country::ZA,

            "ZM" => Country::ZM,

            "ZW" => Country::ZW,

            _ => Country::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Country {
        match s.as_ref() {
            "AD" => Country::AD,

            "AE" => Country::AE,

            "AF" => Country::AF,

            "AG" => Country::AG,

            "AI" => Country::AI,

            "AL" => Country::AL,

            "AM" => Country::AM,

            "AN" => Country::AN,

            "AO" => Country::AO,

            "AQ" => Country::AQ,

            "AR" => Country::AR,

            "AS" => Country::AS,

            "AT" => Country::AT,

            "AU" => Country::AU,

            "AW" => Country::AW,

            "AX" => Country::AX,

            "AZ" => Country::AZ,

            "BA" => Country::BA,

            "BB" => Country::BB,

            "BD" => Country::BD,

            "BE" => Country::BE,

            "BF" => Country::BF,

            "BG" => Country::BG,

            "BH" => Country::BH,

            "BI" => Country::BI,

            "BJ" => Country::BJ,

            "BL" => Country::BL,

            "BM" => Country::BM,

            "BN" => Country::BN,

            "BO" => Country::BO,

            "BR" => Country::BR,

            "BS" => Country::BS,

            "BT" => Country::BT,

            "BV" => Country::BV,

            "BW" => Country::BW,

            "BY" => Country::BY,

            "BZ" => Country::BZ,

            "CA" => Country::CA,

            "CC" => Country::CC,

            "CD" => Country::CD,

            "CF" => Country::CF,

            "CG" => Country::CG,

            "CH" => Country::CH,

            "CI" => Country::CI,

            "CK" => Country::CK,

            "CL" => Country::CL,

            "CM" => Country::CM,

            "CN" => Country::CN,

            "CO" => Country::CO,

            "CR" => Country::CR,

            "CU" => Country::CU,

            "CV" => Country::CV,

            "CX" => Country::CX,

            "CY" => Country::CY,

            "CZ" => Country::CZ,

            "DE" => Country::DE,

            "DJ" => Country::DJ,

            "DK" => Country::DK,

            "DM" => Country::DM,

            "DO" => Country::DO,

            "DZ" => Country::DZ,

            "EC" => Country::EC,

            "EE" => Country::EE,

            "EG" => Country::EG,

            "EH" => Country::EH,

            "ER" => Country::ER,

            "ES" => Country::ES,

            "ET" => Country::ET,

            "FI" => Country::FI,

            "FJ" => Country::FJ,

            "FK" => Country::FK,

            "FM" => Country::FM,

            "FO" => Country::FO,

            "FR" => Country::FR,

            "GA" => Country::GA,

            "GB" => Country::GB,

            "GD" => Country::GD,

            "GE" => Country::GE,

            "GF" => Country::GF,

            "GG" => Country::GG,

            "GH" => Country::GH,

            "GI" => Country::GI,

            "GL" => Country::GL,

            "GM" => Country::GM,

            "GN" => Country::GN,

            "GP" => Country::GP,

            "GQ" => Country::GQ,

            "GR" => Country::GR,

            "GS" => Country::GS,

            "GT" => Country::GT,

            "GU" => Country::GU,

            "GW" => Country::GW,

            "GY" => Country::GY,

            "HK" => Country::HK,

            "HM" => Country::HM,

            "HN" => Country::HN,

            "HR" => Country::HR,

            "HT" => Country::HT,

            "HU" => Country::HU,

            "ID" => Country::ID,

            "IE" => Country::IE,

            "IL" => Country::IL,

            "IM" => Country::IM,

            "IN" => Country::IN,

            "IO" => Country::IO,

            "IQ" => Country::IQ,

            "IR" => Country::IR,

            "IS" => Country::IS,

            "IT" => Country::IT,

            "JE" => Country::JE,

            "JM" => Country::JM,

            "JO" => Country::JO,

            "JP" => Country::JP,

            "KE" => Country::KE,

            "KG" => Country::KG,

            "KH" => Country::KH,

            "KI" => Country::KI,

            "KM" => Country::KM,

            "KN" => Country::KN,

            "KP" => Country::KP,

            "KR" => Country::KR,

            "KW" => Country::KW,

            "KY" => Country::KY,

            "KZ" => Country::KZ,

            "LA" => Country::LA,

            "LB" => Country::LB,

            "LC" => Country::LC,

            "LI" => Country::LI,

            "LK" => Country::LK,

            "LR" => Country::LR,

            "LS" => Country::LS,

            "LT" => Country::LT,

            "LU" => Country::LU,

            "LV" => Country::LV,

            "LY" => Country::LY,

            "MA" => Country::MA,

            "MC" => Country::MC,

            "MD" => Country::MD,

            "ME" => Country::ME,

            "MF" => Country::MF,

            "MG" => Country::MG,

            "MH" => Country::MH,

            "MK" => Country::MK,

            "ML" => Country::ML,

            "MM" => Country::MM,

            "MN" => Country::MN,

            "MO" => Country::MO,

            "MP" => Country::MP,

            "MQ" => Country::MQ,

            "MR" => Country::MR,

            "MS" => Country::MS,

            "MT" => Country::MT,

            "MU" => Country::MU,

            "MV" => Country::MV,

            "MW" => Country::MW,

            "MX" => Country::MX,

            "MY" => Country::MY,

            "MZ" => Country::MZ,

            "NA" => Country::NA,

            "NC" => Country::NC,

            "NE" => Country::NE,

            "NF" => Country::NF,

            "NG" => Country::NG,

            "NI" => Country::NI,

            "NL" => Country::NL,

            "NP" => Country::NP,

            "NR" => Country::NR,

            "NU" => Country::NU,

            "NZ" => Country::NZ,

            "OM" => Country::OM,

            "OT" => Country::OT,

            "PA" => Country::PA,

            "PE" => Country::PE,

            "PF" => Country::PF,

            "PG" => Country::PG,

            "PH" => Country::PH,

            "PK" => Country::PK,

            "PL" => Country::PL,

            "PM" => Country::PM,

            "PN" => Country::PN,

            "PR" => Country::PR,

            "PS" => Country::PS,

            "PT" => Country::PT,

            "PW" => Country::PW,

            "PY" => Country::PY,

            "QA" => Country::QA,

            "RE" => Country::RE,

            "RO" => Country::RO,

            "RS" => Country::RS,

            "RU" => Country::RU,

            "RW" => Country::RW,

            "SA" => Country::SA,

            "SB" => Country::SB,

            "SC" => Country::SC,

            "SD" => Country::SD,

            "SE" => Country::SE,

            "SG" => Country::SG,

            "SH" => Country::SH,

            "SI" => Country::SI,

            "SJ" => Country::SJ,

            "SK" => Country::SK,

            "SL" => Country::SL,

            "SM" => Country::SM,

            "SN" => Country::SN,

            "SO" => Country::SO,

            "SR" => Country::SR,

            "ST" => Country::ST,

            "SV" => Country::SV,

            "SY" => Country::SY,

            "SZ" => Country::SZ,

            "TC" => Country::TC,

            "TD" => Country::TD,

            "TF" => Country::TF,

            "TG" => Country::TG,

            "TH" => Country::TH,

            "TJ" => Country::TJ,

            "TK" => Country::TK,

            "TL" => Country::TL,

            "TM" => Country::TM,

            "TN" => Country::TN,

            "TO" => Country::TO,

            "TR" => Country::TR,

            "TT" => Country::TT,

            "TV" => Country::TV,

            "TW" => Country::TW,

            "TZ" => Country::TZ,

            "UA" => Country::UA,

            "UG" => Country::UG,

            "UM" => Country::UM,

            "US" => Country::US,

            "UY" => Country::UY,

            "UZ" => Country::UZ,

            "VA" => Country::VA,

            "VC" => Country::VC,

            "VE" => Country::VE,

            "VG" => Country::VG,

            "VI" => Country::VI,

            "VN" => Country::VN,

            "VU" => Country::VU,

            "WF" => Country::WF,

            "WS" => Country::WS,

            "YE" => Country::YE,

            "YT" => Country::YT,

            "ZA" => Country::ZA,

            "ZM" => Country::ZM,

            "ZW" => Country::ZW,

            _ => Country::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Country::AD => "AD",

            Country::AE => "AE",

            Country::AF => "AF",

            Country::AG => "AG",

            Country::AI => "AI",

            Country::AL => "AL",

            Country::AM => "AM",

            Country::AN => "AN",

            Country::AO => "AO",

            Country::AQ => "AQ",

            Country::AR => "AR",

            Country::AS => "AS",

            Country::AT => "AT",

            Country::AU => "AU",

            Country::AW => "AW",

            Country::AX => "AX",

            Country::AZ => "AZ",

            Country::BA => "BA",

            Country::BB => "BB",

            Country::BD => "BD",

            Country::BE => "BE",

            Country::BF => "BF",

            Country::BG => "BG",

            Country::BH => "BH",

            Country::BI => "BI",

            Country::BJ => "BJ",

            Country::BL => "BL",

            Country::BM => "BM",

            Country::BN => "BN",

            Country::BO => "BO",

            Country::BR => "BR",

            Country::BS => "BS",

            Country::BT => "BT",

            Country::BV => "BV",

            Country::BW => "BW",

            Country::BY => "BY",

            Country::BZ => "BZ",

            Country::CA => "CA",

            Country::CC => "CC",

            Country::CD => "CD",

            Country::CF => "CF",

            Country::CG => "CG",

            Country::CH => "CH",

            Country::CI => "CI",

            Country::CK => "CK",

            Country::CL => "CL",

            Country::CM => "CM",

            Country::CN => "CN",

            Country::CO => "CO",

            Country::CR => "CR",

            Country::CU => "CU",

            Country::CV => "CV",

            Country::CX => "CX",

            Country::CY => "CY",

            Country::CZ => "CZ",

            Country::DE => "DE",

            Country::DJ => "DJ",

            Country::DK => "DK",

            Country::DM => "DM",

            Country::DO => "DO",

            Country::DZ => "DZ",

            Country::EC => "EC",

            Country::EE => "EE",

            Country::EG => "EG",

            Country::EH => "EH",

            Country::ER => "ER",

            Country::ES => "ES",

            Country::ET => "ET",

            Country::FI => "FI",

            Country::FJ => "FJ",

            Country::FK => "FK",

            Country::FM => "FM",

            Country::FO => "FO",

            Country::FR => "FR",

            Country::GA => "GA",

            Country::GB => "GB",

            Country::GD => "GD",

            Country::GE => "GE",

            Country::GF => "GF",

            Country::GG => "GG",

            Country::GH => "GH",

            Country::GI => "GI",

            Country::GL => "GL",

            Country::GM => "GM",

            Country::GN => "GN",

            Country::GP => "GP",

            Country::GQ => "GQ",

            Country::GR => "GR",

            Country::GS => "GS",

            Country::GT => "GT",

            Country::GU => "GU",

            Country::GW => "GW",

            Country::GY => "GY",

            Country::HK => "HK",

            Country::HM => "HM",

            Country::HN => "HN",

            Country::HR => "HR",

            Country::HT => "HT",

            Country::HU => "HU",

            Country::ID => "ID",

            Country::IE => "IE",

            Country::IL => "IL",

            Country::IM => "IM",

            Country::IN => "IN",

            Country::IO => "IO",

            Country::IQ => "IQ",

            Country::IR => "IR",

            Country::IS => "IS",

            Country::IT => "IT",

            Country::JE => "JE",

            Country::JM => "JM",

            Country::JO => "JO",

            Country::JP => "JP",

            Country::KE => "KE",

            Country::KG => "KG",

            Country::KH => "KH",

            Country::KI => "KI",

            Country::KM => "KM",

            Country::KN => "KN",

            Country::KP => "KP",

            Country::KR => "KR",

            Country::KW => "KW",

            Country::KY => "KY",

            Country::KZ => "KZ",

            Country::LA => "LA",

            Country::LB => "LB",

            Country::LC => "LC",

            Country::LI => "LI",

            Country::LK => "LK",

            Country::LR => "LR",

            Country::LS => "LS",

            Country::LT => "LT",

            Country::LU => "LU",

            Country::LV => "LV",

            Country::LY => "LY",

            Country::MA => "MA",

            Country::MC => "MC",

            Country::MD => "MD",

            Country::ME => "ME",

            Country::MF => "MF",

            Country::MG => "MG",

            Country::MH => "MH",

            Country::MK => "MK",

            Country::ML => "ML",

            Country::MM => "MM",

            Country::MN => "MN",

            Country::MO => "MO",

            Country::MP => "MP",

            Country::MQ => "MQ",

            Country::MR => "MR",

            Country::MS => "MS",

            Country::MT => "MT",

            Country::MU => "MU",

            Country::MV => "MV",

            Country::MW => "MW",

            Country::MX => "MX",

            Country::MY => "MY",

            Country::MZ => "MZ",

            Country::NA => "NA",

            Country::NC => "NC",

            Country::NE => "NE",

            Country::NF => "NF",

            Country::NG => "NG",

            Country::NI => "NI",

            Country::NL => "NL",

            Country::NP => "NP",

            Country::NR => "NR",

            Country::NU => "NU",

            Country::NZ => "NZ",

            Country::OM => "OM",

            Country::OT => "OT",

            Country::PA => "PA",

            Country::PE => "PE",

            Country::PF => "PF",

            Country::PG => "PG",

            Country::PH => "PH",

            Country::PK => "PK",

            Country::PL => "PL",

            Country::PM => "PM",

            Country::PN => "PN",

            Country::PR => "PR",

            Country::PS => "PS",

            Country::PT => "PT",

            Country::PW => "PW",

            Country::PY => "PY",

            Country::QA => "QA",

            Country::RE => "RE",

            Country::RO => "RO",

            Country::RS => "RS",

            Country::RU => "RU",

            Country::RW => "RW",

            Country::SA => "SA",

            Country::SB => "SB",

            Country::SC => "SC",

            Country::SD => "SD",

            Country::SE => "SE",

            Country::SG => "SG",

            Country::SH => "SH",

            Country::SI => "SI",

            Country::SJ => "SJ",

            Country::SK => "SK",

            Country::SL => "SL",

            Country::SM => "SM",

            Country::SN => "SN",

            Country::SO => "SO",

            Country::SR => "SR",

            Country::ST => "ST",

            Country::SV => "SV",

            Country::SY => "SY",

            Country::SZ => "SZ",

            Country::TC => "TC",

            Country::TD => "TD",

            Country::TF => "TF",

            Country::TG => "TG",

            Country::TH => "TH",

            Country::TJ => "TJ",

            Country::TK => "TK",

            Country::TL => "TL",

            Country::TM => "TM",

            Country::TN => "TN",

            Country::TO => "TO",

            Country::TR => "TR",

            Country::TT => "TT",

            Country::TV => "TV",

            Country::TW => "TW",

            Country::TZ => "TZ",

            Country::UA => "UA",

            Country::UG => "UG",

            Country::UM => "UM",

            Country::US => "US",

            Country::UY => "UY",

            Country::UZ => "UZ",

            Country::VA => "VA",

            Country::VC => "VC",

            Country::VE => "VE",

            Country::VG => "VG",

            Country::VI => "VI",

            Country::VN => "VN",

            Country::VU => "VU",

            Country::WF => "WF",

            Country::WS => "WS",

            Country::YE => "YE",

            Country::YT => "YT",

            Country::ZA => "ZA",

            Country::ZM => "ZM",

            Country::ZW => "ZW",

            Country::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Country::AD => "AD".into(),

            Country::AE => "AE".into(),

            Country::AF => "AF".into(),

            Country::AG => "AG".into(),

            Country::AI => "AI".into(),

            Country::AL => "AL".into(),

            Country::AM => "AM".into(),

            Country::AN => "AN".into(),

            Country::AO => "AO".into(),

            Country::AQ => "AQ".into(),

            Country::AR => "AR".into(),

            Country::AS => "AS".into(),

            Country::AT => "AT".into(),

            Country::AU => "AU".into(),

            Country::AW => "AW".into(),

            Country::AX => "AX".into(),

            Country::AZ => "AZ".into(),

            Country::BA => "BA".into(),

            Country::BB => "BB".into(),

            Country::BD => "BD".into(),

            Country::BE => "BE".into(),

            Country::BF => "BF".into(),

            Country::BG => "BG".into(),

            Country::BH => "BH".into(),

            Country::BI => "BI".into(),

            Country::BJ => "BJ".into(),

            Country::BL => "BL".into(),

            Country::BM => "BM".into(),

            Country::BN => "BN".into(),

            Country::BO => "BO".into(),

            Country::BR => "BR".into(),

            Country::BS => "BS".into(),

            Country::BT => "BT".into(),

            Country::BV => "BV".into(),

            Country::BW => "BW".into(),

            Country::BY => "BY".into(),

            Country::BZ => "BZ".into(),

            Country::CA => "CA".into(),

            Country::CC => "CC".into(),

            Country::CD => "CD".into(),

            Country::CF => "CF".into(),

            Country::CG => "CG".into(),

            Country::CH => "CH".into(),

            Country::CI => "CI".into(),

            Country::CK => "CK".into(),

            Country::CL => "CL".into(),

            Country::CM => "CM".into(),

            Country::CN => "CN".into(),

            Country::CO => "CO".into(),

            Country::CR => "CR".into(),

            Country::CU => "CU".into(),

            Country::CV => "CV".into(),

            Country::CX => "CX".into(),

            Country::CY => "CY".into(),

            Country::CZ => "CZ".into(),

            Country::DE => "DE".into(),

            Country::DJ => "DJ".into(),

            Country::DK => "DK".into(),

            Country::DM => "DM".into(),

            Country::DO => "DO".into(),

            Country::DZ => "DZ".into(),

            Country::EC => "EC".into(),

            Country::EE => "EE".into(),

            Country::EG => "EG".into(),

            Country::EH => "EH".into(),

            Country::ER => "ER".into(),

            Country::ES => "ES".into(),

            Country::ET => "ET".into(),

            Country::FI => "FI".into(),

            Country::FJ => "FJ".into(),

            Country::FK => "FK".into(),

            Country::FM => "FM".into(),

            Country::FO => "FO".into(),

            Country::FR => "FR".into(),

            Country::GA => "GA".into(),

            Country::GB => "GB".into(),

            Country::GD => "GD".into(),

            Country::GE => "GE".into(),

            Country::GF => "GF".into(),

            Country::GG => "GG".into(),

            Country::GH => "GH".into(),

            Country::GI => "GI".into(),

            Country::GL => "GL".into(),

            Country::GM => "GM".into(),

            Country::GN => "GN".into(),

            Country::GP => "GP".into(),

            Country::GQ => "GQ".into(),

            Country::GR => "GR".into(),

            Country::GS => "GS".into(),

            Country::GT => "GT".into(),

            Country::GU => "GU".into(),

            Country::GW => "GW".into(),

            Country::GY => "GY".into(),

            Country::HK => "HK".into(),

            Country::HM => "HM".into(),

            Country::HN => "HN".into(),

            Country::HR => "HR".into(),

            Country::HT => "HT".into(),

            Country::HU => "HU".into(),

            Country::ID => "ID".into(),

            Country::IE => "IE".into(),

            Country::IL => "IL".into(),

            Country::IM => "IM".into(),

            Country::IN => "IN".into(),

            Country::IO => "IO".into(),

            Country::IQ => "IQ".into(),

            Country::IR => "IR".into(),

            Country::IS => "IS".into(),

            Country::IT => "IT".into(),

            Country::JE => "JE".into(),

            Country::JM => "JM".into(),

            Country::JO => "JO".into(),

            Country::JP => "JP".into(),

            Country::KE => "KE".into(),

            Country::KG => "KG".into(),

            Country::KH => "KH".into(),

            Country::KI => "KI".into(),

            Country::KM => "KM".into(),

            Country::KN => "KN".into(),

            Country::KP => "KP".into(),

            Country::KR => "KR".into(),

            Country::KW => "KW".into(),

            Country::KY => "KY".into(),

            Country::KZ => "KZ".into(),

            Country::LA => "LA".into(),

            Country::LB => "LB".into(),

            Country::LC => "LC".into(),

            Country::LI => "LI".into(),

            Country::LK => "LK".into(),

            Country::LR => "LR".into(),

            Country::LS => "LS".into(),

            Country::LT => "LT".into(),

            Country::LU => "LU".into(),

            Country::LV => "LV".into(),

            Country::LY => "LY".into(),

            Country::MA => "MA".into(),

            Country::MC => "MC".into(),

            Country::MD => "MD".into(),

            Country::ME => "ME".into(),

            Country::MF => "MF".into(),

            Country::MG => "MG".into(),

            Country::MH => "MH".into(),

            Country::MK => "MK".into(),

            Country::ML => "ML".into(),

            Country::MM => "MM".into(),

            Country::MN => "MN".into(),

            Country::MO => "MO".into(),

            Country::MP => "MP".into(),

            Country::MQ => "MQ".into(),

            Country::MR => "MR".into(),

            Country::MS => "MS".into(),

            Country::MT => "MT".into(),

            Country::MU => "MU".into(),

            Country::MV => "MV".into(),

            Country::MW => "MW".into(),

            Country::MX => "MX".into(),

            Country::MY => "MY".into(),

            Country::MZ => "MZ".into(),

            Country::NA => "NA".into(),

            Country::NC => "NC".into(),

            Country::NE => "NE".into(),

            Country::NF => "NF".into(),

            Country::NG => "NG".into(),

            Country::NI => "NI".into(),

            Country::NL => "NL".into(),

            Country::NP => "NP".into(),

            Country::NR => "NR".into(),

            Country::NU => "NU".into(),

            Country::NZ => "NZ".into(),

            Country::OM => "OM".into(),

            Country::OT => "OT".into(),

            Country::PA => "PA".into(),

            Country::PE => "PE".into(),

            Country::PF => "PF".into(),

            Country::PG => "PG".into(),

            Country::PH => "PH".into(),

            Country::PK => "PK".into(),

            Country::PL => "PL".into(),

            Country::PM => "PM".into(),

            Country::PN => "PN".into(),

            Country::PR => "PR".into(),

            Country::PS => "PS".into(),

            Country::PT => "PT".into(),

            Country::PW => "PW".into(),

            Country::PY => "PY".into(),

            Country::QA => "QA".into(),

            Country::RE => "RE".into(),

            Country::RO => "RO".into(),

            Country::RS => "RS".into(),

            Country::RU => "RU".into(),

            Country::RW => "RW".into(),

            Country::SA => "SA".into(),

            Country::SB => "SB".into(),

            Country::SC => "SC".into(),

            Country::SD => "SD".into(),

            Country::SE => "SE".into(),

            Country::SG => "SG".into(),

            Country::SH => "SH".into(),

            Country::SI => "SI".into(),

            Country::SJ => "SJ".into(),

            Country::SK => "SK".into(),

            Country::SL => "SL".into(),

            Country::SM => "SM".into(),

            Country::SN => "SN".into(),

            Country::SO => "SO".into(),

            Country::SR => "SR".into(),

            Country::ST => "ST".into(),

            Country::SV => "SV".into(),

            Country::SY => "SY".into(),

            Country::SZ => "SZ".into(),

            Country::TC => "TC".into(),

            Country::TD => "TD".into(),

            Country::TF => "TF".into(),

            Country::TG => "TG".into(),

            Country::TH => "TH".into(),

            Country::TJ => "TJ".into(),

            Country::TK => "TK".into(),

            Country::TL => "TL".into(),

            Country::TM => "TM".into(),

            Country::TN => "TN".into(),

            Country::TO => "TO".into(),

            Country::TR => "TR".into(),

            Country::TT => "TT".into(),

            Country::TV => "TV".into(),

            Country::TW => "TW".into(),

            Country::TZ => "TZ".into(),

            Country::UA => "UA".into(),

            Country::UG => "UG".into(),

            Country::UM => "UM".into(),

            Country::US => "US".into(),

            Country::UY => "UY".into(),

            Country::UZ => "UZ".into(),

            Country::VA => "VA".into(),

            Country::VC => "VC".into(),

            Country::VE => "VE".into(),

            Country::VG => "VG".into(),

            Country::VI => "VI".into(),

            Country::VN => "VN".into(),

            Country::VU => "VU".into(),

            Country::WF => "WF".into(),

            Country::WS => "WS".into(),

            Country::YE => "YE".into(),

            Country::YT => "YT".into(),

            Country::ZA => "ZA".into(),

            Country::ZM => "ZM".into(),

            Country::ZW => "ZW".into(),

            Country::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Country::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Country {
    fn from(s: String) -> Country {
        match s.as_ref() {
            "AD" => Country::AD,

            "AE" => Country::AE,

            "AF" => Country::AF,

            "AG" => Country::AG,

            "AI" => Country::AI,

            "AL" => Country::AL,

            "AM" => Country::AM,

            "AN" => Country::AN,

            "AO" => Country::AO,

            "AQ" => Country::AQ,

            "AR" => Country::AR,

            "AS" => Country::AS,

            "AT" => Country::AT,

            "AU" => Country::AU,

            "AW" => Country::AW,

            "AX" => Country::AX,

            "AZ" => Country::AZ,

            "BA" => Country::BA,

            "BB" => Country::BB,

            "BD" => Country::BD,

            "BE" => Country::BE,

            "BF" => Country::BF,

            "BG" => Country::BG,

            "BH" => Country::BH,

            "BI" => Country::BI,

            "BJ" => Country::BJ,

            "BL" => Country::BL,

            "BM" => Country::BM,

            "BN" => Country::BN,

            "BO" => Country::BO,

            "BR" => Country::BR,

            "BS" => Country::BS,

            "BT" => Country::BT,

            "BV" => Country::BV,

            "BW" => Country::BW,

            "BY" => Country::BY,

            "BZ" => Country::BZ,

            "CA" => Country::CA,

            "CC" => Country::CC,

            "CD" => Country::CD,

            "CF" => Country::CF,

            "CG" => Country::CG,

            "CH" => Country::CH,

            "CI" => Country::CI,

            "CK" => Country::CK,

            "CL" => Country::CL,

            "CM" => Country::CM,

            "CN" => Country::CN,

            "CO" => Country::CO,

            "CR" => Country::CR,

            "CU" => Country::CU,

            "CV" => Country::CV,

            "CX" => Country::CX,

            "CY" => Country::CY,

            "CZ" => Country::CZ,

            "DE" => Country::DE,

            "DJ" => Country::DJ,

            "DK" => Country::DK,

            "DM" => Country::DM,

            "DO" => Country::DO,

            "DZ" => Country::DZ,

            "EC" => Country::EC,

            "EE" => Country::EE,

            "EG" => Country::EG,

            "EH" => Country::EH,

            "ER" => Country::ER,

            "ES" => Country::ES,

            "ET" => Country::ET,

            "FI" => Country::FI,

            "FJ" => Country::FJ,

            "FK" => Country::FK,

            "FM" => Country::FM,

            "FO" => Country::FO,

            "FR" => Country::FR,

            "GA" => Country::GA,

            "GB" => Country::GB,

            "GD" => Country::GD,

            "GE" => Country::GE,

            "GF" => Country::GF,

            "GG" => Country::GG,

            "GH" => Country::GH,

            "GI" => Country::GI,

            "GL" => Country::GL,

            "GM" => Country::GM,

            "GN" => Country::GN,

            "GP" => Country::GP,

            "GQ" => Country::GQ,

            "GR" => Country::GR,

            "GS" => Country::GS,

            "GT" => Country::GT,

            "GU" => Country::GU,

            "GW" => Country::GW,

            "GY" => Country::GY,

            "HK" => Country::HK,

            "HM" => Country::HM,

            "HN" => Country::HN,

            "HR" => Country::HR,

            "HT" => Country::HT,

            "HU" => Country::HU,

            "ID" => Country::ID,

            "IE" => Country::IE,

            "IL" => Country::IL,

            "IM" => Country::IM,

            "IN" => Country::IN,

            "IO" => Country::IO,

            "IQ" => Country::IQ,

            "IR" => Country::IR,

            "IS" => Country::IS,

            "IT" => Country::IT,

            "JE" => Country::JE,

            "JM" => Country::JM,

            "JO" => Country::JO,

            "JP" => Country::JP,

            "KE" => Country::KE,

            "KG" => Country::KG,

            "KH" => Country::KH,

            "KI" => Country::KI,

            "KM" => Country::KM,

            "KN" => Country::KN,

            "KP" => Country::KP,

            "KR" => Country::KR,

            "KW" => Country::KW,

            "KY" => Country::KY,

            "KZ" => Country::KZ,

            "LA" => Country::LA,

            "LB" => Country::LB,

            "LC" => Country::LC,

            "LI" => Country::LI,

            "LK" => Country::LK,

            "LR" => Country::LR,

            "LS" => Country::LS,

            "LT" => Country::LT,

            "LU" => Country::LU,

            "LV" => Country::LV,

            "LY" => Country::LY,

            "MA" => Country::MA,

            "MC" => Country::MC,

            "MD" => Country::MD,

            "ME" => Country::ME,

            "MF" => Country::MF,

            "MG" => Country::MG,

            "MH" => Country::MH,

            "MK" => Country::MK,

            "ML" => Country::ML,

            "MM" => Country::MM,

            "MN" => Country::MN,

            "MO" => Country::MO,

            "MP" => Country::MP,

            "MQ" => Country::MQ,

            "MR" => Country::MR,

            "MS" => Country::MS,

            "MT" => Country::MT,

            "MU" => Country::MU,

            "MV" => Country::MV,

            "MW" => Country::MW,

            "MX" => Country::MX,

            "MY" => Country::MY,

            "MZ" => Country::MZ,

            "NA" => Country::NA,

            "NC" => Country::NC,

            "NE" => Country::NE,

            "NF" => Country::NF,

            "NG" => Country::NG,

            "NI" => Country::NI,

            "NL" => Country::NL,

            "NP" => Country::NP,

            "NR" => Country::NR,

            "NU" => Country::NU,

            "NZ" => Country::NZ,

            "OM" => Country::OM,

            "OT" => Country::OT,

            "PA" => Country::PA,

            "PE" => Country::PE,

            "PF" => Country::PF,

            "PG" => Country::PG,

            "PH" => Country::PH,

            "PK" => Country::PK,

            "PL" => Country::PL,

            "PM" => Country::PM,

            "PN" => Country::PN,

            "PR" => Country::PR,

            "PS" => Country::PS,

            "PT" => Country::PT,

            "PW" => Country::PW,

            "PY" => Country::PY,

            "QA" => Country::QA,

            "RE" => Country::RE,

            "RO" => Country::RO,

            "RS" => Country::RS,

            "RU" => Country::RU,

            "RW" => Country::RW,

            "SA" => Country::SA,

            "SB" => Country::SB,

            "SC" => Country::SC,

            "SD" => Country::SD,

            "SE" => Country::SE,

            "SG" => Country::SG,

            "SH" => Country::SH,

            "SI" => Country::SI,

            "SJ" => Country::SJ,

            "SK" => Country::SK,

            "SL" => Country::SL,

            "SM" => Country::SM,

            "SN" => Country::SN,

            "SO" => Country::SO,

            "SR" => Country::SR,

            "ST" => Country::ST,

            "SV" => Country::SV,

            "SY" => Country::SY,

            "SZ" => Country::SZ,

            "TC" => Country::TC,

            "TD" => Country::TD,

            "TF" => Country::TF,

            "TG" => Country::TG,

            "TH" => Country::TH,

            "TJ" => Country::TJ,

            "TK" => Country::TK,

            "TL" => Country::TL,

            "TM" => Country::TM,

            "TN" => Country::TN,

            "TO" => Country::TO,

            "TR" => Country::TR,

            "TT" => Country::TT,

            "TV" => Country::TV,

            "TW" => Country::TW,

            "TZ" => Country::TZ,

            "UA" => Country::UA,

            "UG" => Country::UG,

            "UM" => Country::UM,

            "US" => Country::US,

            "UY" => Country::UY,

            "UZ" => Country::UZ,

            "VA" => Country::VA,

            "VC" => Country::VC,

            "VE" => Country::VE,

            "VG" => Country::VG,

            "VI" => Country::VI,

            "VN" => Country::VN,

            "VU" => Country::VU,

            "WF" => Country::WF,

            "WS" => Country::WS,

            "YE" => Country::YE,

            "YT" => Country::YT,

            "ZA" => Country::ZA,

            "ZM" => Country::ZM,

            "ZW" => Country::ZW,

            _ => Country::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Country {
    fn from(s: &str) -> Country {
        match s {
            "AD" => Country::AD,

            "AE" => Country::AE,

            "AF" => Country::AF,

            "AG" => Country::AG,

            "AI" => Country::AI,

            "AL" => Country::AL,

            "AM" => Country::AM,

            "AN" => Country::AN,

            "AO" => Country::AO,

            "AQ" => Country::AQ,

            "AR" => Country::AR,

            "AS" => Country::AS,

            "AT" => Country::AT,

            "AU" => Country::AU,

            "AW" => Country::AW,

            "AX" => Country::AX,

            "AZ" => Country::AZ,

            "BA" => Country::BA,

            "BB" => Country::BB,

            "BD" => Country::BD,

            "BE" => Country::BE,

            "BF" => Country::BF,

            "BG" => Country::BG,

            "BH" => Country::BH,

            "BI" => Country::BI,

            "BJ" => Country::BJ,

            "BL" => Country::BL,

            "BM" => Country::BM,

            "BN" => Country::BN,

            "BO" => Country::BO,

            "BR" => Country::BR,

            "BS" => Country::BS,

            "BT" => Country::BT,

            "BV" => Country::BV,

            "BW" => Country::BW,

            "BY" => Country::BY,

            "BZ" => Country::BZ,

            "CA" => Country::CA,

            "CC" => Country::CC,

            "CD" => Country::CD,

            "CF" => Country::CF,

            "CG" => Country::CG,

            "CH" => Country::CH,

            "CI" => Country::CI,

            "CK" => Country::CK,

            "CL" => Country::CL,

            "CM" => Country::CM,

            "CN" => Country::CN,

            "CO" => Country::CO,

            "CR" => Country::CR,

            "CU" => Country::CU,

            "CV" => Country::CV,

            "CX" => Country::CX,

            "CY" => Country::CY,

            "CZ" => Country::CZ,

            "DE" => Country::DE,

            "DJ" => Country::DJ,

            "DK" => Country::DK,

            "DM" => Country::DM,

            "DO" => Country::DO,

            "DZ" => Country::DZ,

            "EC" => Country::EC,

            "EE" => Country::EE,

            "EG" => Country::EG,

            "EH" => Country::EH,

            "ER" => Country::ER,

            "ES" => Country::ES,

            "ET" => Country::ET,

            "FI" => Country::FI,

            "FJ" => Country::FJ,

            "FK" => Country::FK,

            "FM" => Country::FM,

            "FO" => Country::FO,

            "FR" => Country::FR,

            "GA" => Country::GA,

            "GB" => Country::GB,

            "GD" => Country::GD,

            "GE" => Country::GE,

            "GF" => Country::GF,

            "GG" => Country::GG,

            "GH" => Country::GH,

            "GI" => Country::GI,

            "GL" => Country::GL,

            "GM" => Country::GM,

            "GN" => Country::GN,

            "GP" => Country::GP,

            "GQ" => Country::GQ,

            "GR" => Country::GR,

            "GS" => Country::GS,

            "GT" => Country::GT,

            "GU" => Country::GU,

            "GW" => Country::GW,

            "GY" => Country::GY,

            "HK" => Country::HK,

            "HM" => Country::HM,

            "HN" => Country::HN,

            "HR" => Country::HR,

            "HT" => Country::HT,

            "HU" => Country::HU,

            "ID" => Country::ID,

            "IE" => Country::IE,

            "IL" => Country::IL,

            "IM" => Country::IM,

            "IN" => Country::IN,

            "IO" => Country::IO,

            "IQ" => Country::IQ,

            "IR" => Country::IR,

            "IS" => Country::IS,

            "IT" => Country::IT,

            "JE" => Country::JE,

            "JM" => Country::JM,

            "JO" => Country::JO,

            "JP" => Country::JP,

            "KE" => Country::KE,

            "KG" => Country::KG,

            "KH" => Country::KH,

            "KI" => Country::KI,

            "KM" => Country::KM,

            "KN" => Country::KN,

            "KP" => Country::KP,

            "KR" => Country::KR,

            "KW" => Country::KW,

            "KY" => Country::KY,

            "KZ" => Country::KZ,

            "LA" => Country::LA,

            "LB" => Country::LB,

            "LC" => Country::LC,

            "LI" => Country::LI,

            "LK" => Country::LK,

            "LR" => Country::LR,

            "LS" => Country::LS,

            "LT" => Country::LT,

            "LU" => Country::LU,

            "LV" => Country::LV,

            "LY" => Country::LY,

            "MA" => Country::MA,

            "MC" => Country::MC,

            "MD" => Country::MD,

            "ME" => Country::ME,

            "MF" => Country::MF,

            "MG" => Country::MG,

            "MH" => Country::MH,

            "MK" => Country::MK,

            "ML" => Country::ML,

            "MM" => Country::MM,

            "MN" => Country::MN,

            "MO" => Country::MO,

            "MP" => Country::MP,

            "MQ" => Country::MQ,

            "MR" => Country::MR,

            "MS" => Country::MS,

            "MT" => Country::MT,

            "MU" => Country::MU,

            "MV" => Country::MV,

            "MW" => Country::MW,

            "MX" => Country::MX,

            "MY" => Country::MY,

            "MZ" => Country::MZ,

            "NA" => Country::NA,

            "NC" => Country::NC,

            "NE" => Country::NE,

            "NF" => Country::NF,

            "NG" => Country::NG,

            "NI" => Country::NI,

            "NL" => Country::NL,

            "NP" => Country::NP,

            "NR" => Country::NR,

            "NU" => Country::NU,

            "NZ" => Country::NZ,

            "OM" => Country::OM,

            "OT" => Country::OT,

            "PA" => Country::PA,

            "PE" => Country::PE,

            "PF" => Country::PF,

            "PG" => Country::PG,

            "PH" => Country::PH,

            "PK" => Country::PK,

            "PL" => Country::PL,

            "PM" => Country::PM,

            "PN" => Country::PN,

            "PR" => Country::PR,

            "PS" => Country::PS,

            "PT" => Country::PT,

            "PW" => Country::PW,

            "PY" => Country::PY,

            "QA" => Country::QA,

            "RE" => Country::RE,

            "RO" => Country::RO,

            "RS" => Country::RS,

            "RU" => Country::RU,

            "RW" => Country::RW,

            "SA" => Country::SA,

            "SB" => Country::SB,

            "SC" => Country::SC,

            "SD" => Country::SD,

            "SE" => Country::SE,

            "SG" => Country::SG,

            "SH" => Country::SH,

            "SI" => Country::SI,

            "SJ" => Country::SJ,

            "SK" => Country::SK,

            "SL" => Country::SL,

            "SM" => Country::SM,

            "SN" => Country::SN,

            "SO" => Country::SO,

            "SR" => Country::SR,

            "ST" => Country::ST,

            "SV" => Country::SV,

            "SY" => Country::SY,

            "SZ" => Country::SZ,

            "TC" => Country::TC,

            "TD" => Country::TD,

            "TF" => Country::TF,

            "TG" => Country::TG,

            "TH" => Country::TH,

            "TJ" => Country::TJ,

            "TK" => Country::TK,

            "TL" => Country::TL,

            "TM" => Country::TM,

            "TN" => Country::TN,

            "TO" => Country::TO,

            "TR" => Country::TR,

            "TT" => Country::TT,

            "TV" => Country::TV,

            "TW" => Country::TW,

            "TZ" => Country::TZ,

            "UA" => Country::UA,

            "UG" => Country::UG,

            "UM" => Country::UM,

            "US" => Country::US,

            "UY" => Country::UY,

            "UZ" => Country::UZ,

            "VA" => Country::VA,

            "VC" => Country::VC,

            "VE" => Country::VE,

            "VG" => Country::VG,

            "VI" => Country::VI,

            "VN" => Country::VN,

            "VU" => Country::VU,

            "WF" => Country::WF,

            "WS" => Country::WS,

            "YE" => Country::YE,

            "YT" => Country::YT,

            "ZA" => Country::ZA,

            "ZM" => Country::ZM,

            "ZW" => Country::ZW,

            _ => Country::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Country> for &'a str {
    fn from(s: &'a Country) -> &'a str {
        match s {
            Country::AD => "AD",

            Country::AE => "AE",

            Country::AF => "AF",

            Country::AG => "AG",

            Country::AI => "AI",

            Country::AL => "AL",

            Country::AM => "AM",

            Country::AN => "AN",

            Country::AO => "AO",

            Country::AQ => "AQ",

            Country::AR => "AR",

            Country::AS => "AS",

            Country::AT => "AT",

            Country::AU => "AU",

            Country::AW => "AW",

            Country::AX => "AX",

            Country::AZ => "AZ",

            Country::BA => "BA",

            Country::BB => "BB",

            Country::BD => "BD",

            Country::BE => "BE",

            Country::BF => "BF",

            Country::BG => "BG",

            Country::BH => "BH",

            Country::BI => "BI",

            Country::BJ => "BJ",

            Country::BL => "BL",

            Country::BM => "BM",

            Country::BN => "BN",

            Country::BO => "BO",

            Country::BR => "BR",

            Country::BS => "BS",

            Country::BT => "BT",

            Country::BV => "BV",

            Country::BW => "BW",

            Country::BY => "BY",

            Country::BZ => "BZ",

            Country::CA => "CA",

            Country::CC => "CC",

            Country::CD => "CD",

            Country::CF => "CF",

            Country::CG => "CG",

            Country::CH => "CH",

            Country::CI => "CI",

            Country::CK => "CK",

            Country::CL => "CL",

            Country::CM => "CM",

            Country::CN => "CN",

            Country::CO => "CO",

            Country::CR => "CR",

            Country::CU => "CU",

            Country::CV => "CV",

            Country::CX => "CX",

            Country::CY => "CY",

            Country::CZ => "CZ",

            Country::DE => "DE",

            Country::DJ => "DJ",

            Country::DK => "DK",

            Country::DM => "DM",

            Country::DO => "DO",

            Country::DZ => "DZ",

            Country::EC => "EC",

            Country::EE => "EE",

            Country::EG => "EG",

            Country::EH => "EH",

            Country::ER => "ER",

            Country::ES => "ES",

            Country::ET => "ET",

            Country::FI => "FI",

            Country::FJ => "FJ",

            Country::FK => "FK",

            Country::FM => "FM",

            Country::FO => "FO",

            Country::FR => "FR",

            Country::GA => "GA",

            Country::GB => "GB",

            Country::GD => "GD",

            Country::GE => "GE",

            Country::GF => "GF",

            Country::GG => "GG",

            Country::GH => "GH",

            Country::GI => "GI",

            Country::GL => "GL",

            Country::GM => "GM",

            Country::GN => "GN",

            Country::GP => "GP",

            Country::GQ => "GQ",

            Country::GR => "GR",

            Country::GS => "GS",

            Country::GT => "GT",

            Country::GU => "GU",

            Country::GW => "GW",

            Country::GY => "GY",

            Country::HK => "HK",

            Country::HM => "HM",

            Country::HN => "HN",

            Country::HR => "HR",

            Country::HT => "HT",

            Country::HU => "HU",

            Country::ID => "ID",

            Country::IE => "IE",

            Country::IL => "IL",

            Country::IM => "IM",

            Country::IN => "IN",

            Country::IO => "IO",

            Country::IQ => "IQ",

            Country::IR => "IR",

            Country::IS => "IS",

            Country::IT => "IT",

            Country::JE => "JE",

            Country::JM => "JM",

            Country::JO => "JO",

            Country::JP => "JP",

            Country::KE => "KE",

            Country::KG => "KG",

            Country::KH => "KH",

            Country::KI => "KI",

            Country::KM => "KM",

            Country::KN => "KN",

            Country::KP => "KP",

            Country::KR => "KR",

            Country::KW => "KW",

            Country::KY => "KY",

            Country::KZ => "KZ",

            Country::LA => "LA",

            Country::LB => "LB",

            Country::LC => "LC",

            Country::LI => "LI",

            Country::LK => "LK",

            Country::LR => "LR",

            Country::LS => "LS",

            Country::LT => "LT",

            Country::LU => "LU",

            Country::LV => "LV",

            Country::LY => "LY",

            Country::MA => "MA",

            Country::MC => "MC",

            Country::MD => "MD",

            Country::ME => "ME",

            Country::MF => "MF",

            Country::MG => "MG",

            Country::MH => "MH",

            Country::MK => "MK",

            Country::ML => "ML",

            Country::MM => "MM",

            Country::MN => "MN",

            Country::MO => "MO",

            Country::MP => "MP",

            Country::MQ => "MQ",

            Country::MR => "MR",

            Country::MS => "MS",

            Country::MT => "MT",

            Country::MU => "MU",

            Country::MV => "MV",

            Country::MW => "MW",

            Country::MX => "MX",

            Country::MY => "MY",

            Country::MZ => "MZ",

            Country::NA => "NA",

            Country::NC => "NC",

            Country::NE => "NE",

            Country::NF => "NF",

            Country::NG => "NG",

            Country::NI => "NI",

            Country::NL => "NL",

            Country::NP => "NP",

            Country::NR => "NR",

            Country::NU => "NU",

            Country::NZ => "NZ",

            Country::OM => "OM",

            Country::OT => "OT",

            Country::PA => "PA",

            Country::PE => "PE",

            Country::PF => "PF",

            Country::PG => "PG",

            Country::PH => "PH",

            Country::PK => "PK",

            Country::PL => "PL",

            Country::PM => "PM",

            Country::PN => "PN",

            Country::PR => "PR",

            Country::PS => "PS",

            Country::PT => "PT",

            Country::PW => "PW",

            Country::PY => "PY",

            Country::QA => "QA",

            Country::RE => "RE",

            Country::RO => "RO",

            Country::RS => "RS",

            Country::RU => "RU",

            Country::RW => "RW",

            Country::SA => "SA",

            Country::SB => "SB",

            Country::SC => "SC",

            Country::SD => "SD",

            Country::SE => "SE",

            Country::SG => "SG",

            Country::SH => "SH",

            Country::SI => "SI",

            Country::SJ => "SJ",

            Country::SK => "SK",

            Country::SL => "SL",

            Country::SM => "SM",

            Country::SN => "SN",

            Country::SO => "SO",

            Country::SR => "SR",

            Country::ST => "ST",

            Country::SV => "SV",

            Country::SY => "SY",

            Country::SZ => "SZ",

            Country::TC => "TC",

            Country::TD => "TD",

            Country::TF => "TF",

            Country::TG => "TG",

            Country::TH => "TH",

            Country::TJ => "TJ",

            Country::TK => "TK",

            Country::TL => "TL",

            Country::TM => "TM",

            Country::TN => "TN",

            Country::TO => "TO",

            Country::TR => "TR",

            Country::TT => "TT",

            Country::TV => "TV",

            Country::TW => "TW",

            Country::TZ => "TZ",

            Country::UA => "UA",

            Country::UG => "UG",

            Country::UM => "UM",

            Country::US => "US",

            Country::UY => "UY",

            Country::UZ => "UZ",

            Country::VA => "VA",

            Country::VC => "VC",

            Country::VE => "VE",

            Country::VG => "VG",

            Country::VI => "VI",

            Country::VN => "VN",

            Country::VU => "VU",

            Country::WF => "WF",

            Country::WS => "WS",

            Country::YE => "YE",

            Country::YT => "YT",

            Country::ZA => "ZA",

            Country::ZM => "ZM",

            Country::ZW => "ZW",

            Country::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Country {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Country {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
