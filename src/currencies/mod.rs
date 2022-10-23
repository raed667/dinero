#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Currency {
    pub code: CountryCode,
    pub base: u32,
    pub exponent: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum CountryCode {
    AED,
    BOV,
    CUP,
    GTQ,
    KHR,
    MOP,
    PHP,
    SSP,
    UYU,
    AFN,
    BRL,
    CVE,
    GYD,
    KMF,
    MRU,
    PKR,
    STN,
    UYW,
    ALL,
    BSD,
    CZK,
    HKD,
    KPW,
    MUR,
    PLN,
    SVC,
    UZS,
    AMD,
    BTN,
    DJF,
    HNL,
    KRW,
    MVR,
    PYG,
    SYP,
    VES,
    ANG,
    BWP,
    DKK,
    HRK,
    KWD,
    MWK,
    QAR,
    SZL,
    VND,
    AOA,
    BYN,
    DOP,
    HTG,
    KYD,
    MXN,
    RON,
    THB,
    VUV,
    ARS,
    BZD,
    DZD,
    HUF,
    KZT,
    MXV,
    RSD,
    TJS,
    WST,
    AUD,
    CAD,
    EGP,
    IDR,
    LAK,
    MYR,
    RUB,
    TMT,
    XAF,
    AWG,
    CDF,
    ERN,
    ILS,
    LBP,
    MZN,
    RWF,
    TND,
    XCD,
    AZN,
    CHE,
    ETB,
    LKR,
    NAD,
    SAR,
    TOP,
    XOF,
    BAM,
    CHF,
    EUR,
    INR,
    LRD,
    NGN,
    SBD,
    TRY,
    XPF,
    BBD,
    CHW,
    FJD,
    IQD,
    LSL,
    NIO,
    SCR,
    TTD,
    YER,
    BDT,
    CLF,
    FKP,
    IRR,
    LYD,
    NOK,
    SDG,
    TWD,
    ZAR,
    BGN,
    CLP,
    GBP,
    ISK,
    MAD,
    NPR,
    SEK,
    TZS,
    ZMW,
    BHD,
    CNY,
    GEL,
    JMD,
    MDL,
    NZD,
    SGD,
    UAH,
    ZWL,
    BIF,
    COP,
    GHS,
    JOD,
    MGA,
    OMR,
    SHP,
    UGX,
    BMD,
    COU,
    GIP,
    JPY,
    MKD,
    PAB,
    SLL,
    USD,
    BND,
    CRC,
    GMD,
    KES,
    MMK,
    PEN,
    SOS,
    USN,
    BOB,
    CUC,
    GNF,
    KGS,
    MNT,
    PGK,
    SRD,
    UYI,
    CUSTOM,
}

/**
 * United Arab Emirates dirham.
 */
pub const AED: Currency = Currency {
    code: CountryCode::AED,
    base: 10,
    exponent: 2,
};

/**
 * Afghan afghani.
 */
pub const AFN: Currency = Currency {
    code: CountryCode::AFN,
    base: 10,
    exponent: 2,
};

/**
 * Albanian lek.
 */
pub const ALL: Currency = Currency {
    code: CountryCode::ALL,
    base: 10,
    exponent: 2,
};

/**
 * Armenian dram.
 */
pub const AMD: Currency = Currency {
    code: CountryCode::AMD,
    base: 10,
    exponent: 2,
};

/**
 * Netherlands Antillean guilder.
 */
pub const ANG: Currency = Currency {
    code: CountryCode::ANG,
    base: 10,
    exponent: 2,
};

/**
 * Angolan kwanza.
 */
pub const AOA: Currency = Currency {
    code: CountryCode::AOA,
    base: 10,
    exponent: 2,
};

/**
 * Argentine peso.
 */
pub const ARS: Currency = Currency {
    code: CountryCode::ARS,
    base: 10,
    exponent: 2,
};

/**
 * Australian dollar.
 */
pub const AUD: Currency = Currency {
    code: CountryCode::AUD,
    base: 10,
    exponent: 2,
};

/**
 * Aruban florin.
 */
pub const AWG: Currency = Currency {
    code: CountryCode::AWG,
    base: 10,
    exponent: 2,
};

/**
 * Azerbaijani manat.
 */
pub const AZN: Currency = Currency {
    code: CountryCode::AZN,
    base: 10,
    exponent: 2,
};

/**
 * Bosnia and Herzegovina convertible mark.
 */
pub const BAM: Currency = Currency {
    code: CountryCode::BAM,
    base: 10,
    exponent: 2,
};

/**
 * Barbados dollar.
 */
pub const BBD: Currency = Currency {
    code: CountryCode::BBD,
    base: 10,
    exponent: 2,
};

/**
 * Bangladeshi taka.
 */
pub const BDT: Currency = Currency {
    code: CountryCode::BDT,
    base: 10,
    exponent: 2,
};

/**
 * Bulgarian lev.
 */
pub const BGN: Currency = Currency {
    code: CountryCode::BGN,
    base: 10,
    exponent: 2,
};

/**
 * Bahraini dinar.
 */
pub const BHD: Currency = Currency {
    code: CountryCode::BHD,
    base: 10,
    exponent: 3,
};

/**
 * Burundian franc.
 */
pub const BIF: Currency = Currency {
    code: CountryCode::BIF,
    base: 10,
    exponent: 0,
};

/**
 * Bermudian dollar.
 */
pub const BMD: Currency = Currency {
    code: CountryCode::BMD,
    base: 10,
    exponent: 2,
};

/**
 * Brunei dollar.
 */
pub const BND: Currency = Currency {
    code: CountryCode::BND,
    base: 10,
    exponent: 2,
};

/**
 * Bolivian boliviano.
 */
pub const BOB: Currency = Currency {
    code: CountryCode::BOB,
    base: 10,
    exponent: 2,
};

/**
 * Bolivian Mvdol.
 */
pub const BOV: Currency = Currency {
    code: CountryCode::BOV,
    base: 10,
    exponent: 2,
};

/**
 * Brazilian real.
 */
pub const BRL: Currency = Currency {
    code: CountryCode::BRL,
    base: 10,
    exponent: 2,
};

/**
 * Bahamian dollar.
 */
pub const BSD: Currency = Currency {
    code: CountryCode::BSD,
    base: 10,
    exponent: 2,
};

/**
 * Bhutanese ngultrum.
 */
pub const BTN: Currency = Currency {
    code: CountryCode::BTN,
    base: 10,
    exponent: 2,
};

/**
 * Botswana pula.
 */
pub const BWP: Currency = Currency {
    code: CountryCode::BWP,
    base: 10,
    exponent: 2,
};

/**
 * Belarusian ruble.
 */
pub const BYN: Currency = Currency {
    code: CountryCode::BYN,
    base: 10,
    exponent: 2,
};

/**
 * Belize dollar.
 */
pub const BZD: Currency = Currency {
    code: CountryCode::BZD,
    base: 10,
    exponent: 2,
};

/**
 * Canadian dollar.
 */
pub const CAD: Currency = Currency {
    code: CountryCode::CAD,
    base: 10,
    exponent: 2,
};

/**
 * Congolese franc.
 */
pub const CDF: Currency = Currency {
    code: CountryCode::CDF,
    base: 10,
    exponent: 2,
};

/**
 * WIR Euro.
 */
pub const CHE: Currency = Currency {
    code: CountryCode::CHE,
    base: 10,
    exponent: 2,
};

/**
 * Swiss franc.
 */
pub const CHF: Currency = Currency {
    code: CountryCode::CHF,
    base: 10,
    exponent: 2,
};

/**
 * WIR Franc.
 */
pub const CHW: Currency = Currency {
    code: CountryCode::CHW,
    base: 10,
    exponent: 2,
};

/**
 * Unidad de Fomento.
 */
pub const CLF: Currency = Currency {
    code: CountryCode::CLF,
    base: 10,
    exponent: 4,
};

/**
 * Chilean peso.
 */
pub const CLP: Currency = Currency {
    code: CountryCode::CLP,
    base: 10,
    exponent: 0,
};

/**
 * Renminbi (Chinese) yuan.
 */
pub const CNY: Currency = Currency {
    code: CountryCode::CNY,
    base: 10,
    exponent: 2,
};

/**
 * Colombian peso.
 */
pub const COP: Currency = Currency {
    code: CountryCode::COP,
    base: 10,
    exponent: 2,
};

/**
 * Unidad de Valor Real.
 */
pub const COU: Currency = Currency {
    code: CountryCode::COU,
    base: 10,
    exponent: 2,
};

/**
 * Costa Rican colón.
 */
pub const CRC: Currency = Currency {
    code: CountryCode::CRC,
    base: 10,
    exponent: 2,
};

/**
 * Cuban convertible peso.
 */
pub const CUC: Currency = Currency {
    code: CountryCode::CUC,
    base: 10,
    exponent: 2,
};

/**
 * Cuban peso.
 */
pub const CUP: Currency = Currency {
    code: CountryCode::CUP,
    base: 10,
    exponent: 2,
};

/**
 * Cape Verdean escudo.
 */
pub const CVE: Currency = Currency {
    code: CountryCode::CVE,
    base: 10,
    exponent: 2,
};

/**
 * Czech koruna.
 */
pub const CZK: Currency = Currency {
    code: CountryCode::CZK,
    base: 10,
    exponent: 2,
};

/**
 * Djiboutian franc.
 */
pub const DJF: Currency = Currency {
    code: CountryCode::DJF,
    base: 10,
    exponent: 0,
};

/**
 * Danish krone.
 */
pub const DKK: Currency = Currency {
    code: CountryCode::DKK,
    base: 10,
    exponent: 2,
};

/**
 * Dominican peso.
 */
pub const DOP: Currency = Currency {
    code: CountryCode::DOP,
    base: 10,
    exponent: 2,
};

/**
 * Algerian dinar.
 */
pub const DZD: Currency = Currency {
    code: CountryCode::DZD,
    base: 10,
    exponent: 2,
};

/**
 * Egyptian pound.
 */
pub const EGP: Currency = Currency {
    code: CountryCode::EGP,
    base: 10,
    exponent: 2,
};

/**
 * Eritrean nakfa.
 */
pub const ERN: Currency = Currency {
    code: CountryCode::ERN,
    base: 10,
    exponent: 2,
};

/**
 * Ethiopian birr.
 */
pub const ETB: Currency = Currency {
    code: CountryCode::ETB,
    base: 10,
    exponent: 2,
};

/**
 * Euro.
 */
pub const EUR: Currency = Currency {
    code: CountryCode::EUR,
    base: 10,
    exponent: 2,
};

/**
 * Fiji dollar.
 */
pub const FJD: Currency = Currency {
    code: CountryCode::FJD,
    base: 10,
    exponent: 2,
};

/**
 * Falkland Islands pound.
 */
pub const FKP: Currency = Currency {
    code: CountryCode::FKP,
    base: 10,
    exponent: 2,
};

/**
 * Pound sterling.
 */
pub const GBP: Currency = Currency {
    code: CountryCode::GBP,
    base: 10,
    exponent: 2,
};

/**
 * Georgian lari.
 */
pub const GEL: Currency = Currency {
    code: CountryCode::GEL,
    base: 10,
    exponent: 2,
};

/**
 * Ghanaian cedi.
 */
pub const GHS: Currency = Currency {
    code: CountryCode::GHS,
    base: 10,
    exponent: 2,
};

/**
 * Gibraltar pound.
 */
pub const GIP: Currency = Currency {
    code: CountryCode::GIP,
    base: 10,
    exponent: 2,
};

/**
 * Gambian dalasi.
 */
pub const GMD: Currency = Currency {
    code: CountryCode::GMD,
    base: 10,
    exponent: 2,
};

/**
 * Guinean franc.
 */
pub const GNF: Currency = Currency {
    code: CountryCode::GNF,
    base: 10,
    exponent: 0,
};

/**
 * Guatemalan quetzal.
 */
pub const GTQ: Currency = Currency {
    code: CountryCode::GTQ,
    base: 10,
    exponent: 2,
};

/**
 * Guyanese dollar.
 */
pub const GYD: Currency = Currency {
    code: CountryCode::GYD,
    base: 10,
    exponent: 2,
};

/**
 * Hong Kong dollar.
 */
pub const HKD: Currency = Currency {
    code: CountryCode::HKD,
    base: 10,
    exponent: 2,
};

/**
 * Honduran lempira.
 */
pub const HNL: Currency = Currency {
    code: CountryCode::HNL,
    base: 10,
    exponent: 2,
};

/**
 * Croatian kuna.
 */
pub const HRK: Currency = Currency {
    code: CountryCode::HRK,
    base: 10,
    exponent: 2,
};

/**
 * Haitian gourde.
 */
pub const HTG: Currency = Currency {
    code: CountryCode::HTG,
    base: 10,
    exponent: 2,
};

/**
 * Hungarian forint.
 */
pub const HUF: Currency = Currency {
    code: CountryCode::HUF,
    base: 10,
    exponent: 2,
};

/**
 * Indonesian rupiah.
 */
pub const IDR: Currency = Currency {
    code: CountryCode::IDR,
    base: 10,
    exponent: 2,
};

/**
 * Israeli new shekel.
 */
pub const ILS: Currency = Currency {
    code: CountryCode::ILS,
    base: 10,
    exponent: 2,
};

/**
 * Indian rupee.
 */
pub const INR: Currency = Currency {
    code: CountryCode::INR,
    base: 10,
    exponent: 2,
};

/**
 * Iraqi dinar.
 */
pub const IQD: Currency = Currency {
    code: CountryCode::IQD,
    base: 10,
    exponent: 3,
};

/**
 * Iranian rial.
 */
pub const IRR: Currency = Currency {
    code: CountryCode::IRR,
    base: 10,
    exponent: 2,
};

/**
 * Icelandic króna.
 */
pub const ISK: Currency = Currency {
    code: CountryCode::ISK,
    base: 10,
    exponent: 0,
};

/**
 * Jamaican dollar.
 */
pub const JMD: Currency = Currency {
    code: CountryCode::JMD,
    base: 10,
    exponent: 2,
};

/**
 * Jordanian dinar.
 */
pub const JOD: Currency = Currency {
    code: CountryCode::JOD,
    base: 10,
    exponent: 3,
};

/**
 * Japanese yen.
 */
pub const JPY: Currency = Currency {
    code: CountryCode::JPY,
    base: 10,
    exponent: 0,
};

/**
 * Kenyan shilling.
 */
pub const KES: Currency = Currency {
    code: CountryCode::KES,
    base: 10,
    exponent: 2,
};

/**
 * Kyrgyzstani som.
 */
pub const KGS: Currency = Currency {
    code: CountryCode::KGS,
    base: 10,
    exponent: 2,
};

/**
 * Cambodian riel.
 */
pub const KHR: Currency = Currency {
    code: CountryCode::KHR,
    base: 10,
    exponent: 2,
};

/**
 * Comoro franc.
 */
pub const KMF: Currency = Currency {
    code: CountryCode::KMF,
    base: 10,
    exponent: 0,
};

/**
 * North Korean won.
 */
pub const KPW: Currency = Currency {
    code: CountryCode::KPW,
    base: 10,
    exponent: 2,
};

/**
 * South Korean won.
 */
pub const KRW: Currency = Currency {
    code: CountryCode::KRW,
    base: 10,
    exponent: 0,
};

/**
 * Kuwaiti dinar.
 */
pub const KWD: Currency = Currency {
    code: CountryCode::KWD,
    base: 10,
    exponent: 3,
};

/**
 * Cayman Islands dollar.
 */
pub const KYD: Currency = Currency {
    code: CountryCode::KYD,
    base: 10,
    exponent: 2,
};

/**
 * Kazakhstani tenge.
 */
pub const KZT: Currency = Currency {
    code: CountryCode::KZT,
    base: 10,
    exponent: 2,
};

/**
 * Lao kip.
 */
pub const LAK: Currency = Currency {
    code: CountryCode::LAK,
    base: 10,
    exponent: 2,
};

/**
 * Lebanese pound.
 */
pub const LBP: Currency = Currency {
    code: CountryCode::LBP,
    base: 10,
    exponent: 2,
};

/**
 * Sri Lankan rupee.
 */
pub const LKR: Currency = Currency {
    code: CountryCode::LKR,
    base: 10,
    exponent: 2,
};

/**
 * Liberian dollar.
 */
pub const LRD: Currency = Currency {
    code: CountryCode::LRD,
    base: 10,
    exponent: 2,
};

/**
 * Lesotho loti.
 */
pub const LSL: Currency = Currency {
    code: CountryCode::LSL,
    base: 10,
    exponent: 2,
};

/**
 * Libyan dinar.
 */
pub const LYD: Currency = Currency {
    code: CountryCode::LYD,
    base: 10,
    exponent: 3,
};

/**
 * Moroccan dirham.
 */
pub const MAD: Currency = Currency {
    code: CountryCode::MAD,
    base: 10,
    exponent: 2,
};

/**
 * Moldovan leu.
 */
pub const MDL: Currency = Currency {
    code: CountryCode::MDL,
    base: 10,
    exponent: 2,
};

/**
 * Malagasy ariary.
 */
pub const MGA: Currency = Currency {
    code: CountryCode::MGA,
    base: 5,
    exponent: 1,
};

/**
 * Macedonian denar.
 */
pub const MKD: Currency = Currency {
    code: CountryCode::MKD,
    base: 10,
    exponent: 2,
};

/**
 * Myanmar kyat.
 */
pub const MMK: Currency = Currency {
    code: CountryCode::MMK,
    base: 10,
    exponent: 2,
};

/**
 * Mongolian tögrög.
 */
pub const MNT: Currency = Currency {
    code: CountryCode::MNT,
    base: 10,
    exponent: 2,
};

/**
 * Macanese pataca.
 */
pub const MOP: Currency = Currency {
    code: CountryCode::MOP,
    base: 10,
    exponent: 2,
};

/**
 * Mauritanian ouguiya.
 */
pub const MRU: Currency = Currency {
    code: CountryCode::MRU,
    base: 5,
    exponent: 1,
};

/**
 * Mauritian rupee.
 */
pub const MUR: Currency = Currency {
    code: CountryCode::MUR,
    base: 10,
    exponent: 2,
};

/**
 * Maldivian rufiyaa.
 */
pub const MVR: Currency = Currency {
    code: CountryCode::MVR,
    base: 10,
    exponent: 2,
};

/**
 * Malawian kwacha.
 */
pub const MWK: Currency = Currency {
    code: CountryCode::MWK,
    base: 10,
    exponent: 2,
};

/**
 * Mexican peso.
 */
pub const MXN: Currency = Currency {
    code: CountryCode::MXN,
    base: 10,
    exponent: 2,
};

/**
 * Mexican Unidad de Inversion.
 */
pub const MXV: Currency = Currency {
    code: CountryCode::MXV,
    base: 10,
    exponent: 2,
};

/**
 * Malaysian ringgit.
 */
pub const MYR: Currency = Currency {
    code: CountryCode::MYR,
    base: 10,
    exponent: 2,
};

/**
 * Mozambican metical.
 */
pub const MZN: Currency = Currency {
    code: CountryCode::MZN,
    base: 10,
    exponent: 2,
};

/**
 * Namibian dollar.
 */
pub const NAD: Currency = Currency {
    code: CountryCode::NAD,
    base: 10,
    exponent: 2,
};

/**
 * Nigerian naira.
 */
pub const NGN: Currency = Currency {
    code: CountryCode::NGN,
    base: 10,
    exponent: 2,
};

/**
 * Nicaraguan córdoba.
 */
pub const NIO: Currency = Currency {
    code: CountryCode::NIO,
    base: 10,
    exponent: 2,
};

/**
 * Norwegian krone.
 */
pub const NOK: Currency = Currency {
    code: CountryCode::NOK,
    base: 10,
    exponent: 2,
};

/**
 * Nepalese rupee.
 */
pub const NPR: Currency = Currency {
    code: CountryCode::NPR,
    base: 10,
    exponent: 2,
};

/**
 * New Zealand dollar.
 */
pub const NZD: Currency = Currency {
    code: CountryCode::NZD,
    base: 10,
    exponent: 2,
};

/**
 * Omani rial.
 */
pub const OMR: Currency = Currency {
    code: CountryCode::OMR,
    base: 10,
    exponent: 3,
};

/**
 * Panamanian balboa.
 */
pub const PAB: Currency = Currency {
    code: CountryCode::PAB,
    base: 10,
    exponent: 2,
};

/**
 * Peruvian sol.
 */
pub const PEN: Currency = Currency {
    code: CountryCode::PEN,
    base: 10,
    exponent: 2,
};

/**
 * Papua New Guinean kina.
 */
pub const PGK: Currency = Currency {
    code: CountryCode::PGK,
    base: 10,
    exponent: 2,
};

/**
 * Philippine peso.
 */
pub const PHP: Currency = Currency {
    code: CountryCode::PHP,
    base: 10,
    exponent: 2,
};

/**
 * Pakistani rupee.
 */
pub const PKR: Currency = Currency {
    code: CountryCode::PKR,
    base: 10,
    exponent: 2,
};

/**
 * Polish złoty.
 */
pub const PLN: Currency = Currency {
    code: CountryCode::PLN,
    base: 10,
    exponent: 2,
};

/**
 * Paraguayan guaraní.
 */
pub const PYG: Currency = Currency {
    code: CountryCode::PYG,
    base: 10,
    exponent: 0,
};

/**
 * Qatari riyal.
 */
pub const QAR: Currency = Currency {
    code: CountryCode::QAR,
    base: 10,
    exponent: 2,
};

/**
 * Romanian leu.
 */
pub const RON: Currency = Currency {
    code: CountryCode::RON,
    base: 10,
    exponent: 2,
};

/**
 * Serbian dinar.
 */
pub const RSD: Currency = Currency {
    code: CountryCode::RSD,
    base: 10,
    exponent: 2,
};

/**
 * Russian ruble.
 */
pub const RUB: Currency = Currency {
    code: CountryCode::RUB,
    base: 10,
    exponent: 2,
};

/**
 * Rwandan franc.
 */
pub const RWF: Currency = Currency {
    code: CountryCode::RWF,
    base: 10,
    exponent: 0,
};

/**
 * Saudi riyal.
 */
pub const SAR: Currency = Currency {
    code: CountryCode::SAR,
    base: 10,
    exponent: 2,
};

/**
 * Solomon Islands dollar.
 */
pub const SBD: Currency = Currency {
    code: CountryCode::SBD,
    base: 10,
    exponent: 2,
};

/**
 * Seychelles rupee.
 */
pub const SCR: Currency = Currency {
    code: CountryCode::SCR,
    base: 10,
    exponent: 2,
};

/**
 * Sudanese pound.
 */
pub const SDG: Currency = Currency {
    code: CountryCode::SDG,
    base: 10,
    exponent: 2,
};

/**
 * Swedish krona.
 */
pub const SEK: Currency = Currency {
    code: CountryCode::SEK,
    base: 10,
    exponent: 2,
};

/**
 * Singapore dollar.
 */
pub const SGD: Currency = Currency {
    code: CountryCode::SGD,
    base: 10,
    exponent: 2,
};

/**
 * Saint Helena pound.
 */
pub const SHP: Currency = Currency {
    code: CountryCode::SHP,
    base: 10,
    exponent: 2,
};

/**
 * Sierra Leonean leone.
 */
pub const SLL: Currency = Currency {
    code: CountryCode::SLL,
    base: 10,
    exponent: 2,
};

/**
 * Somali shilling.
 */
pub const SOS: Currency = Currency {
    code: CountryCode::SOS,
    base: 10,
    exponent: 2,
};

/**
 * Surinamese dollar.
 */
pub const SRD: Currency = Currency {
    code: CountryCode::SRD,
    base: 10,
    exponent: 2,
};

/**
 * South Sudanese pound.
 */
pub const SSP: Currency = Currency {
    code: CountryCode::SSP,
    base: 10,
    exponent: 2,
};

/**
 * São Tomé and Príncipe dobra.
 */
pub const STN: Currency = Currency {
    code: CountryCode::STN,
    base: 10,
    exponent: 2,
};

/**
 * Salvadoran colón.
 */
pub const SVC: Currency = Currency {
    code: CountryCode::SVC,
    base: 10,
    exponent: 2,
};

/**
 * Syrian pound.
 */
pub const SYP: Currency = Currency {
    code: CountryCode::SYP,
    base: 10,
    exponent: 2,
};

/**
 * Swazi lilangeni.
 */
pub const SZL: Currency = Currency {
    code: CountryCode::SZL,
    base: 10,
    exponent: 2,
};

/**
 * Thai baht.
 */
pub const THB: Currency = Currency {
    code: CountryCode::THB,
    base: 10,
    exponent: 2,
};

/**
 * Tajikistani somoni.
 */
pub const TJS: Currency = Currency {
    code: CountryCode::TJS,
    base: 10,
    exponent: 2,
};

/**
 * Turkmenistan manat.
 */
pub const TMT: Currency = Currency {
    code: CountryCode::TMT,
    base: 10,
    exponent: 2,
};

/**
 * Tunisian dinar.
 */
pub const TND: Currency = Currency {
    code: CountryCode::TND,
    base: 10,
    exponent: 3,
};

/**
 * Tongan paʻanga.
 */
pub const TOP: Currency = Currency {
    code: CountryCode::TOP,
    base: 10,
    exponent: 2,
};

/**
 * Turkish lira.
 */
pub const TRY: Currency = Currency {
    code: CountryCode::TRY,
    base: 10,
    exponent: 2,
};

/**
 * Trinidad and Tobago dollar.
 */
pub const TTD: Currency = Currency {
    code: CountryCode::TTD,
    base: 10,
    exponent: 2,
};

/**
 * New Taiwan dollar.
 */
pub const TWD: Currency = Currency {
    code: CountryCode::TWD,
    base: 10,
    exponent: 2,
};

/**
 * Tanzanian shilling.
 */
pub const TZS: Currency = Currency {
    code: CountryCode::TZS,
    base: 10,
    exponent: 2,
};

/**
 * Ukrainian hryvnia.
 */
pub const UAH: Currency = Currency {
    code: CountryCode::UAH,
    base: 10,
    exponent: 2,
};

/**
 * Ugandan shilling.
 */
pub const UGX: Currency = Currency {
    code: CountryCode::UGX,
    base: 10,
    exponent: 0,
};

/**
 * United States dollar.
 */
pub const USD: Currency = Currency {
    code: CountryCode::USD,
    base: 10,
    exponent: 2,
};

/**
 * United States dollar (next day).
 */
pub const USN: Currency = Currency {
    code: CountryCode::USN,
    base: 10,
    exponent: 2,
};

/**
 * Uruguay Peso en Unidades Indexadas.
 */
pub const UYI: Currency = Currency {
    code: CountryCode::UYI,
    base: 10,
    exponent: 0,
};

/**
 * Uruguayan peso.
 */
pub const UYU: Currency = Currency {
    code: CountryCode::UYU,
    base: 10,
    exponent: 2,
};

/**
 * Unidad previsional.
 */
pub const UYW: Currency = Currency {
    code: CountryCode::UYW,
    base: 10,
    exponent: 4,
};

/**
 * Uzbekistani soʻm.
 */
pub const UZS: Currency = Currency {
    code: CountryCode::UZS,
    base: 10,
    exponent: 2,
};

/**
 * Venezuelan bolívar.
 */
pub const VES: Currency = Currency {
    code: CountryCode::VES,
    base: 10,
    exponent: 2,
};

/**
 * Vietnamese đồng.
 */
pub const VND: Currency = Currency {
    code: CountryCode::VND,
    base: 10,
    exponent: 0,
};

/**
 * Vanuatu vatu.
 */
pub const VUV: Currency = Currency {
    code: CountryCode::VUV,
    base: 10,
    exponent: 0,
};

/**
 * Samoan tālā.
 */
pub const WST: Currency = Currency {
    code: CountryCode::WST,
    base: 10,
    exponent: 2,
};

/**
 * Central African CFA franc.
 */
pub const XAF: Currency = Currency {
    code: CountryCode::XAF,
    base: 10,
    exponent: 0,
};

/**
 * East Caribbean dollar.
 */
pub const XCD: Currency = Currency {
    code: CountryCode::XCD,
    base: 10,
    exponent: 2,
};

/**
 * West African CFA franc.
 */
pub const XOF: Currency = Currency {
    code: CountryCode::XOF,
    base: 10,
    exponent: 0,
};

/**
 * CFP franc.
 */
pub const XPF: Currency = Currency {
    code: CountryCode::XPF,
    base: 10,
    exponent: 0,
};

/**
 * Yemeni rial.
 */
pub const YER: Currency = Currency {
    code: CountryCode::YER,
    base: 10,
    exponent: 2,
};

/**
 * South African rand.
 */
pub const ZAR: Currency = Currency {
    code: CountryCode::ZAR,
    base: 10,
    exponent: 2,
};

/**
 * Zambian kwacha.
 */
pub const ZMW: Currency = Currency {
    code: CountryCode::ZMW,
    base: 10,
    exponent: 2,
};

/**
 * Zimbabwean dollar.
 */
pub const ZWL: Currency = Currency {
    code: CountryCode::ZWL,
    base: 10,
    exponent: 2,
};

pub const CUSTOM: Currency = Currency {
    code: CountryCode::CUSTOM,
    base: 10,
    exponent: 2,
};

/**
 * Custom user-defined currency factory
 */
pub fn custom(base: u32, exponent: u32) -> Currency {
    Currency {
        code: CountryCode::CUSTOM,
        base,
        exponent,
    }
}

#[cfg(not(tarpaulin_include))]
impl Currency {
    pub fn from_country_code(country_code: CountryCode) -> Currency {
        match country_code {
            CountryCode::AED => AED,
            CountryCode::BOV => BOV,
            CountryCode::CUP => CUP,
            CountryCode::GTQ => GTQ,
            CountryCode::KHR => KHR,
            CountryCode::MOP => MOP,
            CountryCode::PHP => PHP,
            CountryCode::SSP => SSP,
            CountryCode::UYU => UYU,
            CountryCode::AFN => AFN,
            CountryCode::BRL => BRL,
            CountryCode::CVE => CVE,
            CountryCode::GYD => GYD,
            CountryCode::KMF => KMF,
            CountryCode::MRU => MRU,
            CountryCode::PKR => PKR,
            CountryCode::STN => STN,
            CountryCode::UYW => UYW,
            CountryCode::ALL => ALL,
            CountryCode::BSD => BSD,
            CountryCode::CZK => CZK,
            CountryCode::HKD => HKD,
            CountryCode::KPW => KPW,
            CountryCode::MUR => MUR,
            CountryCode::PLN => PLN,
            CountryCode::SVC => SVC,
            CountryCode::UZS => UZS,
            CountryCode::AMD => AMD,
            CountryCode::BTN => BTN,
            CountryCode::DJF => DJF,
            CountryCode::HNL => HNL,
            CountryCode::KRW => KRW,
            CountryCode::MVR => MVR,
            CountryCode::PYG => PYG,
            CountryCode::SYP => SYP,
            CountryCode::VES => VES,
            CountryCode::ANG => ANG,
            CountryCode::BWP => BWP,
            CountryCode::DKK => DKK,
            CountryCode::HRK => HRK,
            CountryCode::KWD => KWD,
            CountryCode::MWK => MWK,
            CountryCode::QAR => QAR,
            CountryCode::SZL => SZL,
            CountryCode::VND => VND,
            CountryCode::AOA => AOA,
            CountryCode::BYN => BYN,
            CountryCode::DOP => DOP,
            CountryCode::HTG => HTG,
            CountryCode::KYD => KYD,
            CountryCode::MXN => MXN,
            CountryCode::RON => RON,
            CountryCode::THB => THB,
            CountryCode::VUV => VUV,
            CountryCode::ARS => ARS,
            CountryCode::BZD => BZD,
            CountryCode::DZD => DZD,
            CountryCode::HUF => HUF,
            CountryCode::KZT => KZT,
            CountryCode::MXV => MXV,
            CountryCode::RSD => RSD,
            CountryCode::TJS => TJS,
            CountryCode::WST => WST,
            CountryCode::AUD => AUD,
            CountryCode::CAD => CAD,
            CountryCode::EGP => EGP,
            CountryCode::IDR => IDR,
            CountryCode::LAK => LAK,
            CountryCode::MYR => MYR,
            CountryCode::RUB => RUB,
            CountryCode::TMT => TMT,
            CountryCode::XAF => XAF,
            CountryCode::AWG => AWG,
            CountryCode::CDF => CDF,
            CountryCode::ERN => ERN,
            CountryCode::ILS => ILS,
            CountryCode::LBP => LBP,
            CountryCode::MZN => MZN,
            CountryCode::RWF => RWF,
            CountryCode::TND => TND,
            CountryCode::XCD => XCD,
            CountryCode::AZN => AZN,
            CountryCode::CHE => CHE,
            CountryCode::ETB => ETB,
            CountryCode::LKR => LKR,
            CountryCode::NAD => NAD,
            CountryCode::SAR => SAR,
            CountryCode::TOP => TOP,
            CountryCode::XOF => XOF,
            CountryCode::BAM => BAM,
            CountryCode::CHF => CHF,
            CountryCode::EUR => EUR,
            CountryCode::INR => INR,
            CountryCode::LRD => LRD,
            CountryCode::NGN => NGN,
            CountryCode::SBD => SBD,
            CountryCode::TRY => TRY,
            CountryCode::XPF => XPF,
            CountryCode::BBD => BBD,
            CountryCode::CHW => CHW,
            CountryCode::FJD => FJD,
            CountryCode::IQD => IQD,
            CountryCode::LSL => LSL,
            CountryCode::NIO => NIO,
            CountryCode::SCR => SCR,
            CountryCode::TTD => TTD,
            CountryCode::YER => YER,
            CountryCode::BDT => BDT,
            CountryCode::CLF => CLF,
            CountryCode::FKP => FKP,
            CountryCode::IRR => IRR,
            CountryCode::LYD => LYD,
            CountryCode::NOK => NOK,
            CountryCode::SDG => SDG,
            CountryCode::TWD => TWD,
            CountryCode::ZAR => ZAR,
            CountryCode::BGN => BGN,
            CountryCode::CLP => CLP,
            CountryCode::GBP => GBP,
            CountryCode::ISK => ISK,
            CountryCode::MAD => MAD,
            CountryCode::NPR => NPR,
            CountryCode::SEK => SEK,
            CountryCode::TZS => TZS,
            CountryCode::ZMW => ZMW,
            CountryCode::BHD => BHD,
            CountryCode::CNY => CNY,
            CountryCode::GEL => GEL,
            CountryCode::JMD => JMD,
            CountryCode::MDL => MDL,
            CountryCode::NZD => NZD,
            CountryCode::SGD => SGD,
            CountryCode::UAH => UAH,
            CountryCode::ZWL => ZWL,
            CountryCode::BIF => BIF,
            CountryCode::COP => COP,
            CountryCode::GHS => GHS,
            CountryCode::JOD => JOD,
            CountryCode::MGA => MGA,
            CountryCode::OMR => OMR,
            CountryCode::SHP => SHP,
            CountryCode::UGX => UGX,
            CountryCode::BMD => BMD,
            CountryCode::COU => COU,
            CountryCode::GIP => GIP,
            CountryCode::JPY => JPY,
            CountryCode::MKD => MKD,
            CountryCode::PAB => PAB,
            CountryCode::SLL => SLL,
            CountryCode::USD => USD,
            CountryCode::BND => BND,
            CountryCode::CRC => CRC,
            CountryCode::GMD => GMD,
            CountryCode::KES => KES,
            CountryCode::MMK => MMK,
            CountryCode::PEN => PEN,
            CountryCode::SOS => SOS,
            CountryCode::USN => USN,
            CountryCode::BOB => BOB,
            CountryCode::CUC => CUC,
            CountryCode::GNF => GNF,
            CountryCode::KGS => KGS,
            CountryCode::MNT => MNT,
            CountryCode::PGK => PGK,
            CountryCode::SRD => SRD,
            CountryCode::UYI => UYI,
            CountryCode::CUSTOM => CUSTOM,
        }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_custom_currency() {
        assert_eq!(
            custom(10, 2),
            Currency {
                code: CountryCode::CUSTOM,
                base: 10,
                exponent: 2
            }
        )
    }
}
