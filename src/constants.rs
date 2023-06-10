//! Contains all defined [`LanguageId`] returned by the lookups.
use crate::{AnsiCodePage, LanguageId};

/// Arabic
pub const LANG_AR: &LanguageId = &LanguageId {
    name: "ar",
    lcid: 0x0001,
    english_name: "Arabic",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARA",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Bulgarian
pub const LANG_BG: &LanguageId = &LanguageId {
    name: "bg",
    lcid: 0x0002,
    english_name: "Bulgarian",
    iso639_two_letter: "bg",
    iso639_three_letter: "bul",
    windows_three_letter: "BGR",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Catalan
pub const LANG_CA: &LanguageId = &LanguageId {
    name: "ca",
    lcid: 0x0003,
    english_name: "Catalan",
    iso639_two_letter: "ca",
    iso639_three_letter: "cat",
    windows_three_letter: "CAT",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Chinese (Simplified)
pub const LANG_ZH_HANS: &LanguageId = &LanguageId {
    name: "zh-Hans",
    lcid: 0x0004,
    english_name: "Chinese (Simplified)",
    iso639_two_letter: "zh",
    iso639_three_letter: "zho",
    windows_three_letter: "CHS",
    ansi_code_page: Some(AnsiCodePage::GB2312),
};

/// Czech
pub const LANG_CS: &LanguageId = &LanguageId {
    name: "cs",
    lcid: 0x0005,
    english_name: "Czech",
    iso639_two_letter: "cs",
    iso639_three_letter: "ces",
    windows_three_letter: "CSY",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Danish
pub const LANG_DA: &LanguageId = &LanguageId {
    name: "da",
    lcid: 0x0006,
    english_name: "Danish",
    iso639_two_letter: "da",
    iso639_three_letter: "dan",
    windows_three_letter: "DAN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// German
pub const LANG_DE: &LanguageId = &LanguageId {
    name: "de",
    lcid: 0x0007,
    english_name: "German",
    iso639_two_letter: "de",
    iso639_three_letter: "deu",
    windows_three_letter: "DEU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Greek
pub const LANG_EL: &LanguageId = &LanguageId {
    name: "el",
    lcid: 0x0008,
    english_name: "Greek",
    iso639_two_letter: "el",
    iso639_three_letter: "ell",
    windows_three_letter: "ELL",
    ansi_code_page: Some(AnsiCodePage::Windows1253),
};

/// English
pub const LANG_EN: &LanguageId = &LanguageId {
    name: "en",
    lcid: 0x0009,
    english_name: "English",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish
pub const LANG_ES: &LanguageId = &LanguageId {
    name: "es",
    lcid: 0x000A,
    english_name: "Spanish",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESP",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Finnish
pub const LANG_FI: &LanguageId = &LanguageId {
    name: "fi",
    lcid: 0x000B,
    english_name: "Finnish",
    iso639_two_letter: "fi",
    iso639_three_letter: "fin",
    windows_three_letter: "FIN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French
pub const LANG_FR: &LanguageId = &LanguageId {
    name: "fr",
    lcid: 0x000C,
    english_name: "French",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hebrew
pub const LANG_HE: &LanguageId = &LanguageId {
    name: "he",
    lcid: 0x000D,
    english_name: "Hebrew",
    iso639_two_letter: "he",
    iso639_three_letter: "heb",
    windows_three_letter: "HEB",
    ansi_code_page: Some(AnsiCodePage::Windows1255),
};

/// Hungarian
pub const LANG_HU: &LanguageId = &LanguageId {
    name: "hu",
    lcid: 0x000E,
    english_name: "Hungarian",
    iso639_two_letter: "hu",
    iso639_three_letter: "hun",
    windows_three_letter: "HUN",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Icelandic
pub const LANG_IS: &LanguageId = &LanguageId {
    name: "is",
    lcid: 0x000F,
    english_name: "Icelandic",
    iso639_two_letter: "is",
    iso639_three_letter: "isl",
    windows_three_letter: "ISL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Italian
pub const LANG_IT: &LanguageId = &LanguageId {
    name: "it",
    lcid: 0x0010,
    english_name: "Italian",
    iso639_two_letter: "it",
    iso639_three_letter: "ita",
    windows_three_letter: "ITA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Japanese
pub const LANG_JA: &LanguageId = &LanguageId {
    name: "ja",
    lcid: 0x0011,
    english_name: "Japanese",
    iso639_two_letter: "ja",
    iso639_three_letter: "jpn",
    windows_three_letter: "JPN",
    ansi_code_page: Some(AnsiCodePage::ShiftJIS),
};

/// Korean
pub const LANG_KO: &LanguageId = &LanguageId {
    name: "ko",
    lcid: 0x0012,
    english_name: "Korean",
    iso639_two_letter: "ko",
    iso639_three_letter: "kor",
    windows_three_letter: "KOR",
    ansi_code_page: Some(AnsiCodePage::KsC5601),
};

/// Dutch
pub const LANG_NL: &LanguageId = &LanguageId {
    name: "nl",
    lcid: 0x0013,
    english_name: "Dutch",
    iso639_two_letter: "nl",
    iso639_three_letter: "nld",
    windows_three_letter: "NLD",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Norwegian
pub const LANG_NO: &LanguageId = &LanguageId {
    name: "no",
    lcid: 0x0014,
    english_name: "Norwegian",
    iso639_two_letter: "nb",
    iso639_three_letter: "nob",
    windows_three_letter: "NOR",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Polish
pub const LANG_PL: &LanguageId = &LanguageId {
    name: "pl",
    lcid: 0x0015,
    english_name: "Polish",
    iso639_two_letter: "pl",
    iso639_three_letter: "pol",
    windows_three_letter: "PLK",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Portuguese
pub const LANG_PT: &LanguageId = &LanguageId {
    name: "pt",
    lcid: 0x0016,
    english_name: "Portuguese",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "PTB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Romansh
pub const LANG_RM: &LanguageId = &LanguageId {
    name: "rm",
    lcid: 0x0017,
    english_name: "Romansh",
    iso639_two_letter: "rm",
    iso639_three_letter: "roh",
    windows_three_letter: "RMC",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Romanian
pub const LANG_RO: &LanguageId = &LanguageId {
    name: "ro",
    lcid: 0x0018,
    english_name: "Romanian",
    iso639_two_letter: "ro",
    iso639_three_letter: "ron",
    windows_three_letter: "ROM",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Russian
pub const LANG_RU: &LanguageId = &LanguageId {
    name: "ru",
    lcid: 0x0019,
    english_name: "Russian",
    iso639_two_letter: "ru",
    iso639_three_letter: "rus",
    windows_three_letter: "RUS",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Croatian
pub const LANG_HR: &LanguageId = &LanguageId {
    name: "hr",
    lcid: 0x001A,
    english_name: "Croatian",
    iso639_two_letter: "hr",
    iso639_three_letter: "hrv",
    windows_three_letter: "HRV",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Slovak
pub const LANG_SK: &LanguageId = &LanguageId {
    name: "sk",
    lcid: 0x001B,
    english_name: "Slovak",
    iso639_two_letter: "sk",
    iso639_three_letter: "slk",
    windows_three_letter: "SKY",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Albanian
pub const LANG_SQ: &LanguageId = &LanguageId {
    name: "sq",
    lcid: 0x001C,
    english_name: "Albanian",
    iso639_two_letter: "sq",
    iso639_three_letter: "sqi",
    windows_three_letter: "SQI",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Swedish
pub const LANG_SV: &LanguageId = &LanguageId {
    name: "sv",
    lcid: 0x001D,
    english_name: "Swedish",
    iso639_two_letter: "sv",
    iso639_three_letter: "swe",
    windows_three_letter: "SVE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Thai
pub const LANG_TH: &LanguageId = &LanguageId {
    name: "th",
    lcid: 0x001E,
    english_name: "Thai",
    iso639_two_letter: "th",
    iso639_three_letter: "tha",
    windows_three_letter: "THA",
    ansi_code_page: Some(AnsiCodePage::Windows874),
};

/// Turkish
pub const LANG_TR: &LanguageId = &LanguageId {
    name: "tr",
    lcid: 0x001F,
    english_name: "Turkish",
    iso639_two_letter: "tr",
    iso639_three_letter: "tur",
    windows_three_letter: "TRK",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Urdu
pub const LANG_UR: &LanguageId = &LanguageId {
    name: "ur",
    lcid: 0x0020,
    english_name: "Urdu",
    iso639_two_letter: "ur",
    iso639_three_letter: "urd",
    windows_three_letter: "URD",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Indonesian
pub const LANG_ID: &LanguageId = &LanguageId {
    name: "id",
    lcid: 0x0021,
    english_name: "Indonesian",
    iso639_two_letter: "id",
    iso639_three_letter: "ind",
    windows_three_letter: "IND",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Ukrainian
pub const LANG_UK: &LanguageId = &LanguageId {
    name: "uk",
    lcid: 0x0022,
    english_name: "Ukrainian",
    iso639_two_letter: "uk",
    iso639_three_letter: "ukr",
    windows_three_letter: "UKR",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Belarusian
pub const LANG_BE: &LanguageId = &LanguageId {
    name: "be",
    lcid: 0x0023,
    english_name: "Belarusian",
    iso639_two_letter: "be",
    iso639_three_letter: "bel",
    windows_three_letter: "BEL",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Slovenian
pub const LANG_SL: &LanguageId = &LanguageId {
    name: "sl",
    lcid: 0x0024,
    english_name: "Slovenian",
    iso639_two_letter: "sl",
    iso639_three_letter: "slv",
    windows_three_letter: "SLV",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Estonian
pub const LANG_ET: &LanguageId = &LanguageId {
    name: "et",
    lcid: 0x0025,
    english_name: "Estonian",
    iso639_two_letter: "et",
    iso639_three_letter: "est",
    windows_three_letter: "ETI",
    ansi_code_page: Some(AnsiCodePage::Windows1257),
};

/// Latvian
pub const LANG_LV: &LanguageId = &LanguageId {
    name: "lv",
    lcid: 0x0026,
    english_name: "Latvian",
    iso639_two_letter: "lv",
    iso639_three_letter: "lav",
    windows_three_letter: "LVI",
    ansi_code_page: Some(AnsiCodePage::Windows1257),
};

/// Lithuanian
pub const LANG_LT: &LanguageId = &LanguageId {
    name: "lt",
    lcid: 0x0027,
    english_name: "Lithuanian",
    iso639_two_letter: "lt",
    iso639_three_letter: "lit",
    windows_three_letter: "LTH",
    ansi_code_page: Some(AnsiCodePage::Windows1257),
};

/// Tajik
pub const LANG_TG: &LanguageId = &LanguageId {
    name: "tg",
    lcid: 0x0028,
    english_name: "Tajik",
    iso639_two_letter: "tg",
    iso639_three_letter: "tgk",
    windows_three_letter: "TAJ",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Persian
pub const LANG_FA: &LanguageId = &LanguageId {
    name: "fa",
    lcid: 0x0029,
    english_name: "Persian",
    iso639_two_letter: "fa",
    iso639_three_letter: "fas",
    windows_three_letter: "FAR",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Vietnamese
pub const LANG_VI: &LanguageId = &LanguageId {
    name: "vi",
    lcid: 0x002A,
    english_name: "Vietnamese",
    iso639_two_letter: "vi",
    iso639_three_letter: "vie",
    windows_three_letter: "VIT",
    ansi_code_page: Some(AnsiCodePage::Windows1258),
};

/// Armenian
pub const LANG_HY: &LanguageId = &LanguageId {
    name: "hy",
    lcid: 0x002B,
    english_name: "Armenian",
    iso639_two_letter: "hy",
    iso639_three_letter: "hye",
    windows_three_letter: "HYE",
    ansi_code_page: None,
};

/// Azerbaijani
pub const LANG_AZ: &LanguageId = &LanguageId {
    name: "az",
    lcid: 0x002C,
    english_name: "Azerbaijani",
    iso639_two_letter: "az",
    iso639_three_letter: "aze",
    windows_three_letter: "AZE",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Basque
pub const LANG_EU: &LanguageId = &LanguageId {
    name: "eu",
    lcid: 0x002D,
    english_name: "Basque",
    iso639_two_letter: "eu",
    iso639_three_letter: "eus",
    windows_three_letter: "EUQ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Upper Sorbian
pub const LANG_HSB: &LanguageId = &LanguageId {
    name: "hsb",
    lcid: 0x002E,
    english_name: "Upper Sorbian",
    iso639_two_letter: "hsb",
    iso639_three_letter: "hsb",
    windows_three_letter: "HSB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Macedonian
pub const LANG_MK: &LanguageId = &LanguageId {
    name: "mk",
    lcid: 0x002F,
    english_name: "Macedonian",
    iso639_two_letter: "mk",
    iso639_three_letter: "mkd",
    windows_three_letter: "MKI",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Sesotho
pub const LANG_ST: &LanguageId = &LanguageId {
    name: "st",
    lcid: 0x0030,
    english_name: "Sesotho",
    iso639_two_letter: "st",
    iso639_three_letter: "sot",
    windows_three_letter: "SOT",
    ansi_code_page: None,
};

/// Tsonga
pub const LANG_TS: &LanguageId = &LanguageId {
    name: "ts",
    lcid: 0x0031,
    english_name: "Tsonga",
    iso639_two_letter: "ts",
    iso639_three_letter: "tso",
    windows_three_letter: "TSO",
    ansi_code_page: None,
};

/// Setswana
pub const LANG_TN: &LanguageId = &LanguageId {
    name: "tn",
    lcid: 0x0032,
    english_name: "Setswana",
    iso639_two_letter: "tn",
    iso639_three_letter: "tsn",
    windows_three_letter: "TSN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Venda
pub const LANG_VE: &LanguageId = &LanguageId {
    name: "ve",
    lcid: 0x0033,
    english_name: "Venda",
    iso639_two_letter: "ve",
    iso639_three_letter: "ven",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// isiXhosa
pub const LANG_XH: &LanguageId = &LanguageId {
    name: "xh",
    lcid: 0x0034,
    english_name: "isiXhosa",
    iso639_two_letter: "xh",
    iso639_three_letter: "xho",
    windows_three_letter: "XHO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// isiZulu
pub const LANG_ZU: &LanguageId = &LanguageId {
    name: "zu",
    lcid: 0x0035,
    english_name: "isiZulu",
    iso639_two_letter: "zu",
    iso639_three_letter: "zul",
    windows_three_letter: "ZUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Afrikaans
pub const LANG_AF: &LanguageId = &LanguageId {
    name: "af",
    lcid: 0x0036,
    english_name: "Afrikaans",
    iso639_two_letter: "af",
    iso639_three_letter: "afr",
    windows_three_letter: "AFK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Georgian
pub const LANG_KA: &LanguageId = &LanguageId {
    name: "ka",
    lcid: 0x0037,
    english_name: "Georgian",
    iso639_two_letter: "ka",
    iso639_three_letter: "kat",
    windows_three_letter: "KAT",
    ansi_code_page: None,
};

/// Faroese
pub const LANG_FO: &LanguageId = &LanguageId {
    name: "fo",
    lcid: 0x0038,
    english_name: "Faroese",
    iso639_two_letter: "fo",
    iso639_three_letter: "fao",
    windows_three_letter: "FOS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hindi
pub const LANG_HI: &LanguageId = &LanguageId {
    name: "hi",
    lcid: 0x0039,
    english_name: "Hindi",
    iso639_two_letter: "hi",
    iso639_three_letter: "hin",
    windows_three_letter: "HIN",
    ansi_code_page: None,
};

/// Maltese
pub const LANG_MT: &LanguageId = &LanguageId {
    name: "mt",
    lcid: 0x003A,
    english_name: "Maltese",
    iso639_two_letter: "mt",
    iso639_three_letter: "mlt",
    windows_three_letter: "MLT",
    ansi_code_page: None,
};

/// Northern Sami
pub const LANG_SE: &LanguageId = &LanguageId {
    name: "se",
    lcid: 0x003B,
    english_name: "Northern Sami",
    iso639_two_letter: "se",
    iso639_three_letter: "sme",
    windows_three_letter: "SME",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Irish
pub const LANG_GA: &LanguageId = &LanguageId {
    name: "ga",
    lcid: 0x003C,
    english_name: "Irish",
    iso639_two_letter: "ga",
    iso639_three_letter: "gle",
    windows_three_letter: "IRE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Malay
pub const LANG_MS: &LanguageId = &LanguageId {
    name: "ms",
    lcid: 0x003E,
    english_name: "Malay",
    iso639_two_letter: "ms",
    iso639_three_letter: "msa",
    windows_three_letter: "MSL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kazakh
pub const LANG_KK: &LanguageId = &LanguageId {
    name: "kk",
    lcid: 0x003F,
    english_name: "Kazakh",
    iso639_two_letter: "kk",
    iso639_three_letter: "kaz",
    windows_three_letter: "KKZ",
    ansi_code_page: None,
};

/// Kyrgyz
pub const LANG_KY: &LanguageId = &LanguageId {
    name: "ky",
    lcid: 0x0040,
    english_name: "Kyrgyz",
    iso639_two_letter: "ky",
    iso639_three_letter: "kir",
    windows_three_letter: "KYR",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Kiswahili
pub const LANG_SW: &LanguageId = &LanguageId {
    name: "sw",
    lcid: 0x0041,
    english_name: "Kiswahili",
    iso639_two_letter: "sw",
    iso639_three_letter: "swa",
    windows_three_letter: "SWK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Turkmen
pub const LANG_TK: &LanguageId = &LanguageId {
    name: "tk",
    lcid: 0x0042,
    english_name: "Turkmen",
    iso639_two_letter: "tk",
    iso639_three_letter: "tuk",
    windows_three_letter: "TUK",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Uzbek
pub const LANG_UZ: &LanguageId = &LanguageId {
    name: "uz",
    lcid: 0x0043,
    english_name: "Uzbek",
    iso639_two_letter: "uz",
    iso639_three_letter: "uzb",
    windows_three_letter: "UZB",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Tatar
pub const LANG_TT: &LanguageId = &LanguageId {
    name: "tt",
    lcid: 0x0044,
    english_name: "Tatar",
    iso639_two_letter: "tt",
    iso639_three_letter: "tat",
    windows_three_letter: "TTT",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Bangla
pub const LANG_BN: &LanguageId = &LanguageId {
    name: "bn",
    lcid: 0x0045,
    english_name: "Bangla",
    iso639_two_letter: "bn",
    iso639_three_letter: "ben",
    windows_three_letter: "BNB",
    ansi_code_page: None,
};

/// Punjabi
pub const LANG_PA: &LanguageId = &LanguageId {
    name: "pa",
    lcid: 0x0046,
    english_name: "Punjabi",
    iso639_two_letter: "pa",
    iso639_three_letter: "pan",
    windows_three_letter: "PAN",
    ansi_code_page: None,
};

/// Gujarati
pub const LANG_GU: &LanguageId = &LanguageId {
    name: "gu",
    lcid: 0x0047,
    english_name: "Gujarati",
    iso639_two_letter: "gu",
    iso639_three_letter: "guj",
    windows_three_letter: "GUJ",
    ansi_code_page: None,
};

/// Odia
pub const LANG_OR: &LanguageId = &LanguageId {
    name: "or",
    lcid: 0x0048,
    english_name: "Odia",
    iso639_two_letter: "or",
    iso639_three_letter: "ori",
    windows_three_letter: "ORI",
    ansi_code_page: None,
};

/// Tamil
pub const LANG_TA: &LanguageId = &LanguageId {
    name: "ta",
    lcid: 0x0049,
    english_name: "Tamil",
    iso639_two_letter: "ta",
    iso639_three_letter: "tam",
    windows_three_letter: "TAI",
    ansi_code_page: None,
};

/// Telugu
pub const LANG_TE: &LanguageId = &LanguageId {
    name: "te",
    lcid: 0x004A,
    english_name: "Telugu",
    iso639_two_letter: "te",
    iso639_three_letter: "tel",
    windows_three_letter: "TEL",
    ansi_code_page: None,
};

/// Kannada
pub const LANG_KN: &LanguageId = &LanguageId {
    name: "kn",
    lcid: 0x004B,
    english_name: "Kannada",
    iso639_two_letter: "kn",
    iso639_three_letter: "kan",
    windows_three_letter: "KDI",
    ansi_code_page: None,
};

/// Malayalam
pub const LANG_ML: &LanguageId = &LanguageId {
    name: "ml",
    lcid: 0x004C,
    english_name: "Malayalam",
    iso639_two_letter: "ml",
    iso639_three_letter: "mal",
    windows_three_letter: "MYM",
    ansi_code_page: None,
};

/// Assamese
pub const LANG_AS: &LanguageId = &LanguageId {
    name: "as",
    lcid: 0x004D,
    english_name: "Assamese",
    iso639_two_letter: "as",
    iso639_three_letter: "asm",
    windows_three_letter: "ASM",
    ansi_code_page: None,
};

/// Marathi
pub const LANG_MR: &LanguageId = &LanguageId {
    name: "mr",
    lcid: 0x004E,
    english_name: "Marathi",
    iso639_two_letter: "mr",
    iso639_three_letter: "mar",
    windows_three_letter: "MAR",
    ansi_code_page: None,
};

/// Sanskrit
pub const LANG_SA: &LanguageId = &LanguageId {
    name: "sa",
    lcid: 0x004F,
    english_name: "Sanskrit",
    iso639_two_letter: "sa",
    iso639_three_letter: "san",
    windows_three_letter: "SAN",
    ansi_code_page: None,
};

/// Mongolian
pub const LANG_MN: &LanguageId = &LanguageId {
    name: "mn",
    lcid: 0x0050,
    english_name: "Mongolian",
    iso639_two_letter: "mn",
    iso639_three_letter: "mon",
    windows_three_letter: "MON",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Tibetan
pub const LANG_BO: &LanguageId = &LanguageId {
    name: "bo",
    lcid: 0x0051,
    english_name: "Tibetan",
    iso639_two_letter: "bo",
    iso639_three_letter: "bod",
    windows_three_letter: "BOB",
    ansi_code_page: None,
};

/// Welsh
pub const LANG_CY: &LanguageId = &LanguageId {
    name: "cy",
    lcid: 0x0052,
    english_name: "Welsh",
    iso639_two_letter: "cy",
    iso639_three_letter: "cym",
    windows_three_letter: "CYM",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Khmer
pub const LANG_KM: &LanguageId = &LanguageId {
    name: "km",
    lcid: 0x0053,
    english_name: "Khmer",
    iso639_two_letter: "km",
    iso639_three_letter: "khm",
    windows_three_letter: "KHM",
    ansi_code_page: None,
};

/// Lao
pub const LANG_LO: &LanguageId = &LanguageId {
    name: "lo",
    lcid: 0x0054,
    english_name: "Lao",
    iso639_two_letter: "lo",
    iso639_three_letter: "lao",
    windows_three_letter: "LAO",
    ansi_code_page: None,
};

/// Burmese
pub const LANG_MY: &LanguageId = &LanguageId {
    name: "my",
    lcid: 0x0055,
    english_name: "Burmese",
    iso639_two_letter: "my",
    iso639_three_letter: "mya",
    windows_three_letter: "MYA",
    ansi_code_page: None,
};

/// Galician
pub const LANG_GL: &LanguageId = &LanguageId {
    name: "gl",
    lcid: 0x0056,
    english_name: "Galician",
    iso639_two_letter: "gl",
    iso639_three_letter: "glg",
    windows_three_letter: "GLC",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Konkani
pub const LANG_KOK: &LanguageId = &LanguageId {
    name: "kok",
    lcid: 0x0057,
    english_name: "Konkani",
    iso639_two_letter: "kok",
    iso639_three_letter: "kok",
    windows_three_letter: "KNK",
    ansi_code_page: None,
};

/// Sindhi
pub const LANG_SD: &LanguageId = &LanguageId {
    name: "sd",
    lcid: 0x0059,
    english_name: "Sindhi",
    iso639_two_letter: "sd",
    iso639_three_letter: "snd",
    windows_three_letter: "SIP",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Syriac
pub const LANG_SYR: &LanguageId = &LanguageId {
    name: "syr",
    lcid: 0x005A,
    english_name: "Syriac",
    iso639_two_letter: "syr",
    iso639_three_letter: "syr",
    windows_three_letter: "SYR",
    ansi_code_page: None,
};

/// Sinhala
pub const LANG_SI: &LanguageId = &LanguageId {
    name: "si",
    lcid: 0x005B,
    english_name: "Sinhala",
    iso639_two_letter: "si",
    iso639_three_letter: "sin",
    windows_three_letter: "SIN",
    ansi_code_page: None,
};

/// Cherokee
pub const LANG_CHR: &LanguageId = &LanguageId {
    name: "chr",
    lcid: 0x005C,
    english_name: "Cherokee",
    iso639_two_letter: "chr",
    iso639_three_letter: "chr",
    windows_three_letter: "CRE",
    ansi_code_page: None,
};

/// Inuktitut
pub const LANG_IU: &LanguageId = &LanguageId {
    name: "iu",
    lcid: 0x005D,
    english_name: "Inuktitut",
    iso639_two_letter: "iu",
    iso639_three_letter: "iku",
    windows_three_letter: "IUK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Amharic
pub const LANG_AM: &LanguageId = &LanguageId {
    name: "am",
    lcid: 0x005E,
    english_name: "Amharic",
    iso639_two_letter: "am",
    iso639_three_letter: "amh",
    windows_three_letter: "AMH",
    ansi_code_page: None,
};

/// Central Atlas Tamazight
pub const LANG_TZM: &LanguageId = &LanguageId {
    name: "tzm",
    lcid: 0x005F,
    english_name: "Central Atlas Tamazight",
    iso639_two_letter: "tzm",
    iso639_three_letter: "tzm",
    windows_three_letter: "TZA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kashmiri
pub const LANG_KS: &LanguageId = &LanguageId {
    name: "ks",
    lcid: 0x0060,
    english_name: "Kashmiri",
    iso639_two_letter: "ks",
    iso639_three_letter: "kas",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nepali
pub const LANG_NE: &LanguageId = &LanguageId {
    name: "ne",
    lcid: 0x0061,
    english_name: "Nepali",
    iso639_two_letter: "ne",
    iso639_three_letter: "nep",
    windows_three_letter: "NEP",
    ansi_code_page: None,
};

/// Western Frisian
pub const LANG_FY: &LanguageId = &LanguageId {
    name: "fy",
    lcid: 0x0062,
    english_name: "Western Frisian",
    iso639_two_letter: "fy",
    iso639_three_letter: "fry",
    windows_three_letter: "FYN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Pashto
pub const LANG_PS: &LanguageId = &LanguageId {
    name: "ps",
    lcid: 0x0063,
    english_name: "Pashto",
    iso639_two_letter: "ps",
    iso639_three_letter: "pus",
    windows_three_letter: "PAS",
    ansi_code_page: None,
};

/// Filipino
pub const LANG_FIL: &LanguageId = &LanguageId {
    name: "fil",
    lcid: 0x0064,
    english_name: "Filipino",
    iso639_two_letter: "fil",
    iso639_three_letter: "fil",
    windows_three_letter: "FPO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Divehi
pub const LANG_DV: &LanguageId = &LanguageId {
    name: "dv",
    lcid: 0x0065,
    english_name: "Divehi",
    iso639_two_letter: "dv",
    iso639_three_letter: "div",
    windows_three_letter: "DIV",
    ansi_code_page: None,
};

/// Fulah
pub const LANG_FF: &LanguageId = &LanguageId {
    name: "ff",
    lcid: 0x0067,
    english_name: "Fulah",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hausa
pub const LANG_HA: &LanguageId = &LanguageId {
    name: "ha",
    lcid: 0x0068,
    english_name: "Hausa",
    iso639_two_letter: "ha",
    iso639_three_letter: "hau",
    windows_three_letter: "HAU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Yoruba
pub const LANG_YO: &LanguageId = &LanguageId {
    name: "yo",
    lcid: 0x006A,
    english_name: "Yoruba",
    iso639_two_letter: "yo",
    iso639_three_letter: "yor",
    windows_three_letter: "YOR",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Quechua
pub const LANG_QUZ: &LanguageId = &LanguageId {
    name: "quz",
    lcid: 0x006B,
    english_name: "Quechua",
    iso639_two_letter: "quz",
    iso639_three_letter: "qub",
    windows_three_letter: "QUB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Sesotho sa Leboa
pub const LANG_NSO: &LanguageId = &LanguageId {
    name: "nso",
    lcid: 0x006C,
    english_name: "Sesotho sa Leboa",
    iso639_two_letter: "nso",
    iso639_three_letter: "nso",
    windows_three_letter: "NSO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Bashkir
pub const LANG_BA: &LanguageId = &LanguageId {
    name: "ba",
    lcid: 0x006D,
    english_name: "Bashkir",
    iso639_two_letter: "ba",
    iso639_three_letter: "bak",
    windows_three_letter: "BAS",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Luxembourgish
pub const LANG_LB: &LanguageId = &LanguageId {
    name: "lb",
    lcid: 0x006E,
    english_name: "Luxembourgish",
    iso639_two_letter: "lb",
    iso639_three_letter: "ltz",
    windows_three_letter: "LBX",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Greenlandic
pub const LANG_KL: &LanguageId = &LanguageId {
    name: "kl",
    lcid: 0x006F,
    english_name: "Greenlandic",
    iso639_two_letter: "kl",
    iso639_three_letter: "kal",
    windows_three_letter: "KAL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Igbo
pub const LANG_IG: &LanguageId = &LanguageId {
    name: "ig",
    lcid: 0x0070,
    english_name: "Igbo",
    iso639_two_letter: "ig",
    iso639_three_letter: "ibo",
    windows_three_letter: "IBO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Oromo
pub const LANG_OM: &LanguageId = &LanguageId {
    name: "om",
    lcid: 0x0072,
    english_name: "Oromo",
    iso639_two_letter: "om",
    iso639_three_letter: "orm",
    windows_three_letter: "ORM",
    ansi_code_page: None,
};

/// Tigrinya
pub const LANG_TI: &LanguageId = &LanguageId {
    name: "ti",
    lcid: 0x0073,
    english_name: "Tigrinya",
    iso639_two_letter: "ti",
    iso639_three_letter: "tir",
    windows_three_letter: "TIR",
    ansi_code_page: None,
};

/// Guarani
pub const LANG_GN: &LanguageId = &LanguageId {
    name: "gn",
    lcid: 0x0074,
    english_name: "Guarani",
    iso639_two_letter: "gn",
    iso639_three_letter: "grn",
    windows_three_letter: "GRN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hawaiian
pub const LANG_HAW: &LanguageId = &LanguageId {
    name: "haw",
    lcid: 0x0075,
    english_name: "Hawaiian",
    iso639_two_letter: "haw",
    iso639_three_letter: "haw",
    windows_three_letter: "HAW",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Yi
pub const LANG_II: &LanguageId = &LanguageId {
    name: "ii",
    lcid: 0x0078,
    english_name: "Yi",
    iso639_two_letter: "ii",
    iso639_three_letter: "iii",
    windows_three_letter: "III",
    ansi_code_page: None,
};

/// Mapudungun
pub const LANG_ARN: &LanguageId = &LanguageId {
    name: "arn",
    lcid: 0x007A,
    english_name: "Mapudungun",
    iso639_two_letter: "arn",
    iso639_three_letter: "arn",
    windows_three_letter: "MPD",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Mohawk
pub const LANG_MOH: &LanguageId = &LanguageId {
    name: "moh",
    lcid: 0x007C,
    english_name: "Mohawk",
    iso639_two_letter: "moh",
    iso639_three_letter: "moh",
    windows_three_letter: "MWK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Breton
pub const LANG_BR: &LanguageId = &LanguageId {
    name: "br",
    lcid: 0x007E,
    english_name: "Breton",
    iso639_two_letter: "br",
    iso639_three_letter: "bre",
    windows_three_letter: "BRE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Invariant Language (Invariant Country)
pub const LANG_INVARIANT: &LanguageId = &LanguageId {
    name: "",
    lcid: 0x007F,
    english_name: "Invariant Language (Invariant Country)",
    iso639_two_letter: "iv",
    iso639_three_letter: "ivl",
    windows_three_letter: "IVL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Uyghur
pub const LANG_UG: &LanguageId = &LanguageId {
    name: "ug",
    lcid: 0x0080,
    english_name: "Uyghur",
    iso639_two_letter: "ug",
    iso639_three_letter: "uig",
    windows_three_letter: "UIG",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Maori
pub const LANG_MI: &LanguageId = &LanguageId {
    name: "mi",
    lcid: 0x0081,
    english_name: "Maori",
    iso639_two_letter: "mi",
    iso639_three_letter: "mri",
    windows_three_letter: "MRI",
    ansi_code_page: None,
};

/// Occitan
pub const LANG_OC: &LanguageId = &LanguageId {
    name: "oc",
    lcid: 0x0082,
    english_name: "Occitan",
    iso639_two_letter: "oc",
    iso639_three_letter: "oci",
    windows_three_letter: "OCI",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Corsican
pub const LANG_CO: &LanguageId = &LanguageId {
    name: "co",
    lcid: 0x0083,
    english_name: "Corsican",
    iso639_two_letter: "co",
    iso639_three_letter: "cos",
    windows_three_letter: "COS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Swiss German
pub const LANG_GSW: &LanguageId = &LanguageId {
    name: "gsw",
    lcid: 0x0084,
    english_name: "Swiss German",
    iso639_two_letter: "gsw",
    iso639_three_letter: "gsw",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Sakha
pub const LANG_SAH: &LanguageId = &LanguageId {
    name: "sah",
    lcid: 0x0085,
    english_name: "Sakha",
    iso639_two_letter: "sah",
    iso639_three_letter: "sah",
    windows_three_letter: "SAH",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// K'iche'
pub const LANG_QUT: &LanguageId = &LanguageId {
    name: "qut",
    lcid: 0x0086,
    english_name: "K'iche'",
    iso639_two_letter: "quc",
    iso639_three_letter: "quc",
    windows_three_letter: "QUT",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kinyarwanda
pub const LANG_RW: &LanguageId = &LanguageId {
    name: "rw",
    lcid: 0x0087,
    english_name: "Kinyarwanda",
    iso639_two_letter: "rw",
    iso639_three_letter: "kin",
    windows_three_letter: "KIN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Wolof
pub const LANG_WO: &LanguageId = &LanguageId {
    name: "wo",
    lcid: 0x0088,
    english_name: "Wolof",
    iso639_two_letter: "wo",
    iso639_three_letter: "wol",
    windows_three_letter: "WOL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Dari
pub const LANG_PRS: &LanguageId = &LanguageId {
    name: "prs",
    lcid: 0x008C,
    english_name: "Dari",
    iso639_two_letter: "prs",
    iso639_three_letter: "prs",
    windows_three_letter: "PRS",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Scottish Gaelic
pub const LANG_GD: &LanguageId = &LanguageId {
    name: "gd",
    lcid: 0x0091,
    english_name: "Scottish Gaelic",
    iso639_two_letter: "gd",
    iso639_three_letter: "gla",
    windows_three_letter: "GLA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Central Kurdish
pub const LANG_KU: &LanguageId = &LanguageId {
    name: "ku",
    lcid: 0x0092,
    english_name: "Central Kurdish",
    iso639_two_letter: "ku",
    iso639_three_letter: "kur",
    windows_three_letter: "KUR",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Saudi Arabia)
pub const LANG_AR_SA: &LanguageId = &LanguageId {
    name: "ar-SA",
    lcid: 0x0401,
    english_name: "Arabic (Saudi Arabia)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARA",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Bulgarian (Bulgaria)
pub const LANG_BG_BG: &LanguageId = &LanguageId {
    name: "bg-BG",
    lcid: 0x0402,
    english_name: "Bulgarian (Bulgaria)",
    iso639_two_letter: "bg",
    iso639_three_letter: "bul",
    windows_three_letter: "BGR",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Catalan (Catalan)
pub const LANG_CA_ES: &LanguageId = &LanguageId {
    name: "ca-ES",
    lcid: 0x0403,
    english_name: "Catalan (Catalan)",
    iso639_two_letter: "ca",
    iso639_three_letter: "cat",
    windows_three_letter: "CAT",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Chinese (Traditional, Taiwan)
pub const LANG_ZH_TW: &LanguageId = &LanguageId {
    name: "zh-TW",
    lcid: 0x0404,
    english_name: "Chinese (Traditional, Taiwan)",
    iso639_two_letter: "zh",
    iso639_three_letter: "zho",
    windows_three_letter: "CHT",
    ansi_code_page: Some(AnsiCodePage::Big5),
};

/// Czech (Czechia)
pub const LANG_CS_CZ: &LanguageId = &LanguageId {
    name: "cs-CZ",
    lcid: 0x0405,
    english_name: "Czech (Czechia)",
    iso639_two_letter: "cs",
    iso639_three_letter: "ces",
    windows_three_letter: "CSY",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Danish (Denmark)
pub const LANG_DA_DK: &LanguageId = &LanguageId {
    name: "da-DK",
    lcid: 0x0406,
    english_name: "Danish (Denmark)",
    iso639_two_letter: "da",
    iso639_three_letter: "dan",
    windows_three_letter: "DAN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// German (Germany)
pub const LANG_DE_DE: &LanguageId = &LanguageId {
    name: "de-DE",
    lcid: 0x0407,
    english_name: "German (Germany)",
    iso639_two_letter: "de",
    iso639_three_letter: "deu",
    windows_three_letter: "DEU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Greek (Greece)
pub const LANG_EL_GR: &LanguageId = &LanguageId {
    name: "el-GR",
    lcid: 0x0408,
    english_name: "Greek (Greece)",
    iso639_two_letter: "el",
    iso639_three_letter: "ell",
    windows_three_letter: "ELL",
    ansi_code_page: Some(AnsiCodePage::Windows1253),
};

/// English (United States)
pub const LANG_EN_US: &LanguageId = &LanguageId {
    name: "en-US",
    lcid: 0x0409,
    english_name: "English (United States)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Spain, Traditional Sort)
pub const LANG_ES_ES_TRADNL: &LanguageId = &LanguageId {
    name: "es-ES_tradnl",
    lcid: 0x040A,
    english_name: "Spanish (Spain, Traditional Sort)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESP",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Finnish (Finland)
pub const LANG_FI_FI: &LanguageId = &LanguageId {
    name: "fi-FI",
    lcid: 0x040B,
    english_name: "Finnish (Finland)",
    iso639_two_letter: "fi",
    iso639_three_letter: "fin",
    windows_three_letter: "FIN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (France)
pub const LANG_FR_FR: &LanguageId = &LanguageId {
    name: "fr-FR",
    lcid: 0x040C,
    english_name: "French (France)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hebrew (Israel)
pub const LANG_HE_IL: &LanguageId = &LanguageId {
    name: "he-IL",
    lcid: 0x040D,
    english_name: "Hebrew (Israel)",
    iso639_two_letter: "he",
    iso639_three_letter: "heb",
    windows_three_letter: "HEB",
    ansi_code_page: Some(AnsiCodePage::Windows1255),
};

/// Hungarian (Hungary)
pub const LANG_HU_HU: &LanguageId = &LanguageId {
    name: "hu-HU",
    lcid: 0x040E,
    english_name: "Hungarian (Hungary)",
    iso639_two_letter: "hu",
    iso639_three_letter: "hun",
    windows_three_letter: "HUN",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Icelandic (Iceland)
pub const LANG_IS_IS: &LanguageId = &LanguageId {
    name: "is-IS",
    lcid: 0x040F,
    english_name: "Icelandic (Iceland)",
    iso639_two_letter: "is",
    iso639_three_letter: "isl",
    windows_three_letter: "ISL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Italian (Italy)
pub const LANG_IT_IT: &LanguageId = &LanguageId {
    name: "it-IT",
    lcid: 0x0410,
    english_name: "Italian (Italy)",
    iso639_two_letter: "it",
    iso639_three_letter: "ita",
    windows_three_letter: "ITA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Japanese (Japan)
pub const LANG_JA_JP: &LanguageId = &LanguageId {
    name: "ja-JP",
    lcid: 0x0411,
    english_name: "Japanese (Japan)",
    iso639_two_letter: "ja",
    iso639_three_letter: "jpn",
    windows_three_letter: "JPN",
    ansi_code_page: Some(AnsiCodePage::ShiftJIS),
};

/// Korean (Korea)
pub const LANG_KO_KR: &LanguageId = &LanguageId {
    name: "ko-KR",
    lcid: 0x0412,
    english_name: "Korean (Korea)",
    iso639_two_letter: "ko",
    iso639_three_letter: "kor",
    windows_three_letter: "KOR",
    ansi_code_page: Some(AnsiCodePage::KsC5601),
};

/// Dutch (Netherlands)
pub const LANG_NL_NL: &LanguageId = &LanguageId {
    name: "nl-NL",
    lcid: 0x0413,
    english_name: "Dutch (Netherlands)",
    iso639_two_letter: "nl",
    iso639_three_letter: "nld",
    windows_three_letter: "NLD",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Norwegian Bokmål (Norway)
pub const LANG_NB_NO: &LanguageId = &LanguageId {
    name: "nb-NO",
    lcid: 0x0414,
    english_name: "Norwegian Bokmål (Norway)",
    iso639_two_letter: "nb",
    iso639_three_letter: "nob",
    windows_three_letter: "NOR",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Polish (Poland)
pub const LANG_PL_PL: &LanguageId = &LanguageId {
    name: "pl-PL",
    lcid: 0x0415,
    english_name: "Polish (Poland)",
    iso639_two_letter: "pl",
    iso639_three_letter: "pol",
    windows_three_letter: "PLK",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Portuguese (Brazil)
pub const LANG_PT_BR: &LanguageId = &LanguageId {
    name: "pt-BR",
    lcid: 0x0416,
    english_name: "Portuguese (Brazil)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "PTB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Romansh (Switzerland)
pub const LANG_RM_CH: &LanguageId = &LanguageId {
    name: "rm-CH",
    lcid: 0x0417,
    english_name: "Romansh (Switzerland)",
    iso639_two_letter: "rm",
    iso639_three_letter: "roh",
    windows_three_letter: "RMC",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Romanian (Romania)
pub const LANG_RO_RO: &LanguageId = &LanguageId {
    name: "ro-RO",
    lcid: 0x0418,
    english_name: "Romanian (Romania)",
    iso639_two_letter: "ro",
    iso639_three_letter: "ron",
    windows_three_letter: "ROM",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Russian (Russia)
pub const LANG_RU_RU: &LanguageId = &LanguageId {
    name: "ru-RU",
    lcid: 0x0419,
    english_name: "Russian (Russia)",
    iso639_two_letter: "ru",
    iso639_three_letter: "rus",
    windows_three_letter: "RUS",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Croatian (Croatia)
pub const LANG_HR_HR: &LanguageId = &LanguageId {
    name: "hr-HR",
    lcid: 0x041A,
    english_name: "Croatian (Croatia)",
    iso639_two_letter: "hr",
    iso639_three_letter: "hrv",
    windows_three_letter: "HRV",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Slovak (Slovakia)
pub const LANG_SK_SK: &LanguageId = &LanguageId {
    name: "sk-SK",
    lcid: 0x041B,
    english_name: "Slovak (Slovakia)",
    iso639_two_letter: "sk",
    iso639_three_letter: "slk",
    windows_three_letter: "SKY",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Albanian (Albania)
pub const LANG_SQ_AL: &LanguageId = &LanguageId {
    name: "sq-AL",
    lcid: 0x041C,
    english_name: "Albanian (Albania)",
    iso639_two_letter: "sq",
    iso639_three_letter: "sqi",
    windows_three_letter: "SQI",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Swedish (Sweden)
pub const LANG_SV_SE: &LanguageId = &LanguageId {
    name: "sv-SE",
    lcid: 0x041D,
    english_name: "Swedish (Sweden)",
    iso639_two_letter: "sv",
    iso639_three_letter: "swe",
    windows_three_letter: "SVE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Thai (Thailand)
pub const LANG_TH_TH: &LanguageId = &LanguageId {
    name: "th-TH",
    lcid: 0x041E,
    english_name: "Thai (Thailand)",
    iso639_two_letter: "th",
    iso639_three_letter: "tha",
    windows_three_letter: "THA",
    ansi_code_page: Some(AnsiCodePage::Windows874),
};

/// Turkish (Turkey)
pub const LANG_TR_TR: &LanguageId = &LanguageId {
    name: "tr-TR",
    lcid: 0x041F,
    english_name: "Turkish (Turkey)",
    iso639_two_letter: "tr",
    iso639_three_letter: "tur",
    windows_three_letter: "TRK",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Urdu (Pakistan)
pub const LANG_UR_PK: &LanguageId = &LanguageId {
    name: "ur-PK",
    lcid: 0x0420,
    english_name: "Urdu (Pakistan)",
    iso639_two_letter: "ur",
    iso639_three_letter: "urd",
    windows_three_letter: "URD",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Indonesian (Indonesia)
pub const LANG_ID_ID: &LanguageId = &LanguageId {
    name: "id-ID",
    lcid: 0x0421,
    english_name: "Indonesian (Indonesia)",
    iso639_two_letter: "id",
    iso639_three_letter: "ind",
    windows_three_letter: "IND",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Ukrainian (Ukraine)
pub const LANG_UK_UA: &LanguageId = &LanguageId {
    name: "uk-UA",
    lcid: 0x0422,
    english_name: "Ukrainian (Ukraine)",
    iso639_two_letter: "uk",
    iso639_three_letter: "ukr",
    windows_three_letter: "UKR",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Belarusian (Belarus)
pub const LANG_BE_BY: &LanguageId = &LanguageId {
    name: "be-BY",
    lcid: 0x0423,
    english_name: "Belarusian (Belarus)",
    iso639_two_letter: "be",
    iso639_three_letter: "bel",
    windows_three_letter: "BEL",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Slovenian (Slovenia)
pub const LANG_SL_SI: &LanguageId = &LanguageId {
    name: "sl-SI",
    lcid: 0x0424,
    english_name: "Slovenian (Slovenia)",
    iso639_two_letter: "sl",
    iso639_three_letter: "slv",
    windows_three_letter: "SLV",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Estonian (Estonia)
pub const LANG_ET_EE: &LanguageId = &LanguageId {
    name: "et-EE",
    lcid: 0x0425,
    english_name: "Estonian (Estonia)",
    iso639_two_letter: "et",
    iso639_three_letter: "est",
    windows_three_letter: "ETI",
    ansi_code_page: Some(AnsiCodePage::Windows1257),
};

/// Latvian (Latvia)
pub const LANG_LV_LV: &LanguageId = &LanguageId {
    name: "lv-LV",
    lcid: 0x0426,
    english_name: "Latvian (Latvia)",
    iso639_two_letter: "lv",
    iso639_three_letter: "lav",
    windows_three_letter: "LVI",
    ansi_code_page: Some(AnsiCodePage::Windows1257),
};

/// Lithuanian (Lithuania)
pub const LANG_LT_LT: &LanguageId = &LanguageId {
    name: "lt-LT",
    lcid: 0x0427,
    english_name: "Lithuanian (Lithuania)",
    iso639_two_letter: "lt",
    iso639_three_letter: "lit",
    windows_three_letter: "LTH",
    ansi_code_page: Some(AnsiCodePage::Windows1257),
};

/// Tajik (Cyrillic, Tajikistan)
pub const LANG_TG_CYRL_TJ: &LanguageId = &LanguageId {
    name: "tg-Cyrl-TJ",
    lcid: 0x0428,
    english_name: "Tajik (Cyrillic, Tajikistan)",
    iso639_two_letter: "tg",
    iso639_three_letter: "tgk",
    windows_three_letter: "TAJ",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Persian (Iran)
pub const LANG_FA_IR: &LanguageId = &LanguageId {
    name: "fa-IR",
    lcid: 0x0429,
    english_name: "Persian (Iran)",
    iso639_two_letter: "fa",
    iso639_three_letter: "fas",
    windows_three_letter: "FAR",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Vietnamese (Vietnam)
pub const LANG_VI_VN: &LanguageId = &LanguageId {
    name: "vi-VN",
    lcid: 0x042A,
    english_name: "Vietnamese (Vietnam)",
    iso639_two_letter: "vi",
    iso639_three_letter: "vie",
    windows_three_letter: "VIT",
    ansi_code_page: Some(AnsiCodePage::Windows1258),
};

/// Armenian (Armenia)
pub const LANG_HY_AM: &LanguageId = &LanguageId {
    name: "hy-AM",
    lcid: 0x042B,
    english_name: "Armenian (Armenia)",
    iso639_two_letter: "hy",
    iso639_three_letter: "hye",
    windows_three_letter: "HYE",
    ansi_code_page: None,
};

/// Azerbaijani (Latin, Azerbaijan)
pub const LANG_AZ_LATN_AZ: &LanguageId = &LanguageId {
    name: "az-Latn-AZ",
    lcid: 0x042C,
    english_name: "Azerbaijani (Latin, Azerbaijan)",
    iso639_two_letter: "az",
    iso639_three_letter: "aze",
    windows_three_letter: "AZE",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Basque (Basque)
pub const LANG_EU_ES: &LanguageId = &LanguageId {
    name: "eu-ES",
    lcid: 0x042D,
    english_name: "Basque (Basque)",
    iso639_two_letter: "eu",
    iso639_three_letter: "eus",
    windows_three_letter: "EUQ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Upper Sorbian (Germany)
pub const LANG_HSB_DE: &LanguageId = &LanguageId {
    name: "hsb-DE",
    lcid: 0x042E,
    english_name: "Upper Sorbian (Germany)",
    iso639_two_letter: "hsb",
    iso639_three_letter: "hsb",
    windows_three_letter: "HSB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Macedonian (Macedonia, FYRO)
pub const LANG_MK_MK: &LanguageId = &LanguageId {
    name: "mk-MK",
    lcid: 0x042F,
    english_name: "Macedonian (Macedonia, FYRO)",
    iso639_two_letter: "mk",
    iso639_three_letter: "mkd",
    windows_three_letter: "MKI",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Sesotho (South Africa)
pub const LANG_ST_ZA: &LanguageId = &LanguageId {
    name: "st-ZA",
    lcid: 0x0430,
    english_name: "Sesotho (South Africa)",
    iso639_two_letter: "st",
    iso639_three_letter: "sot",
    windows_three_letter: "SOT",
    ansi_code_page: None,
};

/// Xitsonga (South Africa)
pub const LANG_TS_ZA: &LanguageId = &LanguageId {
    name: "ts-ZA",
    lcid: 0x0431,
    english_name: "Xitsonga (South Africa)",
    iso639_two_letter: "ts",
    iso639_three_letter: "tso",
    windows_three_letter: "TSO",
    ansi_code_page: None,
};

/// Setswana (South Africa)
pub const LANG_TN_ZA: &LanguageId = &LanguageId {
    name: "tn-ZA",
    lcid: 0x0432,
    english_name: "Setswana (South Africa)",
    iso639_two_letter: "tn",
    iso639_three_letter: "tsn",
    windows_three_letter: "TSN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Venda (South Africa)
pub const LANG_VE_ZA: &LanguageId = &LanguageId {
    name: "ve-ZA",
    lcid: 0x0433,
    english_name: "Venda (South Africa)",
    iso639_two_letter: "ve",
    iso639_three_letter: "ven",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// isiXhosa (South Africa)
pub const LANG_XH_ZA: &LanguageId = &LanguageId {
    name: "xh-ZA",
    lcid: 0x0434,
    english_name: "isiXhosa (South Africa)",
    iso639_two_letter: "xh",
    iso639_three_letter: "xho",
    windows_three_letter: "XHO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// isiZulu (South Africa)
pub const LANG_ZU_ZA: &LanguageId = &LanguageId {
    name: "zu-ZA",
    lcid: 0x0435,
    english_name: "isiZulu (South Africa)",
    iso639_two_letter: "zu",
    iso639_three_letter: "zul",
    windows_three_letter: "ZUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Afrikaans (South Africa)
pub const LANG_AF_ZA: &LanguageId = &LanguageId {
    name: "af-ZA",
    lcid: 0x0436,
    english_name: "Afrikaans (South Africa)",
    iso639_two_letter: "af",
    iso639_three_letter: "afr",
    windows_three_letter: "AFK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Georgian (Georgia)
pub const LANG_KA_GE: &LanguageId = &LanguageId {
    name: "ka-GE",
    lcid: 0x0437,
    english_name: "Georgian (Georgia)",
    iso639_two_letter: "ka",
    iso639_three_letter: "kat",
    windows_three_letter: "KAT",
    ansi_code_page: None,
};

/// Faroese (Faroe Islands)
pub const LANG_FO_FO: &LanguageId = &LanguageId {
    name: "fo-FO",
    lcid: 0x0438,
    english_name: "Faroese (Faroe Islands)",
    iso639_two_letter: "fo",
    iso639_three_letter: "fao",
    windows_three_letter: "FOS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hindi (India)
pub const LANG_HI_IN: &LanguageId = &LanguageId {
    name: "hi-IN",
    lcid: 0x0439,
    english_name: "Hindi (India)",
    iso639_two_letter: "hi",
    iso639_three_letter: "hin",
    windows_three_letter: "HIN",
    ansi_code_page: None,
};

/// Maltese (Malta)
pub const LANG_MT_MT: &LanguageId = &LanguageId {
    name: "mt-MT",
    lcid: 0x043A,
    english_name: "Maltese (Malta)",
    iso639_two_letter: "mt",
    iso639_three_letter: "mlt",
    windows_three_letter: "MLT",
    ansi_code_page: None,
};

/// Sami, Northern (Norway)
pub const LANG_SE_NO: &LanguageId = &LanguageId {
    name: "se-NO",
    lcid: 0x043B,
    english_name: "Sami, Northern (Norway)",
    iso639_two_letter: "se",
    iso639_three_letter: "sme",
    windows_three_letter: "SME",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Malay (Malaysia)
pub const LANG_MS_MY: &LanguageId = &LanguageId {
    name: "ms-MY",
    lcid: 0x043E,
    english_name: "Malay (Malaysia)",
    iso639_two_letter: "ms",
    iso639_three_letter: "msa",
    windows_three_letter: "MSL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kazakh (Kazakhstan)
pub const LANG_KK_KZ: &LanguageId = &LanguageId {
    name: "kk-KZ",
    lcid: 0x043F,
    english_name: "Kazakh (Kazakhstan)",
    iso639_two_letter: "kk",
    iso639_three_letter: "kaz",
    windows_three_letter: "KKZ",
    ansi_code_page: None,
};

/// Kyrgyz (Kyrgyzstan)
pub const LANG_KY_KG: &LanguageId = &LanguageId {
    name: "ky-KG",
    lcid: 0x0440,
    english_name: "Kyrgyz (Kyrgyzstan)",
    iso639_two_letter: "ky",
    iso639_three_letter: "kir",
    windows_three_letter: "KYR",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Kiswahili (Kenya)
pub const LANG_SW_KE: &LanguageId = &LanguageId {
    name: "sw-KE",
    lcid: 0x0441,
    english_name: "Kiswahili (Kenya)",
    iso639_two_letter: "sw",
    iso639_three_letter: "swa",
    windows_three_letter: "SWK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Turkmen (Turkmenistan)
pub const LANG_TK_TM: &LanguageId = &LanguageId {
    name: "tk-TM",
    lcid: 0x0442,
    english_name: "Turkmen (Turkmenistan)",
    iso639_two_letter: "tk",
    iso639_three_letter: "tuk",
    windows_three_letter: "TUK",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Uzbek (Latin, Uzbekistan)
pub const LANG_UZ_LATN_UZ: &LanguageId = &LanguageId {
    name: "uz-Latn-UZ",
    lcid: 0x0443,
    english_name: "Uzbek (Latin, Uzbekistan)",
    iso639_two_letter: "uz",
    iso639_three_letter: "uzb",
    windows_three_letter: "UZB",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Tatar (Russia)
pub const LANG_TT_RU: &LanguageId = &LanguageId {
    name: "tt-RU",
    lcid: 0x0444,
    english_name: "Tatar (Russia)",
    iso639_two_letter: "tt",
    iso639_three_letter: "tat",
    windows_three_letter: "TTT",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Bangla (India)
pub const LANG_BN_IN: &LanguageId = &LanguageId {
    name: "bn-IN",
    lcid: 0x0445,
    english_name: "Bangla (India)",
    iso639_two_letter: "bn",
    iso639_three_letter: "ben",
    windows_three_letter: "BNG",
    ansi_code_page: None,
};

/// Punjabi (India)
pub const LANG_PA_IN: &LanguageId = &LanguageId {
    name: "pa-IN",
    lcid: 0x0446,
    english_name: "Punjabi (India)",
    iso639_two_letter: "pa",
    iso639_three_letter: "pan",
    windows_three_letter: "PAN",
    ansi_code_page: None,
};

/// Gujarati (India)
pub const LANG_GU_IN: &LanguageId = &LanguageId {
    name: "gu-IN",
    lcid: 0x0447,
    english_name: "Gujarati (India)",
    iso639_two_letter: "gu",
    iso639_three_letter: "guj",
    windows_three_letter: "GUJ",
    ansi_code_page: None,
};

/// Odia (India)
pub const LANG_OR_IN: &LanguageId = &LanguageId {
    name: "or-IN",
    lcid: 0x0448,
    english_name: "Odia (India)",
    iso639_two_letter: "or",
    iso639_three_letter: "ori",
    windows_three_letter: "ORI",
    ansi_code_page: None,
};

/// Tamil (India)
pub const LANG_TA_IN: &LanguageId = &LanguageId {
    name: "ta-IN",
    lcid: 0x0449,
    english_name: "Tamil (India)",
    iso639_two_letter: "ta",
    iso639_three_letter: "tam",
    windows_three_letter: "TAI",
    ansi_code_page: None,
};

/// Telugu (India)
pub const LANG_TE_IN: &LanguageId = &LanguageId {
    name: "te-IN",
    lcid: 0x044A,
    english_name: "Telugu (India)",
    iso639_two_letter: "te",
    iso639_three_letter: "tel",
    windows_three_letter: "TEL",
    ansi_code_page: None,
};

/// Kannada (India)
pub const LANG_KN_IN: &LanguageId = &LanguageId {
    name: "kn-IN",
    lcid: 0x044B,
    english_name: "Kannada (India)",
    iso639_two_letter: "kn",
    iso639_three_letter: "kan",
    windows_three_letter: "KDI",
    ansi_code_page: None,
};

/// Malayalam (India)
pub const LANG_ML_IN: &LanguageId = &LanguageId {
    name: "ml-IN",
    lcid: 0x044C,
    english_name: "Malayalam (India)",
    iso639_two_letter: "ml",
    iso639_three_letter: "mal",
    windows_three_letter: "MYM",
    ansi_code_page: None,
};

/// Assamese (India)
pub const LANG_AS_IN: &LanguageId = &LanguageId {
    name: "as-IN",
    lcid: 0x044D,
    english_name: "Assamese (India)",
    iso639_two_letter: "as",
    iso639_three_letter: "asm",
    windows_three_letter: "ASM",
    ansi_code_page: None,
};

/// Marathi (India)
pub const LANG_MR_IN: &LanguageId = &LanguageId {
    name: "mr-IN",
    lcid: 0x044E,
    english_name: "Marathi (India)",
    iso639_two_letter: "mr",
    iso639_three_letter: "mar",
    windows_three_letter: "MAR",
    ansi_code_page: None,
};

/// Sanskrit (India)
pub const LANG_SA_IN: &LanguageId = &LanguageId {
    name: "sa-IN",
    lcid: 0x044F,
    english_name: "Sanskrit (India)",
    iso639_two_letter: "sa",
    iso639_three_letter: "san",
    windows_three_letter: "SAN",
    ansi_code_page: None,
};

/// Mongolian (Mongolia)
pub const LANG_MN_MN: &LanguageId = &LanguageId {
    name: "mn-MN",
    lcid: 0x0450,
    english_name: "Mongolian (Mongolia)",
    iso639_two_letter: "mn",
    iso639_three_letter: "mon",
    windows_three_letter: "MNN",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Tibetan (China)
pub const LANG_BO_CN: &LanguageId = &LanguageId {
    name: "bo-CN",
    lcid: 0x0451,
    english_name: "Tibetan (China)",
    iso639_two_letter: "bo",
    iso639_three_letter: "bod",
    windows_three_letter: "BOB",
    ansi_code_page: None,
};

/// Welsh (United Kingdom)
pub const LANG_CY_GB: &LanguageId = &LanguageId {
    name: "cy-GB",
    lcid: 0x0452,
    english_name: "Welsh (United Kingdom)",
    iso639_two_letter: "cy",
    iso639_three_letter: "cym",
    windows_three_letter: "CYM",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Khmer (Cambodia)
pub const LANG_KM_KH: &LanguageId = &LanguageId {
    name: "km-KH",
    lcid: 0x0453,
    english_name: "Khmer (Cambodia)",
    iso639_two_letter: "km",
    iso639_three_letter: "khm",
    windows_three_letter: "KHM",
    ansi_code_page: None,
};

/// Lao (Laos)
pub const LANG_LO_LA: &LanguageId = &LanguageId {
    name: "lo-LA",
    lcid: 0x0454,
    english_name: "Lao (Laos)",
    iso639_two_letter: "lo",
    iso639_three_letter: "lao",
    windows_three_letter: "LAO",
    ansi_code_page: None,
};

/// Burmese (Myanmar)
pub const LANG_MY_MM: &LanguageId = &LanguageId {
    name: "my-MM",
    lcid: 0x0455,
    english_name: "Burmese (Myanmar)",
    iso639_two_letter: "my",
    iso639_three_letter: "mya",
    windows_three_letter: "MYA",
    ansi_code_page: None,
};

/// Galician (Galician)
pub const LANG_GL_ES: &LanguageId = &LanguageId {
    name: "gl-ES",
    lcid: 0x0456,
    english_name: "Galician (Galician)",
    iso639_two_letter: "gl",
    iso639_three_letter: "glg",
    windows_three_letter: "GLC",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Konkani (India)
pub const LANG_KOK_IN: &LanguageId = &LanguageId {
    name: "kok-IN",
    lcid: 0x0457,
    english_name: "Konkani (India)",
    iso639_two_letter: "kok",
    iso639_three_letter: "kok",
    windows_three_letter: "KNK",
    ansi_code_page: None,
};

/// Syriac (Syria)
pub const LANG_SYR_SY: &LanguageId = &LanguageId {
    name: "syr-SY",
    lcid: 0x045A,
    english_name: "Syriac (Syria)",
    iso639_two_letter: "syr",
    iso639_three_letter: "syr",
    windows_three_letter: "SYR",
    ansi_code_page: None,
};

/// Sinhala (Sri Lanka)
pub const LANG_SI_LK: &LanguageId = &LanguageId {
    name: "si-LK",
    lcid: 0x045B,
    english_name: "Sinhala (Sri Lanka)",
    iso639_two_letter: "si",
    iso639_three_letter: "sin",
    windows_three_letter: "SIN",
    ansi_code_page: None,
};

/// Cherokee (Cherokee, United States)
pub const LANG_CHR_CHER_US: &LanguageId = &LanguageId {
    name: "chr-Cher-US",
    lcid: 0x045C,
    english_name: "Cherokee (Cherokee, United States)",
    iso639_two_letter: "chr",
    iso639_three_letter: "chr",
    windows_three_letter: "CRE",
    ansi_code_page: None,
};

/// Inuktitut (Syllabics, Canada)
pub const LANG_IU_CANS_CA: &LanguageId = &LanguageId {
    name: "iu-Cans-CA",
    lcid: 0x045D,
    english_name: "Inuktitut (Syllabics, Canada)",
    iso639_two_letter: "iu",
    iso639_three_letter: "iku",
    windows_three_letter: "IUS",
    ansi_code_page: None,
};

/// Amharic (Ethiopia)
pub const LANG_AM_ET: &LanguageId = &LanguageId {
    name: "am-ET",
    lcid: 0x045E,
    english_name: "Amharic (Ethiopia)",
    iso639_two_letter: "am",
    iso639_three_letter: "amh",
    windows_three_letter: "AMH",
    ansi_code_page: None,
};

/// Kashmiri (Perso-Arabic)
pub const LANG_KS_ARAB: &LanguageId = &LanguageId {
    name: "ks-Arab",
    lcid: 0x0460,
    english_name: "Kashmiri (Perso-Arabic)",
    iso639_two_letter: "ks",
    iso639_three_letter: "kas",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nepali (Nepal)
pub const LANG_NE_NP: &LanguageId = &LanguageId {
    name: "ne-NP",
    lcid: 0x0461,
    english_name: "Nepali (Nepal)",
    iso639_two_letter: "ne",
    iso639_three_letter: "nep",
    windows_three_letter: "NEP",
    ansi_code_page: None,
};

/// Western Frisian (Netherlands)
pub const LANG_FY_NL: &LanguageId = &LanguageId {
    name: "fy-NL",
    lcid: 0x0462,
    english_name: "Western Frisian (Netherlands)",
    iso639_two_letter: "fy",
    iso639_three_letter: "fry",
    windows_three_letter: "FYN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Pashto (Afghanistan)
pub const LANG_PS_AF: &LanguageId = &LanguageId {
    name: "ps-AF",
    lcid: 0x0463,
    english_name: "Pashto (Afghanistan)",
    iso639_two_letter: "ps",
    iso639_three_letter: "pus",
    windows_three_letter: "PAS",
    ansi_code_page: None,
};

/// Filipino (Philippines)
pub const LANG_FIL_PH: &LanguageId = &LanguageId {
    name: "fil-PH",
    lcid: 0x0464,
    english_name: "Filipino (Philippines)",
    iso639_two_letter: "fil",
    iso639_three_letter: "fil",
    windows_three_letter: "FPO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Divehi (Maldives)
pub const LANG_DV_MV: &LanguageId = &LanguageId {
    name: "dv-MV",
    lcid: 0x0465,
    english_name: "Divehi (Maldives)",
    iso639_two_letter: "dv",
    iso639_three_letter: "div",
    windows_three_letter: "DIV",
    ansi_code_page: None,
};

/// Hausa (Latin, Nigeria)
pub const LANG_HA_LATN_NG: &LanguageId = &LanguageId {
    name: "ha-Latn-NG",
    lcid: 0x0468,
    english_name: "Hausa (Latin, Nigeria)",
    iso639_two_letter: "ha",
    iso639_three_letter: "hau",
    windows_three_letter: "HAU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Yoruba (Nigeria)
pub const LANG_YO_NG: &LanguageId = &LanguageId {
    name: "yo-NG",
    lcid: 0x046A,
    english_name: "Yoruba (Nigeria)",
    iso639_two_letter: "yo",
    iso639_three_letter: "yor",
    windows_three_letter: "YOR",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Quechua (Bolivia)
pub const LANG_QUZ_BO: &LanguageId = &LanguageId {
    name: "quz-BO",
    lcid: 0x046B,
    english_name: "Quechua (Bolivia)",
    iso639_two_letter: "quz",
    iso639_three_letter: "qub",
    windows_three_letter: "QUB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Sesotho sa Leboa (South Africa)
pub const LANG_NSO_ZA: &LanguageId = &LanguageId {
    name: "nso-ZA",
    lcid: 0x046C,
    english_name: "Sesotho sa Leboa (South Africa)",
    iso639_two_letter: "nso",
    iso639_three_letter: "nso",
    windows_three_letter: "NSO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Bashkir (Russia)
pub const LANG_BA_RU: &LanguageId = &LanguageId {
    name: "ba-RU",
    lcid: 0x046D,
    english_name: "Bashkir (Russia)",
    iso639_two_letter: "ba",
    iso639_three_letter: "bak",
    windows_three_letter: "BAS",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Luxembourgish (Luxembourg)
pub const LANG_LB_LU: &LanguageId = &LanguageId {
    name: "lb-LU",
    lcid: 0x046E,
    english_name: "Luxembourgish (Luxembourg)",
    iso639_two_letter: "lb",
    iso639_three_letter: "ltz",
    windows_three_letter: "LBX",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Greenlandic (Greenland)
pub const LANG_KL_GL: &LanguageId = &LanguageId {
    name: "kl-GL",
    lcid: 0x046F,
    english_name: "Greenlandic (Greenland)",
    iso639_two_letter: "kl",
    iso639_three_letter: "kal",
    windows_three_letter: "KAL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Igbo (Nigeria)
pub const LANG_IG_NG: &LanguageId = &LanguageId {
    name: "ig-NG",
    lcid: 0x0470,
    english_name: "Igbo (Nigeria)",
    iso639_two_letter: "ig",
    iso639_three_letter: "ibo",
    windows_three_letter: "IBO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Oromo (Ethiopia)
pub const LANG_OM_ET: &LanguageId = &LanguageId {
    name: "om-ET",
    lcid: 0x0472,
    english_name: "Oromo (Ethiopia)",
    iso639_two_letter: "om",
    iso639_three_letter: "orm",
    windows_three_letter: "ORM",
    ansi_code_page: None,
};

/// Tigrinya (Ethiopia)
pub const LANG_TI_ET: &LanguageId = &LanguageId {
    name: "ti-ET",
    lcid: 0x0473,
    english_name: "Tigrinya (Ethiopia)",
    iso639_two_letter: "ti",
    iso639_three_letter: "tir",
    windows_three_letter: "TIE",
    ansi_code_page: None,
};

/// Guarani (Paraguay)
pub const LANG_GN_PY: &LanguageId = &LanguageId {
    name: "gn-PY",
    lcid: 0x0474,
    english_name: "Guarani (Paraguay)",
    iso639_two_letter: "gn",
    iso639_three_letter: "grn",
    windows_three_letter: "GRN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hawaiian (United States)
pub const LANG_HAW_US: &LanguageId = &LanguageId {
    name: "haw-US",
    lcid: 0x0475,
    english_name: "Hawaiian (United States)",
    iso639_two_letter: "haw",
    iso639_three_letter: "haw",
    windows_three_letter: "HAW",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Somali (Somalia)
pub const LANG_SO_SO: &LanguageId = &LanguageId {
    name: "so-SO",
    lcid: 0x0477,
    english_name: "Somali (Somalia)",
    iso639_two_letter: "so",
    iso639_three_letter: "som",
    windows_three_letter: "SOM",
    ansi_code_page: None,
};

/// Yi (China)
pub const LANG_II_CN: &LanguageId = &LanguageId {
    name: "ii-CN",
    lcid: 0x0478,
    english_name: "Yi (China)",
    iso639_two_letter: "ii",
    iso639_three_letter: "iii",
    windows_three_letter: "III",
    ansi_code_page: None,
};

/// Mapudungun (Chile)
pub const LANG_ARN_CL: &LanguageId = &LanguageId {
    name: "arn-CL",
    lcid: 0x047A,
    english_name: "Mapudungun (Chile)",
    iso639_two_letter: "arn",
    iso639_three_letter: "arn",
    windows_three_letter: "MPD",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Mohawk (Mohawk)
pub const LANG_MOH_CA: &LanguageId = &LanguageId {
    name: "moh-CA",
    lcid: 0x047C,
    english_name: "Mohawk (Mohawk)",
    iso639_two_letter: "moh",
    iso639_three_letter: "moh",
    windows_three_letter: "MWK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Breton (France)
pub const LANG_BR_FR: &LanguageId = &LanguageId {
    name: "br-FR",
    lcid: 0x047E,
    english_name: "Breton (France)",
    iso639_two_letter: "br",
    iso639_three_letter: "bre",
    windows_three_letter: "BRE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Uyghur (China)
pub const LANG_UG_CN: &LanguageId = &LanguageId {
    name: "ug-CN",
    lcid: 0x0480,
    english_name: "Uyghur (China)",
    iso639_two_letter: "ug",
    iso639_three_letter: "uig",
    windows_three_letter: "UIG",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Maori (New Zealand)
pub const LANG_MI_NZ: &LanguageId = &LanguageId {
    name: "mi-NZ",
    lcid: 0x0481,
    english_name: "Maori (New Zealand)",
    iso639_two_letter: "mi",
    iso639_three_letter: "mri",
    windows_three_letter: "MRI",
    ansi_code_page: None,
};

/// Occitan (France)
pub const LANG_OC_FR: &LanguageId = &LanguageId {
    name: "oc-FR",
    lcid: 0x0482,
    english_name: "Occitan (France)",
    iso639_two_letter: "oc",
    iso639_three_letter: "oci",
    windows_three_letter: "OCI",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Corsican (France)
pub const LANG_CO_FR: &LanguageId = &LanguageId {
    name: "co-FR",
    lcid: 0x0483,
    english_name: "Corsican (France)",
    iso639_two_letter: "co",
    iso639_three_letter: "cos",
    windows_three_letter: "COS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Alsatian (France)
pub const LANG_GSW_FR: &LanguageId = &LanguageId {
    name: "gsw-FR",
    lcid: 0x0484,
    english_name: "Alsatian (France)",
    iso639_two_letter: "gsw",
    iso639_three_letter: "gsw",
    windows_three_letter: "GSW",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Sakha (Russia)
pub const LANG_SAH_RU: &LanguageId = &LanguageId {
    name: "sah-RU",
    lcid: 0x0485,
    english_name: "Sakha (Russia)",
    iso639_two_letter: "sah",
    iso639_three_letter: "sah",
    windows_three_letter: "SAH",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Kinyarwanda (Rwanda)
pub const LANG_RW_RW: &LanguageId = &LanguageId {
    name: "rw-RW",
    lcid: 0x0487,
    english_name: "Kinyarwanda (Rwanda)",
    iso639_two_letter: "rw",
    iso639_three_letter: "kin",
    windows_three_letter: "KIN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Wolof (Senegal)
pub const LANG_WO_SN: &LanguageId = &LanguageId {
    name: "wo-SN",
    lcid: 0x0488,
    english_name: "Wolof (Senegal)",
    iso639_two_letter: "wo",
    iso639_three_letter: "wol",
    windows_three_letter: "WOL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Dari (Afghanistan)
pub const LANG_PRS_AF: &LanguageId = &LanguageId {
    name: "prs-AF",
    lcid: 0x048C,
    english_name: "Dari (Afghanistan)",
    iso639_two_letter: "prs",
    iso639_three_letter: "prs",
    windows_three_letter: "PRS",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Scottish Gaelic (United Kingdom)
pub const LANG_GD_GB: &LanguageId = &LanguageId {
    name: "gd-GB",
    lcid: 0x0491,
    english_name: "Scottish Gaelic (United Kingdom)",
    iso639_two_letter: "gd",
    iso639_three_letter: "gla",
    windows_three_letter: "GLA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Central Kurdish (Iraq)
pub const LANG_KU_ARAB_IQ: &LanguageId = &LanguageId {
    name: "ku-Arab-IQ",
    lcid: 0x0492,
    english_name: "Central Kurdish (Iraq)",
    iso639_two_letter: "ku",
    iso639_three_letter: "kur",
    windows_three_letter: "KUR",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Pseudo (Pseudo)
pub const LANG_QPS_PLOC: &LanguageId = &LanguageId {
    name: "qps-ploc",
    lcid: 0x0501,
    english_name: "Pseudo (Pseudo)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENU",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Pseudo (Pseudo Asia)
pub const LANG_QPS_PLOCA: &LanguageId = &LanguageId {
    name: "qps-ploca",
    lcid: 0x05FE,
    english_name: "Pseudo (Pseudo Asia)",
    iso639_two_letter: "qps",
    iso639_three_letter: "jpn",
    windows_three_letter: "JPN",
    ansi_code_page: Some(AnsiCodePage::ShiftJIS),
};

/// Arabic (Iraq)
pub const LANG_AR_IQ: &LanguageId = &LanguageId {
    name: "ar-IQ",
    lcid: 0x0801,
    english_name: "Arabic (Iraq)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARI",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Valencian (Spain)
pub const LANG_CA_ES_VALENCIA: &LanguageId = &LanguageId {
    name: "ca-ES-valencia",
    lcid: 0x0803,
    english_name: "Valencian (Spain)",
    iso639_two_letter: "ca",
    iso639_three_letter: "cat",
    windows_three_letter: "VAL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Chinese (Simplified, China)
pub const LANG_ZH_CN: &LanguageId = &LanguageId {
    name: "zh-CN",
    lcid: 0x0804,
    english_name: "Chinese (Simplified, China)",
    iso639_two_letter: "zh",
    iso639_three_letter: "zho",
    windows_three_letter: "CHS",
    ansi_code_page: Some(AnsiCodePage::GB2312),
};

/// German (Switzerland)
pub const LANG_DE_CH: &LanguageId = &LanguageId {
    name: "de-CH",
    lcid: 0x0807,
    english_name: "German (Switzerland)",
    iso639_two_letter: "de",
    iso639_three_letter: "deu",
    windows_three_letter: "DES",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (United Kingdom)
pub const LANG_EN_GB: &LanguageId = &LanguageId {
    name: "en-GB",
    lcid: 0x0809,
    english_name: "English (United Kingdom)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENG",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Mexico)
pub const LANG_ES_MX: &LanguageId = &LanguageId {
    name: "es-MX",
    lcid: 0x080A,
    english_name: "Spanish (Mexico)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESM",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Belgium)
pub const LANG_FR_BE: &LanguageId = &LanguageId {
    name: "fr-BE",
    lcid: 0x080C,
    english_name: "French (Belgium)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Italian (Switzerland)
pub const LANG_IT_CH: &LanguageId = &LanguageId {
    name: "it-CH",
    lcid: 0x0810,
    english_name: "Italian (Switzerland)",
    iso639_two_letter: "it",
    iso639_three_letter: "ita",
    windows_three_letter: "ITS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Dutch (Belgium)
pub const LANG_NL_BE: &LanguageId = &LanguageId {
    name: "nl-BE",
    lcid: 0x0813,
    english_name: "Dutch (Belgium)",
    iso639_two_letter: "nl",
    iso639_three_letter: "nld",
    windows_three_letter: "NLB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Norwegian Nynorsk (Norway)
pub const LANG_NN_NO: &LanguageId = &LanguageId {
    name: "nn-NO",
    lcid: 0x0814,
    english_name: "Norwegian Nynorsk (Norway)",
    iso639_two_letter: "nn",
    iso639_three_letter: "nno",
    windows_three_letter: "NON",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Portuguese (Portugal)
pub const LANG_PT_PT: &LanguageId = &LanguageId {
    name: "pt-PT",
    lcid: 0x0816,
    english_name: "Portuguese (Portugal)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "PTG",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Romanian (Moldova)
pub const LANG_RO_MD: &LanguageId = &LanguageId {
    name: "ro-MD",
    lcid: 0x0818,
    english_name: "Romanian (Moldova)",
    iso639_two_letter: "ro",
    iso639_three_letter: "ron",
    windows_three_letter: "ROD",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Russian (Moldova)
pub const LANG_RU_MD: &LanguageId = &LanguageId {
    name: "ru-MD",
    lcid: 0x0819,
    english_name: "Russian (Moldova)",
    iso639_two_letter: "ru",
    iso639_three_letter: "rus",
    windows_three_letter: "RUM",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Serbian (Latin, Serbia and Montenegro (Former))
pub const LANG_SR_LATN_CS: &LanguageId = &LanguageId {
    name: "sr-Latn-CS",
    lcid: 0x081A,
    english_name: "Serbian (Latin, Serbia and Montenegro (Former))",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRL",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Swedish (Finland)
pub const LANG_SV_FI: &LanguageId = &LanguageId {
    name: "sv-FI",
    lcid: 0x081D,
    english_name: "Swedish (Finland)",
    iso639_two_letter: "sv",
    iso639_three_letter: "swe",
    windows_three_letter: "SVF",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Urdu (India)
pub const LANG_UR_IN: &LanguageId = &LanguageId {
    name: "ur-IN",
    lcid: 0x0820,
    english_name: "Urdu (India)",
    iso639_two_letter: "ur",
    iso639_three_letter: "urd",
    windows_three_letter: "URI",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Lower Sorbian (Germany)
pub const LANG_DSB_DE: &LanguageId = &LanguageId {
    name: "dsb-DE",
    lcid: 0x082E,
    english_name: "Lower Sorbian (Germany)",
    iso639_two_letter: "dsb",
    iso639_three_letter: "dsb",
    windows_three_letter: "DSB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Setswana (Botswana)
pub const LANG_TN_BW: &LanguageId = &LanguageId {
    name: "tn-BW",
    lcid: 0x0832,
    english_name: "Setswana (Botswana)",
    iso639_two_letter: "tn",
    iso639_three_letter: "tsn",
    windows_three_letter: "TSB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Sami, Northern (Sweden)
pub const LANG_SE_SE: &LanguageId = &LanguageId {
    name: "se-SE",
    lcid: 0x083B,
    english_name: "Sami, Northern (Sweden)",
    iso639_two_letter: "se",
    iso639_three_letter: "sme",
    windows_three_letter: "SMF",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Irish (Ireland)
pub const LANG_GA_IE: &LanguageId = &LanguageId {
    name: "ga-IE",
    lcid: 0x083C,
    english_name: "Irish (Ireland)",
    iso639_two_letter: "ga",
    iso639_three_letter: "gle",
    windows_three_letter: "IRE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Malay (Brunei)
pub const LANG_MS_BN: &LanguageId = &LanguageId {
    name: "ms-BN",
    lcid: 0x083E,
    english_name: "Malay (Brunei)",
    iso639_two_letter: "ms",
    iso639_three_letter: "msa",
    windows_three_letter: "MSB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Bangla (Bangladesh)
pub const LANG_BN_BD: &LanguageId = &LanguageId {
    name: "bn-BD",
    lcid: 0x0845,
    english_name: "Bangla (Bangladesh)",
    iso639_two_letter: "bn",
    iso639_three_letter: "ben",
    windows_three_letter: "BNB",
    ansi_code_page: None,
};

/// Punjabi (Pakistan)
pub const LANG_PA_ARAB_PK: &LanguageId = &LanguageId {
    name: "pa-Arab-PK",
    lcid: 0x0846,
    english_name: "Punjabi (Pakistan)",
    iso639_two_letter: "pa",
    iso639_three_letter: "pan",
    windows_three_letter: "PAP",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Tamil (Sri Lanka)
pub const LANG_TA_LK: &LanguageId = &LanguageId {
    name: "ta-LK",
    lcid: 0x0849,
    english_name: "Tamil (Sri Lanka)",
    iso639_two_letter: "ta",
    iso639_three_letter: "tam",
    windows_three_letter: "TAM",
    ansi_code_page: None,
};

/// Sindhi (Pakistan)
pub const LANG_SD_ARAB_PK: &LanguageId = &LanguageId {
    name: "sd-Arab-PK",
    lcid: 0x0859,
    english_name: "Sindhi (Pakistan)",
    iso639_two_letter: "sd",
    iso639_three_letter: "snd",
    windows_three_letter: "SIP",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Inuktitut (Latin, Canada)
pub const LANG_IU_LATN_CA: &LanguageId = &LanguageId {
    name: "iu-Latn-CA",
    lcid: 0x085D,
    english_name: "Inuktitut (Latin, Canada)",
    iso639_two_letter: "iu",
    iso639_three_letter: "iku",
    windows_three_letter: "IUK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Central Atlas Tamazight (Latin, Algeria)
pub const LANG_TZM_LATN_DZ: &LanguageId = &LanguageId {
    name: "tzm-Latn-DZ",
    lcid: 0x085F,
    english_name: "Central Atlas Tamazight (Latin, Algeria)",
    iso639_two_letter: "tzm",
    iso639_three_letter: "tzm",
    windows_three_letter: "TZA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Nepali (India)
pub const LANG_NE_IN: &LanguageId = &LanguageId {
    name: "ne-IN",
    lcid: 0x0861,
    english_name: "Nepali (India)",
    iso639_two_letter: "ne",
    iso639_three_letter: "nep",
    windows_three_letter: "NEI",
    ansi_code_page: None,
};

/// Fulah (Latin, Senegal)
pub const LANG_FF_LATN_SN: &LanguageId = &LanguageId {
    name: "ff-Latn-SN",
    lcid: 0x0867,
    english_name: "Fulah (Latin, Senegal)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Quichua (Ecuador)
pub const LANG_QUZ_EC: &LanguageId = &LanguageId {
    name: "quz-EC",
    lcid: 0x086B,
    english_name: "Quichua (Ecuador)",
    iso639_two_letter: "quz",
    iso639_three_letter: "que",
    windows_three_letter: "QUE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Tigrinya (Eritrea)
pub const LANG_TI_ER: &LanguageId = &LanguageId {
    name: "ti-ER",
    lcid: 0x0873,
    english_name: "Tigrinya (Eritrea)",
    iso639_two_letter: "ti",
    iso639_three_letter: "tir",
    windows_three_letter: "TIR",
    ansi_code_page: None,
};

/// Pseudo (Pseudo Mirrored)
pub const LANG_QPS_PLOCM: &LanguageId = &LanguageId {
    name: "qps-plocm",
    lcid: 0x09FF,
    english_name: "Pseudo (Pseudo Mirrored)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARA",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Egypt)
pub const LANG_AR_EG: &LanguageId = &LanguageId {
    name: "ar-EG",
    lcid: 0x0C01,
    english_name: "Arabic (Egypt)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARE",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Chinese (Traditional, Hong Kong SAR)
pub const LANG_ZH_HK: &LanguageId = &LanguageId {
    name: "zh-HK",
    lcid: 0x0C04,
    english_name: "Chinese (Traditional, Hong Kong SAR)",
    iso639_two_letter: "zh",
    iso639_three_letter: "zho",
    windows_three_letter: "ZHH",
    ansi_code_page: Some(AnsiCodePage::Big5),
};

/// German (Austria)
pub const LANG_DE_AT: &LanguageId = &LanguageId {
    name: "de-AT",
    lcid: 0x0C07,
    english_name: "German (Austria)",
    iso639_two_letter: "de",
    iso639_three_letter: "deu",
    windows_three_letter: "DEA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Australia)
pub const LANG_EN_AU: &LanguageId = &LanguageId {
    name: "en-AU",
    lcid: 0x0C09,
    english_name: "English (Australia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Spain, International Sort)
pub const LANG_ES_ES: &LanguageId = &LanguageId {
    name: "es-ES",
    lcid: 0x0C0A,
    english_name: "Spanish (Spain, International Sort)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Canada)
pub const LANG_FR_CA: &LanguageId = &LanguageId {
    name: "fr-CA",
    lcid: 0x0C0C,
    english_name: "French (Canada)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRC",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Serbian (Cyrillic, Serbia and Montenegro (Former))
pub const LANG_SR_CYRL_CS: &LanguageId = &LanguageId {
    name: "sr-Cyrl-CS",
    lcid: 0x0C1A,
    english_name: "Serbian (Cyrillic, Serbia and Montenegro (Former))",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRB",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Sami, Northern (Finland)
pub const LANG_SE_FI: &LanguageId = &LanguageId {
    name: "se-FI",
    lcid: 0x0C3B,
    english_name: "Sami, Northern (Finland)",
    iso639_two_letter: "se",
    iso639_three_letter: "sme",
    windows_three_letter: "SMG",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Mongolian (Traditional Mongolian, Mongolia)
pub const LANG_MN_MONG_MN: &LanguageId = &LanguageId {
    name: "mn-Mong-MN",
    lcid: 0x0C50,
    english_name: "Mongolian (Traditional Mongolian, Mongolia)",
    iso639_two_letter: "mn",
    iso639_three_letter: "mon",
    windows_three_letter: "MNM",
    ansi_code_page: None,
};

/// Dzongkha (Bhutan)
pub const LANG_DZ_BT: &LanguageId = &LanguageId {
    name: "dz-BT",
    lcid: 0x0C51,
    english_name: "Dzongkha (Bhutan)",
    iso639_two_letter: "dz",
    iso639_three_letter: "dzo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Quechua (Peru)
pub const LANG_QUZ_PE: &LanguageId = &LanguageId {
    name: "quz-PE",
    lcid: 0x0C6B,
    english_name: "Quechua (Peru)",
    iso639_two_letter: "quz",
    iso639_three_letter: "qup",
    windows_three_letter: "QUP",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Afar
pub const LANG_AA: &LanguageId = &LanguageId {
    name: "aa",
    lcid: 0x1000,
    english_name: "Afar",
    iso639_two_letter: "aa",
    iso639_three_letter: "aar",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Afar (Djibouti)
pub const LANG_AA_DJ: &LanguageId = &LanguageId {
    name: "aa-DJ",
    lcid: 0x1000,
    english_name: "Afar (Djibouti)",
    iso639_two_letter: "aa",
    iso639_three_letter: "aar",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Afar (Eritrea)
pub const LANG_AA_ER: &LanguageId = &LanguageId {
    name: "aa-ER",
    lcid: 0x1000,
    english_name: "Afar (Eritrea)",
    iso639_two_letter: "aa",
    iso639_three_letter: "aar",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Afar (Ethiopia)
pub const LANG_AA_ET: &LanguageId = &LanguageId {
    name: "aa-ET",
    lcid: 0x1000,
    english_name: "Afar (Ethiopia)",
    iso639_two_letter: "aa",
    iso639_three_letter: "aar",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Afrikaans (Namibia)
pub const LANG_AF_NA: &LanguageId = &LanguageId {
    name: "af-NA",
    lcid: 0x1000,
    english_name: "Afrikaans (Namibia)",
    iso639_two_letter: "af",
    iso639_three_letter: "afr",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Aghem
pub const LANG_AGQ: &LanguageId = &LanguageId {
    name: "agq",
    lcid: 0x1000,
    english_name: "Aghem",
    iso639_two_letter: "agq",
    iso639_three_letter: "agq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Aghem (Cameroon)
pub const LANG_AGQ_CM: &LanguageId = &LanguageId {
    name: "agq-CM",
    lcid: 0x1000,
    english_name: "Aghem (Cameroon)",
    iso639_two_letter: "agq",
    iso639_three_letter: "agq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Akan
pub const LANG_AK: &LanguageId = &LanguageId {
    name: "ak",
    lcid: 0x1000,
    english_name: "Akan",
    iso639_two_letter: "ak",
    iso639_three_letter: "aka",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Akan (Ghana)
pub const LANG_AK_GH: &LanguageId = &LanguageId {
    name: "ak-GH",
    lcid: 0x1000,
    english_name: "Akan (Ghana)",
    iso639_two_letter: "ak",
    iso639_three_letter: "aka",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Arabic (World)
pub const LANG_AR_001: &LanguageId = &LanguageId {
    name: "ar-001",
    lcid: 0x1000,
    english_name: "Arabic (World)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Djibouti)
pub const LANG_AR_DJ: &LanguageId = &LanguageId {
    name: "ar-DJ",
    lcid: 0x1000,
    english_name: "Arabic (Djibouti)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Eritrea)
pub const LANG_AR_ER: &LanguageId = &LanguageId {
    name: "ar-ER",
    lcid: 0x1000,
    english_name: "Arabic (Eritrea)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Israel)
pub const LANG_AR_IL: &LanguageId = &LanguageId {
    name: "ar-IL",
    lcid: 0x1000,
    english_name: "Arabic (Israel)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Comoros)
pub const LANG_AR_KM: &LanguageId = &LanguageId {
    name: "ar-KM",
    lcid: 0x1000,
    english_name: "Arabic (Comoros)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Mauritania)
pub const LANG_AR_MR: &LanguageId = &LanguageId {
    name: "ar-MR",
    lcid: 0x1000,
    english_name: "Arabic (Mauritania)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Palestinian Authority)
pub const LANG_AR_PS: &LanguageId = &LanguageId {
    name: "ar-PS",
    lcid: 0x1000,
    english_name: "Arabic (Palestinian Authority)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Sudan)
pub const LANG_AR_SD: &LanguageId = &LanguageId {
    name: "ar-SD",
    lcid: 0x1000,
    english_name: "Arabic (Sudan)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Somalia)
pub const LANG_AR_SO: &LanguageId = &LanguageId {
    name: "ar-SO",
    lcid: 0x1000,
    english_name: "Arabic (Somalia)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (South Sudan)
pub const LANG_AR_SS: &LanguageId = &LanguageId {
    name: "ar-SS",
    lcid: 0x1000,
    english_name: "Arabic (South Sudan)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Arabic (Chad)
pub const LANG_AR_TD: &LanguageId = &LanguageId {
    name: "ar-TD",
    lcid: 0x1000,
    english_name: "Arabic (Chad)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Asu
pub const LANG_ASA: &LanguageId = &LanguageId {
    name: "asa",
    lcid: 0x1000,
    english_name: "Asu",
    iso639_two_letter: "asa",
    iso639_three_letter: "asa",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Asu (Tanzania)
pub const LANG_ASA_TZ: &LanguageId = &LanguageId {
    name: "asa-TZ",
    lcid: 0x1000,
    english_name: "Asu (Tanzania)",
    iso639_two_letter: "asa",
    iso639_three_letter: "asa",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Asturian
pub const LANG_AST: &LanguageId = &LanguageId {
    name: "ast",
    lcid: 0x1000,
    english_name: "Asturian",
    iso639_two_letter: "ast",
    iso639_three_letter: "ast",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Asturian (Spain)
pub const LANG_AST_ES: &LanguageId = &LanguageId {
    name: "ast-ES",
    lcid: 0x1000,
    english_name: "Asturian (Spain)",
    iso639_two_letter: "ast",
    iso639_three_letter: "ast",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Basaa
pub const LANG_BAS: &LanguageId = &LanguageId {
    name: "bas",
    lcid: 0x1000,
    english_name: "Basaa",
    iso639_two_letter: "bas",
    iso639_three_letter: "bas",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Basaa (Cameroon)
pub const LANG_BAS_CM: &LanguageId = &LanguageId {
    name: "bas-CM",
    lcid: 0x1000,
    english_name: "Basaa (Cameroon)",
    iso639_two_letter: "bas",
    iso639_three_letter: "bas",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bemba
pub const LANG_BEM: &LanguageId = &LanguageId {
    name: "bem",
    lcid: 0x1000,
    english_name: "Bemba",
    iso639_two_letter: "bem",
    iso639_three_letter: "bem",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bemba (Zambia)
pub const LANG_BEM_ZM: &LanguageId = &LanguageId {
    name: "bem-ZM",
    lcid: 0x1000,
    english_name: "Bemba (Zambia)",
    iso639_two_letter: "bem",
    iso639_three_letter: "bem",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bena
pub const LANG_BEZ: &LanguageId = &LanguageId {
    name: "bez",
    lcid: 0x1000,
    english_name: "Bena",
    iso639_two_letter: "bez",
    iso639_three_letter: "bez",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bena (Tanzania)
pub const LANG_BEZ_TZ: &LanguageId = &LanguageId {
    name: "bez-TZ",
    lcid: 0x1000,
    english_name: "Bena (Tanzania)",
    iso639_two_letter: "bez",
    iso639_three_letter: "bez",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bamanankan
pub const LANG_BM: &LanguageId = &LanguageId {
    name: "bm",
    lcid: 0x1000,
    english_name: "Bamanankan",
    iso639_two_letter: "bm",
    iso639_three_letter: "bam",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bamanankan (Latin, Mali)
pub const LANG_BM_ML: &LanguageId = &LanguageId {
    name: "bm-ML",
    lcid: 0x1000,
    english_name: "Bamanankan (Latin, Mali)",
    iso639_two_letter: "bm",
    iso639_three_letter: "bam",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tibetan (India)
pub const LANG_BO_IN: &LanguageId = &LanguageId {
    name: "bo-IN",
    lcid: 0x1000,
    english_name: "Tibetan (India)",
    iso639_two_letter: "bo",
    iso639_three_letter: "bod",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bodo
pub const LANG_BRX: &LanguageId = &LanguageId {
    name: "brx",
    lcid: 0x1000,
    english_name: "Bodo",
    iso639_two_letter: "brx",
    iso639_three_letter: "brx",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bodo (India)
pub const LANG_BRX_IN: &LanguageId = &LanguageId {
    name: "brx-IN",
    lcid: 0x1000,
    english_name: "Bodo (India)",
    iso639_two_letter: "brx",
    iso639_three_letter: "brx",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Blin
pub const LANG_BYN: &LanguageId = &LanguageId {
    name: "byn",
    lcid: 0x1000,
    english_name: "Blin",
    iso639_two_letter: "byn",
    iso639_three_letter: "byn",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Blin (Eritrea)
pub const LANG_BYN_ER: &LanguageId = &LanguageId {
    name: "byn-ER",
    lcid: 0x1000,
    english_name: "Blin (Eritrea)",
    iso639_two_letter: "byn",
    iso639_three_letter: "byn",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Catalan (Andorra)
pub const LANG_CA_AD: &LanguageId = &LanguageId {
    name: "ca-AD",
    lcid: 0x1000,
    english_name: "Catalan (Andorra)",
    iso639_two_letter: "ca",
    iso639_three_letter: "cat",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Catalan (France)
pub const LANG_CA_FR: &LanguageId = &LanguageId {
    name: "ca-FR",
    lcid: 0x1000,
    english_name: "Catalan (France)",
    iso639_two_letter: "ca",
    iso639_three_letter: "cat",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Catalan (Italy)
pub const LANG_CA_IT: &LanguageId = &LanguageId {
    name: "ca-IT",
    lcid: 0x1000,
    english_name: "Catalan (Italy)",
    iso639_two_letter: "ca",
    iso639_three_letter: "cat",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Language (ccp)
pub const LANG_CCP: &LanguageId = &LanguageId {
    name: "ccp",
    lcid: 0x1000,
    english_name: "Unknown Language (ccp)",
    iso639_two_letter: "ccp",
    iso639_three_letter: "ccp",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Language (ccp-Cakm)
pub const LANG_CCP_CAKM: &LanguageId = &LanguageId {
    name: "ccp-Cakm",
    lcid: 0x1000,
    english_name: "Unknown Language (ccp-Cakm)",
    iso639_two_letter: "ccp",
    iso639_three_letter: "ccp",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ccp-Cakm-BD)
pub const LANG_CCP_CAKM_BD: &LanguageId = &LanguageId {
    name: "ccp-Cakm-BD",
    lcid: 0x1000,
    english_name: "Unknown Locale (ccp-Cakm-BD)",
    iso639_two_letter: "ccp",
    iso639_three_letter: "ccp",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ccp-Cakm-IN)
pub const LANG_CCP_CAKM_IN: &LanguageId = &LanguageId {
    name: "ccp-Cakm-IN",
    lcid: 0x1000,
    english_name: "Unknown Locale (ccp-Cakm-IN)",
    iso639_two_letter: "ccp",
    iso639_three_letter: "ccp",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Chechen (Russia)
pub const LANG_CE_RU: &LanguageId = &LanguageId {
    name: "ce-RU",
    lcid: 0x1000,
    english_name: "Chechen (Russia)",
    iso639_two_letter: "ce",
    iso639_three_letter: "che",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Unknown Language (ceb)
pub const LANG_CEB: &LanguageId = &LanguageId {
    name: "ceb",
    lcid: 0x1000,
    english_name: "Unknown Language (ceb)",
    iso639_two_letter: "ceb",
    iso639_three_letter: "ceb",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Language (ceb-Latn)
pub const LANG_CEB_LATN: &LanguageId = &LanguageId {
    name: "ceb-Latn",
    lcid: 0x1000,
    english_name: "Unknown Language (ceb-Latn)",
    iso639_two_letter: "ceb",
    iso639_three_letter: "ceb",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ceb-Latn-PH)
pub const LANG_CEB_LATN_PH: &LanguageId = &LanguageId {
    name: "ceb-Latn-PH",
    lcid: 0x1000,
    english_name: "Unknown Locale (ceb-Latn-PH)",
    iso639_two_letter: "ceb",
    iso639_three_letter: "ceb",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Chiga
pub const LANG_CGG: &LanguageId = &LanguageId {
    name: "cgg",
    lcid: 0x1000,
    english_name: "Chiga",
    iso639_two_letter: "cgg",
    iso639_three_letter: "cgg",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Chiga (Uganda)
pub const LANG_CGG_UG: &LanguageId = &LanguageId {
    name: "cgg-UG",
    lcid: 0x1000,
    english_name: "Chiga (Uganda)",
    iso639_two_letter: "cgg",
    iso639_three_letter: "cgg",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Church Slavic (Russia)
pub const LANG_CU_RU: &LanguageId = &LanguageId {
    name: "cu-RU",
    lcid: 0x1000,
    english_name: "Church Slavic (Russia)",
    iso639_two_letter: "cu",
    iso639_three_letter: "chu",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Danish (Greenland)
pub const LANG_DA_GL: &LanguageId = &LanguageId {
    name: "da-GL",
    lcid: 0x1000,
    english_name: "Danish (Greenland)",
    iso639_two_letter: "da",
    iso639_three_letter: "dan",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Taita
pub const LANG_DAV: &LanguageId = &LanguageId {
    name: "dav",
    lcid: 0x1000,
    english_name: "Taita",
    iso639_two_letter: "dav",
    iso639_three_letter: "dav",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Taita (Kenya)
pub const LANG_DAV_KE: &LanguageId = &LanguageId {
    name: "dav-KE",
    lcid: 0x1000,
    english_name: "Taita (Kenya)",
    iso639_two_letter: "dav",
    iso639_three_letter: "dav",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// German (Belgium)
pub const LANG_DE_BE: &LanguageId = &LanguageId {
    name: "de-BE",
    lcid: 0x1000,
    english_name: "German (Belgium)",
    iso639_two_letter: "de",
    iso639_three_letter: "deu",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// German (Italy)
pub const LANG_DE_IT: &LanguageId = &LanguageId {
    name: "de-IT",
    lcid: 0x1000,
    english_name: "German (Italy)",
    iso639_two_letter: "de",
    iso639_three_letter: "deu",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Zarma
pub const LANG_DJE: &LanguageId = &LanguageId {
    name: "dje",
    lcid: 0x1000,
    english_name: "Zarma",
    iso639_two_letter: "dje",
    iso639_three_letter: "dje",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Zarma (Niger)
pub const LANG_DJE_NE: &LanguageId = &LanguageId {
    name: "dje-NE",
    lcid: 0x1000,
    english_name: "Zarma (Niger)",
    iso639_two_letter: "dje",
    iso639_three_letter: "dje",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Duala
pub const LANG_DUA: &LanguageId = &LanguageId {
    name: "dua",
    lcid: 0x1000,
    english_name: "Duala",
    iso639_two_letter: "dua",
    iso639_three_letter: "dua",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Duala (Cameroon)
pub const LANG_DUA_CM: &LanguageId = &LanguageId {
    name: "dua-CM",
    lcid: 0x1000,
    english_name: "Duala (Cameroon)",
    iso639_two_letter: "dua",
    iso639_three_letter: "dua",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Jola-Fonyi
pub const LANG_DYO: &LanguageId = &LanguageId {
    name: "dyo",
    lcid: 0x1000,
    english_name: "Jola-Fonyi",
    iso639_two_letter: "dyo",
    iso639_three_letter: "dyo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Jola-Fonyi (Senegal)
pub const LANG_DYO_SN: &LanguageId = &LanguageId {
    name: "dyo-SN",
    lcid: 0x1000,
    english_name: "Jola-Fonyi (Senegal)",
    iso639_two_letter: "dyo",
    iso639_three_letter: "dyo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Dzongkha
pub const LANG_DZ: &LanguageId = &LanguageId {
    name: "dz",
    lcid: 0x1000,
    english_name: "Dzongkha",
    iso639_two_letter: "dz",
    iso639_three_letter: "dzo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Embu
pub const LANG_EBU: &LanguageId = &LanguageId {
    name: "ebu",
    lcid: 0x1000,
    english_name: "Embu",
    iso639_two_letter: "ebu",
    iso639_three_letter: "ebu",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Embu (Kenya)
pub const LANG_EBU_KE: &LanguageId = &LanguageId {
    name: "ebu-KE",
    lcid: 0x1000,
    english_name: "Embu (Kenya)",
    iso639_two_letter: "ebu",
    iso639_three_letter: "ebu",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ewe
pub const LANG_EE: &LanguageId = &LanguageId {
    name: "ee",
    lcid: 0x1000,
    english_name: "Ewe",
    iso639_two_letter: "ee",
    iso639_three_letter: "ewe",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ewe (Ghana)
pub const LANG_EE_GH: &LanguageId = &LanguageId {
    name: "ee-GH",
    lcid: 0x1000,
    english_name: "Ewe (Ghana)",
    iso639_two_letter: "ee",
    iso639_three_letter: "ewe",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ewe (Togo)
pub const LANG_EE_TG: &LanguageId = &LanguageId {
    name: "ee-TG",
    lcid: 0x1000,
    english_name: "Ewe (Togo)",
    iso639_two_letter: "ee",
    iso639_three_letter: "ewe",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Greek (Cyprus)
pub const LANG_EL_CY: &LanguageId = &LanguageId {
    name: "el-CY",
    lcid: 0x1000,
    english_name: "Greek (Cyprus)",
    iso639_two_letter: "el",
    iso639_three_letter: "ell",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1253),
};

/// English (World)
pub const LANG_EN_001: &LanguageId = &LanguageId {
    name: "en-001",
    lcid: 0x1000,
    english_name: "English (World)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Europe)
pub const LANG_EN_150: &LanguageId = &LanguageId {
    name: "en-150",
    lcid: 0x1000,
    english_name: "English (Europe)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Antigua and Barbuda)
pub const LANG_EN_AG: &LanguageId = &LanguageId {
    name: "en-AG",
    lcid: 0x1000,
    english_name: "English (Antigua and Barbuda)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Anguilla)
pub const LANG_EN_AI: &LanguageId = &LanguageId {
    name: "en-AI",
    lcid: 0x1000,
    english_name: "English (Anguilla)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (American Samoa)
pub const LANG_EN_AS: &LanguageId = &LanguageId {
    name: "en-AS",
    lcid: 0x1000,
    english_name: "English (American Samoa)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Austria)
pub const LANG_EN_AT: &LanguageId = &LanguageId {
    name: "en-AT",
    lcid: 0x1000,
    english_name: "English (Austria)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Barbados)
pub const LANG_EN_BB: &LanguageId = &LanguageId {
    name: "en-BB",
    lcid: 0x1000,
    english_name: "English (Barbados)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Belgium)
pub const LANG_EN_BE: &LanguageId = &LanguageId {
    name: "en-BE",
    lcid: 0x1000,
    english_name: "English (Belgium)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Burundi)
pub const LANG_EN_BI: &LanguageId = &LanguageId {
    name: "en-BI",
    lcid: 0x1000,
    english_name: "English (Burundi)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Bermuda)
pub const LANG_EN_BM: &LanguageId = &LanguageId {
    name: "en-BM",
    lcid: 0x1000,
    english_name: "English (Bermuda)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Bahamas)
pub const LANG_EN_BS: &LanguageId = &LanguageId {
    name: "en-BS",
    lcid: 0x1000,
    english_name: "English (Bahamas)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Botswana)
pub const LANG_EN_BW: &LanguageId = &LanguageId {
    name: "en-BW",
    lcid: 0x1000,
    english_name: "English (Botswana)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Cocos (Keeling) Islands)
pub const LANG_EN_CC: &LanguageId = &LanguageId {
    name: "en-CC",
    lcid: 0x1000,
    english_name: "English (Cocos (Keeling) Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Switzerland)
pub const LANG_EN_CH: &LanguageId = &LanguageId {
    name: "en-CH",
    lcid: 0x1000,
    english_name: "English (Switzerland)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Cook Islands)
pub const LANG_EN_CK: &LanguageId = &LanguageId {
    name: "en-CK",
    lcid: 0x1000,
    english_name: "English (Cook Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Cameroon)
pub const LANG_EN_CM: &LanguageId = &LanguageId {
    name: "en-CM",
    lcid: 0x1000,
    english_name: "English (Cameroon)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Christmas Island)
pub const LANG_EN_CX: &LanguageId = &LanguageId {
    name: "en-CX",
    lcid: 0x1000,
    english_name: "English (Christmas Island)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Cyprus)
pub const LANG_EN_CY: &LanguageId = &LanguageId {
    name: "en-CY",
    lcid: 0x1000,
    english_name: "English (Cyprus)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Germany)
pub const LANG_EN_DE: &LanguageId = &LanguageId {
    name: "en-DE",
    lcid: 0x1000,
    english_name: "English (Germany)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Denmark)
pub const LANG_EN_DK: &LanguageId = &LanguageId {
    name: "en-DK",
    lcid: 0x1000,
    english_name: "English (Denmark)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Dominica)
pub const LANG_EN_DM: &LanguageId = &LanguageId {
    name: "en-DM",
    lcid: 0x1000,
    english_name: "English (Dominica)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Eritrea)
pub const LANG_EN_ER: &LanguageId = &LanguageId {
    name: "en-ER",
    lcid: 0x1000,
    english_name: "English (Eritrea)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Finland)
pub const LANG_EN_FI: &LanguageId = &LanguageId {
    name: "en-FI",
    lcid: 0x1000,
    english_name: "English (Finland)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Fiji)
pub const LANG_EN_FJ: &LanguageId = &LanguageId {
    name: "en-FJ",
    lcid: 0x1000,
    english_name: "English (Fiji)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Falkland Islands)
pub const LANG_EN_FK: &LanguageId = &LanguageId {
    name: "en-FK",
    lcid: 0x1000,
    english_name: "English (Falkland Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Micronesia)
pub const LANG_EN_FM: &LanguageId = &LanguageId {
    name: "en-FM",
    lcid: 0x1000,
    english_name: "English (Micronesia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Grenada)
pub const LANG_EN_GD: &LanguageId = &LanguageId {
    name: "en-GD",
    lcid: 0x1000,
    english_name: "English (Grenada)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Guernsey)
pub const LANG_EN_GG: &LanguageId = &LanguageId {
    name: "en-GG",
    lcid: 0x1000,
    english_name: "English (Guernsey)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Ghana)
pub const LANG_EN_GH: &LanguageId = &LanguageId {
    name: "en-GH",
    lcid: 0x1000,
    english_name: "English (Ghana)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Gibraltar)
pub const LANG_EN_GI: &LanguageId = &LanguageId {
    name: "en-GI",
    lcid: 0x1000,
    english_name: "English (Gibraltar)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Gambia)
pub const LANG_EN_GM: &LanguageId = &LanguageId {
    name: "en-GM",
    lcid: 0x1000,
    english_name: "English (Gambia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Guam)
pub const LANG_EN_GU: &LanguageId = &LanguageId {
    name: "en-GU",
    lcid: 0x1000,
    english_name: "English (Guam)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Guyana)
pub const LANG_EN_GY: &LanguageId = &LanguageId {
    name: "en-GY",
    lcid: 0x1000,
    english_name: "English (Guyana)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Israel)
pub const LANG_EN_IL: &LanguageId = &LanguageId {
    name: "en-IL",
    lcid: 0x1000,
    english_name: "English (Israel)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Isle of Man)
pub const LANG_EN_IM: &LanguageId = &LanguageId {
    name: "en-IM",
    lcid: 0x1000,
    english_name: "English (Isle of Man)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (British Indian Ocean Territory)
pub const LANG_EN_IO: &LanguageId = &LanguageId {
    name: "en-IO",
    lcid: 0x1000,
    english_name: "English (British Indian Ocean Territory)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Jersey)
pub const LANG_EN_JE: &LanguageId = &LanguageId {
    name: "en-JE",
    lcid: 0x1000,
    english_name: "English (Jersey)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Kenya)
pub const LANG_EN_KE: &LanguageId = &LanguageId {
    name: "en-KE",
    lcid: 0x1000,
    english_name: "English (Kenya)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Kiribati)
pub const LANG_EN_KI: &LanguageId = &LanguageId {
    name: "en-KI",
    lcid: 0x1000,
    english_name: "English (Kiribati)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Saint Kitts and Nevis)
pub const LANG_EN_KN: &LanguageId = &LanguageId {
    name: "en-KN",
    lcid: 0x1000,
    english_name: "English (Saint Kitts and Nevis)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Cayman Islands)
pub const LANG_EN_KY: &LanguageId = &LanguageId {
    name: "en-KY",
    lcid: 0x1000,
    english_name: "English (Cayman Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Saint Lucia)
pub const LANG_EN_LC: &LanguageId = &LanguageId {
    name: "en-LC",
    lcid: 0x1000,
    english_name: "English (Saint Lucia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Liberia)
pub const LANG_EN_LR: &LanguageId = &LanguageId {
    name: "en-LR",
    lcid: 0x1000,
    english_name: "English (Liberia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Lesotho)
pub const LANG_EN_LS: &LanguageId = &LanguageId {
    name: "en-LS",
    lcid: 0x1000,
    english_name: "English (Lesotho)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Madagascar)
pub const LANG_EN_MG: &LanguageId = &LanguageId {
    name: "en-MG",
    lcid: 0x1000,
    english_name: "English (Madagascar)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Marshall Islands)
pub const LANG_EN_MH: &LanguageId = &LanguageId {
    name: "en-MH",
    lcid: 0x1000,
    english_name: "English (Marshall Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Macao SAR)
pub const LANG_EN_MO: &LanguageId = &LanguageId {
    name: "en-MO",
    lcid: 0x1000,
    english_name: "English (Macao SAR)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Northern Mariana Islands)
pub const LANG_EN_MP: &LanguageId = &LanguageId {
    name: "en-MP",
    lcid: 0x1000,
    english_name: "English (Northern Mariana Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Montserrat)
pub const LANG_EN_MS: &LanguageId = &LanguageId {
    name: "en-MS",
    lcid: 0x1000,
    english_name: "English (Montserrat)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Malta)
pub const LANG_EN_MT: &LanguageId = &LanguageId {
    name: "en-MT",
    lcid: 0x1000,
    english_name: "English (Malta)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Mauritius)
pub const LANG_EN_MU: &LanguageId = &LanguageId {
    name: "en-MU",
    lcid: 0x1000,
    english_name: "English (Mauritius)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Malawi)
pub const LANG_EN_MW: &LanguageId = &LanguageId {
    name: "en-MW",
    lcid: 0x1000,
    english_name: "English (Malawi)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Namibia)
pub const LANG_EN_NA: &LanguageId = &LanguageId {
    name: "en-NA",
    lcid: 0x1000,
    english_name: "English (Namibia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Norfolk Island)
pub const LANG_EN_NF: &LanguageId = &LanguageId {
    name: "en-NF",
    lcid: 0x1000,
    english_name: "English (Norfolk Island)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Nigeria)
pub const LANG_EN_NG: &LanguageId = &LanguageId {
    name: "en-NG",
    lcid: 0x1000,
    english_name: "English (Nigeria)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Netherlands)
pub const LANG_EN_NL: &LanguageId = &LanguageId {
    name: "en-NL",
    lcid: 0x1000,
    english_name: "English (Netherlands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Nauru)
pub const LANG_EN_NR: &LanguageId = &LanguageId {
    name: "en-NR",
    lcid: 0x1000,
    english_name: "English (Nauru)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Niue)
pub const LANG_EN_NU: &LanguageId = &LanguageId {
    name: "en-NU",
    lcid: 0x1000,
    english_name: "English (Niue)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Papua New Guinea)
pub const LANG_EN_PG: &LanguageId = &LanguageId {
    name: "en-PG",
    lcid: 0x1000,
    english_name: "English (Papua New Guinea)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Pakistan)
pub const LANG_EN_PK: &LanguageId = &LanguageId {
    name: "en-PK",
    lcid: 0x1000,
    english_name: "English (Pakistan)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Pitcairn Islands)
pub const LANG_EN_PN: &LanguageId = &LanguageId {
    name: "en-PN",
    lcid: 0x1000,
    english_name: "English (Pitcairn Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Puerto Rico)
pub const LANG_EN_PR: &LanguageId = &LanguageId {
    name: "en-PR",
    lcid: 0x1000,
    english_name: "English (Puerto Rico)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Palau)
pub const LANG_EN_PW: &LanguageId = &LanguageId {
    name: "en-PW",
    lcid: 0x1000,
    english_name: "English (Palau)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Rwanda)
pub const LANG_EN_RW: &LanguageId = &LanguageId {
    name: "en-RW",
    lcid: 0x1000,
    english_name: "English (Rwanda)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Solomon Islands)
pub const LANG_EN_SB: &LanguageId = &LanguageId {
    name: "en-SB",
    lcid: 0x1000,
    english_name: "English (Solomon Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Seychelles)
pub const LANG_EN_SC: &LanguageId = &LanguageId {
    name: "en-SC",
    lcid: 0x1000,
    english_name: "English (Seychelles)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Sudan)
pub const LANG_EN_SD: &LanguageId = &LanguageId {
    name: "en-SD",
    lcid: 0x1000,
    english_name: "English (Sudan)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Sweden)
pub const LANG_EN_SE: &LanguageId = &LanguageId {
    name: "en-SE",
    lcid: 0x1000,
    english_name: "English (Sweden)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (St Helena, Ascension, Tristan da Cunha)
pub const LANG_EN_SH: &LanguageId = &LanguageId {
    name: "en-SH",
    lcid: 0x1000,
    english_name: "English (St Helena, Ascension, Tristan da Cunha)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Slovenia)
pub const LANG_EN_SI: &LanguageId = &LanguageId {
    name: "en-SI",
    lcid: 0x1000,
    english_name: "English (Slovenia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// English (Sierra Leone)
pub const LANG_EN_SL: &LanguageId = &LanguageId {
    name: "en-SL",
    lcid: 0x1000,
    english_name: "English (Sierra Leone)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (South Sudan)
pub const LANG_EN_SS: &LanguageId = &LanguageId {
    name: "en-SS",
    lcid: 0x1000,
    english_name: "English (South Sudan)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Sint Maarten)
pub const LANG_EN_SX: &LanguageId = &LanguageId {
    name: "en-SX",
    lcid: 0x1000,
    english_name: "English (Sint Maarten)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Swaziland)
pub const LANG_EN_SZ: &LanguageId = &LanguageId {
    name: "en-SZ",
    lcid: 0x1000,
    english_name: "English (Swaziland)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Turks and Caicos Islands)
pub const LANG_EN_TC: &LanguageId = &LanguageId {
    name: "en-TC",
    lcid: 0x1000,
    english_name: "English (Turks and Caicos Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Tokelau)
pub const LANG_EN_TK: &LanguageId = &LanguageId {
    name: "en-TK",
    lcid: 0x1000,
    english_name: "English (Tokelau)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Tonga)
pub const LANG_EN_TO: &LanguageId = &LanguageId {
    name: "en-TO",
    lcid: 0x1000,
    english_name: "English (Tonga)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Tuvalu)
pub const LANG_EN_TV: &LanguageId = &LanguageId {
    name: "en-TV",
    lcid: 0x1000,
    english_name: "English (Tuvalu)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Tanzania)
pub const LANG_EN_TZ: &LanguageId = &LanguageId {
    name: "en-TZ",
    lcid: 0x1000,
    english_name: "English (Tanzania)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Uganda)
pub const LANG_EN_UG: &LanguageId = &LanguageId {
    name: "en-UG",
    lcid: 0x1000,
    english_name: "English (Uganda)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (U.S. Outlying Islands)
pub const LANG_EN_UM: &LanguageId = &LanguageId {
    name: "en-UM",
    lcid: 0x1000,
    english_name: "English (U.S. Outlying Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Saint Vincent and the Grenadines)
pub const LANG_EN_VC: &LanguageId = &LanguageId {
    name: "en-VC",
    lcid: 0x1000,
    english_name: "English (Saint Vincent and the Grenadines)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (British Virgin Islands)
pub const LANG_EN_VG: &LanguageId = &LanguageId {
    name: "en-VG",
    lcid: 0x1000,
    english_name: "English (British Virgin Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (U.S. Virgin Islands)
pub const LANG_EN_VI: &LanguageId = &LanguageId {
    name: "en-VI",
    lcid: 0x1000,
    english_name: "English (U.S. Virgin Islands)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Vanuatu)
pub const LANG_EN_VU: &LanguageId = &LanguageId {
    name: "en-VU",
    lcid: 0x1000,
    english_name: "English (Vanuatu)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Samoa)
pub const LANG_EN_WS: &LanguageId = &LanguageId {
    name: "en-WS",
    lcid: 0x1000,
    english_name: "English (Samoa)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Zambia)
pub const LANG_EN_ZM: &LanguageId = &LanguageId {
    name: "en-ZM",
    lcid: 0x1000,
    english_name: "English (Zambia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Esperanto
pub const LANG_EO: &LanguageId = &LanguageId {
    name: "eo",
    lcid: 0x1000,
    english_name: "Esperanto",
    iso639_two_letter: "eo",
    iso639_three_letter: "epo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Esperanto (World)
pub const LANG_EO_001: &LanguageId = &LanguageId {
    name: "eo-001",
    lcid: 0x1000,
    english_name: "Esperanto (World)",
    iso639_two_letter: "eo",
    iso639_three_letter: "epo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Spanish (Brazil)
pub const LANG_ES_BR: &LanguageId = &LanguageId {
    name: "es-BR",
    lcid: 0x1000,
    english_name: "Spanish (Brazil)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Spanish (Belize)
pub const LANG_ES_BZ: &LanguageId = &LanguageId {
    name: "es-BZ",
    lcid: 0x1000,
    english_name: "Spanish (Belize)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Spanish (Equatorial Guinea)
pub const LANG_ES_GQ: &LanguageId = &LanguageId {
    name: "es-GQ",
    lcid: 0x1000,
    english_name: "Spanish (Equatorial Guinea)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Philippines)
pub const LANG_ES_PH: &LanguageId = &LanguageId {
    name: "es-PH",
    lcid: 0x1000,
    english_name: "Spanish (Philippines)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Ewondo
pub const LANG_EWO: &LanguageId = &LanguageId {
    name: "ewo",
    lcid: 0x1000,
    english_name: "Ewondo",
    iso639_two_letter: "ewo",
    iso639_three_letter: "ewo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ewondo (Cameroon)
pub const LANG_EWO_CM: &LanguageId = &LanguageId {
    name: "ewo-CM",
    lcid: 0x1000,
    english_name: "Ewondo (Cameroon)",
    iso639_two_letter: "ewo",
    iso639_three_letter: "ewo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Dari (Afghanistan)
pub const LANG_FA_AF: &LanguageId = &LanguageId {
    name: "fa-AF",
    lcid: 0x1000,
    english_name: "Dari (Afghanistan)",
    iso639_two_letter: "prs",
    iso639_three_letter: "prs",
    windows_three_letter: "PRS",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Fulah (Cameroon)
pub const LANG_FF_CM: &LanguageId = &LanguageId {
    name: "ff-CM",
    lcid: 0x1000,
    english_name: "Fulah (Cameroon)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Fulah (Guinea)
pub const LANG_FF_GN: &LanguageId = &LanguageId {
    name: "ff-GN",
    lcid: 0x1000,
    english_name: "Fulah (Guinea)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-BF)
pub const LANG_FF_LATN_BF: &LanguageId = &LanguageId {
    name: "ff-Latn-BF",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-BF)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-CM)
pub const LANG_FF_LATN_CM: &LanguageId = &LanguageId {
    name: "ff-Latn-CM",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-CM)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-GH)
pub const LANG_FF_LATN_GH: &LanguageId = &LanguageId {
    name: "ff-Latn-GH",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-GH)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-GM)
pub const LANG_FF_LATN_GM: &LanguageId = &LanguageId {
    name: "ff-Latn-GM",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-GM)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-GN)
pub const LANG_FF_LATN_GN: &LanguageId = &LanguageId {
    name: "ff-Latn-GN",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-GN)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-GW)
pub const LANG_FF_LATN_GW: &LanguageId = &LanguageId {
    name: "ff-Latn-GW",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-GW)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-LR)
pub const LANG_FF_LATN_LR: &LanguageId = &LanguageId {
    name: "ff-Latn-LR",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-LR)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-MR)
pub const LANG_FF_LATN_MR: &LanguageId = &LanguageId {
    name: "ff-Latn-MR",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-MR)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-NE)
pub const LANG_FF_LATN_NE: &LanguageId = &LanguageId {
    name: "ff-Latn-NE",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-NE)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-NG)
pub const LANG_FF_LATN_NG: &LanguageId = &LanguageId {
    name: "ff-Latn-NG",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-NG)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ff-Latn-SL)
pub const LANG_FF_LATN_SL: &LanguageId = &LanguageId {
    name: "ff-Latn-SL",
    lcid: 0x1000,
    english_name: "Unknown Locale (ff-Latn-SL)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Fulah (Mauritania)
pub const LANG_FF_MR: &LanguageId = &LanguageId {
    name: "ff-MR",
    lcid: 0x1000,
    english_name: "Fulah (Mauritania)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Fulah (Nigeria)
pub const LANG_FF_NG: &LanguageId = &LanguageId {
    name: "ff-NG",
    lcid: 0x1000,
    english_name: "Fulah (Nigeria)",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Faroese (Denmark)
pub const LANG_FO_DK: &LanguageId = &LanguageId {
    name: "fo-DK",
    lcid: 0x1000,
    english_name: "Faroese (Denmark)",
    iso639_two_letter: "fo",
    iso639_three_letter: "fao",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// French (Burkina Faso)
pub const LANG_FR_BF: &LanguageId = &LanguageId {
    name: "fr-BF",
    lcid: 0x1000,
    english_name: "French (Burkina Faso)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Burundi)
pub const LANG_FR_BI: &LanguageId = &LanguageId {
    name: "fr-BI",
    lcid: 0x1000,
    english_name: "French (Burundi)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Benin)
pub const LANG_FR_BJ: &LanguageId = &LanguageId {
    name: "fr-BJ",
    lcid: 0x1000,
    english_name: "French (Benin)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Saint Barthélemy)
pub const LANG_FR_BL: &LanguageId = &LanguageId {
    name: "fr-BL",
    lcid: 0x1000,
    english_name: "French (Saint Barthélemy)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Central African Republic)
pub const LANG_FR_CF: &LanguageId = &LanguageId {
    name: "fr-CF",
    lcid: 0x1000,
    english_name: "French (Central African Republic)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Congo)
pub const LANG_FR_CG: &LanguageId = &LanguageId {
    name: "fr-CG",
    lcid: 0x1000,
    english_name: "French (Congo)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Djibouti)
pub const LANG_FR_DJ: &LanguageId = &LanguageId {
    name: "fr-DJ",
    lcid: 0x1000,
    english_name: "French (Djibouti)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Algeria)
pub const LANG_FR_DZ: &LanguageId = &LanguageId {
    name: "fr-DZ",
    lcid: 0x1000,
    english_name: "French (Algeria)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Gabon)
pub const LANG_FR_GA: &LanguageId = &LanguageId {
    name: "fr-GA",
    lcid: 0x1000,
    english_name: "French (Gabon)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (French Guiana)
pub const LANG_FR_GF: &LanguageId = &LanguageId {
    name: "fr-GF",
    lcid: 0x1000,
    english_name: "French (French Guiana)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Guinea)
pub const LANG_FR_GN: &LanguageId = &LanguageId {
    name: "fr-GN",
    lcid: 0x1000,
    english_name: "French (Guinea)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Guadeloupe)
pub const LANG_FR_GP: &LanguageId = &LanguageId {
    name: "fr-GP",
    lcid: 0x1000,
    english_name: "French (Guadeloupe)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Equatorial Guinea)
pub const LANG_FR_GQ: &LanguageId = &LanguageId {
    name: "fr-GQ",
    lcid: 0x1000,
    english_name: "French (Equatorial Guinea)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Comoros)
pub const LANG_FR_KM: &LanguageId = &LanguageId {
    name: "fr-KM",
    lcid: 0x1000,
    english_name: "French (Comoros)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Saint Martin)
pub const LANG_FR_MF: &LanguageId = &LanguageId {
    name: "fr-MF",
    lcid: 0x1000,
    english_name: "French (Saint Martin)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Madagascar)
pub const LANG_FR_MG: &LanguageId = &LanguageId {
    name: "fr-MG",
    lcid: 0x1000,
    english_name: "French (Madagascar)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Martinique)
pub const LANG_FR_MQ: &LanguageId = &LanguageId {
    name: "fr-MQ",
    lcid: 0x1000,
    english_name: "French (Martinique)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Mauritania)
pub const LANG_FR_MR: &LanguageId = &LanguageId {
    name: "fr-MR",
    lcid: 0x1000,
    english_name: "French (Mauritania)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Mauritius)
pub const LANG_FR_MU: &LanguageId = &LanguageId {
    name: "fr-MU",
    lcid: 0x1000,
    english_name: "French (Mauritius)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (New Caledonia)
pub const LANG_FR_NC: &LanguageId = &LanguageId {
    name: "fr-NC",
    lcid: 0x1000,
    english_name: "French (New Caledonia)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Niger)
pub const LANG_FR_NE: &LanguageId = &LanguageId {
    name: "fr-NE",
    lcid: 0x1000,
    english_name: "French (Niger)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (French Polynesia)
pub const LANG_FR_PF: &LanguageId = &LanguageId {
    name: "fr-PF",
    lcid: 0x1000,
    english_name: "French (French Polynesia)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Saint Pierre and Miquelon)
pub const LANG_FR_PM: &LanguageId = &LanguageId {
    name: "fr-PM",
    lcid: 0x1000,
    english_name: "French (Saint Pierre and Miquelon)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Rwanda)
pub const LANG_FR_RW: &LanguageId = &LanguageId {
    name: "fr-RW",
    lcid: 0x1000,
    english_name: "French (Rwanda)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Seychelles)
pub const LANG_FR_SC: &LanguageId = &LanguageId {
    name: "fr-SC",
    lcid: 0x1000,
    english_name: "French (Seychelles)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Syria)
pub const LANG_FR_SY: &LanguageId = &LanguageId {
    name: "fr-SY",
    lcid: 0x1000,
    english_name: "French (Syria)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Chad)
pub const LANG_FR_TD: &LanguageId = &LanguageId {
    name: "fr-TD",
    lcid: 0x1000,
    english_name: "French (Chad)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Togo)
pub const LANG_FR_TG: &LanguageId = &LanguageId {
    name: "fr-TG",
    lcid: 0x1000,
    english_name: "French (Togo)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Tunisia)
pub const LANG_FR_TN: &LanguageId = &LanguageId {
    name: "fr-TN",
    lcid: 0x1000,
    english_name: "French (Tunisia)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Vanuatu)
pub const LANG_FR_VU: &LanguageId = &LanguageId {
    name: "fr-VU",
    lcid: 0x1000,
    english_name: "French (Vanuatu)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Wallis and Futuna)
pub const LANG_FR_WF: &LanguageId = &LanguageId {
    name: "fr-WF",
    lcid: 0x1000,
    english_name: "French (Wallis and Futuna)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Mayotte)
pub const LANG_FR_YT: &LanguageId = &LanguageId {
    name: "fr-YT",
    lcid: 0x1000,
    english_name: "French (Mayotte)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Friulian
pub const LANG_FUR: &LanguageId = &LanguageId {
    name: "fur",
    lcid: 0x1000,
    english_name: "Friulian",
    iso639_two_letter: "fur",
    iso639_three_letter: "fur",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Friulian (Italy)
pub const LANG_FUR_IT: &LanguageId = &LanguageId {
    name: "fur-IT",
    lcid: 0x1000,
    english_name: "Friulian (Italy)",
    iso639_two_letter: "fur",
    iso639_three_letter: "fur",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Swiss German (Switzerland)
pub const LANG_GSW_CH: &LanguageId = &LanguageId {
    name: "gsw-CH",
    lcid: 0x1000,
    english_name: "Swiss German (Switzerland)",
    iso639_two_letter: "gsw",
    iso639_three_letter: "gsw",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Swiss German (Liechtenstein)
pub const LANG_GSW_LI: &LanguageId = &LanguageId {
    name: "gsw-LI",
    lcid: 0x1000,
    english_name: "Swiss German (Liechtenstein)",
    iso639_two_letter: "gsw",
    iso639_three_letter: "gsw",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Gusii
pub const LANG_GUZ: &LanguageId = &LanguageId {
    name: "guz",
    lcid: 0x1000,
    english_name: "Gusii",
    iso639_two_letter: "guz",
    iso639_three_letter: "guz",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Gusii (Kenya)
pub const LANG_GUZ_KE: &LanguageId = &LanguageId {
    name: "guz-KE",
    lcid: 0x1000,
    english_name: "Gusii (Kenya)",
    iso639_two_letter: "guz",
    iso639_three_letter: "guz",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Manx
pub const LANG_GV: &LanguageId = &LanguageId {
    name: "gv",
    lcid: 0x1000,
    english_name: "Manx",
    iso639_two_letter: "gv",
    iso639_three_letter: "glv",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Manx (Isle of Man)
pub const LANG_GV_IM: &LanguageId = &LanguageId {
    name: "gv-IM",
    lcid: 0x1000,
    english_name: "Manx (Isle of Man)",
    iso639_two_letter: "gv",
    iso639_three_letter: "glv",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Hausa (Latin, Ghana)
pub const LANG_HA_LATN_GH: &LanguageId = &LanguageId {
    name: "ha-Latn-GH",
    lcid: 0x1000,
    english_name: "Hausa (Latin, Ghana)",
    iso639_two_letter: "ha",
    iso639_three_letter: "hau",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hausa (Latin, Niger)
pub const LANG_HA_LATN_NE: &LanguageId = &LanguageId {
    name: "ha-Latn-NE",
    lcid: 0x1000,
    english_name: "Hausa (Latin, Niger)",
    iso639_two_letter: "ha",
    iso639_three_letter: "hau",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Interlingua
pub const LANG_IA: &LanguageId = &LanguageId {
    name: "ia",
    lcid: 0x1000,
    english_name: "Interlingua",
    iso639_two_letter: "ia",
    iso639_three_letter: "ina",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Interlingua (World)
pub const LANG_IA_001: &LanguageId = &LanguageId {
    name: "ia-001",
    lcid: 0x1000,
    english_name: "Interlingua (World)",
    iso639_two_letter: "ia",
    iso639_three_letter: "ina",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Interlingua (France)
pub const LANG_IA_FR: &LanguageId = &LanguageId {
    name: "ia-FR",
    lcid: 0x1000,
    english_name: "Interlingua (France)",
    iso639_two_letter: "ia",
    iso639_three_letter: "ina",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Italian (San Marino)
pub const LANG_IT_SM: &LanguageId = &LanguageId {
    name: "it-SM",
    lcid: 0x1000,
    english_name: "Italian (San Marino)",
    iso639_two_letter: "it",
    iso639_three_letter: "ita",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Italian (Vatican City)
pub const LANG_IT_VA: &LanguageId = &LanguageId {
    name: "it-VA",
    lcid: 0x1000,
    english_name: "Italian (Vatican City)",
    iso639_two_letter: "it",
    iso639_three_letter: "ita",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ngomba
pub const LANG_JGO: &LanguageId = &LanguageId {
    name: "jgo",
    lcid: 0x1000,
    english_name: "Ngomba",
    iso639_two_letter: "jgo",
    iso639_three_letter: "jgo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ngomba (Cameroon)
pub const LANG_JGO_CM: &LanguageId = &LanguageId {
    name: "jgo-CM",
    lcid: 0x1000,
    english_name: "Ngomba (Cameroon)",
    iso639_two_letter: "jgo",
    iso639_three_letter: "jgo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Machame
pub const LANG_JMC: &LanguageId = &LanguageId {
    name: "jmc",
    lcid: 0x1000,
    english_name: "Machame",
    iso639_two_letter: "jmc",
    iso639_three_letter: "jmc",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Machame (Tanzania)
pub const LANG_JMC_TZ: &LanguageId = &LanguageId {
    name: "jmc-TZ",
    lcid: 0x1000,
    english_name: "Machame (Tanzania)",
    iso639_two_letter: "jmc",
    iso639_three_letter: "jmc",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Javanese
pub const LANG_JV: &LanguageId = &LanguageId {
    name: "jv",
    lcid: 0x1000,
    english_name: "Javanese",
    iso639_two_letter: "jv",
    iso639_three_letter: "jav",
    windows_three_letter: "JAV",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Javanese
pub const LANG_JV_LATN: &LanguageId = &LanguageId {
    name: "jv-Latn",
    lcid: 0x1000,
    english_name: "Javanese",
    iso639_two_letter: "jv",
    iso639_three_letter: "jav",
    windows_three_letter: "JAV",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Javanese (Indonesia)
pub const LANG_JV_LATN_ID: &LanguageId = &LanguageId {
    name: "jv-Latn-ID",
    lcid: 0x1000,
    english_name: "Javanese (Indonesia)",
    iso639_two_letter: "jv",
    iso639_three_letter: "jav",
    windows_three_letter: "JAV",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kabyle
pub const LANG_KAB: &LanguageId = &LanguageId {
    name: "kab",
    lcid: 0x1000,
    english_name: "Kabyle",
    iso639_two_letter: "kab",
    iso639_three_letter: "kab",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kabyle (Algeria)
pub const LANG_KAB_DZ: &LanguageId = &LanguageId {
    name: "kab-DZ",
    lcid: 0x1000,
    english_name: "Kabyle (Algeria)",
    iso639_two_letter: "kab",
    iso639_three_letter: "kab",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kamba
pub const LANG_KAM: &LanguageId = &LanguageId {
    name: "kam",
    lcid: 0x1000,
    english_name: "Kamba",
    iso639_two_letter: "kam",
    iso639_three_letter: "kam",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kamba (Kenya)
pub const LANG_KAM_KE: &LanguageId = &LanguageId {
    name: "kam-KE",
    lcid: 0x1000,
    english_name: "Kamba (Kenya)",
    iso639_two_letter: "kam",
    iso639_three_letter: "kam",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Makonde
pub const LANG_KDE: &LanguageId = &LanguageId {
    name: "kde",
    lcid: 0x1000,
    english_name: "Makonde",
    iso639_two_letter: "kde",
    iso639_three_letter: "kde",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Makonde (Tanzania)
pub const LANG_KDE_TZ: &LanguageId = &LanguageId {
    name: "kde-TZ",
    lcid: 0x1000,
    english_name: "Makonde (Tanzania)",
    iso639_two_letter: "kde",
    iso639_three_letter: "kde",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kabuverdianu
pub const LANG_KEA: &LanguageId = &LanguageId {
    name: "kea",
    lcid: 0x1000,
    english_name: "Kabuverdianu",
    iso639_two_letter: "kea",
    iso639_three_letter: "kea",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kabuverdianu (Cabo Verde)
pub const LANG_KEA_CV: &LanguageId = &LanguageId {
    name: "kea-CV",
    lcid: 0x1000,
    english_name: "Kabuverdianu (Cabo Verde)",
    iso639_two_letter: "kea",
    iso639_three_letter: "kea",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Koyra Chiini
pub const LANG_KHQ: &LanguageId = &LanguageId {
    name: "khq",
    lcid: 0x1000,
    english_name: "Koyra Chiini",
    iso639_two_letter: "khq",
    iso639_three_letter: "khq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Koyra Chiini (Mali)
pub const LANG_KHQ_ML: &LanguageId = &LanguageId {
    name: "khq-ML",
    lcid: 0x1000,
    english_name: "Koyra Chiini (Mali)",
    iso639_two_letter: "khq",
    iso639_three_letter: "khq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kikuyu
pub const LANG_KI: &LanguageId = &LanguageId {
    name: "ki",
    lcid: 0x1000,
    english_name: "Kikuyu",
    iso639_two_letter: "ki",
    iso639_three_letter: "kik",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kikuyu (Kenya)
pub const LANG_KI_KE: &LanguageId = &LanguageId {
    name: "ki-KE",
    lcid: 0x1000,
    english_name: "Kikuyu (Kenya)",
    iso639_two_letter: "ki",
    iso639_three_letter: "kik",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kako
pub const LANG_KKJ: &LanguageId = &LanguageId {
    name: "kkj",
    lcid: 0x1000,
    english_name: "Kako",
    iso639_two_letter: "kkj",
    iso639_three_letter: "kkj",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kako (Cameroon)
pub const LANG_KKJ_CM: &LanguageId = &LanguageId {
    name: "kkj-CM",
    lcid: 0x1000,
    english_name: "Kako (Cameroon)",
    iso639_two_letter: "kkj",
    iso639_three_letter: "kkj",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kalenjin
pub const LANG_KLN: &LanguageId = &LanguageId {
    name: "kln",
    lcid: 0x1000,
    english_name: "Kalenjin",
    iso639_two_letter: "kln",
    iso639_three_letter: "kln",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kalenjin (Kenya)
pub const LANG_KLN_KE: &LanguageId = &LanguageId {
    name: "kln-KE",
    lcid: 0x1000,
    english_name: "Kalenjin (Kenya)",
    iso639_two_letter: "kln",
    iso639_three_letter: "kln",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Korean (North Korea)
pub const LANG_KO_KP: &LanguageId = &LanguageId {
    name: "ko-KP",
    lcid: 0x1000,
    english_name: "Korean (North Korea)",
    iso639_two_letter: "ko",
    iso639_three_letter: "kor",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kashmiri (Perso-Arabic)
pub const LANG_KS_ARAB_IN: &LanguageId = &LanguageId {
    name: "ks-Arab-IN",
    lcid: 0x1000,
    english_name: "Kashmiri (Perso-Arabic)",
    iso639_two_letter: "ks",
    iso639_three_letter: "kas",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Shambala
pub const LANG_KSB: &LanguageId = &LanguageId {
    name: "ksb",
    lcid: 0x1000,
    english_name: "Shambala",
    iso639_two_letter: "ksb",
    iso639_three_letter: "ksb",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Shambala (Tanzania)
pub const LANG_KSB_TZ: &LanguageId = &LanguageId {
    name: "ksb-TZ",
    lcid: 0x1000,
    english_name: "Shambala (Tanzania)",
    iso639_two_letter: "ksb",
    iso639_three_letter: "ksb",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bafia
pub const LANG_KSF: &LanguageId = &LanguageId {
    name: "ksf",
    lcid: 0x1000,
    english_name: "Bafia",
    iso639_two_letter: "ksf",
    iso639_three_letter: "ksf",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Bafia (Cameroon)
pub const LANG_KSF_CM: &LanguageId = &LanguageId {
    name: "ksf-CM",
    lcid: 0x1000,
    english_name: "Bafia (Cameroon)",
    iso639_two_letter: "ksf",
    iso639_three_letter: "ksf",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Colognian
pub const LANG_KSH: &LanguageId = &LanguageId {
    name: "ksh",
    lcid: 0x1000,
    english_name: "Colognian",
    iso639_two_letter: "ksh",
    iso639_three_letter: "ksh",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Colognian (Germany)
pub const LANG_KSH_DE: &LanguageId = &LanguageId {
    name: "ksh-DE",
    lcid: 0x1000,
    english_name: "Colognian (Germany)",
    iso639_two_letter: "ksh",
    iso639_three_letter: "ksh",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kurdish (Perso-Arabic, Iran)
pub const LANG_KU_ARAB_IR: &LanguageId = &LanguageId {
    name: "ku-Arab-IR",
    lcid: 0x1000,
    english_name: "Kurdish (Perso-Arabic, Iran)",
    iso639_two_letter: "ku",
    iso639_three_letter: "kur",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Cornish
pub const LANG_KW: &LanguageId = &LanguageId {
    name: "kw",
    lcid: 0x1000,
    english_name: "Cornish",
    iso639_two_letter: "kw",
    iso639_three_letter: "cor",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Cornish (United Kingdom)
pub const LANG_KW_GB: &LanguageId = &LanguageId {
    name: "kw-GB",
    lcid: 0x1000,
    english_name: "Cornish (United Kingdom)",
    iso639_two_letter: "kw",
    iso639_three_letter: "cor",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Langi
pub const LANG_LAG: &LanguageId = &LanguageId {
    name: "lag",
    lcid: 0x1000,
    english_name: "Langi",
    iso639_two_letter: "lag",
    iso639_three_letter: "lag",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Langi (Tanzania)
pub const LANG_LAG_TZ: &LanguageId = &LanguageId {
    name: "lag-TZ",
    lcid: 0x1000,
    english_name: "Langi (Tanzania)",
    iso639_two_letter: "lag",
    iso639_three_letter: "lag",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ganda
pub const LANG_LG: &LanguageId = &LanguageId {
    name: "lg",
    lcid: 0x1000,
    english_name: "Ganda",
    iso639_two_letter: "lg",
    iso639_three_letter: "lug",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ganda (Uganda)
pub const LANG_LG_UG: &LanguageId = &LanguageId {
    name: "lg-UG",
    lcid: 0x1000,
    english_name: "Ganda (Uganda)",
    iso639_two_letter: "lg",
    iso639_three_letter: "lug",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Lakota
pub const LANG_LKT: &LanguageId = &LanguageId {
    name: "lkt",
    lcid: 0x1000,
    english_name: "Lakota",
    iso639_two_letter: "lkt",
    iso639_three_letter: "lkt",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Lakota (United States)
pub const LANG_LKT_US: &LanguageId = &LanguageId {
    name: "lkt-US",
    lcid: 0x1000,
    english_name: "Lakota (United States)",
    iso639_two_letter: "lkt",
    iso639_three_letter: "lkt",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Lingala
pub const LANG_LN: &LanguageId = &LanguageId {
    name: "ln",
    lcid: 0x1000,
    english_name: "Lingala",
    iso639_two_letter: "ln",
    iso639_three_letter: "lin",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Lingala (Angola)
pub const LANG_LN_AO: &LanguageId = &LanguageId {
    name: "ln-AO",
    lcid: 0x1000,
    english_name: "Lingala (Angola)",
    iso639_two_letter: "ln",
    iso639_three_letter: "lin",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Lingala (Congo DRC)
pub const LANG_LN_CD: &LanguageId = &LanguageId {
    name: "ln-CD",
    lcid: 0x1000,
    english_name: "Lingala (Congo DRC)",
    iso639_two_letter: "ln",
    iso639_three_letter: "lin",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Lingala (Central African Republic)
pub const LANG_LN_CF: &LanguageId = &LanguageId {
    name: "ln-CF",
    lcid: 0x1000,
    english_name: "Lingala (Central African Republic)",
    iso639_two_letter: "ln",
    iso639_three_letter: "lin",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Lingala (Congo)
pub const LANG_LN_CG: &LanguageId = &LanguageId {
    name: "ln-CG",
    lcid: 0x1000,
    english_name: "Lingala (Congo)",
    iso639_two_letter: "ln",
    iso639_three_letter: "lin",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Northern Luri (Iraq)
pub const LANG_LRC_IQ: &LanguageId = &LanguageId {
    name: "lrc-IQ",
    lcid: 0x1000,
    english_name: "Northern Luri (Iraq)",
    iso639_two_letter: "lrc",
    iso639_three_letter: "lrc",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Northern Luri (Iran)
pub const LANG_LRC_IR: &LanguageId = &LanguageId {
    name: "lrc-IR",
    lcid: 0x1000,
    english_name: "Northern Luri (Iran)",
    iso639_two_letter: "lrc",
    iso639_three_letter: "lrc",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Luba-Katanga
pub const LANG_LU: &LanguageId = &LanguageId {
    name: "lu",
    lcid: 0x1000,
    english_name: "Luba-Katanga",
    iso639_two_letter: "lu",
    iso639_three_letter: "lub",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Luba-Katanga (Congo DRC)
pub const LANG_LU_CD: &LanguageId = &LanguageId {
    name: "lu-CD",
    lcid: 0x1000,
    english_name: "Luba-Katanga (Congo DRC)",
    iso639_two_letter: "lu",
    iso639_three_letter: "lub",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Luo
pub const LANG_LUO: &LanguageId = &LanguageId {
    name: "luo",
    lcid: 0x1000,
    english_name: "Luo",
    iso639_two_letter: "luo",
    iso639_three_letter: "luo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Luo (Kenya)
pub const LANG_LUO_KE: &LanguageId = &LanguageId {
    name: "luo-KE",
    lcid: 0x1000,
    english_name: "Luo (Kenya)",
    iso639_two_letter: "luo",
    iso639_three_letter: "luo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Luyia
pub const LANG_LUY: &LanguageId = &LanguageId {
    name: "luy",
    lcid: 0x1000,
    english_name: "Luyia",
    iso639_two_letter: "luy",
    iso639_three_letter: "luy",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Luyia (Kenya)
pub const LANG_LUY_KE: &LanguageId = &LanguageId {
    name: "luy-KE",
    lcid: 0x1000,
    english_name: "Luyia (Kenya)",
    iso639_two_letter: "luy",
    iso639_three_letter: "luy",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Masai
pub const LANG_MAS: &LanguageId = &LanguageId {
    name: "mas",
    lcid: 0x1000,
    english_name: "Masai",
    iso639_two_letter: "mas",
    iso639_three_letter: "mas",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Masai (Kenya)
pub const LANG_MAS_KE: &LanguageId = &LanguageId {
    name: "mas-KE",
    lcid: 0x1000,
    english_name: "Masai (Kenya)",
    iso639_two_letter: "mas",
    iso639_three_letter: "mas",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Masai (Tanzania)
pub const LANG_MAS_TZ: &LanguageId = &LanguageId {
    name: "mas-TZ",
    lcid: 0x1000,
    english_name: "Masai (Tanzania)",
    iso639_two_letter: "mas",
    iso639_three_letter: "mas",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Meru
pub const LANG_MER: &LanguageId = &LanguageId {
    name: "mer",
    lcid: 0x1000,
    english_name: "Meru",
    iso639_two_letter: "mer",
    iso639_three_letter: "mer",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Meru (Kenya)
pub const LANG_MER_KE: &LanguageId = &LanguageId {
    name: "mer-KE",
    lcid: 0x1000,
    english_name: "Meru (Kenya)",
    iso639_two_letter: "mer",
    iso639_three_letter: "mer",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Morisyen
pub const LANG_MFE: &LanguageId = &LanguageId {
    name: "mfe",
    lcid: 0x1000,
    english_name: "Morisyen",
    iso639_two_letter: "mfe",
    iso639_three_letter: "mfe",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Morisyen (Mauritius)
pub const LANG_MFE_MU: &LanguageId = &LanguageId {
    name: "mfe-MU",
    lcid: 0x1000,
    english_name: "Morisyen (Mauritius)",
    iso639_two_letter: "mfe",
    iso639_three_letter: "mfe",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Malagasy
pub const LANG_MG: &LanguageId = &LanguageId {
    name: "mg",
    lcid: 0x1000,
    english_name: "Malagasy",
    iso639_two_letter: "mg",
    iso639_three_letter: "mlg",
    windows_three_letter: "MLG",
    ansi_code_page: None,
};

/// Malagasy (Madagascar)
pub const LANG_MG_MG: &LanguageId = &LanguageId {
    name: "mg-MG",
    lcid: 0x1000,
    english_name: "Malagasy (Madagascar)",
    iso639_two_letter: "mg",
    iso639_three_letter: "mlg",
    windows_three_letter: "MLG",
    ansi_code_page: None,
};

/// Makhuwa-Meetto
pub const LANG_MGH: &LanguageId = &LanguageId {
    name: "mgh",
    lcid: 0x1000,
    english_name: "Makhuwa-Meetto",
    iso639_two_letter: "mgh",
    iso639_three_letter: "mgh",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Makhuwa-Meetto (Mozambique)
pub const LANG_MGH_MZ: &LanguageId = &LanguageId {
    name: "mgh-MZ",
    lcid: 0x1000,
    english_name: "Makhuwa-Meetto (Mozambique)",
    iso639_two_letter: "mgh",
    iso639_three_letter: "mgh",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Metaʼ
pub const LANG_MGO: &LanguageId = &LanguageId {
    name: "mgo",
    lcid: 0x1000,
    english_name: "Metaʼ",
    iso639_two_letter: "mgo",
    iso639_three_letter: "mgo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Metaʼ (Cameroon)
pub const LANG_MGO_CM: &LanguageId = &LanguageId {
    name: "mgo-CM",
    lcid: 0x1000,
    english_name: "Metaʼ (Cameroon)",
    iso639_two_letter: "mgo",
    iso639_three_letter: "mgo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Mundang
pub const LANG_MUA: &LanguageId = &LanguageId {
    name: "mua",
    lcid: 0x1000,
    english_name: "Mundang",
    iso639_two_letter: "mua",
    iso639_three_letter: "mua",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Mundang (Cameroon)
pub const LANG_MUA_CM: &LanguageId = &LanguageId {
    name: "mua-CM",
    lcid: 0x1000,
    english_name: "Mundang (Cameroon)",
    iso639_two_letter: "mua",
    iso639_three_letter: "mua",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Mazanderani (Iran)
pub const LANG_MZN_IR: &LanguageId = &LanguageId {
    name: "mzn-IR",
    lcid: 0x1000,
    english_name: "Mazanderani (Iran)",
    iso639_two_letter: "mzn",
    iso639_three_letter: "mzn",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nama
pub const LANG_NAQ: &LanguageId = &LanguageId {
    name: "naq",
    lcid: 0x1000,
    english_name: "Nama",
    iso639_two_letter: "naq",
    iso639_three_letter: "naq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nama (Namibia)
pub const LANG_NAQ_NA: &LanguageId = &LanguageId {
    name: "naq-NA",
    lcid: 0x1000,
    english_name: "Nama (Namibia)",
    iso639_two_letter: "naq",
    iso639_three_letter: "naq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Norwegian Bokmål (Svalbard and Jan Mayen)
pub const LANG_NB_SJ: &LanguageId = &LanguageId {
    name: "nb-SJ",
    lcid: 0x1000,
    english_name: "Norwegian Bokmål (Svalbard and Jan Mayen)",
    iso639_two_letter: "nb",
    iso639_three_letter: "nob",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// North Ndebele
pub const LANG_ND: &LanguageId = &LanguageId {
    name: "nd",
    lcid: 0x1000,
    english_name: "North Ndebele",
    iso639_two_letter: "nd",
    iso639_three_letter: "nde",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// North Ndebele (Zimbabwe)
pub const LANG_ND_ZW: &LanguageId = &LanguageId {
    name: "nd-ZW",
    lcid: 0x1000,
    english_name: "North Ndebele (Zimbabwe)",
    iso639_two_letter: "nd",
    iso639_three_letter: "nde",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Low German
pub const LANG_NDS: &LanguageId = &LanguageId {
    name: "nds",
    lcid: 0x1000,
    english_name: "Low German",
    iso639_two_letter: "nds",
    iso639_three_letter: "nds",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Low German (Germany)
pub const LANG_NDS_DE: &LanguageId = &LanguageId {
    name: "nds-DE",
    lcid: 0x1000,
    english_name: "Low German (Germany)",
    iso639_two_letter: "nds",
    iso639_three_letter: "nds",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Low German (Netherlands)
pub const LANG_NDS_NL: &LanguageId = &LanguageId {
    name: "nds-NL",
    lcid: 0x1000,
    english_name: "Low German (Netherlands)",
    iso639_two_letter: "nds",
    iso639_three_letter: "nds",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Unknown Language (ngo)
pub const LANG_NGO: &LanguageId = &LanguageId {
    name: "ngo",
    lcid: 0x1000,
    english_name: "Unknown Language (ngo)",
    iso639_two_letter: "ngo",
    iso639_three_letter: "ngo",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Locale (ngo-GN)
pub const LANG_NGO_GN: &LanguageId = &LanguageId {
    name: "ngo-GN",
    lcid: 0x1000,
    english_name: "Unknown Locale (ngo-GN)",
    iso639_two_letter: "ngo",
    iso639_three_letter: "ngo",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Dutch (Aruba)
pub const LANG_NL_AW: &LanguageId = &LanguageId {
    name: "nl-AW",
    lcid: 0x1000,
    english_name: "Dutch (Aruba)",
    iso639_two_letter: "nl",
    iso639_three_letter: "nld",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Dutch (Bonaire, Sint Eustatius and Saba)
pub const LANG_NL_BQ: &LanguageId = &LanguageId {
    name: "nl-BQ",
    lcid: 0x1000,
    english_name: "Dutch (Bonaire, Sint Eustatius and Saba)",
    iso639_two_letter: "nl",
    iso639_three_letter: "nld",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Dutch (Curaçao)
pub const LANG_NL_CW: &LanguageId = &LanguageId {
    name: "nl-CW",
    lcid: 0x1000,
    english_name: "Dutch (Curaçao)",
    iso639_two_letter: "nl",
    iso639_three_letter: "nld",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Dutch (Suriname)
pub const LANG_NL_SR: &LanguageId = &LanguageId {
    name: "nl-SR",
    lcid: 0x1000,
    english_name: "Dutch (Suriname)",
    iso639_two_letter: "nl",
    iso639_three_letter: "nld",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Dutch (Sint Maarten)
pub const LANG_NL_SX: &LanguageId = &LanguageId {
    name: "nl-SX",
    lcid: 0x1000,
    english_name: "Dutch (Sint Maarten)",
    iso639_two_letter: "nl",
    iso639_three_letter: "nld",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kwasio
pub const LANG_NMG: &LanguageId = &LanguageId {
    name: "nmg",
    lcid: 0x1000,
    english_name: "Kwasio",
    iso639_two_letter: "nmg",
    iso639_three_letter: "nmg",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Kwasio (Cameroon)
pub const LANG_NMG_CM: &LanguageId = &LanguageId {
    name: "nmg-CM",
    lcid: 0x1000,
    english_name: "Kwasio (Cameroon)",
    iso639_two_letter: "nmg",
    iso639_three_letter: "nmg",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ngiemboon
pub const LANG_NNH: &LanguageId = &LanguageId {
    name: "nnh",
    lcid: 0x1000,
    english_name: "Ngiemboon",
    iso639_two_letter: "nnh",
    iso639_three_letter: "nnh",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ngiemboon (Cameroon)
pub const LANG_NNH_CM: &LanguageId = &LanguageId {
    name: "nnh-CM",
    lcid: 0x1000,
    english_name: "Ngiemboon (Cameroon)",
    iso639_two_letter: "nnh",
    iso639_three_letter: "nnh",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// South Ndebele
pub const LANG_NR: &LanguageId = &LanguageId {
    name: "nr",
    lcid: 0x1000,
    english_name: "South Ndebele",
    iso639_two_letter: "nr",
    iso639_three_letter: "nbl",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// South Ndebele (South Africa)
pub const LANG_NR_ZA: &LanguageId = &LanguageId {
    name: "nr-ZA",
    lcid: 0x1000,
    english_name: "South Ndebele (South Africa)",
    iso639_two_letter: "nr",
    iso639_three_letter: "nbl",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nuer
pub const LANG_NUS: &LanguageId = &LanguageId {
    name: "nus",
    lcid: 0x1000,
    english_name: "Nuer",
    iso639_two_letter: "nus",
    iso639_three_letter: "nus",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nuer (South Sudan)
pub const LANG_NUS_SD: &LanguageId = &LanguageId {
    name: "nus-SD",
    lcid: 0x1000,
    english_name: "Nuer (South Sudan)",
    iso639_two_letter: "nus",
    iso639_three_letter: "nus",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nuer (South Sudan)
pub const LANG_NUS_SS: &LanguageId = &LanguageId {
    name: "nus-SS",
    lcid: 0x1000,
    english_name: "Nuer (South Sudan)",
    iso639_two_letter: "nus",
    iso639_three_letter: "nus",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nyankole
pub const LANG_NYN: &LanguageId = &LanguageId {
    name: "nyn",
    lcid: 0x1000,
    english_name: "Nyankole",
    iso639_two_letter: "nyn",
    iso639_three_letter: "nyn",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Nyankole (Uganda)
pub const LANG_NYN_UG: &LanguageId = &LanguageId {
    name: "nyn-UG",
    lcid: 0x1000,
    english_name: "Nyankole (Uganda)",
    iso639_two_letter: "nyn",
    iso639_three_letter: "nyn",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Oromo (Kenya)
pub const LANG_OM_KE: &LanguageId = &LanguageId {
    name: "om-KE",
    lcid: 0x1000,
    english_name: "Oromo (Kenya)",
    iso639_two_letter: "om",
    iso639_three_letter: "orm",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ossetic
pub const LANG_OS: &LanguageId = &LanguageId {
    name: "os",
    lcid: 0x1000,
    english_name: "Ossetic",
    iso639_two_letter: "os",
    iso639_three_letter: "oss",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ossetic (Georgia)
pub const LANG_OS_GE: &LanguageId = &LanguageId {
    name: "os-GE",
    lcid: 0x1000,
    english_name: "Ossetic (Georgia)",
    iso639_two_letter: "os",
    iso639_three_letter: "oss",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Ossetic (Russia)
pub const LANG_OS_RU: &LanguageId = &LanguageId {
    name: "os-RU",
    lcid: 0x1000,
    english_name: "Ossetic (Russia)",
    iso639_two_letter: "os",
    iso639_three_letter: "oss",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Prussian (World)
pub const LANG_PRG_001: &LanguageId = &LanguageId {
    name: "prg-001",
    lcid: 0x1000,
    english_name: "Prussian (World)",
    iso639_two_letter: "prg",
    iso639_three_letter: "prg",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Unknown Locale (ps-PK)
pub const LANG_PS_PK: &LanguageId = &LanguageId {
    name: "ps-PK",
    lcid: 0x1000,
    english_name: "Unknown Locale (ps-PK)",
    iso639_two_letter: "ps",
    iso639_three_letter: "pus",
    windows_three_letter: "PAS",
    ansi_code_page: None,
};

/// Portuguese (Angola)
pub const LANG_PT_AO: &LanguageId = &LanguageId {
    name: "pt-AO",
    lcid: 0x1000,
    english_name: "Portuguese (Angola)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "PTA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Portuguese (Switzerland)
pub const LANG_PT_CH: &LanguageId = &LanguageId {
    name: "pt-CH",
    lcid: 0x1000,
    english_name: "Portuguese (Switzerland)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Portuguese (Cabo Verde)
pub const LANG_PT_CV: &LanguageId = &LanguageId {
    name: "pt-CV",
    lcid: 0x1000,
    english_name: "Portuguese (Cabo Verde)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Portuguese (Equatorial Guinea)
pub const LANG_PT_GQ: &LanguageId = &LanguageId {
    name: "pt-GQ",
    lcid: 0x1000,
    english_name: "Portuguese (Equatorial Guinea)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Portuguese (Guinea-Bissau)
pub const LANG_PT_GW: &LanguageId = &LanguageId {
    name: "pt-GW",
    lcid: 0x1000,
    english_name: "Portuguese (Guinea-Bissau)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Portuguese (Luxembourg)
pub const LANG_PT_LU: &LanguageId = &LanguageId {
    name: "pt-LU",
    lcid: 0x1000,
    english_name: "Portuguese (Luxembourg)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Portuguese (Macao SAR)
pub const LANG_PT_MO: &LanguageId = &LanguageId {
    name: "pt-MO",
    lcid: 0x1000,
    english_name: "Portuguese (Macao SAR)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Portuguese (Mozambique)
pub const LANG_PT_MZ: &LanguageId = &LanguageId {
    name: "pt-MZ",
    lcid: 0x1000,
    english_name: "Portuguese (Mozambique)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Portuguese (São Tomé and Príncipe)
pub const LANG_PT_ST: &LanguageId = &LanguageId {
    name: "pt-ST",
    lcid: 0x1000,
    english_name: "Portuguese (São Tomé and Príncipe)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Portuguese (Timor-Leste)
pub const LANG_PT_TL: &LanguageId = &LanguageId {
    name: "pt-TL",
    lcid: 0x1000,
    english_name: "Portuguese (Timor-Leste)",
    iso639_two_letter: "pt",
    iso639_three_letter: "por",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Rundi
pub const LANG_RN: &LanguageId = &LanguageId {
    name: "rn",
    lcid: 0x1000,
    english_name: "Rundi",
    iso639_two_letter: "rn",
    iso639_three_letter: "run",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Rundi (Burundi)
pub const LANG_RN_BI: &LanguageId = &LanguageId {
    name: "rn-BI",
    lcid: 0x1000,
    english_name: "Rundi (Burundi)",
    iso639_two_letter: "rn",
    iso639_three_letter: "run",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Rombo
pub const LANG_ROF: &LanguageId = &LanguageId {
    name: "rof",
    lcid: 0x1000,
    english_name: "Rombo",
    iso639_two_letter: "rof",
    iso639_three_letter: "rof",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Rombo (Tanzania)
pub const LANG_ROF_TZ: &LanguageId = &LanguageId {
    name: "rof-TZ",
    lcid: 0x1000,
    english_name: "Rombo (Tanzania)",
    iso639_two_letter: "rof",
    iso639_three_letter: "rof",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Russian (Belarus)
pub const LANG_RU_BY: &LanguageId = &LanguageId {
    name: "ru-BY",
    lcid: 0x1000,
    english_name: "Russian (Belarus)",
    iso639_two_letter: "ru",
    iso639_three_letter: "rus",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Russian (Kyrgyzstan)
pub const LANG_RU_KG: &LanguageId = &LanguageId {
    name: "ru-KG",
    lcid: 0x1000,
    english_name: "Russian (Kyrgyzstan)",
    iso639_two_letter: "ru",
    iso639_three_letter: "rus",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Russian (Kazakhstan)
pub const LANG_RU_KZ: &LanguageId = &LanguageId {
    name: "ru-KZ",
    lcid: 0x1000,
    english_name: "Russian (Kazakhstan)",
    iso639_two_letter: "ru",
    iso639_three_letter: "rus",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Russian (Ukraine)
pub const LANG_RU_UA: &LanguageId = &LanguageId {
    name: "ru-UA",
    lcid: 0x1000,
    english_name: "Russian (Ukraine)",
    iso639_two_letter: "ru",
    iso639_three_letter: "rus",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Rwa
pub const LANG_RWK: &LanguageId = &LanguageId {
    name: "rwk",
    lcid: 0x1000,
    english_name: "Rwa",
    iso639_two_letter: "rwk",
    iso639_three_letter: "rwk",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Rwa (Tanzania)
pub const LANG_RWK_TZ: &LanguageId = &LanguageId {
    name: "rwk-TZ",
    lcid: 0x1000,
    english_name: "Rwa (Tanzania)",
    iso639_two_letter: "rwk",
    iso639_three_letter: "rwk",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Samburu
pub const LANG_SAQ: &LanguageId = &LanguageId {
    name: "saq",
    lcid: 0x1000,
    english_name: "Samburu",
    iso639_two_letter: "saq",
    iso639_three_letter: "saq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Samburu (Kenya)
pub const LANG_SAQ_KE: &LanguageId = &LanguageId {
    name: "saq-KE",
    lcid: 0x1000,
    english_name: "Samburu (Kenya)",
    iso639_two_letter: "saq",
    iso639_three_letter: "saq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Sangu
pub const LANG_SBP: &LanguageId = &LanguageId {
    name: "sbp",
    lcid: 0x1000,
    english_name: "Sangu",
    iso639_two_letter: "sbp",
    iso639_three_letter: "sbp",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Sangu (Tanzania)
pub const LANG_SBP_TZ: &LanguageId = &LanguageId {
    name: "sbp-TZ",
    lcid: 0x1000,
    english_name: "Sangu (Tanzania)",
    iso639_two_letter: "sbp",
    iso639_three_letter: "sbp",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Sena
pub const LANG_SEH: &LanguageId = &LanguageId {
    name: "seh",
    lcid: 0x1000,
    english_name: "Sena",
    iso639_two_letter: "seh",
    iso639_three_letter: "seh",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Sena (Mozambique)
pub const LANG_SEH_MZ: &LanguageId = &LanguageId {
    name: "seh-MZ",
    lcid: 0x1000,
    english_name: "Sena (Mozambique)",
    iso639_two_letter: "seh",
    iso639_three_letter: "seh",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Koyraboro Senni
pub const LANG_SES: &LanguageId = &LanguageId {
    name: "ses",
    lcid: 0x1000,
    english_name: "Koyraboro Senni",
    iso639_two_letter: "ses",
    iso639_three_letter: "ses",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Koyraboro Senni (Mali)
pub const LANG_SES_ML: &LanguageId = &LanguageId {
    name: "ses-ML",
    lcid: 0x1000,
    english_name: "Koyraboro Senni (Mali)",
    iso639_two_letter: "ses",
    iso639_three_letter: "ses",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Sango
pub const LANG_SG: &LanguageId = &LanguageId {
    name: "sg",
    lcid: 0x1000,
    english_name: "Sango",
    iso639_two_letter: "sg",
    iso639_three_letter: "sag",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Sango (Central African Republic)
pub const LANG_SG_CF: &LanguageId = &LanguageId {
    name: "sg-CF",
    lcid: 0x1000,
    english_name: "Sango (Central African Republic)",
    iso639_two_letter: "sg",
    iso639_three_letter: "sag",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tachelhit
pub const LANG_SHI: &LanguageId = &LanguageId {
    name: "shi",
    lcid: 0x1000,
    english_name: "Tachelhit",
    iso639_two_letter: "shi",
    iso639_three_letter: "shi",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tachelhit (Latin)
pub const LANG_SHI_LATN: &LanguageId = &LanguageId {
    name: "shi-Latn",
    lcid: 0x1000,
    english_name: "Tachelhit (Latin)",
    iso639_two_letter: "shi",
    iso639_three_letter: "shi",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tachelhit (Latin, Morocco)
pub const LANG_SHI_LATN_MA: &LanguageId = &LanguageId {
    name: "shi-Latn-MA",
    lcid: 0x1000,
    english_name: "Tachelhit (Latin, Morocco)",
    iso639_two_letter: "shi",
    iso639_three_letter: "shi",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tachelhit (Tifinagh)
pub const LANG_SHI_TFNG: &LanguageId = &LanguageId {
    name: "shi-Tfng",
    lcid: 0x1000,
    english_name: "Tachelhit (Tifinagh)",
    iso639_two_letter: "shi",
    iso639_three_letter: "shi",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tachelhit (Tifinagh, Morocco)
pub const LANG_SHI_TFNG_MA: &LanguageId = &LanguageId {
    name: "shi-Tfng-MA",
    lcid: 0x1000,
    english_name: "Tachelhit (Tifinagh, Morocco)",
    iso639_two_letter: "shi",
    iso639_three_letter: "shi",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Shona
pub const LANG_SN: &LanguageId = &LanguageId {
    name: "sn",
    lcid: 0x1000,
    english_name: "Shona",
    iso639_two_letter: "sn",
    iso639_three_letter: "sna",
    windows_three_letter: "SNA",
    ansi_code_page: None,
};

/// Shona (Latin)
pub const LANG_SN_LATN: &LanguageId = &LanguageId {
    name: "sn-Latn",
    lcid: 0x1000,
    english_name: "Shona (Latin)",
    iso639_two_letter: "sn",
    iso639_three_letter: "sna",
    windows_three_letter: "SNA",
    ansi_code_page: None,
};

/// Shona (Latin, Zimbabwe)
pub const LANG_SN_LATN_ZW: &LanguageId = &LanguageId {
    name: "sn-Latn-ZW",
    lcid: 0x1000,
    english_name: "Shona (Latin, Zimbabwe)",
    iso639_two_letter: "sn",
    iso639_three_letter: "sna",
    windows_three_letter: "SNA",
    ansi_code_page: None,
};

/// Somali (Djibouti)
pub const LANG_SO_DJ: &LanguageId = &LanguageId {
    name: "so-DJ",
    lcid: 0x1000,
    english_name: "Somali (Djibouti)",
    iso639_two_letter: "so",
    iso639_three_letter: "som",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Somali (Ethiopia)
pub const LANG_SO_ET: &LanguageId = &LanguageId {
    name: "so-ET",
    lcid: 0x1000,
    english_name: "Somali (Ethiopia)",
    iso639_two_letter: "so",
    iso639_three_letter: "som",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Somali (Kenya)
pub const LANG_SO_KE: &LanguageId = &LanguageId {
    name: "so-KE",
    lcid: 0x1000,
    english_name: "Somali (Kenya)",
    iso639_two_letter: "so",
    iso639_three_letter: "som",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Albanian (Macedonia, FYRO)
pub const LANG_SQ_MK: &LanguageId = &LanguageId {
    name: "sq-MK",
    lcid: 0x1000,
    english_name: "Albanian (Macedonia, FYRO)",
    iso639_two_letter: "sq",
    iso639_three_letter: "sqi",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// siSwati
pub const LANG_SS: &LanguageId = &LanguageId {
    name: "ss",
    lcid: 0x1000,
    english_name: "siSwati",
    iso639_two_letter: "ss",
    iso639_three_letter: "ssw",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// siSwati (Swaziland)
pub const LANG_SS_SZ: &LanguageId = &LanguageId {
    name: "ss-SZ",
    lcid: 0x1000,
    english_name: "siSwati (Swaziland)",
    iso639_two_letter: "ss",
    iso639_three_letter: "ssw",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// siSwati (South Africa)
pub const LANG_SS_ZA: &LanguageId = &LanguageId {
    name: "ss-ZA",
    lcid: 0x1000,
    english_name: "siSwati (South Africa)",
    iso639_two_letter: "ss",
    iso639_three_letter: "ssw",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Saho
pub const LANG_SSY: &LanguageId = &LanguageId {
    name: "ssy",
    lcid: 0x1000,
    english_name: "Saho",
    iso639_two_letter: "ssy",
    iso639_three_letter: "ssy",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Saho (Eritrea)
pub const LANG_SSY_ER: &LanguageId = &LanguageId {
    name: "ssy-ER",
    lcid: 0x1000,
    english_name: "Saho (Eritrea)",
    iso639_two_letter: "ssy",
    iso639_three_letter: "ssy",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Sesotho (Lesotho)
pub const LANG_ST_LS: &LanguageId = &LanguageId {
    name: "st-LS",
    lcid: 0x1000,
    english_name: "Sesotho (Lesotho)",
    iso639_two_letter: "st",
    iso639_three_letter: "sot",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Swedish (Åland Islands)
pub const LANG_SV_AX: &LanguageId = &LanguageId {
    name: "sv-AX",
    lcid: 0x1000,
    english_name: "Swedish (Åland Islands)",
    iso639_two_letter: "sv",
    iso639_three_letter: "swe",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kiswahili (Tanzania)
pub const LANG_SW_TZ: &LanguageId = &LanguageId {
    name: "sw-TZ",
    lcid: 0x1000,
    english_name: "Kiswahili (Tanzania)",
    iso639_two_letter: "sw",
    iso639_three_letter: "swa",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kiswahili (Uganda)
pub const LANG_SW_UG: &LanguageId = &LanguageId {
    name: "sw-UG",
    lcid: 0x1000,
    english_name: "Kiswahili (Uganda)",
    iso639_two_letter: "sw",
    iso639_three_letter: "swa",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Unknown Language (swc)
pub const LANG_SWC: &LanguageId = &LanguageId {
    name: "swc",
    lcid: 0x1000,
    english_name: "Unknown Language (swc)",
    iso639_two_letter: "swc",
    iso639_three_letter: "swc",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Kiswahili (Congo DRC)
pub const LANG_SWC_CD: &LanguageId = &LanguageId {
    name: "swc-CD",
    lcid: 0x1000,
    english_name: "Kiswahili (Congo DRC)",
    iso639_two_letter: "sw",
    iso639_three_letter: "swa",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Tamil (Malaysia)
pub const LANG_TA_MY: &LanguageId = &LanguageId {
    name: "ta-MY",
    lcid: 0x1000,
    english_name: "Tamil (Malaysia)",
    iso639_two_letter: "ta",
    iso639_three_letter: "tam",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tamil (Singapore)
pub const LANG_TA_SG: &LanguageId = &LanguageId {
    name: "ta-SG",
    lcid: 0x1000,
    english_name: "Tamil (Singapore)",
    iso639_two_letter: "ta",
    iso639_three_letter: "tam",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Teso
pub const LANG_TEO: &LanguageId = &LanguageId {
    name: "teo",
    lcid: 0x1000,
    english_name: "Teso",
    iso639_two_letter: "teo",
    iso639_three_letter: "teo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Teso (Kenya)
pub const LANG_TEO_KE: &LanguageId = &LanguageId {
    name: "teo-KE",
    lcid: 0x1000,
    english_name: "Teso (Kenya)",
    iso639_two_letter: "teo",
    iso639_three_letter: "teo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Teso (Uganda)
pub const LANG_TEO_UG: &LanguageId = &LanguageId {
    name: "teo-UG",
    lcid: 0x1000,
    english_name: "Teso (Uganda)",
    iso639_two_letter: "teo",
    iso639_three_letter: "teo",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tigre
pub const LANG_TIG: &LanguageId = &LanguageId {
    name: "tig",
    lcid: 0x1000,
    english_name: "Tigre",
    iso639_two_letter: "tig",
    iso639_three_letter: "tig",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tigre (Eritrea)
pub const LANG_TIG_ER: &LanguageId = &LanguageId {
    name: "tig-ER",
    lcid: 0x1000,
    english_name: "Tigre (Eritrea)",
    iso639_two_letter: "tig",
    iso639_three_letter: "tig",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tongan
pub const LANG_TO: &LanguageId = &LanguageId {
    name: "to",
    lcid: 0x1000,
    english_name: "Tongan",
    iso639_two_letter: "to",
    iso639_three_letter: "ton",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tongan (Tonga)
pub const LANG_TO_TO: &LanguageId = &LanguageId {
    name: "to-TO",
    lcid: 0x1000,
    english_name: "Tongan (Tonga)",
    iso639_two_letter: "to",
    iso639_three_letter: "ton",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Turkish (Cyprus)
pub const LANG_TR_CY: &LanguageId = &LanguageId {
    name: "tr-CY",
    lcid: 0x1000,
    english_name: "Turkish (Cyprus)",
    iso639_two_letter: "tr",
    iso639_three_letter: "tur",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Tasawaq
pub const LANG_TWQ: &LanguageId = &LanguageId {
    name: "twq",
    lcid: 0x1000,
    english_name: "Tasawaq",
    iso639_two_letter: "twq",
    iso639_three_letter: "twq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Tasawaq (Niger)
pub const LANG_TWQ_NE: &LanguageId = &LanguageId {
    name: "twq-NE",
    lcid: 0x1000,
    english_name: "Tasawaq (Niger)",
    iso639_two_letter: "twq",
    iso639_three_letter: "twq",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Central Atlas Tamazight (Latin, Morocco)
pub const LANG_TZM_LATN_MA: &LanguageId = &LanguageId {
    name: "tzm-Latn-MA",
    lcid: 0x1000,
    english_name: "Central Atlas Tamazight (Latin, Morocco)",
    iso639_two_letter: "tzm",
    iso639_three_letter: "tzm",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Uzbek (Perso-Arabic)
pub const LANG_UZ_ARAB: &LanguageId = &LanguageId {
    name: "uz-Arab",
    lcid: 0x1000,
    english_name: "Uzbek (Perso-Arabic)",
    iso639_two_letter: "uz",
    iso639_three_letter: "uzb",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Uzbek (Perso-Arabic, Afghanistan)
pub const LANG_UZ_ARAB_AF: &LanguageId = &LanguageId {
    name: "uz-Arab-AF",
    lcid: 0x1000,
    english_name: "Uzbek (Perso-Arabic, Afghanistan)",
    iso639_two_letter: "uz",
    iso639_three_letter: "uzb",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Vai
pub const LANG_VAI: &LanguageId = &LanguageId {
    name: "vai",
    lcid: 0x1000,
    english_name: "Vai",
    iso639_two_letter: "vai",
    iso639_three_letter: "vai",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Vai (Latin)
pub const LANG_VAI_LATN: &LanguageId = &LanguageId {
    name: "vai-Latn",
    lcid: 0x1000,
    english_name: "Vai (Latin)",
    iso639_two_letter: "vai",
    iso639_three_letter: "vai",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Vai (Latin, Liberia)
pub const LANG_VAI_LATN_LR: &LanguageId = &LanguageId {
    name: "vai-Latn-LR",
    lcid: 0x1000,
    english_name: "Vai (Latin, Liberia)",
    iso639_two_letter: "vai",
    iso639_three_letter: "vai",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Vai (Vai)
pub const LANG_VAI_VAII: &LanguageId = &LanguageId {
    name: "vai-Vaii",
    lcid: 0x1000,
    english_name: "Vai (Vai)",
    iso639_two_letter: "vai",
    iso639_three_letter: "vai",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Vai (Vai, Liberia)
pub const LANG_VAI_VAII_LR: &LanguageId = &LanguageId {
    name: "vai-Vaii-LR",
    lcid: 0x1000,
    english_name: "Vai (Vai, Liberia)",
    iso639_two_letter: "vai",
    iso639_three_letter: "vai",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Volapük
pub const LANG_VO: &LanguageId = &LanguageId {
    name: "vo",
    lcid: 0x1000,
    english_name: "Volapük",
    iso639_two_letter: "vo",
    iso639_three_letter: "vol",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Volapük (World)
pub const LANG_VO_001: &LanguageId = &LanguageId {
    name: "vo-001",
    lcid: 0x1000,
    english_name: "Volapük (World)",
    iso639_two_letter: "vo",
    iso639_three_letter: "vol",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Vunjo
pub const LANG_VUN: &LanguageId = &LanguageId {
    name: "vun",
    lcid: 0x1000,
    english_name: "Vunjo",
    iso639_two_letter: "vun",
    iso639_three_letter: "vun",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Vunjo (Tanzania)
pub const LANG_VUN_TZ: &LanguageId = &LanguageId {
    name: "vun-TZ",
    lcid: 0x1000,
    english_name: "Vunjo (Tanzania)",
    iso639_two_letter: "vun",
    iso639_three_letter: "vun",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Walser
pub const LANG_WAE: &LanguageId = &LanguageId {
    name: "wae",
    lcid: 0x1000,
    english_name: "Walser",
    iso639_two_letter: "wae",
    iso639_three_letter: "wae",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Walser (Switzerland)
pub const LANG_WAE_CH: &LanguageId = &LanguageId {
    name: "wae-CH",
    lcid: 0x1000,
    english_name: "Walser (Switzerland)",
    iso639_two_letter: "wae",
    iso639_three_letter: "wae",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Wolaytta
pub const LANG_WAL: &LanguageId = &LanguageId {
    name: "wal",
    lcid: 0x1000,
    english_name: "Wolaytta",
    iso639_two_letter: "wal",
    iso639_three_letter: "wal",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Wolaytta (Ethiopia)
pub const LANG_WAL_ET: &LanguageId = &LanguageId {
    name: "wal-ET",
    lcid: 0x1000,
    english_name: "Wolaytta (Ethiopia)",
    iso639_two_letter: "wal",
    iso639_three_letter: "wal",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Soga
pub const LANG_XOG: &LanguageId = &LanguageId {
    name: "xog",
    lcid: 0x1000,
    english_name: "Soga",
    iso639_two_letter: "xog",
    iso639_three_letter: "xog",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Soga (Uganda)
pub const LANG_XOG_UG: &LanguageId = &LanguageId {
    name: "xog-UG",
    lcid: 0x1000,
    english_name: "Soga (Uganda)",
    iso639_two_letter: "xog",
    iso639_three_letter: "xog",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Yangben
pub const LANG_YAV: &LanguageId = &LanguageId {
    name: "yav",
    lcid: 0x1000,
    english_name: "Yangben",
    iso639_two_letter: "yav",
    iso639_three_letter: "yav",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Yangben (Cameroon)
pub const LANG_YAV_CM: &LanguageId = &LanguageId {
    name: "yav-CM",
    lcid: 0x1000,
    english_name: "Yangben (Cameroon)",
    iso639_two_letter: "yav",
    iso639_three_letter: "yav",
    windows_three_letter: "ZZZ",
    ansi_code_page: None,
};

/// Yoruba (Benin)
pub const LANG_YO_BJ: &LanguageId = &LanguageId {
    name: "yo-BJ",
    lcid: 0x1000,
    english_name: "Yoruba (Benin)",
    iso639_two_letter: "yo",
    iso639_three_letter: "yor",
    windows_three_letter: "ZZZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Standard Moroccan Tamazight
pub const LANG_ZGH: &LanguageId = &LanguageId {
    name: "zgh",
    lcid: 0x1000,
    english_name: "Standard Moroccan Tamazight",
    iso639_two_letter: "zgh",
    iso639_three_letter: "zgh",
    windows_three_letter: "ZHG",
    ansi_code_page: None,
};

/// Standard Moroccan Tamazight (Tifinagh)
pub const LANG_ZGH_TFNG: &LanguageId = &LanguageId {
    name: "zgh-Tfng",
    lcid: 0x1000,
    english_name: "Standard Moroccan Tamazight (Tifinagh)",
    iso639_two_letter: "zgh",
    iso639_three_letter: "zgh",
    windows_three_letter: "ZHG",
    ansi_code_page: None,
};

/// Standard Moroccan Tamazight (Tifinagh, Morocco)
pub const LANG_ZGH_TFNG_MA: &LanguageId = &LanguageId {
    name: "zgh-Tfng-MA",
    lcid: 0x1000,
    english_name: "Standard Moroccan Tamazight (Tifinagh, Morocco)",
    iso639_two_letter: "zgh",
    iso639_three_letter: "zgh",
    windows_three_letter: "ZHG",
    ansi_code_page: None,
};

/// Arabic (Libya)
pub const LANG_AR_LY: &LanguageId = &LanguageId {
    name: "ar-LY",
    lcid: 0x1001,
    english_name: "Arabic (Libya)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARL",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Chinese (Simplified, Singapore)
pub const LANG_ZH_SG: &LanguageId = &LanguageId {
    name: "zh-SG",
    lcid: 0x1004,
    english_name: "Chinese (Simplified, Singapore)",
    iso639_two_letter: "zh",
    iso639_three_letter: "zho",
    windows_three_letter: "ZHI",
    ansi_code_page: Some(AnsiCodePage::GB2312),
};

/// German (Luxembourg)
pub const LANG_DE_LU: &LanguageId = &LanguageId {
    name: "de-LU",
    lcid: 0x1007,
    english_name: "German (Luxembourg)",
    iso639_two_letter: "de",
    iso639_three_letter: "deu",
    windows_three_letter: "DEL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Canada)
pub const LANG_EN_CA: &LanguageId = &LanguageId {
    name: "en-CA",
    lcid: 0x1009,
    english_name: "English (Canada)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENC",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Guatemala)
pub const LANG_ES_GT: &LanguageId = &LanguageId {
    name: "es-GT",
    lcid: 0x100A,
    english_name: "Spanish (Guatemala)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESG",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Switzerland)
pub const LANG_FR_CH: &LanguageId = &LanguageId {
    name: "fr-CH",
    lcid: 0x100C,
    english_name: "French (Switzerland)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Croatian (Bosnia and Herzegovina)
pub const LANG_HR_BA: &LanguageId = &LanguageId {
    name: "hr-BA",
    lcid: 0x101A,
    english_name: "Croatian (Bosnia and Herzegovina)",
    iso639_two_letter: "hr",
    iso639_three_letter: "hrv",
    windows_three_letter: "HRB",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Sami, Lule (Norway)
pub const LANG_SMJ_NO: &LanguageId = &LanguageId {
    name: "smj-NO",
    lcid: 0x103B,
    english_name: "Sami, Lule (Norway)",
    iso639_two_letter: "smj",
    iso639_three_letter: "smj",
    windows_three_letter: "SMJ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Central Atlas Tamazight (Tifinagh, Morocco)
pub const LANG_TZM_TFNG_MA: &LanguageId = &LanguageId {
    name: "tzm-Tfng-MA",
    lcid: 0x105F,
    english_name: "Central Atlas Tamazight (Tifinagh, Morocco)",
    iso639_two_letter: "tzm",
    iso639_three_letter: "tzm",
    windows_three_letter: "TZM",
    ansi_code_page: None,
};

/// Arabic (Algeria)
pub const LANG_AR_DZ: &LanguageId = &LanguageId {
    name: "ar-DZ",
    lcid: 0x1401,
    english_name: "Arabic (Algeria)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARG",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Chinese (Traditional, Macao SAR)
pub const LANG_ZH_MO: &LanguageId = &LanguageId {
    name: "zh-MO",
    lcid: 0x1404,
    english_name: "Chinese (Traditional, Macao SAR)",
    iso639_two_letter: "zh",
    iso639_three_letter: "zho",
    windows_three_letter: "ZHM",
    ansi_code_page: Some(AnsiCodePage::Big5),
};

/// German (Liechtenstein)
pub const LANG_DE_LI: &LanguageId = &LanguageId {
    name: "de-LI",
    lcid: 0x1407,
    english_name: "German (Liechtenstein)",
    iso639_two_letter: "de",
    iso639_three_letter: "deu",
    windows_three_letter: "DEC",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (New Zealand)
pub const LANG_EN_NZ: &LanguageId = &LanguageId {
    name: "en-NZ",
    lcid: 0x1409,
    english_name: "English (New Zealand)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Costa Rica)
pub const LANG_ES_CR: &LanguageId = &LanguageId {
    name: "es-CR",
    lcid: 0x140A,
    english_name: "Spanish (Costa Rica)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESC",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Luxembourg)
pub const LANG_FR_LU: &LanguageId = &LanguageId {
    name: "fr-LU",
    lcid: 0x140C,
    english_name: "French (Luxembourg)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Bosnian (Latin, Bosnia and Herzegovina)
pub const LANG_BS_LATN_BA: &LanguageId = &LanguageId {
    name: "bs-Latn-BA",
    lcid: 0x141A,
    english_name: "Bosnian (Latin, Bosnia and Herzegovina)",
    iso639_two_letter: "bs",
    iso639_three_letter: "bos",
    windows_three_letter: "BSB",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Sami, Lule (Sweden)
pub const LANG_SMJ_SE: &LanguageId = &LanguageId {
    name: "smj-SE",
    lcid: 0x143B,
    english_name: "Sami, Lule (Sweden)",
    iso639_two_letter: "smj",
    iso639_three_letter: "smj",
    windows_three_letter: "SMK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Arabic (Morocco)
pub const LANG_AR_MA: &LanguageId = &LanguageId {
    name: "ar-MA",
    lcid: 0x1801,
    english_name: "Arabic (Morocco)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARM",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (Ireland)
pub const LANG_EN_IE: &LanguageId = &LanguageId {
    name: "en-IE",
    lcid: 0x1809,
    english_name: "English (Ireland)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENI",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Panama)
pub const LANG_ES_PA: &LanguageId = &LanguageId {
    name: "es-PA",
    lcid: 0x180A,
    english_name: "Spanish (Panama)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Monaco)
pub const LANG_FR_MC: &LanguageId = &LanguageId {
    name: "fr-MC",
    lcid: 0x180C,
    english_name: "French (Monaco)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRM",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Serbian (Latin, Bosnia and Herzegovina)
pub const LANG_SR_LATN_BA: &LanguageId = &LanguageId {
    name: "sr-Latn-BA",
    lcid: 0x181A,
    english_name: "Serbian (Latin, Bosnia and Herzegovina)",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRS",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Sami, Southern (Norway)
pub const LANG_SMA_NO: &LanguageId = &LanguageId {
    name: "sma-NO",
    lcid: 0x183B,
    english_name: "Sami, Southern (Norway)",
    iso639_two_letter: "sma",
    iso639_three_letter: "sma",
    windows_three_letter: "SMA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Arabic (Tunisia)
pub const LANG_AR_TN: &LanguageId = &LanguageId {
    name: "ar-TN",
    lcid: 0x1C01,
    english_name: "Arabic (Tunisia)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ART",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (South Africa)
pub const LANG_EN_ZA: &LanguageId = &LanguageId {
    name: "en-ZA",
    lcid: 0x1C09,
    english_name: "English (South Africa)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Dominican Republic)
pub const LANG_ES_DO: &LanguageId = &LanguageId {
    name: "es-DO",
    lcid: 0x1C0A,
    english_name: "Spanish (Dominican Republic)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESD",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Serbian (Cyrillic, Bosnia and Herzegovina)
pub const LANG_SR_CYRL_BA: &LanguageId = &LanguageId {
    name: "sr-Cyrl-BA",
    lcid: 0x1C1A,
    english_name: "Serbian (Cyrillic, Bosnia and Herzegovina)",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRN",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Sami, Southern (Sweden)
pub const LANG_SMA_SE: &LanguageId = &LanguageId {
    name: "sma-SE",
    lcid: 0x1C3B,
    english_name: "Sami, Southern (Sweden)",
    iso639_two_letter: "sma",
    iso639_three_letter: "sma",
    windows_three_letter: "SMB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Arabic (Oman)
pub const LANG_AR_OM: &LanguageId = &LanguageId {
    name: "ar-OM",
    lcid: 0x2001,
    english_name: "Arabic (Oman)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARO",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (Jamaica)
pub const LANG_EN_JM: &LanguageId = &LanguageId {
    name: "en-JM",
    lcid: 0x2009,
    english_name: "English (Jamaica)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENJ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Venezuela)
pub const LANG_ES_VE: &LanguageId = &LanguageId {
    name: "es-VE",
    lcid: 0x200A,
    english_name: "Spanish (Venezuela)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESV",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Réunion)
pub const LANG_FR_RE: &LanguageId = &LanguageId {
    name: "fr-RE",
    lcid: 0x200C,
    english_name: "French (Réunion)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRR",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Bosnian (Cyrillic, Bosnia and Herzegovina)
pub const LANG_BS_CYRL_BA: &LanguageId = &LanguageId {
    name: "bs-Cyrl-BA",
    lcid: 0x201A,
    english_name: "Bosnian (Cyrillic, Bosnia and Herzegovina)",
    iso639_two_letter: "bs",
    iso639_three_letter: "bos",
    windows_three_letter: "BSC",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Sami, Skolt (Finland)
pub const LANG_SMS_FI: &LanguageId = &LanguageId {
    name: "sms-FI",
    lcid: 0x203B,
    english_name: "Sami, Skolt (Finland)",
    iso639_two_letter: "sms",
    iso639_three_letter: "sms",
    windows_three_letter: "SMS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Arabic (Yemen)
pub const LANG_AR_YE: &LanguageId = &LanguageId {
    name: "ar-YE",
    lcid: 0x2401,
    english_name: "Arabic (Yemen)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARY",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Spanish (Colombia)
pub const LANG_ES_CO: &LanguageId = &LanguageId {
    name: "es-CO",
    lcid: 0x240A,
    english_name: "Spanish (Colombia)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French Congo (DRC)
pub const LANG_FR_CD: &LanguageId = &LanguageId {
    name: "fr-CD",
    lcid: 0x240C,
    english_name: "French Congo (DRC)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRD",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Serbian (Latin, Serbia)
pub const LANG_SR_LATN_RS: &LanguageId = &LanguageId {
    name: "sr-Latn-RS",
    lcid: 0x241A,
    english_name: "Serbian (Latin, Serbia)",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRM",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Sami, Inari (Finland)
pub const LANG_SMN_FI: &LanguageId = &LanguageId {
    name: "smn-FI",
    lcid: 0x243B,
    english_name: "Sami, Inari (Finland)",
    iso639_two_letter: "smn",
    iso639_three_letter: "smn",
    windows_three_letter: "SMN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Arabic (Syria)
pub const LANG_AR_SY: &LanguageId = &LanguageId {
    name: "ar-SY",
    lcid: 0x2801,
    english_name: "Arabic (Syria)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARS",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (Belize)
pub const LANG_EN_BZ: &LanguageId = &LanguageId {
    name: "en-BZ",
    lcid: 0x2809,
    english_name: "English (Belize)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Peru)
pub const LANG_ES_PE: &LanguageId = &LanguageId {
    name: "es-PE",
    lcid: 0x280A,
    english_name: "Spanish (Peru)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESR",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Senegal)
pub const LANG_FR_SN: &LanguageId = &LanguageId {
    name: "fr-SN",
    lcid: 0x280C,
    english_name: "French (Senegal)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Serbian (Cyrillic, Serbia)
pub const LANG_SR_CYRL_RS: &LanguageId = &LanguageId {
    name: "sr-Cyrl-RS",
    lcid: 0x281A,
    english_name: "Serbian (Cyrillic, Serbia)",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRO",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Arabic (Jordan)
pub const LANG_AR_JO: &LanguageId = &LanguageId {
    name: "ar-JO",
    lcid: 0x2C01,
    english_name: "Arabic (Jordan)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARJ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (Trinidad and Tobago)
pub const LANG_EN_TT: &LanguageId = &LanguageId {
    name: "en-TT",
    lcid: 0x2C09,
    english_name: "English (Trinidad and Tobago)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENT",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Argentina)
pub const LANG_ES_AR: &LanguageId = &LanguageId {
    name: "es-AR",
    lcid: 0x2C0A,
    english_name: "Spanish (Argentina)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Cameroon)
pub const LANG_FR_CM: &LanguageId = &LanguageId {
    name: "fr-CM",
    lcid: 0x2C0C,
    english_name: "French (Cameroon)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Serbian (Latin, Montenegro)
pub const LANG_SR_LATN_ME: &LanguageId = &LanguageId {
    name: "sr-Latn-ME",
    lcid: 0x2C1A,
    english_name: "Serbian (Latin, Montenegro)",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRP",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Arabic (Lebanon)
pub const LANG_AR_LB: &LanguageId = &LanguageId {
    name: "ar-LB",
    lcid: 0x3001,
    english_name: "Arabic (Lebanon)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARB",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (Zimbabwe)
pub const LANG_EN_ZW: &LanguageId = &LanguageId {
    name: "en-ZW",
    lcid: 0x3009,
    english_name: "English (Zimbabwe)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENW",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Ecuador)
pub const LANG_ES_EC: &LanguageId = &LanguageId {
    name: "es-EC",
    lcid: 0x300A,
    english_name: "Spanish (Ecuador)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESF",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Côte d’Ivoire)
pub const LANG_FR_CI: &LanguageId = &LanguageId {
    name: "fr-CI",
    lcid: 0x300C,
    english_name: "French (Côte d’Ivoire)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRI",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Serbian (Cyrillic, Montenegro)
pub const LANG_SR_CYRL_ME: &LanguageId = &LanguageId {
    name: "sr-Cyrl-ME",
    lcid: 0x301A,
    english_name: "Serbian (Cyrillic, Montenegro)",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRQ",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Arabic (Kuwait)
pub const LANG_AR_KW: &LanguageId = &LanguageId {
    name: "ar-KW",
    lcid: 0x3401,
    english_name: "Arabic (Kuwait)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARK",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (Philippines)
pub const LANG_EN_PH: &LanguageId = &LanguageId {
    name: "en-PH",
    lcid: 0x3409,
    english_name: "English (Philippines)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENP",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Chile)
pub const LANG_ES_CL: &LanguageId = &LanguageId {
    name: "es-CL",
    lcid: 0x340A,
    english_name: "Spanish (Chile)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Mali)
pub const LANG_FR_ML: &LanguageId = &LanguageId {
    name: "fr-ML",
    lcid: 0x340C,
    english_name: "French (Mali)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRF",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Arabic (United Arab Emirates)
pub const LANG_AR_AE: &LanguageId = &LanguageId {
    name: "ar-AE",
    lcid: 0x3801,
    english_name: "Arabic (United Arab Emirates)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARU",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Spanish (Uruguay)
pub const LANG_ES_UY: &LanguageId = &LanguageId {
    name: "es-UY",
    lcid: 0x380A,
    english_name: "Spanish (Uruguay)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESY",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Morocco)
pub const LANG_FR_MA: &LanguageId = &LanguageId {
    name: "fr-MA",
    lcid: 0x380C,
    english_name: "French (Morocco)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRO",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Arabic (Bahrain)
pub const LANG_AR_BH: &LanguageId = &LanguageId {
    name: "ar-BH",
    lcid: 0x3C01,
    english_name: "Arabic (Bahrain)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARH",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (Hong Kong SAR)
pub const LANG_EN_HK: &LanguageId = &LanguageId {
    name: "en-HK",
    lcid: 0x3C09,
    english_name: "English (Hong Kong SAR)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENH",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Paraguay)
pub const LANG_ES_PY: &LanguageId = &LanguageId {
    name: "es-PY",
    lcid: 0x3C0A,
    english_name: "Spanish (Paraguay)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESZ",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// French (Haiti)
pub const LANG_FR_HT: &LanguageId = &LanguageId {
    name: "fr-HT",
    lcid: 0x3C0C,
    english_name: "French (Haiti)",
    iso639_two_letter: "fr",
    iso639_three_letter: "fra",
    windows_three_letter: "FRH",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Arabic (Qatar)
pub const LANG_AR_QA: &LanguageId = &LanguageId {
    name: "ar-QA",
    lcid: 0x4001,
    english_name: "Arabic (Qatar)",
    iso639_two_letter: "ar",
    iso639_three_letter: "ara",
    windows_three_letter: "ARQ",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// English (India)
pub const LANG_EN_IN: &LanguageId = &LanguageId {
    name: "en-IN",
    lcid: 0x4009,
    english_name: "English (India)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Bolivia)
pub const LANG_ES_BO: &LanguageId = &LanguageId {
    name: "es-BO",
    lcid: 0x400A,
    english_name: "Spanish (Bolivia)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Malaysia)
pub const LANG_EN_MY: &LanguageId = &LanguageId {
    name: "en-MY",
    lcid: 0x4409,
    english_name: "English (Malaysia)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENM",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (El Salvador)
pub const LANG_ES_SV: &LanguageId = &LanguageId {
    name: "es-SV",
    lcid: 0x440A,
    english_name: "Spanish (El Salvador)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (Singapore)
pub const LANG_EN_SG: &LanguageId = &LanguageId {
    name: "en-SG",
    lcid: 0x4809,
    english_name: "English (Singapore)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENE",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Honduras)
pub const LANG_ES_HN: &LanguageId = &LanguageId {
    name: "es-HN",
    lcid: 0x480A,
    english_name: "Spanish (Honduras)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESH",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// English (United Arab Emirates)
pub const LANG_EN_AE: &LanguageId = &LanguageId {
    name: "en-AE",
    lcid: 0x4C09,
    english_name: "English (United Arab Emirates)",
    iso639_two_letter: "en",
    iso639_three_letter: "eng",
    windows_three_letter: "ENU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Nicaragua)
pub const LANG_ES_NI: &LanguageId = &LanguageId {
    name: "es-NI",
    lcid: 0x4C0A,
    english_name: "Spanish (Nicaragua)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESI",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Puerto Rico)
pub const LANG_ES_PR: &LanguageId = &LanguageId {
    name: "es-PR",
    lcid: 0x500A,
    english_name: "Spanish (Puerto Rico)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (United States)
pub const LANG_ES_US: &LanguageId = &LanguageId {
    name: "es-US",
    lcid: 0x540A,
    english_name: "Spanish (United States)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "EST",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Spanish (Cuba)
pub const LANG_ES_CU: &LanguageId = &LanguageId {
    name: "es-CU",
    lcid: 0x5C0A,
    english_name: "Spanish (Cuba)",
    iso639_two_letter: "es",
    iso639_three_letter: "spa",
    windows_three_letter: "ESK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Bosnian (Cyrillic)
pub const LANG_BS_CYRL: &LanguageId = &LanguageId {
    name: "bs-Cyrl",
    lcid: 0x641A,
    english_name: "Bosnian (Cyrillic)",
    iso639_two_letter: "bs",
    iso639_three_letter: "bos",
    windows_three_letter: "BSC",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Bosnian (Latin)
pub const LANG_BS_LATN: &LanguageId = &LanguageId {
    name: "bs-Latn",
    lcid: 0x681A,
    english_name: "Bosnian (Latin)",
    iso639_two_letter: "bs",
    iso639_three_letter: "bos",
    windows_three_letter: "BSB",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Serbian (Cyrillic)
pub const LANG_SR_CYRL: &LanguageId = &LanguageId {
    name: "sr-Cyrl",
    lcid: 0x6C1A,
    english_name: "Serbian (Cyrillic)",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRO",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Serbian (Latin)
pub const LANG_SR_LATN: &LanguageId = &LanguageId {
    name: "sr-Latn",
    lcid: 0x701A,
    english_name: "Serbian (Latin)",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRM",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Sami (Inari)
pub const LANG_SMN: &LanguageId = &LanguageId {
    name: "smn",
    lcid: 0x703B,
    english_name: "Sami (Inari)",
    iso639_two_letter: "smn",
    iso639_three_letter: "smn",
    windows_three_letter: "SMN",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Azerbaijani (Cyrillic)
pub const LANG_AZ_CYRL: &LanguageId = &LanguageId {
    name: "az-Cyrl",
    lcid: 0x742C,
    english_name: "Azerbaijani (Cyrillic)",
    iso639_two_letter: "az",
    iso639_three_letter: "aze",
    windows_three_letter: "AZC",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Sami (Skolt)
pub const LANG_SMS: &LanguageId = &LanguageId {
    name: "sms",
    lcid: 0x743B,
    english_name: "Sami (Skolt)",
    iso639_two_letter: "sms",
    iso639_three_letter: "sms",
    windows_three_letter: "SMS",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Chinese
pub const LANG_ZH: &LanguageId = &LanguageId {
    name: "zh",
    lcid: 0x7804,
    english_name: "Chinese",
    iso639_two_letter: "zh",
    iso639_three_letter: "zho",
    windows_three_letter: "CHS",
    ansi_code_page: Some(AnsiCodePage::GB2312),
};

/// Norwegian Nynorsk
pub const LANG_NN: &LanguageId = &LanguageId {
    name: "nn",
    lcid: 0x7814,
    english_name: "Norwegian Nynorsk",
    iso639_two_letter: "nn",
    iso639_three_letter: "nno",
    windows_three_letter: "NON",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Bosnian
pub const LANG_BS: &LanguageId = &LanguageId {
    name: "bs",
    lcid: 0x781A,
    english_name: "Bosnian",
    iso639_two_letter: "bs",
    iso639_three_letter: "bos",
    windows_three_letter: "BSB",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Azerbaijani (Latin)
pub const LANG_AZ_LATN: &LanguageId = &LanguageId {
    name: "az-Latn",
    lcid: 0x782C,
    english_name: "Azerbaijani (Latin)",
    iso639_two_letter: "az",
    iso639_three_letter: "aze",
    windows_three_letter: "AZE",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Sami (Southern)
pub const LANG_SMA: &LanguageId = &LanguageId {
    name: "sma",
    lcid: 0x783B,
    english_name: "Sami (Southern)",
    iso639_two_letter: "sma",
    iso639_three_letter: "sma",
    windows_three_letter: "SMB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Uzbek (Cyrillic)
pub const LANG_UZ_CYRL: &LanguageId = &LanguageId {
    name: "uz-Cyrl",
    lcid: 0x7843,
    english_name: "Uzbek (Cyrillic)",
    iso639_two_letter: "uz",
    iso639_three_letter: "uzb",
    windows_three_letter: "UZC",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Mongolian
pub const LANG_MN_CYRL: &LanguageId = &LanguageId {
    name: "mn-Cyrl",
    lcid: 0x7850,
    english_name: "Mongolian",
    iso639_two_letter: "mn",
    iso639_three_letter: "mon",
    windows_three_letter: "MNN",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Inuktitut (Syllabics)
pub const LANG_IU_CANS: &LanguageId = &LanguageId {
    name: "iu-Cans",
    lcid: 0x785D,
    english_name: "Inuktitut (Syllabics)",
    iso639_two_letter: "iu",
    iso639_three_letter: "iku",
    windows_three_letter: "IUS",
    ansi_code_page: None,
};

/// Central Atlas Tamazight (Tifinagh)
pub const LANG_TZM_TFNG: &LanguageId = &LanguageId {
    name: "tzm-Tfng",
    lcid: 0x785F,
    english_name: "Central Atlas Tamazight (Tifinagh)",
    iso639_two_letter: "tzm",
    iso639_three_letter: "tzm",
    windows_three_letter: "TZM",
    ansi_code_page: None,
};

/// Chinese (Traditional)
pub const LANG_ZH_HANT: &LanguageId = &LanguageId {
    name: "zh-Hant",
    lcid: 0x7C04,
    english_name: "Chinese (Traditional)",
    iso639_two_letter: "zh",
    iso639_three_letter: "zho",
    windows_three_letter: "CHT",
    ansi_code_page: Some(AnsiCodePage::Big5),
};

/// Norwegian Bokmål
pub const LANG_NB: &LanguageId = &LanguageId {
    name: "nb",
    lcid: 0x7C14,
    english_name: "Norwegian Bokmål",
    iso639_two_letter: "nb",
    iso639_three_letter: "nob",
    windows_three_letter: "NOR",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Serbian
pub const LANG_SR: &LanguageId = &LanguageId {
    name: "sr",
    lcid: 0x7C1A,
    english_name: "Serbian",
    iso639_two_letter: "sr",
    iso639_three_letter: "srp",
    windows_three_letter: "SRB",
    ansi_code_page: Some(AnsiCodePage::Windows1250),
};

/// Tajik (Cyrillic)
pub const LANG_TG_CYRL: &LanguageId = &LanguageId {
    name: "tg-Cyrl",
    lcid: 0x7C28,
    english_name: "Tajik (Cyrillic)",
    iso639_two_letter: "tg",
    iso639_three_letter: "tgk",
    windows_three_letter: "TAJ",
    ansi_code_page: Some(AnsiCodePage::Windows1251),
};

/// Lower Sorbian
pub const LANG_DSB: &LanguageId = &LanguageId {
    name: "dsb",
    lcid: 0x7C2E,
    english_name: "Lower Sorbian",
    iso639_two_letter: "dsb",
    iso639_three_letter: "dsb",
    windows_three_letter: "DSB",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Sami (Lule)
pub const LANG_SMJ: &LanguageId = &LanguageId {
    name: "smj",
    lcid: 0x7C3B,
    english_name: "Sami (Lule)",
    iso639_two_letter: "smj",
    iso639_three_letter: "smj",
    windows_three_letter: "SMK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Uzbek (Latin)
pub const LANG_UZ_LATN: &LanguageId = &LanguageId {
    name: "uz-Latn",
    lcid: 0x7C43,
    english_name: "Uzbek (Latin)",
    iso639_two_letter: "uz",
    iso639_three_letter: "uzb",
    windows_three_letter: "UZB",
    ansi_code_page: Some(AnsiCodePage::Windows1254),
};

/// Punjabi
pub const LANG_PA_ARAB: &LanguageId = &LanguageId {
    name: "pa-Arab",
    lcid: 0x7C46,
    english_name: "Punjabi",
    iso639_two_letter: "pa",
    iso639_three_letter: "pan",
    windows_three_letter: "PAP",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Mongolian (Traditional Mongolian)
pub const LANG_MN_MONG: &LanguageId = &LanguageId {
    name: "mn-Mong",
    lcid: 0x7C50,
    english_name: "Mongolian (Traditional Mongolian)",
    iso639_two_letter: "mn",
    iso639_three_letter: "mon",
    windows_three_letter: "MNG",
    ansi_code_page: None,
};

/// Sindhi
pub const LANG_SD_ARAB: &LanguageId = &LanguageId {
    name: "sd-Arab",
    lcid: 0x7C59,
    english_name: "Sindhi",
    iso639_two_letter: "sd",
    iso639_three_letter: "snd",
    windows_three_letter: "SIP",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

/// Cherokee
pub const LANG_CHR_CHER: &LanguageId = &LanguageId {
    name: "chr-Cher",
    lcid: 0x7C5C,
    english_name: "Cherokee",
    iso639_two_letter: "chr",
    iso639_three_letter: "chr",
    windows_three_letter: "CRE",
    ansi_code_page: None,
};

/// Inuktitut (Latin)
pub const LANG_IU_LATN: &LanguageId = &LanguageId {
    name: "iu-Latn",
    lcid: 0x7C5D,
    english_name: "Inuktitut (Latin)",
    iso639_two_letter: "iu",
    iso639_three_letter: "iku",
    windows_three_letter: "IUK",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Central Atlas Tamazight (Latin)
pub const LANG_TZM_LATN: &LanguageId = &LanguageId {
    name: "tzm-Latn",
    lcid: 0x7C5F,
    english_name: "Central Atlas Tamazight (Latin)",
    iso639_two_letter: "tzm",
    iso639_three_letter: "tzm",
    windows_three_letter: "TZA",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Fulah
pub const LANG_FF_LATN: &LanguageId = &LanguageId {
    name: "ff-Latn",
    lcid: 0x7C67,
    english_name: "Fulah",
    iso639_two_letter: "ff",
    iso639_three_letter: "ful",
    windows_three_letter: "FUL",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Hausa (Latin)
pub const LANG_HA_LATN: &LanguageId = &LanguageId {
    name: "ha-Latn",
    lcid: 0x7C68,
    english_name: "Hausa (Latin)",
    iso639_two_letter: "ha",
    iso639_three_letter: "hau",
    windows_three_letter: "HAU",
    ansi_code_page: Some(AnsiCodePage::Windows1252),
};

/// Central Kurdish
pub const LANG_KU_ARAB: &LanguageId = &LanguageId {
    name: "ku-Arab",
    lcid: 0x7C92,
    english_name: "Central Kurdish",
    iso639_two_letter: "ku",
    iso639_three_letter: "kur",
    windows_three_letter: "KUR",
    ansi_code_page: Some(AnsiCodePage::Windows1256),
};

macro_rules! parse_lcid {
    ($value:expr) => {
        match $value {
            0x0001 => Ok(constants::LANG_AR),
            0x0002 => Ok(constants::LANG_BG),
            0x0003 => Ok(constants::LANG_CA),
            0x0004 => Ok(constants::LANG_ZH_HANS),
            0x0005 => Ok(constants::LANG_CS),
            0x0006 => Ok(constants::LANG_DA),
            0x0007 => Ok(constants::LANG_DE),
            0x0008 => Ok(constants::LANG_EL),
            0x0009 => Ok(constants::LANG_EN),
            0x000A => Ok(constants::LANG_ES),
            0x000B => Ok(constants::LANG_FI),
            0x000C => Ok(constants::LANG_FR),
            0x000D => Ok(constants::LANG_HE),
            0x000E => Ok(constants::LANG_HU),
            0x000F => Ok(constants::LANG_IS),
            0x0010 => Ok(constants::LANG_IT),
            0x0011 => Ok(constants::LANG_JA),
            0x0012 => Ok(constants::LANG_KO),
            0x0013 => Ok(constants::LANG_NL),
            0x0014 => Ok(constants::LANG_NO),
            0x0015 => Ok(constants::LANG_PL),
            0x0016 => Ok(constants::LANG_PT),
            0x0017 => Ok(constants::LANG_RM),
            0x0018 => Ok(constants::LANG_RO),
            0x0019 => Ok(constants::LANG_RU),
            0x001A => Ok(constants::LANG_HR),
            0x001B => Ok(constants::LANG_SK),
            0x001C => Ok(constants::LANG_SQ),
            0x001D => Ok(constants::LANG_SV),
            0x001E => Ok(constants::LANG_TH),
            0x001F => Ok(constants::LANG_TR),
            0x0020 => Ok(constants::LANG_UR),
            0x0021 => Ok(constants::LANG_ID),
            0x0022 => Ok(constants::LANG_UK),
            0x0023 => Ok(constants::LANG_BE),
            0x0024 => Ok(constants::LANG_SL),
            0x0025 => Ok(constants::LANG_ET),
            0x0026 => Ok(constants::LANG_LV),
            0x0027 => Ok(constants::LANG_LT),
            0x0028 => Ok(constants::LANG_TG),
            0x0029 => Ok(constants::LANG_FA),
            0x002A => Ok(constants::LANG_VI),
            0x002B => Ok(constants::LANG_HY),
            0x002C => Ok(constants::LANG_AZ),
            0x002D => Ok(constants::LANG_EU),
            0x002E => Ok(constants::LANG_HSB),
            0x002F => Ok(constants::LANG_MK),
            0x0030 => Ok(constants::LANG_ST),
            0x0031 => Ok(constants::LANG_TS),
            0x0032 => Ok(constants::LANG_TN),
            0x0033 => Ok(constants::LANG_VE),
            0x0034 => Ok(constants::LANG_XH),
            0x0035 => Ok(constants::LANG_ZU),
            0x0036 => Ok(constants::LANG_AF),
            0x0037 => Ok(constants::LANG_KA),
            0x0038 => Ok(constants::LANG_FO),
            0x0039 => Ok(constants::LANG_HI),
            0x003A => Ok(constants::LANG_MT),
            0x003B => Ok(constants::LANG_SE),
            0x003C => Ok(constants::LANG_GA),
            0x003D => Err(Self::Error::Reserved(0x003D, "yi")),
            0x003E => Ok(constants::LANG_MS),
            0x003F => Ok(constants::LANG_KK),
            0x0040 => Ok(constants::LANG_KY),
            0x0041 => Ok(constants::LANG_SW),
            0x0042 => Ok(constants::LANG_TK),
            0x0043 => Ok(constants::LANG_UZ),
            0x0044 => Ok(constants::LANG_TT),
            0x0045 => Ok(constants::LANG_BN),
            0x0046 => Ok(constants::LANG_PA),
            0x0047 => Ok(constants::LANG_GU),
            0x0048 => Ok(constants::LANG_OR),
            0x0049 => Ok(constants::LANG_TA),
            0x004A => Ok(constants::LANG_TE),
            0x004B => Ok(constants::LANG_KN),
            0x004C => Ok(constants::LANG_ML),
            0x004D => Ok(constants::LANG_AS),
            0x004E => Ok(constants::LANG_MR),
            0x004F => Ok(constants::LANG_SA),
            0x0050 => Ok(constants::LANG_MN),
            0x0051 => Ok(constants::LANG_BO),
            0x0052 => Ok(constants::LANG_CY),
            0x0053 => Ok(constants::LANG_KM),
            0x0054 => Ok(constants::LANG_LO),
            0x0055 => Ok(constants::LANG_MY),
            0x0056 => Ok(constants::LANG_GL),
            0x0057 => Ok(constants::LANG_KOK),
            0x0058 => Err(Self::Error::Reserved(0x0058, "mni")),
            0x0059 => Ok(constants::LANG_SD),
            0x005A => Ok(constants::LANG_SYR),
            0x005B => Ok(constants::LANG_SI),
            0x005C => Ok(constants::LANG_CHR),
            0x005D => Ok(constants::LANG_IU),
            0x005E => Ok(constants::LANG_AM),
            0x005F => Ok(constants::LANG_TZM),
            0x0060 => Ok(constants::LANG_KS),
            0x0061 => Ok(constants::LANG_NE),
            0x0062 => Ok(constants::LANG_FY),
            0x0063 => Ok(constants::LANG_PS),
            0x0064 => Ok(constants::LANG_FIL),
            0x0065 => Ok(constants::LANG_DV),
            0x0066 => Err(Self::Error::Reserved(0x0066, "bin")),
            0x0067 => Ok(constants::LANG_FF),
            0x0068 => Ok(constants::LANG_HA),
            0x0069 => Err(Self::Error::Reserved(0x0069, "ibb")),
            0x006A => Ok(constants::LANG_YO),
            0x006B => Ok(constants::LANG_QUZ),
            0x006C => Ok(constants::LANG_NSO),
            0x006D => Ok(constants::LANG_BA),
            0x006E => Ok(constants::LANG_LB),
            0x006F => Ok(constants::LANG_KL),
            0x0070 => Ok(constants::LANG_IG),
            0x0071 => Err(Self::Error::Reserved(0x0071, "kr")),
            0x0072 => Ok(constants::LANG_OM),
            0x0073 => Ok(constants::LANG_TI),
            0x0074 => Ok(constants::LANG_GN),
            0x0075 => Ok(constants::LANG_HAW),
            0x0076 => Err(Self::Error::Reserved(0x0076, "la")),
            0x0077 => Err(Self::Error::Reserved(0x0077, "so")),
            0x0078 => Ok(constants::LANG_II),
            0x0079 => Err(Self::Error::Reserved(0x0079, "pap")),
            0x007A => Ok(constants::LANG_ARN),
            0x007B => Err(Self::Error::NeitherDefinedNorReserved(0x007B)),
            0x007C => Ok(constants::LANG_MOH),
            0x007D => Err(Self::Error::NeitherDefinedNorReserved(0x007D)),
            0x007E => Ok(constants::LANG_BR),
            0x007F => Ok(constants::LANG_INVARIANT),
            0x0080 => Ok(constants::LANG_UG),
            0x0081 => Ok(constants::LANG_MI),
            0x0082 => Ok(constants::LANG_OC),
            0x0083 => Ok(constants::LANG_CO),
            0x0084 => Ok(constants::LANG_GSW),
            0x0085 => Ok(constants::LANG_SAH),
            0x0086 => Ok(constants::LANG_QUT),
            0x0087 => Ok(constants::LANG_RW),
            0x0088 => Ok(constants::LANG_WO),
            0x0089 => Err(Self::Error::NeitherDefinedNorReserved(0x0089)),
            0x008A => Err(Self::Error::NeitherDefinedNorReserved(0x008A)),
            0x008B => Err(Self::Error::NeitherDefinedNorReserved(0x008B)),
            0x008C => Ok(constants::LANG_PRS),
            0x008D => Err(Self::Error::NeitherDefinedNorReserved(0x008D)),
            0x008E => Err(Self::Error::NeitherDefinedNorReserved(0x008E)),
            0x008F => Err(Self::Error::NeitherDefinedNorReserved(0x008F)),
            0x0090 => Err(Self::Error::NeitherDefinedNorReserved(0x0090)),
            0x0091 => Ok(constants::LANG_GD),
            0x0092 => Ok(constants::LANG_KU),
            0x0093 => Err(Self::Error::Reserved(0x0093, "quc")),
            0x0401 => Ok(constants::LANG_AR_SA),
            0x0402 => Ok(constants::LANG_BG_BG),
            0x0403 => Ok(constants::LANG_CA_ES),
            0x0404 => Ok(constants::LANG_ZH_TW),
            0x0405 => Ok(constants::LANG_CS_CZ),
            0x0406 => Ok(constants::LANG_DA_DK),
            0x0407 => Ok(constants::LANG_DE_DE),
            0x0408 => Ok(constants::LANG_EL_GR),
            0x0409 => Ok(constants::LANG_EN_US),
            0x040A => Ok(constants::LANG_ES_ES_TRADNL),
            0x040B => Ok(constants::LANG_FI_FI),
            0x040C => Ok(constants::LANG_FR_FR),
            0x040D => Ok(constants::LANG_HE_IL),
            0x040E => Ok(constants::LANG_HU_HU),
            0x040F => Ok(constants::LANG_IS_IS),
            0x0410 => Ok(constants::LANG_IT_IT),
            0x0411 => Ok(constants::LANG_JA_JP),
            0x0412 => Ok(constants::LANG_KO_KR),
            0x0413 => Ok(constants::LANG_NL_NL),
            0x0414 => Ok(constants::LANG_NB_NO),
            0x0415 => Ok(constants::LANG_PL_PL),
            0x0416 => Ok(constants::LANG_PT_BR),
            0x0417 => Ok(constants::LANG_RM_CH),
            0x0418 => Ok(constants::LANG_RO_RO),
            0x0419 => Ok(constants::LANG_RU_RU),
            0x041A => Ok(constants::LANG_HR_HR),
            0x041B => Ok(constants::LANG_SK_SK),
            0x041C => Ok(constants::LANG_SQ_AL),
            0x041D => Ok(constants::LANG_SV_SE),
            0x041E => Ok(constants::LANG_TH_TH),
            0x041F => Ok(constants::LANG_TR_TR),
            0x0420 => Ok(constants::LANG_UR_PK),
            0x0421 => Ok(constants::LANG_ID_ID),
            0x0422 => Ok(constants::LANG_UK_UA),
            0x0423 => Ok(constants::LANG_BE_BY),
            0x0424 => Ok(constants::LANG_SL_SI),
            0x0425 => Ok(constants::LANG_ET_EE),
            0x0426 => Ok(constants::LANG_LV_LV),
            0x0427 => Ok(constants::LANG_LT_LT),
            0x0428 => Ok(constants::LANG_TG_CYRL_TJ),
            0x0429 => Ok(constants::LANG_FA_IR),
            0x042A => Ok(constants::LANG_VI_VN),
            0x042B => Ok(constants::LANG_HY_AM),
            0x042C => Ok(constants::LANG_AZ_LATN_AZ),
            0x042D => Ok(constants::LANG_EU_ES),
            0x042E => Ok(constants::LANG_HSB_DE),
            0x042F => Ok(constants::LANG_MK_MK),
            0x0430 => Ok(constants::LANG_ST_ZA),
            0x0431 => Ok(constants::LANG_TS_ZA),
            0x0432 => Ok(constants::LANG_TN_ZA),
            0x0433 => Ok(constants::LANG_VE_ZA),
            0x0434 => Ok(constants::LANG_XH_ZA),
            0x0435 => Ok(constants::LANG_ZU_ZA),
            0x0436 => Ok(constants::LANG_AF_ZA),
            0x0437 => Ok(constants::LANG_KA_GE),
            0x0438 => Ok(constants::LANG_FO_FO),
            0x0439 => Ok(constants::LANG_HI_IN),
            0x043A => Ok(constants::LANG_MT_MT),
            0x043B => Ok(constants::LANG_SE_NO),
            0x043D => Err(Self::Error::Reserved(0x043D, "yi-Hebr")),
            0x043E => Ok(constants::LANG_MS_MY),
            0x043F => Ok(constants::LANG_KK_KZ),
            0x0440 => Ok(constants::LANG_KY_KG),
            0x0441 => Ok(constants::LANG_SW_KE),
            0x0442 => Ok(constants::LANG_TK_TM),
            0x0443 => Ok(constants::LANG_UZ_LATN_UZ),
            0x0444 => Ok(constants::LANG_TT_RU),
            0x0445 => Ok(constants::LANG_BN_IN),
            0x0446 => Ok(constants::LANG_PA_IN),
            0x0447 => Ok(constants::LANG_GU_IN),
            0x0448 => Ok(constants::LANG_OR_IN),
            0x0449 => Ok(constants::LANG_TA_IN),
            0x044A => Ok(constants::LANG_TE_IN),
            0x044B => Ok(constants::LANG_KN_IN),
            0x044C => Ok(constants::LANG_ML_IN),
            0x044D => Ok(constants::LANG_AS_IN),
            0x044E => Ok(constants::LANG_MR_IN),
            0x044F => Ok(constants::LANG_SA_IN),
            0x0450 => Ok(constants::LANG_MN_MN),
            0x0451 => Ok(constants::LANG_BO_CN),
            0x0452 => Ok(constants::LANG_CY_GB),
            0x0453 => Ok(constants::LANG_KM_KH),
            0x0454 => Ok(constants::LANG_LO_LA),
            0x0455 => Ok(constants::LANG_MY_MM),
            0x0456 => Ok(constants::LANG_GL_ES),
            0x0457 => Ok(constants::LANG_KOK_IN),
            0x0458 => Err(Self::Error::Reserved(0x0458, "mni-IN")),
            0x0459 => Err(Self::Error::Reserved(0x0459, "sd-Deva-IN")),
            0x045A => Ok(constants::LANG_SYR_SY),
            0x045B => Ok(constants::LANG_SI_LK),
            0x045C => Ok(constants::LANG_CHR_CHER_US),
            0x045D => Ok(constants::LANG_IU_CANS_CA),
            0x045E => Ok(constants::LANG_AM_ET),
            0x045F => Err(Self::Error::Reserved(0x045F, "tzm-Arab-MA")),
            0x0460 => Ok(constants::LANG_KS_ARAB),
            0x0461 => Ok(constants::LANG_NE_NP),
            0x0462 => Ok(constants::LANG_FY_NL),
            0x0463 => Ok(constants::LANG_PS_AF),
            0x0464 => Ok(constants::LANG_FIL_PH),
            0x0465 => Ok(constants::LANG_DV_MV),
            0x0466 => Err(Self::Error::Reserved(0x0466, "bin-NG")),
            0x0467 => Err(Self::Error::Reserved(0x0467, "fuv-NG")),
            0x0468 => Ok(constants::LANG_HA_LATN_NG),
            0x0469 => Err(Self::Error::Reserved(0x0469, "ibb-NG")),
            0x046A => Ok(constants::LANG_YO_NG),
            0x046B => Ok(constants::LANG_QUZ_BO),
            0x046C => Ok(constants::LANG_NSO_ZA),
            0x046D => Ok(constants::LANG_BA_RU),
            0x046E => Ok(constants::LANG_LB_LU),
            0x046F => Ok(constants::LANG_KL_GL),
            0x0470 => Ok(constants::LANG_IG_NG),
            0x0471 => Err(Self::Error::Reserved(0x0471, "kr-NG")),
            0x0472 => Ok(constants::LANG_OM_ET),
            0x0473 => Ok(constants::LANG_TI_ET),
            0x0474 => Ok(constants::LANG_GN_PY),
            0x0475 => Ok(constants::LANG_HAW_US),
            0x0476 => Err(Self::Error::Reserved(0x0476, "la-Latn")),
            0x0477 => Ok(constants::LANG_SO_SO),
            0x0478 => Ok(constants::LANG_II_CN),
            0x0479 => Err(Self::Error::Reserved(0x0479, "pap-029")),
            0x047A => Ok(constants::LANG_ARN_CL),
            0x047C => Ok(constants::LANG_MOH_CA),
            0x047E => Ok(constants::LANG_BR_FR),
            0x0480 => Ok(constants::LANG_UG_CN),
            0x0481 => Ok(constants::LANG_MI_NZ),
            0x0482 => Ok(constants::LANG_OC_FR),
            0x0483 => Ok(constants::LANG_CO_FR),
            0x0484 => Ok(constants::LANG_GSW_FR),
            0x0485 => Ok(constants::LANG_SAH_RU),
            0x0486 => Err(Self::Error::Reserved(0x0486, "qut-GT")),
            0x0487 => Ok(constants::LANG_RW_RW),
            0x0488 => Ok(constants::LANG_WO_SN),
            0x048C => Ok(constants::LANG_PRS_AF),
            0x048D => Err(Self::Error::Reserved(0x048D, "plt-MG")),
            0x048E => Err(Self::Error::Reserved(0x048E, "zh-yue-HK")),
            0x048F => Err(Self::Error::Reserved(0x048F, "tdd-Tale-CN")),
            0x0490 => Err(Self::Error::Reserved(0x0490, "khb-Talu-CN")),
            0x0491 => Ok(constants::LANG_GD_GB),
            0x0492 => Ok(constants::LANG_KU_ARAB_IQ),
            0x0493 => Err(Self::Error::Reserved(0x0493, "quc-CO")),
            0x0501 => Ok(constants::LANG_QPS_PLOC),
            0x05FE => Ok(constants::LANG_QPS_PLOCA),
            0x0801 => Ok(constants::LANG_AR_IQ),
            0x0803 => Ok(constants::LANG_CA_ES_VALENCIA),
            0x0804 => Ok(constants::LANG_ZH_CN),
            0x0807 => Ok(constants::LANG_DE_CH),
            0x0809 => Ok(constants::LANG_EN_GB),
            0x080A => Ok(constants::LANG_ES_MX),
            0x080C => Ok(constants::LANG_FR_BE),
            0x0810 => Ok(constants::LANG_IT_CH),
            0x0811 => Err(Self::Error::Reserved(0x0811, "ja-Ploc-JP")),
            0x0813 => Ok(constants::LANG_NL_BE),
            0x0814 => Ok(constants::LANG_NN_NO),
            0x0816 => Ok(constants::LANG_PT_PT),
            0x0818 => Ok(constants::LANG_RO_MD),
            0x0819 => Ok(constants::LANG_RU_MD),
            0x081A => Ok(constants::LANG_SR_LATN_CS),
            0x081D => Ok(constants::LANG_SV_FI),
            0x0820 => Ok(constants::LANG_UR_IN),
            0x0827 => Err(Self::Error::NeitherDefinedNorReserved(0x0827)),
            0x082C => Err(Self::Error::Reserved(0x082C, "az-Cyrl-AZ")),
            0x082E => Ok(constants::LANG_DSB_DE),
            0x0832 => Ok(constants::LANG_TN_BW),
            0x083B => Ok(constants::LANG_SE_SE),
            0x083C => Ok(constants::LANG_GA_IE),
            0x083E => Ok(constants::LANG_MS_BN),
            0x083F => Err(Self::Error::Reserved(0x083F, "kk-Latn-KZ")),
            0x0843 => Err(Self::Error::Reserved(0x0843, "uz-Cyrl-UZ")),
            0x0845 => Ok(constants::LANG_BN_BD),
            0x0846 => Ok(constants::LANG_PA_ARAB_PK),
            0x0849 => Ok(constants::LANG_TA_LK),
            0x0850 => Err(Self::Error::Reserved(0x0850, "mn-Mong-CN")),
            0x0851 => Err(Self::Error::Reserved(0x0851, "bo-BT")),
            0x0859 => Ok(constants::LANG_SD_ARAB_PK),
            0x085D => Ok(constants::LANG_IU_LATN_CA),
            0x085F => Ok(constants::LANG_TZM_LATN_DZ),
            0x0860 => Err(Self::Error::Reserved(0x0860, "ks-Deva")),
            0x0861 => Ok(constants::LANG_NE_IN),
            0x0867 => Ok(constants::LANG_FF_LATN_SN),
            0x086B => Ok(constants::LANG_QUZ_EC),
            0x0873 => Ok(constants::LANG_TI_ER),
            0x09FF => Ok(constants::LANG_QPS_PLOCM),
            0x0C01 => Ok(constants::LANG_AR_EG),
            0x0C04 => Ok(constants::LANG_ZH_HK),
            0x0C07 => Ok(constants::LANG_DE_AT),
            0x0C09 => Ok(constants::LANG_EN_AU),
            0x0C0A => Ok(constants::LANG_ES_ES),
            0x0C0C => Ok(constants::LANG_FR_CA),
            0x0C1A => Ok(constants::LANG_SR_CYRL_CS),
            0x0C3B => Ok(constants::LANG_SE_FI),
            0x0C50 => Ok(constants::LANG_MN_MONG_MN),
            0x0C51 => Ok(constants::LANG_DZ_BT),
            0x0C5F => Err(Self::Error::Reserved(0x0C5F, "tmz-MA")),
            0x0C6B => Ok(constants::LANG_QUZ_PE),
            0x1001 => Ok(constants::LANG_AR_LY),
            0x1004 => Ok(constants::LANG_ZH_SG),
            0x1007 => Ok(constants::LANG_DE_LU),
            0x1009 => Ok(constants::LANG_EN_CA),
            0x100A => Ok(constants::LANG_ES_GT),
            0x100C => Ok(constants::LANG_FR_CH),
            0x101A => Ok(constants::LANG_HR_BA),
            0x103B => Ok(constants::LANG_SMJ_NO),
            0x105F => Ok(constants::LANG_TZM_TFNG_MA),
            0x1401 => Ok(constants::LANG_AR_DZ),
            0x1404 => Ok(constants::LANG_ZH_MO),
            0x1407 => Ok(constants::LANG_DE_LI),
            0x1409 => Ok(constants::LANG_EN_NZ),
            0x140A => Ok(constants::LANG_ES_CR),
            0x140C => Ok(constants::LANG_FR_LU),
            0x141A => Ok(constants::LANG_BS_LATN_BA),
            0x143B => Ok(constants::LANG_SMJ_SE),
            0x1801 => Ok(constants::LANG_AR_MA),
            0x1809 => Ok(constants::LANG_EN_IE),
            0x180A => Ok(constants::LANG_ES_PA),
            0x180C => Ok(constants::LANG_FR_MC),
            0x181A => Ok(constants::LANG_SR_LATN_BA),
            0x183B => Ok(constants::LANG_SMA_NO),
            0x1C01 => Ok(constants::LANG_AR_TN),
            0x1C09 => Ok(constants::LANG_EN_ZA),
            0x1C0A => Ok(constants::LANG_ES_DO),
            0x1C0C => Err(Self::Error::NeitherDefinedNorReserved(0x1C0C)),
            0x1C1A => Ok(constants::LANG_SR_CYRL_BA),
            0x1C3B => Ok(constants::LANG_SMA_SE),
            0x2001 => Ok(constants::LANG_AR_OM),
            0x2008 => Err(Self::Error::NeitherDefinedNorReserved(0x2008)),
            0x2009 => Ok(constants::LANG_EN_JM),
            0x200A => Ok(constants::LANG_ES_VE),
            0x200C => Ok(constants::LANG_FR_RE),
            0x201A => Ok(constants::LANG_BS_CYRL_BA),
            0x203B => Ok(constants::LANG_SMS_FI),
            0x2401 => Ok(constants::LANG_AR_YE),
            0x2409 => Err(Self::Error::Reserved(0x2409, "en-029")),
            0x240A => Ok(constants::LANG_ES_CO),
            0x240C => Ok(constants::LANG_FR_CD),
            0x241A => Ok(constants::LANG_SR_LATN_RS),
            0x243B => Ok(constants::LANG_SMN_FI),
            0x2801 => Ok(constants::LANG_AR_SY),
            0x2809 => Ok(constants::LANG_EN_BZ),
            0x280A => Ok(constants::LANG_ES_PE),
            0x280C => Ok(constants::LANG_FR_SN),
            0x281A => Ok(constants::LANG_SR_CYRL_RS),
            0x2C01 => Ok(constants::LANG_AR_JO),
            0x2C09 => Ok(constants::LANG_EN_TT),
            0x2C0A => Ok(constants::LANG_ES_AR),
            0x2C0C => Ok(constants::LANG_FR_CM),
            0x2C1A => Ok(constants::LANG_SR_LATN_ME),
            0x3001 => Ok(constants::LANG_AR_LB),
            0x3009 => Ok(constants::LANG_EN_ZW),
            0x300A => Ok(constants::LANG_ES_EC),
            0x300C => Ok(constants::LANG_FR_CI),
            0x301A => Ok(constants::LANG_SR_CYRL_ME),
            0x3401 => Ok(constants::LANG_AR_KW),
            0x3409 => Ok(constants::LANG_EN_PH),
            0x340A => Ok(constants::LANG_ES_CL),
            0x340C => Ok(constants::LANG_FR_ML),
            0x3801 => Ok(constants::LANG_AR_AE),
            0x3809 => Err(Self::Error::Reserved(0x3809, "en-ID")),
            0x380A => Ok(constants::LANG_ES_UY),
            0x380C => Ok(constants::LANG_FR_MA),
            0x3C01 => Ok(constants::LANG_AR_BH),
            0x3C09 => Ok(constants::LANG_EN_HK),
            0x3C0A => Ok(constants::LANG_ES_PY),
            0x3C0C => Ok(constants::LANG_FR_HT),
            0x4001 => Ok(constants::LANG_AR_QA),
            0x4009 => Ok(constants::LANG_EN_IN),
            0x400A => Ok(constants::LANG_ES_BO),
            0x4401 => Err(Self::Error::Reserved(0x4401, "ar-Ploc-SA")),
            0x4409 => Ok(constants::LANG_EN_MY),
            0x440A => Ok(constants::LANG_ES_SV),
            0x4801 => Err(Self::Error::Reserved(0x4801, "ar-145")),
            0x4809 => Ok(constants::LANG_EN_SG),
            0x480A => Ok(constants::LANG_ES_HN),
            0x4C09 => Ok(constants::LANG_EN_AE),
            0x4C0A => Ok(constants::LANG_ES_NI),
            0x5009 => Err(Self::Error::Reserved(0x5009, "en-BH")),
            0x500A => Ok(constants::LANG_ES_PR),
            0x5409 => Err(Self::Error::Reserved(0x5409, "en-EG")),
            0x540A => Ok(constants::LANG_ES_US),
            0x5809 => Err(Self::Error::Reserved(0x5809, "en-JO")),
            0x580A => Err(Self::Error::Reserved(0x580A, "es-419")),
            0x5C09 => Err(Self::Error::Reserved(0x5C09, "en-KW")),
            0x5C0A => Ok(constants::LANG_ES_CU),
            0x6009 => Err(Self::Error::Reserved(0x6009, "en-TR")),
            0x6409 => Err(Self::Error::Reserved(0x6409, "en-YE")),
            0x641A => Ok(constants::LANG_BS_CYRL),
            0x681A => Ok(constants::LANG_BS_LATN),
            0x6C1A => Ok(constants::LANG_SR_CYRL),
            0x701A => Ok(constants::LANG_SR_LATN),
            0x703B => Ok(constants::LANG_SMN),
            0x742C => Ok(constants::LANG_AZ_CYRL),
            0x743B => Ok(constants::LANG_SMS),
            0x7804 => Ok(constants::LANG_ZH),
            0x7814 => Ok(constants::LANG_NN),
            0x781A => Ok(constants::LANG_BS),
            0x782C => Ok(constants::LANG_AZ_LATN),
            0x783B => Ok(constants::LANG_SMA),
            0x783F => Err(Self::Error::Reserved(0x783F, "kk-Cyrl")),
            0x7843 => Ok(constants::LANG_UZ_CYRL),
            0x7850 => Ok(constants::LANG_MN_CYRL),
            0x785D => Ok(constants::LANG_IU_CANS),
            0x785F => Ok(constants::LANG_TZM_TFNG),
            0x7C04 => Ok(constants::LANG_ZH_HANT),
            0x7C14 => Ok(constants::LANG_NB),
            0x7C1A => Ok(constants::LANG_SR),
            0x7C28 => Ok(constants::LANG_TG_CYRL),
            0x7C2E => Ok(constants::LANG_DSB),
            0x7C3B => Ok(constants::LANG_SMJ),
            0x7C3F => Err(Self::Error::Reserved(0x7C3F, "kk-Latn")),
            0x7C43 => Ok(constants::LANG_UZ_LATN),
            0x7C46 => Ok(constants::LANG_PA_ARAB),
            0x7C50 => Ok(constants::LANG_MN_MONG),
            0x7C59 => Ok(constants::LANG_SD_ARAB),
            0x7C5C => Ok(constants::LANG_CHR_CHER),
            0x7C5D => Ok(constants::LANG_IU_LATN),
            0x7C5F => Ok(constants::LANG_TZM_LATN),
            0x7C67 => Ok(constants::LANG_FF_LATN),
            0x7C68 => Ok(constants::LANG_HA_LATN),
            0x7C92 => Ok(constants::LANG_KU_ARAB),
            0xF2EE => Err(Self::Error::ReservedUnknown(0xF2EE)),
            0xE40C => Err(Self::Error::Reserved(0xE40C, "fr-015")),
            0xEEEE => Err(Self::Error::ReservedUnknown(0xEEEE)),
            undef => Err(Self::Error::Undefined(undef)),
        }
    };
}

macro_rules! parse_name {
    ($value:expr) => {
        match $value {
            "ar" => Ok(constants::LANG_AR),
            "bg" => Ok(constants::LANG_BG),
            "ca" => Ok(constants::LANG_CA),
            "zh-Hans" => Ok(constants::LANG_ZH_HANS),
            "cs" => Ok(constants::LANG_CS),
            "da" => Ok(constants::LANG_DA),
            "de" => Ok(constants::LANG_DE),
            "el" => Ok(constants::LANG_EL),
            "en" => Ok(constants::LANG_EN),
            "es" => Ok(constants::LANG_ES),
            "fi" => Ok(constants::LANG_FI),
            "fr" => Ok(constants::LANG_FR),
            "he" => Ok(constants::LANG_HE),
            "hu" => Ok(constants::LANG_HU),
            "is" => Ok(constants::LANG_IS),
            "it" => Ok(constants::LANG_IT),
            "ja" => Ok(constants::LANG_JA),
            "ko" => Ok(constants::LANG_KO),
            "nl" => Ok(constants::LANG_NL),
            "no" => Ok(constants::LANG_NO),
            "pl" => Ok(constants::LANG_PL),
            "pt" => Ok(constants::LANG_PT),
            "rm" => Ok(constants::LANG_RM),
            "ro" => Ok(constants::LANG_RO),
            "ru" => Ok(constants::LANG_RU),
            "hr" => Ok(constants::LANG_HR),
            "sk" => Ok(constants::LANG_SK),
            "sq" => Ok(constants::LANG_SQ),
            "sv" => Ok(constants::LANG_SV),
            "th" => Ok(constants::LANG_TH),
            "tr" => Ok(constants::LANG_TR),
            "ur" => Ok(constants::LANG_UR),
            "id" => Ok(constants::LANG_ID),
            "uk" => Ok(constants::LANG_UK),
            "be" => Ok(constants::LANG_BE),
            "sl" => Ok(constants::LANG_SL),
            "et" => Ok(constants::LANG_ET),
            "lv" => Ok(constants::LANG_LV),
            "lt" => Ok(constants::LANG_LT),
            "tg" => Ok(constants::LANG_TG),
            "fa" => Ok(constants::LANG_FA),
            "vi" => Ok(constants::LANG_VI),
            "hy" => Ok(constants::LANG_HY),
            "az" => Ok(constants::LANG_AZ),
            "eu" => Ok(constants::LANG_EU),
            "hsb" => Ok(constants::LANG_HSB),
            "mk" => Ok(constants::LANG_MK),
            "st" => Ok(constants::LANG_ST),
            "ts" => Ok(constants::LANG_TS),
            "tn" => Ok(constants::LANG_TN),
            "ve" => Ok(constants::LANG_VE),
            "xh" => Ok(constants::LANG_XH),
            "zu" => Ok(constants::LANG_ZU),
            "af" => Ok(constants::LANG_AF),
            "ka" => Ok(constants::LANG_KA),
            "fo" => Ok(constants::LANG_FO),
            "hi" => Ok(constants::LANG_HI),
            "mt" => Ok(constants::LANG_MT),
            "se" => Ok(constants::LANG_SE),
            "ga" => Ok(constants::LANG_GA),
            "yi" => Err(Self::Error::Reserved("yi", 61)),
            "ms" => Ok(constants::LANG_MS),
            "kk" => Ok(constants::LANG_KK),
            "ky" => Ok(constants::LANG_KY),
            "sw" => Ok(constants::LANG_SW),
            "tk" => Ok(constants::LANG_TK),
            "uz" => Ok(constants::LANG_UZ),
            "tt" => Ok(constants::LANG_TT),
            "bn" => Ok(constants::LANG_BN),
            "pa" => Ok(constants::LANG_PA),
            "gu" => Ok(constants::LANG_GU),
            "or" => Ok(constants::LANG_OR),
            "ta" => Ok(constants::LANG_TA),
            "te" => Ok(constants::LANG_TE),
            "kn" => Ok(constants::LANG_KN),
            "ml" => Ok(constants::LANG_ML),
            "as" => Ok(constants::LANG_AS),
            "mr" => Ok(constants::LANG_MR),
            "sa" => Ok(constants::LANG_SA),
            "mn" => Ok(constants::LANG_MN),
            "bo" => Ok(constants::LANG_BO),
            "cy" => Ok(constants::LANG_CY),
            "km" => Ok(constants::LANG_KM),
            "lo" => Ok(constants::LANG_LO),
            "my" => Ok(constants::LANG_MY),
            "gl" => Ok(constants::LANG_GL),
            "kok" => Ok(constants::LANG_KOK),
            "mni" => Err(Self::Error::Reserved("mni", 88)),
            "sd" => Ok(constants::LANG_SD),
            "syr" => Ok(constants::LANG_SYR),
            "si" => Ok(constants::LANG_SI),
            "chr" => Ok(constants::LANG_CHR),
            "iu" => Ok(constants::LANG_IU),
            "am" => Ok(constants::LANG_AM),
            "tzm" => Ok(constants::LANG_TZM),
            "ks" => Ok(constants::LANG_KS),
            "ne" => Ok(constants::LANG_NE),
            "fy" => Ok(constants::LANG_FY),
            "ps" => Ok(constants::LANG_PS),
            "fil" => Ok(constants::LANG_FIL),
            "dv" => Ok(constants::LANG_DV),
            "bin" => Err(Self::Error::Reserved("bin", 102)),
            "ff" => Ok(constants::LANG_FF),
            "ha" => Ok(constants::LANG_HA),
            "ibb" => Err(Self::Error::Reserved("ibb", 105)),
            "yo" => Ok(constants::LANG_YO),
            "quz" => Ok(constants::LANG_QUZ),
            "nso" => Ok(constants::LANG_NSO),
            "ba" => Ok(constants::LANG_BA),
            "lb" => Ok(constants::LANG_LB),
            "kl" => Ok(constants::LANG_KL),
            "ig" => Ok(constants::LANG_IG),
            "kr" => Err(Self::Error::Reserved("kr", 113)),
            "om" => Ok(constants::LANG_OM),
            "ti" => Ok(constants::LANG_TI),
            "gn" => Ok(constants::LANG_GN),
            "haw" => Ok(constants::LANG_HAW),
            "la" => Err(Self::Error::Reserved("la", 118)),
            "so" => Err(Self::Error::Reserved("so", 119)),
            "ii" => Ok(constants::LANG_II),
            "pap" => Err(Self::Error::Reserved("pap", 121)),
            "arn" => Ok(constants::LANG_ARN),
            "moh" => Ok(constants::LANG_MOH),
            "br" => Ok(constants::LANG_BR),
            "" => Ok(constants::LANG_INVARIANT),
            "ug" => Ok(constants::LANG_UG),
            "mi" => Ok(constants::LANG_MI),
            "oc" => Ok(constants::LANG_OC),
            "co" => Ok(constants::LANG_CO),
            "gsw" => Ok(constants::LANG_GSW),
            "sah" => Ok(constants::LANG_SAH),
            "qut" => Ok(constants::LANG_QUT),
            "rw" => Ok(constants::LANG_RW),
            "wo" => Ok(constants::LANG_WO),
            "prs" => Ok(constants::LANG_PRS),
            "gd" => Ok(constants::LANG_GD),
            "ku" => Ok(constants::LANG_KU),
            "quc" => Err(Self::Error::Reserved("quc", 147)),
            "ar-SA" => Ok(constants::LANG_AR_SA),
            "bg-BG" => Ok(constants::LANG_BG_BG),
            "ca-ES" => Ok(constants::LANG_CA_ES),
            "zh-TW" => Ok(constants::LANG_ZH_TW),
            "cs-CZ" => Ok(constants::LANG_CS_CZ),
            "da-DK" => Ok(constants::LANG_DA_DK),
            "de-DE" => Ok(constants::LANG_DE_DE),
            "el-GR" => Ok(constants::LANG_EL_GR),
            "en-US" => Ok(constants::LANG_EN_US),
            "es-ES_tradnl" => Ok(constants::LANG_ES_ES_TRADNL),
            "fi-FI" => Ok(constants::LANG_FI_FI),
            "fr-FR" => Ok(constants::LANG_FR_FR),
            "he-IL" => Ok(constants::LANG_HE_IL),
            "hu-HU" => Ok(constants::LANG_HU_HU),
            "is-IS" => Ok(constants::LANG_IS_IS),
            "it-IT" => Ok(constants::LANG_IT_IT),
            "ja-JP" => Ok(constants::LANG_JA_JP),
            "ko-KR" => Ok(constants::LANG_KO_KR),
            "nl-NL" => Ok(constants::LANG_NL_NL),
            "nb-NO" => Ok(constants::LANG_NB_NO),
            "pl-PL" => Ok(constants::LANG_PL_PL),
            "pt-BR" => Ok(constants::LANG_PT_BR),
            "rm-CH" => Ok(constants::LANG_RM_CH),
            "ro-RO" => Ok(constants::LANG_RO_RO),
            "ru-RU" => Ok(constants::LANG_RU_RU),
            "hr-HR" => Ok(constants::LANG_HR_HR),
            "sk-SK" => Ok(constants::LANG_SK_SK),
            "sq-AL" => Ok(constants::LANG_SQ_AL),
            "sv-SE" => Ok(constants::LANG_SV_SE),
            "th-TH" => Ok(constants::LANG_TH_TH),
            "tr-TR" => Ok(constants::LANG_TR_TR),
            "ur-PK" => Ok(constants::LANG_UR_PK),
            "id-ID" => Ok(constants::LANG_ID_ID),
            "uk-UA" => Ok(constants::LANG_UK_UA),
            "be-BY" => Ok(constants::LANG_BE_BY),
            "sl-SI" => Ok(constants::LANG_SL_SI),
            "et-EE" => Ok(constants::LANG_ET_EE),
            "lv-LV" => Ok(constants::LANG_LV_LV),
            "lt-LT" => Ok(constants::LANG_LT_LT),
            "tg-Cyrl-TJ" => Ok(constants::LANG_TG_CYRL_TJ),
            "fa-IR" => Ok(constants::LANG_FA_IR),
            "vi-VN" => Ok(constants::LANG_VI_VN),
            "hy-AM" => Ok(constants::LANG_HY_AM),
            "az-Latn-AZ" => Ok(constants::LANG_AZ_LATN_AZ),
            "eu-ES" => Ok(constants::LANG_EU_ES),
            "hsb-DE" => Ok(constants::LANG_HSB_DE),
            "mk-MK" => Ok(constants::LANG_MK_MK),
            "st-ZA" => Ok(constants::LANG_ST_ZA),
            "ts-ZA" => Ok(constants::LANG_TS_ZA),
            "tn-ZA" => Ok(constants::LANG_TN_ZA),
            "ve-ZA" => Ok(constants::LANG_VE_ZA),
            "xh-ZA" => Ok(constants::LANG_XH_ZA),
            "zu-ZA" => Ok(constants::LANG_ZU_ZA),
            "af-ZA" => Ok(constants::LANG_AF_ZA),
            "ka-GE" => Ok(constants::LANG_KA_GE),
            "fo-FO" => Ok(constants::LANG_FO_FO),
            "hi-IN" => Ok(constants::LANG_HI_IN),
            "mt-MT" => Ok(constants::LANG_MT_MT),
            "se-NO" => Ok(constants::LANG_SE_NO),
            "yi-Hebr" => Err(Self::Error::Reserved("yi-Hebr", 1085)),
            "ms-MY" => Ok(constants::LANG_MS_MY),
            "kk-KZ" => Ok(constants::LANG_KK_KZ),
            "ky-KG" => Ok(constants::LANG_KY_KG),
            "sw-KE" => Ok(constants::LANG_SW_KE),
            "tk-TM" => Ok(constants::LANG_TK_TM),
            "uz-Latn-UZ" => Ok(constants::LANG_UZ_LATN_UZ),
            "tt-RU" => Ok(constants::LANG_TT_RU),
            "bn-IN" => Ok(constants::LANG_BN_IN),
            "pa-IN" => Ok(constants::LANG_PA_IN),
            "gu-IN" => Ok(constants::LANG_GU_IN),
            "or-IN" => Ok(constants::LANG_OR_IN),
            "ta-IN" => Ok(constants::LANG_TA_IN),
            "te-IN" => Ok(constants::LANG_TE_IN),
            "kn-IN" => Ok(constants::LANG_KN_IN),
            "ml-IN" => Ok(constants::LANG_ML_IN),
            "as-IN" => Ok(constants::LANG_AS_IN),
            "mr-IN" => Ok(constants::LANG_MR_IN),
            "sa-IN" => Ok(constants::LANG_SA_IN),
            "mn-MN" => Ok(constants::LANG_MN_MN),
            "bo-CN" => Ok(constants::LANG_BO_CN),
            "cy-GB" => Ok(constants::LANG_CY_GB),
            "km-KH" => Ok(constants::LANG_KM_KH),
            "lo-LA" => Ok(constants::LANG_LO_LA),
            "my-MM" => Ok(constants::LANG_MY_MM),
            "gl-ES" => Ok(constants::LANG_GL_ES),
            "kok-IN" => Ok(constants::LANG_KOK_IN),
            "mni-IN" => Err(Self::Error::Reserved("mni-IN", 1112)),
            "sd-Deva-IN" => Err(Self::Error::Reserved("sd-Deva-IN", 1113)),
            "syr-SY" => Ok(constants::LANG_SYR_SY),
            "si-LK" => Ok(constants::LANG_SI_LK),
            "chr-Cher-US" => Ok(constants::LANG_CHR_CHER_US),
            "iu-Cans-CA" => Ok(constants::LANG_IU_CANS_CA),
            "am-ET" => Ok(constants::LANG_AM_ET),
            "tzm-Arab-MA" => Err(Self::Error::Reserved("tzm-Arab-MA", 1119)),
            "ks-Arab" => Ok(constants::LANG_KS_ARAB),
            "ne-NP" => Ok(constants::LANG_NE_NP),
            "fy-NL" => Ok(constants::LANG_FY_NL),
            "ps-AF" => Ok(constants::LANG_PS_AF),
            "fil-PH" => Ok(constants::LANG_FIL_PH),
            "dv-MV" => Ok(constants::LANG_DV_MV),
            "bin-NG" => Err(Self::Error::Reserved("bin-NG", 1126)),
            "fuv-NG" => Err(Self::Error::Reserved("fuv-NG", 1127)),
            "ha-Latn-NG" => Ok(constants::LANG_HA_LATN_NG),
            "ibb-NG" => Err(Self::Error::Reserved("ibb-NG", 1129)),
            "yo-NG" => Ok(constants::LANG_YO_NG),
            "quz-BO" => Ok(constants::LANG_QUZ_BO),
            "nso-ZA" => Ok(constants::LANG_NSO_ZA),
            "ba-RU" => Ok(constants::LANG_BA_RU),
            "lb-LU" => Ok(constants::LANG_LB_LU),
            "kl-GL" => Ok(constants::LANG_KL_GL),
            "ig-NG" => Ok(constants::LANG_IG_NG),
            "kr-NG" => Err(Self::Error::Reserved("kr-NG", 1137)),
            "om-ET" => Ok(constants::LANG_OM_ET),
            "ti-ET" => Ok(constants::LANG_TI_ET),
            "gn-PY" => Ok(constants::LANG_GN_PY),
            "haw-US" => Ok(constants::LANG_HAW_US),
            "la-Latn" => Err(Self::Error::Reserved("la-Latn", 1142)),
            "so-SO" => Ok(constants::LANG_SO_SO),
            "ii-CN" => Ok(constants::LANG_II_CN),
            "pap-029" => Err(Self::Error::Reserved("pap-029", 1145)),
            "arn-CL" => Ok(constants::LANG_ARN_CL),
            "moh-CA" => Ok(constants::LANG_MOH_CA),
            "br-FR" => Ok(constants::LANG_BR_FR),
            "ug-CN" => Ok(constants::LANG_UG_CN),
            "mi-NZ" => Ok(constants::LANG_MI_NZ),
            "oc-FR" => Ok(constants::LANG_OC_FR),
            "co-FR" => Ok(constants::LANG_CO_FR),
            "gsw-FR" => Ok(constants::LANG_GSW_FR),
            "sah-RU" => Ok(constants::LANG_SAH_RU),
            "qut-GT" => Err(Self::Error::Reserved("qut-GT", 1158)),
            "rw-RW" => Ok(constants::LANG_RW_RW),
            "wo-SN" => Ok(constants::LANG_WO_SN),
            "prs-AF" => Ok(constants::LANG_PRS_AF),
            "plt-MG" => Err(Self::Error::Reserved("plt-MG", 1165)),
            "zh-yue-HK" => Err(Self::Error::Reserved("zh-yue-HK", 1166)),
            "tdd-Tale-CN" => Err(Self::Error::Reserved("tdd-Tale-CN", 1167)),
            "khb-Talu-CN" => Err(Self::Error::Reserved("khb-Talu-CN", 1168)),
            "gd-GB" => Ok(constants::LANG_GD_GB),
            "ku-Arab-IQ" => Ok(constants::LANG_KU_ARAB_IQ),
            "quc-CO" => Err(Self::Error::Reserved("quc-CO", 1171)),
            "qps-ploc" => Ok(constants::LANG_QPS_PLOC),
            "qps-ploca" => Ok(constants::LANG_QPS_PLOCA),
            "ar-IQ" => Ok(constants::LANG_AR_IQ),
            "ca-ES-valencia" => Ok(constants::LANG_CA_ES_VALENCIA),
            "zh-CN" => Ok(constants::LANG_ZH_CN),
            "de-CH" => Ok(constants::LANG_DE_CH),
            "en-GB" => Ok(constants::LANG_EN_GB),
            "es-MX" => Ok(constants::LANG_ES_MX),
            "fr-BE" => Ok(constants::LANG_FR_BE),
            "it-CH" => Ok(constants::LANG_IT_CH),
            "ja-Ploc-JP" => Err(Self::Error::Reserved("ja-Ploc-JP", 2065)),
            "nl-BE" => Ok(constants::LANG_NL_BE),
            "nn-NO" => Ok(constants::LANG_NN_NO),
            "pt-PT" => Ok(constants::LANG_PT_PT),
            "ro-MD" => Ok(constants::LANG_RO_MD),
            "ru-MD" => Ok(constants::LANG_RU_MD),
            "sr-Latn-CS" => Ok(constants::LANG_SR_LATN_CS),
            "sv-FI" => Ok(constants::LANG_SV_FI),
            "ur-IN" => Ok(constants::LANG_UR_IN),
            "az-Cyrl-AZ" => Err(Self::Error::Reserved("az-Cyrl-AZ", 2092)),
            "dsb-DE" => Ok(constants::LANG_DSB_DE),
            "tn-BW" => Ok(constants::LANG_TN_BW),
            "se-SE" => Ok(constants::LANG_SE_SE),
            "ga-IE" => Ok(constants::LANG_GA_IE),
            "ms-BN" => Ok(constants::LANG_MS_BN),
            "kk-Latn-KZ" => Err(Self::Error::Reserved("kk-Latn-KZ", 2111)),
            "uz-Cyrl-UZ" => Err(Self::Error::Reserved("uz-Cyrl-UZ", 2115)),
            "bn-BD" => Ok(constants::LANG_BN_BD),
            "pa-Arab-PK" => Ok(constants::LANG_PA_ARAB_PK),
            "ta-LK" => Ok(constants::LANG_TA_LK),
            "mn-Mong-CN" => Err(Self::Error::Reserved("mn-Mong-CN", 2128)),
            "bo-BT" => Err(Self::Error::Reserved("bo-BT", 2129)),
            "sd-Arab-PK" => Ok(constants::LANG_SD_ARAB_PK),
            "iu-Latn-CA" => Ok(constants::LANG_IU_LATN_CA),
            "tzm-Latn-DZ" => Ok(constants::LANG_TZM_LATN_DZ),
            "ks-Deva" => Err(Self::Error::Reserved("ks-Deva", 2144)),
            "ne-IN" => Ok(constants::LANG_NE_IN),
            "ff-Latn-SN" => Ok(constants::LANG_FF_LATN_SN),
            "quz-EC" => Ok(constants::LANG_QUZ_EC),
            "ti-ER" => Ok(constants::LANG_TI_ER),
            "qps-plocm" => Ok(constants::LANG_QPS_PLOCM),
            "ar-EG" => Ok(constants::LANG_AR_EG),
            "zh-HK" => Ok(constants::LANG_ZH_HK),
            "de-AT" => Ok(constants::LANG_DE_AT),
            "en-AU" => Ok(constants::LANG_EN_AU),
            "es-ES" => Ok(constants::LANG_ES_ES),
            "fr-CA" => Ok(constants::LANG_FR_CA),
            "sr-Cyrl-CS" => Ok(constants::LANG_SR_CYRL_CS),
            "se-FI" => Ok(constants::LANG_SE_FI),
            "mn-Mong-MN" => Ok(constants::LANG_MN_MONG_MN),
            "dz-BT" => Ok(constants::LANG_DZ_BT),
            "tmz-MA" => Err(Self::Error::Reserved("tmz-MA", 3167)),
            "quz-PE" => Ok(constants::LANG_QUZ_PE),
            "ar-LY" => Ok(constants::LANG_AR_LY),
            "zh-SG" => Ok(constants::LANG_ZH_SG),
            "de-LU" => Ok(constants::LANG_DE_LU),
            "en-CA" => Ok(constants::LANG_EN_CA),
            "es-GT" => Ok(constants::LANG_ES_GT),
            "fr-CH" => Ok(constants::LANG_FR_CH),
            "hr-BA" => Ok(constants::LANG_HR_BA),
            "smj-NO" => Ok(constants::LANG_SMJ_NO),
            "tzm-Tfng-MA" => Ok(constants::LANG_TZM_TFNG_MA),
            "ar-DZ" => Ok(constants::LANG_AR_DZ),
            "zh-MO" => Ok(constants::LANG_ZH_MO),
            "de-LI" => Ok(constants::LANG_DE_LI),
            "en-NZ" => Ok(constants::LANG_EN_NZ),
            "es-CR" => Ok(constants::LANG_ES_CR),
            "fr-LU" => Ok(constants::LANG_FR_LU),
            "bs-Latn-BA" => Ok(constants::LANG_BS_LATN_BA),
            "smj-SE" => Ok(constants::LANG_SMJ_SE),
            "ar-MA" => Ok(constants::LANG_AR_MA),
            "en-IE" => Ok(constants::LANG_EN_IE),
            "es-PA" => Ok(constants::LANG_ES_PA),
            "fr-MC" => Ok(constants::LANG_FR_MC),
            "sr-Latn-BA" => Ok(constants::LANG_SR_LATN_BA),
            "sma-NO" => Ok(constants::LANG_SMA_NO),
            "ar-TN" => Ok(constants::LANG_AR_TN),
            "en-ZA" => Ok(constants::LANG_EN_ZA),
            "es-DO" => Ok(constants::LANG_ES_DO),
            "sr-Cyrl-BA" => Ok(constants::LANG_SR_CYRL_BA),
            "sma-SE" => Ok(constants::LANG_SMA_SE),
            "ar-OM" => Ok(constants::LANG_AR_OM),
            "en-JM" => Ok(constants::LANG_EN_JM),
            "es-VE" => Ok(constants::LANG_ES_VE),
            "fr-RE" => Ok(constants::LANG_FR_RE),
            "bs-Cyrl-BA" => Ok(constants::LANG_BS_CYRL_BA),
            "sms-FI" => Ok(constants::LANG_SMS_FI),
            "ar-YE" => Ok(constants::LANG_AR_YE),
            "en-029" => Err(Self::Error::Reserved("en-029", 9225)),
            "es-CO" => Ok(constants::LANG_ES_CO),
            "fr-CD" => Ok(constants::LANG_FR_CD),
            "sr-Latn-RS" => Ok(constants::LANG_SR_LATN_RS),
            "smn-FI" => Ok(constants::LANG_SMN_FI),
            "ar-SY" => Ok(constants::LANG_AR_SY),
            "en-BZ" => Ok(constants::LANG_EN_BZ),
            "es-PE" => Ok(constants::LANG_ES_PE),
            "fr-SN" => Ok(constants::LANG_FR_SN),
            "sr-Cyrl-RS" => Ok(constants::LANG_SR_CYRL_RS),
            "ar-JO" => Ok(constants::LANG_AR_JO),
            "en-TT" => Ok(constants::LANG_EN_TT),
            "es-AR" => Ok(constants::LANG_ES_AR),
            "fr-CM" => Ok(constants::LANG_FR_CM),
            "sr-Latn-ME" => Ok(constants::LANG_SR_LATN_ME),
            "ar-LB" => Ok(constants::LANG_AR_LB),
            "en-ZW" => Ok(constants::LANG_EN_ZW),
            "es-EC" => Ok(constants::LANG_ES_EC),
            "fr-CI" => Ok(constants::LANG_FR_CI),
            "sr-Cyrl-ME" => Ok(constants::LANG_SR_CYRL_ME),
            "ar-KW" => Ok(constants::LANG_AR_KW),
            "en-PH" => Ok(constants::LANG_EN_PH),
            "es-CL" => Ok(constants::LANG_ES_CL),
            "fr-ML" => Ok(constants::LANG_FR_ML),
            "ar-AE" => Ok(constants::LANG_AR_AE),
            "en-ID" => Err(Self::Error::Reserved("en-ID", 14345)),
            "es-UY" => Ok(constants::LANG_ES_UY),
            "fr-MA" => Ok(constants::LANG_FR_MA),
            "ar-BH" => Ok(constants::LANG_AR_BH),
            "en-HK" => Ok(constants::LANG_EN_HK),
            "es-PY" => Ok(constants::LANG_ES_PY),
            "fr-HT" => Ok(constants::LANG_FR_HT),
            "ar-QA" => Ok(constants::LANG_AR_QA),
            "en-IN" => Ok(constants::LANG_EN_IN),
            "es-BO" => Ok(constants::LANG_ES_BO),
            "ar-Ploc-SA" => Err(Self::Error::Reserved("ar-Ploc-SA", 17409)),
            "en-MY" => Ok(constants::LANG_EN_MY),
            "es-SV" => Ok(constants::LANG_ES_SV),
            "ar-145" => Err(Self::Error::Reserved("ar-145", 18433)),
            "en-SG" => Ok(constants::LANG_EN_SG),
            "es-HN" => Ok(constants::LANG_ES_HN),
            "en-AE" => Ok(constants::LANG_EN_AE),
            "es-NI" => Ok(constants::LANG_ES_NI),
            "en-BH" => Err(Self::Error::Reserved("en-BH", 20489)),
            "es-PR" => Ok(constants::LANG_ES_PR),
            "en-EG" => Err(Self::Error::Reserved("en-EG", 21513)),
            "es-US" => Ok(constants::LANG_ES_US),
            "en-JO" => Err(Self::Error::Reserved("en-JO", 22537)),
            "es-419" => Err(Self::Error::Reserved("es-419", 22538)),
            "en-KW" => Err(Self::Error::Reserved("en-KW", 23561)),
            "es-CU" => Ok(constants::LANG_ES_CU),
            "en-TR" => Err(Self::Error::Reserved("en-TR", 24585)),
            "en-YE" => Err(Self::Error::Reserved("en-YE", 25609)),
            "bs-Cyrl" => Ok(constants::LANG_BS_CYRL),
            "bs-Latn" => Ok(constants::LANG_BS_LATN),
            "sr-Cyrl" => Ok(constants::LANG_SR_CYRL),
            "sr-Latn" => Ok(constants::LANG_SR_LATN),
            "smn" => Ok(constants::LANG_SMN),
            "az-Cyrl" => Ok(constants::LANG_AZ_CYRL),
            "sms" => Ok(constants::LANG_SMS),
            "zh" => Ok(constants::LANG_ZH),
            "nn" => Ok(constants::LANG_NN),
            "bs" => Ok(constants::LANG_BS),
            "az-Latn" => Ok(constants::LANG_AZ_LATN),
            "sma" => Ok(constants::LANG_SMA),
            "kk-Cyrl" => Err(Self::Error::Reserved("kk-Cyrl", 30783)),
            "uz-Cyrl" => Ok(constants::LANG_UZ_CYRL),
            "mn-Cyrl" => Ok(constants::LANG_MN_CYRL),
            "iu-Cans" => Ok(constants::LANG_IU_CANS),
            "tzm-Tfng" => Ok(constants::LANG_TZM_TFNG),
            "zh-Hant" => Ok(constants::LANG_ZH_HANT),
            "nb" => Ok(constants::LANG_NB),
            "sr" => Ok(constants::LANG_SR),
            "tg-Cyrl" => Ok(constants::LANG_TG_CYRL),
            "dsb" => Ok(constants::LANG_DSB),
            "smj" => Ok(constants::LANG_SMJ),
            "kk-Latn" => Err(Self::Error::Reserved("kk-Latn", 31807)),
            "uz-Latn" => Ok(constants::LANG_UZ_LATN),
            "pa-Arab" => Ok(constants::LANG_PA_ARAB),
            "mn-Mong" => Ok(constants::LANG_MN_MONG),
            "sd-Arab" => Ok(constants::LANG_SD_ARAB),
            "chr-Cher" => Ok(constants::LANG_CHR_CHER),
            "iu-Latn" => Ok(constants::LANG_IU_LATN),
            "tzm-Latn" => Ok(constants::LANG_TZM_LATN),
            "ff-Latn" => Ok(constants::LANG_FF_LATN),
            "ha-Latn" => Ok(constants::LANG_HA_LATN),
            "ku-Arab" => Ok(constants::LANG_KU_ARAB),
            "fr-015" => Err(Self::Error::Reserved("fr-015", 58380)),
            "aa" => Ok(constants::LANG_AA),
            "aa-DJ" => Ok(constants::LANG_AA_DJ),
            "aa-ER" => Ok(constants::LANG_AA_ER),
            "aa-ET" => Ok(constants::LANG_AA_ET),
            "af-NA" => Ok(constants::LANG_AF_NA),
            "agq" => Ok(constants::LANG_AGQ),
            "agq-CM" => Ok(constants::LANG_AGQ_CM),
            "ak" => Ok(constants::LANG_AK),
            "ak-GH" => Ok(constants::LANG_AK_GH),
            "ar-001" => Ok(constants::LANG_AR_001),
            "ar-DJ" => Ok(constants::LANG_AR_DJ),
            "ar-ER" => Ok(constants::LANG_AR_ER),
            "ar-IL" => Ok(constants::LANG_AR_IL),
            "ar-KM" => Ok(constants::LANG_AR_KM),
            "ar-MR" => Ok(constants::LANG_AR_MR),
            "ar-PS" => Ok(constants::LANG_AR_PS),
            "ar-SD" => Ok(constants::LANG_AR_SD),
            "ar-SO" => Ok(constants::LANG_AR_SO),
            "ar-SS" => Ok(constants::LANG_AR_SS),
            "ar-TD" => Ok(constants::LANG_AR_TD),
            "asa" => Ok(constants::LANG_ASA),
            "asa-TZ" => Ok(constants::LANG_ASA_TZ),
            "ast" => Ok(constants::LANG_AST),
            "ast-ES" => Ok(constants::LANG_AST_ES),
            "bas" => Ok(constants::LANG_BAS),
            "bas-CM" => Ok(constants::LANG_BAS_CM),
            "bem" => Ok(constants::LANG_BEM),
            "bem-ZM" => Ok(constants::LANG_BEM_ZM),
            "bez" => Ok(constants::LANG_BEZ),
            "bez-TZ" => Ok(constants::LANG_BEZ_TZ),
            "bm" => Ok(constants::LANG_BM),
            "bm-ML" => Ok(constants::LANG_BM_ML),
            "bo-IN" => Ok(constants::LANG_BO_IN),
            "brx" => Ok(constants::LANG_BRX),
            "brx-IN" => Ok(constants::LANG_BRX_IN),
            "byn" => Ok(constants::LANG_BYN),
            "byn-ER" => Ok(constants::LANG_BYN_ER),
            "ca-AD" => Ok(constants::LANG_CA_AD),
            "ca-FR" => Ok(constants::LANG_CA_FR),
            "ca-IT" => Ok(constants::LANG_CA_IT),
            "ccp" => Ok(constants::LANG_CCP),
            "ccp-Cakm" => Ok(constants::LANG_CCP_CAKM),
            "ccp-Cakm-BD" => Ok(constants::LANG_CCP_CAKM_BD),
            "ccp-Cakm-IN" => Ok(constants::LANG_CCP_CAKM_IN),
            "ce-RU" => Ok(constants::LANG_CE_RU),
            "ceb" => Ok(constants::LANG_CEB),
            "ceb-Latn" => Ok(constants::LANG_CEB_LATN),
            "ceb-Latn-PH" => Ok(constants::LANG_CEB_LATN_PH),
            "cgg" => Ok(constants::LANG_CGG),
            "cgg-UG" => Ok(constants::LANG_CGG_UG),
            "cu-RU" => Ok(constants::LANG_CU_RU),
            "da-GL" => Ok(constants::LANG_DA_GL),
            "dav" => Ok(constants::LANG_DAV),
            "dav-KE" => Ok(constants::LANG_DAV_KE),
            "de-BE" => Ok(constants::LANG_DE_BE),
            "de-IT" => Ok(constants::LANG_DE_IT),
            "dje" => Ok(constants::LANG_DJE),
            "dje-NE" => Ok(constants::LANG_DJE_NE),
            "dua" => Ok(constants::LANG_DUA),
            "dua-CM" => Ok(constants::LANG_DUA_CM),
            "dyo" => Ok(constants::LANG_DYO),
            "dyo-SN" => Ok(constants::LANG_DYO_SN),
            "dz" => Ok(constants::LANG_DZ),
            "ebu" => Ok(constants::LANG_EBU),
            "ebu-KE" => Ok(constants::LANG_EBU_KE),
            "ee" => Ok(constants::LANG_EE),
            "ee-GH" => Ok(constants::LANG_EE_GH),
            "ee-TG" => Ok(constants::LANG_EE_TG),
            "el-CY" => Ok(constants::LANG_EL_CY),
            "en-001" => Ok(constants::LANG_EN_001),
            "en-150" => Ok(constants::LANG_EN_150),
            "en-AG" => Ok(constants::LANG_EN_AG),
            "en-AI" => Ok(constants::LANG_EN_AI),
            "en-AS" => Ok(constants::LANG_EN_AS),
            "en-AT" => Ok(constants::LANG_EN_AT),
            "en-BB" => Ok(constants::LANG_EN_BB),
            "en-BE" => Ok(constants::LANG_EN_BE),
            "en-BI" => Ok(constants::LANG_EN_BI),
            "en-BM" => Ok(constants::LANG_EN_BM),
            "en-BS" => Ok(constants::LANG_EN_BS),
            "en-BW" => Ok(constants::LANG_EN_BW),
            "en-CC" => Ok(constants::LANG_EN_CC),
            "en-CH" => Ok(constants::LANG_EN_CH),
            "en-CK" => Ok(constants::LANG_EN_CK),
            "en-CM" => Ok(constants::LANG_EN_CM),
            "en-CX" => Ok(constants::LANG_EN_CX),
            "en-CY" => Ok(constants::LANG_EN_CY),
            "en-DE" => Ok(constants::LANG_EN_DE),
            "en-DK" => Ok(constants::LANG_EN_DK),
            "en-DM" => Ok(constants::LANG_EN_DM),
            "en-ER" => Ok(constants::LANG_EN_ER),
            "en-FI" => Ok(constants::LANG_EN_FI),
            "en-FJ" => Ok(constants::LANG_EN_FJ),
            "en-FK" => Ok(constants::LANG_EN_FK),
            "en-FM" => Ok(constants::LANG_EN_FM),
            "en-GD" => Ok(constants::LANG_EN_GD),
            "en-GG" => Ok(constants::LANG_EN_GG),
            "en-GH" => Ok(constants::LANG_EN_GH),
            "en-GI" => Ok(constants::LANG_EN_GI),
            "en-GM" => Ok(constants::LANG_EN_GM),
            "en-GU" => Ok(constants::LANG_EN_GU),
            "en-GY" => Ok(constants::LANG_EN_GY),
            "en-IL" => Ok(constants::LANG_EN_IL),
            "en-IM" => Ok(constants::LANG_EN_IM),
            "en-IO" => Ok(constants::LANG_EN_IO),
            "en-JE" => Ok(constants::LANG_EN_JE),
            "en-KE" => Ok(constants::LANG_EN_KE),
            "en-KI" => Ok(constants::LANG_EN_KI),
            "en-KN" => Ok(constants::LANG_EN_KN),
            "en-KY" => Ok(constants::LANG_EN_KY),
            "en-LC" => Ok(constants::LANG_EN_LC),
            "en-LR" => Ok(constants::LANG_EN_LR),
            "en-LS" => Ok(constants::LANG_EN_LS),
            "en-MG" => Ok(constants::LANG_EN_MG),
            "en-MH" => Ok(constants::LANG_EN_MH),
            "en-MO" => Ok(constants::LANG_EN_MO),
            "en-MP" => Ok(constants::LANG_EN_MP),
            "en-MS" => Ok(constants::LANG_EN_MS),
            "en-MT" => Ok(constants::LANG_EN_MT),
            "en-MU" => Ok(constants::LANG_EN_MU),
            "en-MW" => Ok(constants::LANG_EN_MW),
            "en-NA" => Ok(constants::LANG_EN_NA),
            "en-NF" => Ok(constants::LANG_EN_NF),
            "en-NG" => Ok(constants::LANG_EN_NG),
            "en-NL" => Ok(constants::LANG_EN_NL),
            "en-NR" => Ok(constants::LANG_EN_NR),
            "en-NU" => Ok(constants::LANG_EN_NU),
            "en-PG" => Ok(constants::LANG_EN_PG),
            "en-PK" => Ok(constants::LANG_EN_PK),
            "en-PN" => Ok(constants::LANG_EN_PN),
            "en-PR" => Ok(constants::LANG_EN_PR),
            "en-PW" => Ok(constants::LANG_EN_PW),
            "en-RW" => Ok(constants::LANG_EN_RW),
            "en-SB" => Ok(constants::LANG_EN_SB),
            "en-SC" => Ok(constants::LANG_EN_SC),
            "en-SD" => Ok(constants::LANG_EN_SD),
            "en-SE" => Ok(constants::LANG_EN_SE),
            "en-SH" => Ok(constants::LANG_EN_SH),
            "en-SI" => Ok(constants::LANG_EN_SI),
            "en-SL" => Ok(constants::LANG_EN_SL),
            "en-SS" => Ok(constants::LANG_EN_SS),
            "en-SX" => Ok(constants::LANG_EN_SX),
            "en-SZ" => Ok(constants::LANG_EN_SZ),
            "en-TC" => Ok(constants::LANG_EN_TC),
            "en-TK" => Ok(constants::LANG_EN_TK),
            "en-TO" => Ok(constants::LANG_EN_TO),
            "en-TV" => Ok(constants::LANG_EN_TV),
            "en-TZ" => Ok(constants::LANG_EN_TZ),
            "en-UG" => Ok(constants::LANG_EN_UG),
            "en-UM" => Ok(constants::LANG_EN_UM),
            "en-VC" => Ok(constants::LANG_EN_VC),
            "en-VG" => Ok(constants::LANG_EN_VG),
            "en-VI" => Ok(constants::LANG_EN_VI),
            "en-VU" => Ok(constants::LANG_EN_VU),
            "en-WS" => Ok(constants::LANG_EN_WS),
            "en-ZM" => Ok(constants::LANG_EN_ZM),
            "eo" => Ok(constants::LANG_EO),
            "eo-001" => Ok(constants::LANG_EO_001),
            "es-BR" => Ok(constants::LANG_ES_BR),
            "es-BZ" => Ok(constants::LANG_ES_BZ),
            "es-GQ" => Ok(constants::LANG_ES_GQ),
            "es-PH" => Ok(constants::LANG_ES_PH),
            "ewo" => Ok(constants::LANG_EWO),
            "ewo-CM" => Ok(constants::LANG_EWO_CM),
            "fa-AF" => Ok(constants::LANG_FA_AF),
            "ff-Latn-BF" => Ok(constants::LANG_FF_LATN_BF),
            "ff-CM" => Ok(constants::LANG_FF_CM),
            "ff-Latn-CM" => Ok(constants::LANG_FF_LATN_CM),
            "ff-Latn-GH" => Ok(constants::LANG_FF_LATN_GH),
            "ff-Latn-GM" => Ok(constants::LANG_FF_LATN_GM),
            "ff-GN" => Ok(constants::LANG_FF_GN),
            "ff-Latn-GN" => Ok(constants::LANG_FF_LATN_GN),
            "ff-Latn-GW" => Ok(constants::LANG_FF_LATN_GW),
            "ff-Latn-LR" => Ok(constants::LANG_FF_LATN_LR),
            "ff-MR" => Ok(constants::LANG_FF_MR),
            "ff-Latn-MR" => Ok(constants::LANG_FF_LATN_MR),
            "ff-Latn-NE" => Ok(constants::LANG_FF_LATN_NE),
            "ff-NG" => Ok(constants::LANG_FF_NG),
            "ff-Latn-NG" => Ok(constants::LANG_FF_LATN_NG),
            "ff-Latn-SL" => Ok(constants::LANG_FF_LATN_SL),
            "fo-DK" => Ok(constants::LANG_FO_DK),
            "fr-BF" => Ok(constants::LANG_FR_BF),
            "fr-BI" => Ok(constants::LANG_FR_BI),
            "fr-BJ" => Ok(constants::LANG_FR_BJ),
            "fr-BL" => Ok(constants::LANG_FR_BL),
            "fr-CF" => Ok(constants::LANG_FR_CF),
            "fr-CG" => Ok(constants::LANG_FR_CG),
            "fr-DJ" => Ok(constants::LANG_FR_DJ),
            "fr-DZ" => Ok(constants::LANG_FR_DZ),
            "fr-GA" => Ok(constants::LANG_FR_GA),
            "fr-GF" => Ok(constants::LANG_FR_GF),
            "fr-GN" => Ok(constants::LANG_FR_GN),
            "fr-GP" => Ok(constants::LANG_FR_GP),
            "fr-GQ" => Ok(constants::LANG_FR_GQ),
            "fr-KM" => Ok(constants::LANG_FR_KM),
            "fr-MF" => Ok(constants::LANG_FR_MF),
            "fr-MG" => Ok(constants::LANG_FR_MG),
            "fr-MQ" => Ok(constants::LANG_FR_MQ),
            "fr-MR" => Ok(constants::LANG_FR_MR),
            "fr-MU" => Ok(constants::LANG_FR_MU),
            "fr-NC" => Ok(constants::LANG_FR_NC),
            "fr-NE" => Ok(constants::LANG_FR_NE),
            "fr-PF" => Ok(constants::LANG_FR_PF),
            "fr-PM" => Ok(constants::LANG_FR_PM),
            "fr-RW" => Ok(constants::LANG_FR_RW),
            "fr-SC" => Ok(constants::LANG_FR_SC),
            "fr-SY" => Ok(constants::LANG_FR_SY),
            "fr-TD" => Ok(constants::LANG_FR_TD),
            "fr-TG" => Ok(constants::LANG_FR_TG),
            "fr-TN" => Ok(constants::LANG_FR_TN),
            "fr-VU" => Ok(constants::LANG_FR_VU),
            "fr-WF" => Ok(constants::LANG_FR_WF),
            "fr-YT" => Ok(constants::LANG_FR_YT),
            "fur" => Ok(constants::LANG_FUR),
            "fur-IT" => Ok(constants::LANG_FUR_IT),
            "gsw-CH" => Ok(constants::LANG_GSW_CH),
            "gsw-LI" => Ok(constants::LANG_GSW_LI),
            "guz" => Ok(constants::LANG_GUZ),
            "guz-KE" => Ok(constants::LANG_GUZ_KE),
            "gv" => Ok(constants::LANG_GV),
            "gv-IM" => Ok(constants::LANG_GV_IM),
            "ha-Latn-GH" => Ok(constants::LANG_HA_LATN_GH),
            "ha-Latn-NE" => Ok(constants::LANG_HA_LATN_NE),
            "ia" => Ok(constants::LANG_IA),
            "ia-001" => Ok(constants::LANG_IA_001),
            "ia-FR" => Ok(constants::LANG_IA_FR),
            "it-SM" => Ok(constants::LANG_IT_SM),
            "it-VA" => Ok(constants::LANG_IT_VA),
            "jgo" => Ok(constants::LANG_JGO),
            "jgo-CM" => Ok(constants::LANG_JGO_CM),
            "jmc" => Ok(constants::LANG_JMC),
            "jmc-TZ" => Ok(constants::LANG_JMC_TZ),
            "jv" => Ok(constants::LANG_JV),
            "jv-Latn" => Ok(constants::LANG_JV_LATN),
            "jv-Latn-ID" => Ok(constants::LANG_JV_LATN_ID),
            "kab" => Ok(constants::LANG_KAB),
            "kab-DZ" => Ok(constants::LANG_KAB_DZ),
            "kam" => Ok(constants::LANG_KAM),
            "kam-KE" => Ok(constants::LANG_KAM_KE),
            "kde" => Ok(constants::LANG_KDE),
            "kde-TZ" => Ok(constants::LANG_KDE_TZ),
            "kea" => Ok(constants::LANG_KEA),
            "kea-CV" => Ok(constants::LANG_KEA_CV),
            "khq" => Ok(constants::LANG_KHQ),
            "khq-ML" => Ok(constants::LANG_KHQ_ML),
            "ki" => Ok(constants::LANG_KI),
            "ki-KE" => Ok(constants::LANG_KI_KE),
            "kkj" => Ok(constants::LANG_KKJ),
            "kkj-CM" => Ok(constants::LANG_KKJ_CM),
            "kln" => Ok(constants::LANG_KLN),
            "kln-KE" => Ok(constants::LANG_KLN_KE),
            "ko-KP" => Ok(constants::LANG_KO_KP),
            "ks-Arab-IN" => Ok(constants::LANG_KS_ARAB_IN),
            "ksb" => Ok(constants::LANG_KSB),
            "ksb-TZ" => Ok(constants::LANG_KSB_TZ),
            "ksf" => Ok(constants::LANG_KSF),
            "ksf-CM" => Ok(constants::LANG_KSF_CM),
            "ksh" => Ok(constants::LANG_KSH),
            "ksh-DE" => Ok(constants::LANG_KSH_DE),
            "ku-Arab-IR" => Ok(constants::LANG_KU_ARAB_IR),
            "kw" => Ok(constants::LANG_KW),
            "kw-GB" => Ok(constants::LANG_KW_GB),
            "lag" => Ok(constants::LANG_LAG),
            "lag-TZ" => Ok(constants::LANG_LAG_TZ),
            "lg" => Ok(constants::LANG_LG),
            "lg-UG" => Ok(constants::LANG_LG_UG),
            "lkt" => Ok(constants::LANG_LKT),
            "lkt-US" => Ok(constants::LANG_LKT_US),
            "ln" => Ok(constants::LANG_LN),
            "ln-AO" => Ok(constants::LANG_LN_AO),
            "ln-CD" => Ok(constants::LANG_LN_CD),
            "ln-CF" => Ok(constants::LANG_LN_CF),
            "ln-CG" => Ok(constants::LANG_LN_CG),
            "lrc-IQ" => Ok(constants::LANG_LRC_IQ),
            "lrc-IR" => Ok(constants::LANG_LRC_IR),
            "lu" => Ok(constants::LANG_LU),
            "lu-CD" => Ok(constants::LANG_LU_CD),
            "luo" => Ok(constants::LANG_LUO),
            "luo-KE" => Ok(constants::LANG_LUO_KE),
            "luy" => Ok(constants::LANG_LUY),
            "luy-KE" => Ok(constants::LANG_LUY_KE),
            "mas" => Ok(constants::LANG_MAS),
            "mas-KE" => Ok(constants::LANG_MAS_KE),
            "mas-TZ" => Ok(constants::LANG_MAS_TZ),
            "mer" => Ok(constants::LANG_MER),
            "mer-KE" => Ok(constants::LANG_MER_KE),
            "mfe" => Ok(constants::LANG_MFE),
            "mfe-MU" => Ok(constants::LANG_MFE_MU),
            "mg" => Ok(constants::LANG_MG),
            "mgh" => Ok(constants::LANG_MGH),
            "mgh-MZ" => Ok(constants::LANG_MGH_MZ),
            "mg-MG" => Ok(constants::LANG_MG_MG),
            "mgo" => Ok(constants::LANG_MGO),
            "mgo-CM" => Ok(constants::LANG_MGO_CM),
            "mzn-IR" => Ok(constants::LANG_MZN_IR),
            "mua" => Ok(constants::LANG_MUA),
            "mua-CM" => Ok(constants::LANG_MUA_CM),
            "naq" => Ok(constants::LANG_NAQ),
            "naq-NA" => Ok(constants::LANG_NAQ_NA),
            "nb-SJ" => Ok(constants::LANG_NB_SJ),
            "nd" => Ok(constants::LANG_ND),
            "nd-ZW" => Ok(constants::LANG_ND_ZW),
            "nds" => Ok(constants::LANG_NDS),
            "nds-DE" => Ok(constants::LANG_NDS_DE),
            "nds-NL" => Ok(constants::LANG_NDS_NL),
            "ngo" => Ok(constants::LANG_NGO),
            "ngo-GN" => Ok(constants::LANG_NGO_GN),
            "nl-AW" => Ok(constants::LANG_NL_AW),
            "nl-BQ" => Ok(constants::LANG_NL_BQ),
            "nl-CW" => Ok(constants::LANG_NL_CW),
            "nl-SR" => Ok(constants::LANG_NL_SR),
            "nl-SX" => Ok(constants::LANG_NL_SX),
            "nmg" => Ok(constants::LANG_NMG),
            "nmg-CM" => Ok(constants::LANG_NMG_CM),
            "nnh" => Ok(constants::LANG_NNH),
            "nnh-CM" => Ok(constants::LANG_NNH_CM),
            "nr" => Ok(constants::LANG_NR),
            "nr-ZA" => Ok(constants::LANG_NR_ZA),
            "nus" => Ok(constants::LANG_NUS),
            "nus-SD" => Ok(constants::LANG_NUS_SD),
            "nus-SS" => Ok(constants::LANG_NUS_SS),
            "nyn" => Ok(constants::LANG_NYN),
            "nyn-UG" => Ok(constants::LANG_NYN_UG),
            "om-KE" => Ok(constants::LANG_OM_KE),
            "os" => Ok(constants::LANG_OS),
            "os-GE" => Ok(constants::LANG_OS_GE),
            "os-RU" => Ok(constants::LANG_OS_RU),
            "prg-001" => Ok(constants::LANG_PRG_001),
            "ps-PK" => Ok(constants::LANG_PS_PK),
            "pt-AO" => Ok(constants::LANG_PT_AO),
            "pt-CH" => Ok(constants::LANG_PT_CH),
            "pt-GQ" => Ok(constants::LANG_PT_GQ),
            "pt-LU" => Ok(constants::LANG_PT_LU),
            "pt-CV" => Ok(constants::LANG_PT_CV),
            "pt-GW" => Ok(constants::LANG_PT_GW),
            "pt-MO" => Ok(constants::LANG_PT_MO),
            "pt-MZ" => Ok(constants::LANG_PT_MZ),
            "pt-ST" => Ok(constants::LANG_PT_ST),
            "pt-TL" => Ok(constants::LANG_PT_TL),
            "rn" => Ok(constants::LANG_RN),
            "rn-BI" => Ok(constants::LANG_RN_BI),
            "rof" => Ok(constants::LANG_ROF),
            "rof-TZ" => Ok(constants::LANG_ROF_TZ),
            "ru-BY" => Ok(constants::LANG_RU_BY),
            "ru-KG" => Ok(constants::LANG_RU_KG),
            "ru-KZ" => Ok(constants::LANG_RU_KZ),
            "ru-UA" => Ok(constants::LANG_RU_UA),
            "rwk" => Ok(constants::LANG_RWK),
            "rwk-TZ" => Ok(constants::LANG_RWK_TZ),
            "saq" => Ok(constants::LANG_SAQ),
            "saq-KE" => Ok(constants::LANG_SAQ_KE),
            "sbp" => Ok(constants::LANG_SBP),
            "sbp-TZ" => Ok(constants::LANG_SBP_TZ),
            "seh" => Ok(constants::LANG_SEH),
            "seh-MZ" => Ok(constants::LANG_SEH_MZ),
            "ses" => Ok(constants::LANG_SES),
            "ses-ML" => Ok(constants::LANG_SES_ML),
            "sg" => Ok(constants::LANG_SG),
            "sg-CF" => Ok(constants::LANG_SG_CF),
            "shi" => Ok(constants::LANG_SHI),
            "shi-Latn" => Ok(constants::LANG_SHI_LATN),
            "shi-Latn-MA" => Ok(constants::LANG_SHI_LATN_MA),
            "shi-Tfng" => Ok(constants::LANG_SHI_TFNG),
            "shi-Tfng-MA" => Ok(constants::LANG_SHI_TFNG_MA),
            "sn" => Ok(constants::LANG_SN),
            "sn-Latn" => Ok(constants::LANG_SN_LATN),
            "sn-Latn-ZW" => Ok(constants::LANG_SN_LATN_ZW),
            "so-DJ" => Ok(constants::LANG_SO_DJ),
            "so-ET" => Ok(constants::LANG_SO_ET),
            "so-KE" => Ok(constants::LANG_SO_KE),
            "sq-MK" => Ok(constants::LANG_SQ_MK),
            "ss" => Ok(constants::LANG_SS),
            "ss-SZ" => Ok(constants::LANG_SS_SZ),
            "ssy" => Ok(constants::LANG_SSY),
            "ssy-ER" => Ok(constants::LANG_SSY_ER),
            "ss-ZA" => Ok(constants::LANG_SS_ZA),
            "st-LS" => Ok(constants::LANG_ST_LS),
            "sv-AX" => Ok(constants::LANG_SV_AX),
            "swc" => Ok(constants::LANG_SWC),
            "swc-CD" => Ok(constants::LANG_SWC_CD),
            "sw-TZ" => Ok(constants::LANG_SW_TZ),
            "sw-UG" => Ok(constants::LANG_SW_UG),
            "ta-MY" => Ok(constants::LANG_TA_MY),
            "ta-SG" => Ok(constants::LANG_TA_SG),
            "teo" => Ok(constants::LANG_TEO),
            "teo-KE" => Ok(constants::LANG_TEO_KE),
            "teo-UG" => Ok(constants::LANG_TEO_UG),
            "tig" => Ok(constants::LANG_TIG),
            "tig-ER" => Ok(constants::LANG_TIG_ER),
            "to" => Ok(constants::LANG_TO),
            "to-TO" => Ok(constants::LANG_TO_TO),
            "tr-CY" => Ok(constants::LANG_TR_CY),
            "twq" => Ok(constants::LANG_TWQ),
            "twq-NE" => Ok(constants::LANG_TWQ_NE),
            "tzm-Latn-MA" => Ok(constants::LANG_TZM_LATN_MA),
            "uz-Arab" => Ok(constants::LANG_UZ_ARAB),
            "uz-Arab-AF" => Ok(constants::LANG_UZ_ARAB_AF),
            "vai" => Ok(constants::LANG_VAI),
            "vai-Latn" => Ok(constants::LANG_VAI_LATN),
            "vai-Latn-LR" => Ok(constants::LANG_VAI_LATN_LR),
            "vai-Vaii" => Ok(constants::LANG_VAI_VAII),
            "vai-Vaii-LR" => Ok(constants::LANG_VAI_VAII_LR),
            "vo" => Ok(constants::LANG_VO),
            "vo-001" => Ok(constants::LANG_VO_001),
            "vun" => Ok(constants::LANG_VUN),
            "vun-TZ" => Ok(constants::LANG_VUN_TZ),
            "wae" => Ok(constants::LANG_WAE),
            "wae-CH" => Ok(constants::LANG_WAE_CH),
            "wal" => Ok(constants::LANG_WAL),
            "wal-ET" => Ok(constants::LANG_WAL_ET),
            "xog" => Ok(constants::LANG_XOG),
            "xog-UG" => Ok(constants::LANG_XOG_UG),
            "yav" => Ok(constants::LANG_YAV),
            "yav-CM" => Ok(constants::LANG_YAV_CM),
            "yo-BJ" => Ok(constants::LANG_YO_BJ),
            "zgh" => Ok(constants::LANG_ZGH),
            "zgh-Tfng" => Ok(constants::LANG_ZGH_TFNG),
            "zgh-Tfng-MA" => Ok(constants::LANG_ZGH_TFNG_MA),
            undef => Err(Self::Error::Undefined(undef.to_owned())),
        }
    };
}

pub mod lcid {
    //! Contains constant LCIDs corresponding to the full language information
    //! in the parent module, for easy use in e.g. match statements.
    /// Arabic
    pub const LCID_AR: u32 = 0x0001;
    /// Bulgarian
    pub const LCID_BG: u32 = 0x0002;
    /// Catalan
    pub const LCID_CA: u32 = 0x0003;
    /// Chinese (Simplified)
    pub const LCID_ZH_HANS: u32 = 0x0004;
    /// Czech
    pub const LCID_CS: u32 = 0x0005;
    /// Danish
    pub const LCID_DA: u32 = 0x0006;
    /// German
    pub const LCID_DE: u32 = 0x0007;
    /// Greek
    pub const LCID_EL: u32 = 0x0008;
    /// English
    pub const LCID_EN: u32 = 0x0009;
    /// Spanish
    pub const LCID_ES: u32 = 0x000A;
    /// Finnish
    pub const LCID_FI: u32 = 0x000B;
    /// French
    pub const LCID_FR: u32 = 0x000C;
    /// Hebrew
    pub const LCID_HE: u32 = 0x000D;
    /// Hungarian
    pub const LCID_HU: u32 = 0x000E;
    /// Icelandic
    pub const LCID_IS: u32 = 0x000F;
    /// Italian
    pub const LCID_IT: u32 = 0x0010;
    /// Japanese
    pub const LCID_JA: u32 = 0x0011;
    /// Korean
    pub const LCID_KO: u32 = 0x0012;
    /// Dutch
    pub const LCID_NL: u32 = 0x0013;
    /// Norwegian
    pub const LCID_NO: u32 = 0x0014;
    /// Polish
    pub const LCID_PL: u32 = 0x0015;
    /// Portuguese
    pub const LCID_PT: u32 = 0x0016;
    /// Romansh
    pub const LCID_RM: u32 = 0x0017;
    /// Romanian
    pub const LCID_RO: u32 = 0x0018;
    /// Russian
    pub const LCID_RU: u32 = 0x0019;
    /// Croatian
    pub const LCID_HR: u32 = 0x001A;
    /// Slovak
    pub const LCID_SK: u32 = 0x001B;
    /// Albanian
    pub const LCID_SQ: u32 = 0x001C;
    /// Swedish
    pub const LCID_SV: u32 = 0x001D;
    /// Thai
    pub const LCID_TH: u32 = 0x001E;
    /// Turkish
    pub const LCID_TR: u32 = 0x001F;
    /// Urdu
    pub const LCID_UR: u32 = 0x0020;
    /// Indonesian
    pub const LCID_ID: u32 = 0x0021;
    /// Ukrainian
    pub const LCID_UK: u32 = 0x0022;
    /// Belarusian
    pub const LCID_BE: u32 = 0x0023;
    /// Slovenian
    pub const LCID_SL: u32 = 0x0024;
    /// Estonian
    pub const LCID_ET: u32 = 0x0025;
    /// Latvian
    pub const LCID_LV: u32 = 0x0026;
    /// Lithuanian
    pub const LCID_LT: u32 = 0x0027;
    /// Tajik
    pub const LCID_TG: u32 = 0x0028;
    /// Persian
    pub const LCID_FA: u32 = 0x0029;
    /// Vietnamese
    pub const LCID_VI: u32 = 0x002A;
    /// Armenian
    pub const LCID_HY: u32 = 0x002B;
    /// Azerbaijani
    pub const LCID_AZ: u32 = 0x002C;
    /// Basque
    pub const LCID_EU: u32 = 0x002D;
    /// Upper Sorbian
    pub const LCID_HSB: u32 = 0x002E;
    /// Macedonian
    pub const LCID_MK: u32 = 0x002F;
    /// Sesotho
    pub const LCID_ST: u32 = 0x0030;
    /// Tsonga
    pub const LCID_TS: u32 = 0x0031;
    /// Setswana
    pub const LCID_TN: u32 = 0x0032;
    /// Venda
    pub const LCID_VE: u32 = 0x0033;
    /// isiXhosa
    pub const LCID_XH: u32 = 0x0034;
    /// isiZulu
    pub const LCID_ZU: u32 = 0x0035;
    /// Afrikaans
    pub const LCID_AF: u32 = 0x0036;
    /// Georgian
    pub const LCID_KA: u32 = 0x0037;
    /// Faroese
    pub const LCID_FO: u32 = 0x0038;
    /// Hindi
    pub const LCID_HI: u32 = 0x0039;
    /// Maltese
    pub const LCID_MT: u32 = 0x003A;
    /// Northern Sami
    pub const LCID_SE: u32 = 0x003B;
    /// Irish
    pub const LCID_GA: u32 = 0x003C;
    /// Malay
    pub const LCID_MS: u32 = 0x003E;
    /// Kazakh
    pub const LCID_KK: u32 = 0x003F;
    /// Kyrgyz
    pub const LCID_KY: u32 = 0x0040;
    /// Kiswahili
    pub const LCID_SW: u32 = 0x0041;
    /// Turkmen
    pub const LCID_TK: u32 = 0x0042;
    /// Uzbek
    pub const LCID_UZ: u32 = 0x0043;
    /// Tatar
    pub const LCID_TT: u32 = 0x0044;
    /// Bangla
    pub const LCID_BN: u32 = 0x0045;
    /// Punjabi
    pub const LCID_PA: u32 = 0x0046;
    /// Gujarati
    pub const LCID_GU: u32 = 0x0047;
    /// Odia
    pub const LCID_OR: u32 = 0x0048;
    /// Tamil
    pub const LCID_TA: u32 = 0x0049;
    /// Telugu
    pub const LCID_TE: u32 = 0x004A;
    /// Kannada
    pub const LCID_KN: u32 = 0x004B;
    /// Malayalam
    pub const LCID_ML: u32 = 0x004C;
    /// Assamese
    pub const LCID_AS: u32 = 0x004D;
    /// Marathi
    pub const LCID_MR: u32 = 0x004E;
    /// Sanskrit
    pub const LCID_SA: u32 = 0x004F;
    /// Mongolian
    pub const LCID_MN: u32 = 0x0050;
    /// Tibetan
    pub const LCID_BO: u32 = 0x0051;
    /// Welsh
    pub const LCID_CY: u32 = 0x0052;
    /// Khmer
    pub const LCID_KM: u32 = 0x0053;
    /// Lao
    pub const LCID_LO: u32 = 0x0054;
    /// Burmese
    pub const LCID_MY: u32 = 0x0055;
    /// Galician
    pub const LCID_GL: u32 = 0x0056;
    /// Konkani
    pub const LCID_KOK: u32 = 0x0057;
    /// Sindhi
    pub const LCID_SD: u32 = 0x0059;
    /// Syriac
    pub const LCID_SYR: u32 = 0x005A;
    /// Sinhala
    pub const LCID_SI: u32 = 0x005B;
    /// Cherokee
    pub const LCID_CHR: u32 = 0x005C;
    /// Inuktitut
    pub const LCID_IU: u32 = 0x005D;
    /// Amharic
    pub const LCID_AM: u32 = 0x005E;
    /// Central Atlas Tamazight
    pub const LCID_TZM: u32 = 0x005F;
    /// Kashmiri
    pub const LCID_KS: u32 = 0x0060;
    /// Nepali
    pub const LCID_NE: u32 = 0x0061;
    /// Western Frisian
    pub const LCID_FY: u32 = 0x0062;
    /// Pashto
    pub const LCID_PS: u32 = 0x0063;
    /// Filipino
    pub const LCID_FIL: u32 = 0x0064;
    /// Divehi
    pub const LCID_DV: u32 = 0x0065;
    /// Fulah
    pub const LCID_FF: u32 = 0x0067;
    /// Hausa
    pub const LCID_HA: u32 = 0x0068;
    /// Yoruba
    pub const LCID_YO: u32 = 0x006A;
    /// Quechua
    pub const LCID_QUZ: u32 = 0x006B;
    /// Sesotho sa Leboa
    pub const LCID_NSO: u32 = 0x006C;
    /// Bashkir
    pub const LCID_BA: u32 = 0x006D;
    /// Luxembourgish
    pub const LCID_LB: u32 = 0x006E;
    /// Greenlandic
    pub const LCID_KL: u32 = 0x006F;
    /// Igbo
    pub const LCID_IG: u32 = 0x0070;
    /// Oromo
    pub const LCID_OM: u32 = 0x0072;
    /// Tigrinya
    pub const LCID_TI: u32 = 0x0073;
    /// Guarani
    pub const LCID_GN: u32 = 0x0074;
    /// Hawaiian
    pub const LCID_HAW: u32 = 0x0075;
    /// Yi
    pub const LCID_II: u32 = 0x0078;
    /// Mapudungun
    pub const LCID_ARN: u32 = 0x007A;
    /// Mohawk
    pub const LCID_MOH: u32 = 0x007C;
    /// Breton
    pub const LCID_BR: u32 = 0x007E;
    /// Invariant Language (Invariant Country)
    pub const LCID_INVARIANT: u32 = 0x007F;
    /// Uyghur
    pub const LCID_UG: u32 = 0x0080;
    /// Maori
    pub const LCID_MI: u32 = 0x0081;
    /// Occitan
    pub const LCID_OC: u32 = 0x0082;
    /// Corsican
    pub const LCID_CO: u32 = 0x0083;
    /// Swiss German
    pub const LCID_GSW: u32 = 0x0084;
    /// Sakha
    pub const LCID_SAH: u32 = 0x0085;
    /// K'iche'
    pub const LCID_QUT: u32 = 0x0086;
    /// Kinyarwanda
    pub const LCID_RW: u32 = 0x0087;
    /// Wolof
    pub const LCID_WO: u32 = 0x0088;
    /// Dari
    pub const LCID_PRS: u32 = 0x008C;
    /// Scottish Gaelic
    pub const LCID_GD: u32 = 0x0091;
    /// Central Kurdish
    pub const LCID_KU: u32 = 0x0092;
    /// Arabic (Saudi Arabia)
    pub const LCID_AR_SA: u32 = 0x0401;
    /// Bulgarian (Bulgaria)
    pub const LCID_BG_BG: u32 = 0x0402;
    /// Catalan (Catalan)
    pub const LCID_CA_ES: u32 = 0x0403;
    /// Chinese (Traditional, Taiwan)
    pub const LCID_ZH_TW: u32 = 0x0404;
    /// Czech (Czechia)
    pub const LCID_CS_CZ: u32 = 0x0405;
    /// Danish (Denmark)
    pub const LCID_DA_DK: u32 = 0x0406;
    /// German (Germany)
    pub const LCID_DE_DE: u32 = 0x0407;
    /// Greek (Greece)
    pub const LCID_EL_GR: u32 = 0x0408;
    /// English (United States)
    pub const LCID_EN_US: u32 = 0x0409;
    /// Spanish (Spain, Traditional Sort)
    pub const LCID_ES_ES_TRADNL: u32 = 0x040A;
    /// Finnish (Finland)
    pub const LCID_FI_FI: u32 = 0x040B;
    /// French (France)
    pub const LCID_FR_FR: u32 = 0x040C;
    /// Hebrew (Israel)
    pub const LCID_HE_IL: u32 = 0x040D;
    /// Hungarian (Hungary)
    pub const LCID_HU_HU: u32 = 0x040E;
    /// Icelandic (Iceland)
    pub const LCID_IS_IS: u32 = 0x040F;
    /// Italian (Italy)
    pub const LCID_IT_IT: u32 = 0x0410;
    /// Japanese (Japan)
    pub const LCID_JA_JP: u32 = 0x0411;
    /// Korean (Korea)
    pub const LCID_KO_KR: u32 = 0x0412;
    /// Dutch (Netherlands)
    pub const LCID_NL_NL: u32 = 0x0413;
    /// Norwegian Bokmål (Norway)
    pub const LCID_NB_NO: u32 = 0x0414;
    /// Polish (Poland)
    pub const LCID_PL_PL: u32 = 0x0415;
    /// Portuguese (Brazil)
    pub const LCID_PT_BR: u32 = 0x0416;
    /// Romansh (Switzerland)
    pub const LCID_RM_CH: u32 = 0x0417;
    /// Romanian (Romania)
    pub const LCID_RO_RO: u32 = 0x0418;
    /// Russian (Russia)
    pub const LCID_RU_RU: u32 = 0x0419;
    /// Croatian (Croatia)
    pub const LCID_HR_HR: u32 = 0x041A;
    /// Slovak (Slovakia)
    pub const LCID_SK_SK: u32 = 0x041B;
    /// Albanian (Albania)
    pub const LCID_SQ_AL: u32 = 0x041C;
    /// Swedish (Sweden)
    pub const LCID_SV_SE: u32 = 0x041D;
    /// Thai (Thailand)
    pub const LCID_TH_TH: u32 = 0x041E;
    /// Turkish (Turkey)
    pub const LCID_TR_TR: u32 = 0x041F;
    /// Urdu (Pakistan)
    pub const LCID_UR_PK: u32 = 0x0420;
    /// Indonesian (Indonesia)
    pub const LCID_ID_ID: u32 = 0x0421;
    /// Ukrainian (Ukraine)
    pub const LCID_UK_UA: u32 = 0x0422;
    /// Belarusian (Belarus)
    pub const LCID_BE_BY: u32 = 0x0423;
    /// Slovenian (Slovenia)
    pub const LCID_SL_SI: u32 = 0x0424;
    /// Estonian (Estonia)
    pub const LCID_ET_EE: u32 = 0x0425;
    /// Latvian (Latvia)
    pub const LCID_LV_LV: u32 = 0x0426;
    /// Lithuanian (Lithuania)
    pub const LCID_LT_LT: u32 = 0x0427;
    /// Tajik (Cyrillic, Tajikistan)
    pub const LCID_TG_CYRL_TJ: u32 = 0x0428;
    /// Persian (Iran)
    pub const LCID_FA_IR: u32 = 0x0429;
    /// Vietnamese (Vietnam)
    pub const LCID_VI_VN: u32 = 0x042A;
    /// Armenian (Armenia)
    pub const LCID_HY_AM: u32 = 0x042B;
    /// Azerbaijani (Latin, Azerbaijan)
    pub const LCID_AZ_LATN_AZ: u32 = 0x042C;
    /// Basque (Basque)
    pub const LCID_EU_ES: u32 = 0x042D;
    /// Upper Sorbian (Germany)
    pub const LCID_HSB_DE: u32 = 0x042E;
    /// Macedonian (Macedonia, FYRO)
    pub const LCID_MK_MK: u32 = 0x042F;
    /// Sesotho (South Africa)
    pub const LCID_ST_ZA: u32 = 0x0430;
    /// Xitsonga (South Africa)
    pub const LCID_TS_ZA: u32 = 0x0431;
    /// Setswana (South Africa)
    pub const LCID_TN_ZA: u32 = 0x0432;
    /// Venda (South Africa)
    pub const LCID_VE_ZA: u32 = 0x0433;
    /// isiXhosa (South Africa)
    pub const LCID_XH_ZA: u32 = 0x0434;
    /// isiZulu (South Africa)
    pub const LCID_ZU_ZA: u32 = 0x0435;
    /// Afrikaans (South Africa)
    pub const LCID_AF_ZA: u32 = 0x0436;
    /// Georgian (Georgia)
    pub const LCID_KA_GE: u32 = 0x0437;
    /// Faroese (Faroe Islands)
    pub const LCID_FO_FO: u32 = 0x0438;
    /// Hindi (India)
    pub const LCID_HI_IN: u32 = 0x0439;
    /// Maltese (Malta)
    pub const LCID_MT_MT: u32 = 0x043A;
    /// Sami, Northern (Norway)
    pub const LCID_SE_NO: u32 = 0x043B;
    /// Malay (Malaysia)
    pub const LCID_MS_MY: u32 = 0x043E;
    /// Kazakh (Kazakhstan)
    pub const LCID_KK_KZ: u32 = 0x043F;
    /// Kyrgyz (Kyrgyzstan)
    pub const LCID_KY_KG: u32 = 0x0440;
    /// Kiswahili (Kenya)
    pub const LCID_SW_KE: u32 = 0x0441;
    /// Turkmen (Turkmenistan)
    pub const LCID_TK_TM: u32 = 0x0442;
    /// Uzbek (Latin, Uzbekistan)
    pub const LCID_UZ_LATN_UZ: u32 = 0x0443;
    /// Tatar (Russia)
    pub const LCID_TT_RU: u32 = 0x0444;
    /// Bangla (India)
    pub const LCID_BN_IN: u32 = 0x0445;
    /// Punjabi (India)
    pub const LCID_PA_IN: u32 = 0x0446;
    /// Gujarati (India)
    pub const LCID_GU_IN: u32 = 0x0447;
    /// Odia (India)
    pub const LCID_OR_IN: u32 = 0x0448;
    /// Tamil (India)
    pub const LCID_TA_IN: u32 = 0x0449;
    /// Telugu (India)
    pub const LCID_TE_IN: u32 = 0x044A;
    /// Kannada (India)
    pub const LCID_KN_IN: u32 = 0x044B;
    /// Malayalam (India)
    pub const LCID_ML_IN: u32 = 0x044C;
    /// Assamese (India)
    pub const LCID_AS_IN: u32 = 0x044D;
    /// Marathi (India)
    pub const LCID_MR_IN: u32 = 0x044E;
    /// Sanskrit (India)
    pub const LCID_SA_IN: u32 = 0x044F;
    /// Mongolian (Mongolia)
    pub const LCID_MN_MN: u32 = 0x0450;
    /// Tibetan (China)
    pub const LCID_BO_CN: u32 = 0x0451;
    /// Welsh (United Kingdom)
    pub const LCID_CY_GB: u32 = 0x0452;
    /// Khmer (Cambodia)
    pub const LCID_KM_KH: u32 = 0x0453;
    /// Lao (Laos)
    pub const LCID_LO_LA: u32 = 0x0454;
    /// Burmese (Myanmar)
    pub const LCID_MY_MM: u32 = 0x0455;
    /// Galician (Galician)
    pub const LCID_GL_ES: u32 = 0x0456;
    /// Konkani (India)
    pub const LCID_KOK_IN: u32 = 0x0457;
    /// Syriac (Syria)
    pub const LCID_SYR_SY: u32 = 0x045A;
    /// Sinhala (Sri Lanka)
    pub const LCID_SI_LK: u32 = 0x045B;
    /// Cherokee (Cherokee, United States)
    pub const LCID_CHR_CHER_US: u32 = 0x045C;
    /// Inuktitut (Syllabics, Canada)
    pub const LCID_IU_CANS_CA: u32 = 0x045D;
    /// Amharic (Ethiopia)
    pub const LCID_AM_ET: u32 = 0x045E;
    /// Kashmiri (Perso-Arabic)
    pub const LCID_KS_ARAB: u32 = 0x0460;
    /// Nepali (Nepal)
    pub const LCID_NE_NP: u32 = 0x0461;
    /// Western Frisian (Netherlands)
    pub const LCID_FY_NL: u32 = 0x0462;
    /// Pashto (Afghanistan)
    pub const LCID_PS_AF: u32 = 0x0463;
    /// Filipino (Philippines)
    pub const LCID_FIL_PH: u32 = 0x0464;
    /// Divehi (Maldives)
    pub const LCID_DV_MV: u32 = 0x0465;
    /// Hausa (Latin, Nigeria)
    pub const LCID_HA_LATN_NG: u32 = 0x0468;
    /// Yoruba (Nigeria)
    pub const LCID_YO_NG: u32 = 0x046A;
    /// Quechua (Bolivia)
    pub const LCID_QUZ_BO: u32 = 0x046B;
    /// Sesotho sa Leboa (South Africa)
    pub const LCID_NSO_ZA: u32 = 0x046C;
    /// Bashkir (Russia)
    pub const LCID_BA_RU: u32 = 0x046D;
    /// Luxembourgish (Luxembourg)
    pub const LCID_LB_LU: u32 = 0x046E;
    /// Greenlandic (Greenland)
    pub const LCID_KL_GL: u32 = 0x046F;
    /// Igbo (Nigeria)
    pub const LCID_IG_NG: u32 = 0x0470;
    /// Oromo (Ethiopia)
    pub const LCID_OM_ET: u32 = 0x0472;
    /// Tigrinya (Ethiopia)
    pub const LCID_TI_ET: u32 = 0x0473;
    /// Guarani (Paraguay)
    pub const LCID_GN_PY: u32 = 0x0474;
    /// Hawaiian (United States)
    pub const LCID_HAW_US: u32 = 0x0475;
    /// Somali (Somalia)
    pub const LCID_SO_SO: u32 = 0x0477;
    /// Yi (China)
    pub const LCID_II_CN: u32 = 0x0478;
    /// Mapudungun (Chile)
    pub const LCID_ARN_CL: u32 = 0x047A;
    /// Mohawk (Mohawk)
    pub const LCID_MOH_CA: u32 = 0x047C;
    /// Breton (France)
    pub const LCID_BR_FR: u32 = 0x047E;
    /// Uyghur (China)
    pub const LCID_UG_CN: u32 = 0x0480;
    /// Maori (New Zealand)
    pub const LCID_MI_NZ: u32 = 0x0481;
    /// Occitan (France)
    pub const LCID_OC_FR: u32 = 0x0482;
    /// Corsican (France)
    pub const LCID_CO_FR: u32 = 0x0483;
    /// Alsatian (France)
    pub const LCID_GSW_FR: u32 = 0x0484;
    /// Sakha (Russia)
    pub const LCID_SAH_RU: u32 = 0x0485;
    /// Kinyarwanda (Rwanda)
    pub const LCID_RW_RW: u32 = 0x0487;
    /// Wolof (Senegal)
    pub const LCID_WO_SN: u32 = 0x0488;
    /// Dari (Afghanistan)
    pub const LCID_PRS_AF: u32 = 0x048C;
    /// Scottish Gaelic (United Kingdom)
    pub const LCID_GD_GB: u32 = 0x0491;
    /// Central Kurdish (Iraq)
    pub const LCID_KU_ARAB_IQ: u32 = 0x0492;
    /// Pseudo (Pseudo)
    pub const LCID_QPS_PLOC: u32 = 0x0501;
    /// Pseudo (Pseudo Asia)
    pub const LCID_QPS_PLOCA: u32 = 0x05FE;
    /// Arabic (Iraq)
    pub const LCID_AR_IQ: u32 = 0x0801;
    /// Valencian (Spain)
    pub const LCID_CA_ES_VALENCIA: u32 = 0x0803;
    /// Chinese (Simplified, China)
    pub const LCID_ZH_CN: u32 = 0x0804;
    /// German (Switzerland)
    pub const LCID_DE_CH: u32 = 0x0807;
    /// English (United Kingdom)
    pub const LCID_EN_GB: u32 = 0x0809;
    /// Spanish (Mexico)
    pub const LCID_ES_MX: u32 = 0x080A;
    /// French (Belgium)
    pub const LCID_FR_BE: u32 = 0x080C;
    /// Italian (Switzerland)
    pub const LCID_IT_CH: u32 = 0x0810;
    /// Dutch (Belgium)
    pub const LCID_NL_BE: u32 = 0x0813;
    /// Norwegian Nynorsk (Norway)
    pub const LCID_NN_NO: u32 = 0x0814;
    /// Portuguese (Portugal)
    pub const LCID_PT_PT: u32 = 0x0816;
    /// Romanian (Moldova)
    pub const LCID_RO_MD: u32 = 0x0818;
    /// Russian (Moldova)
    pub const LCID_RU_MD: u32 = 0x0819;
    /// Serbian (Latin, Serbia and Montenegro (Former))
    pub const LCID_SR_LATN_CS: u32 = 0x081A;
    /// Swedish (Finland)
    pub const LCID_SV_FI: u32 = 0x081D;
    /// Urdu (India)
    pub const LCID_UR_IN: u32 = 0x0820;
    /// Lower Sorbian (Germany)
    pub const LCID_DSB_DE: u32 = 0x082E;
    /// Setswana (Botswana)
    pub const LCID_TN_BW: u32 = 0x0832;
    /// Sami, Northern (Sweden)
    pub const LCID_SE_SE: u32 = 0x083B;
    /// Irish (Ireland)
    pub const LCID_GA_IE: u32 = 0x083C;
    /// Malay (Brunei)
    pub const LCID_MS_BN: u32 = 0x083E;
    /// Bangla (Bangladesh)
    pub const LCID_BN_BD: u32 = 0x0845;
    /// Punjabi (Pakistan)
    pub const LCID_PA_ARAB_PK: u32 = 0x0846;
    /// Tamil (Sri Lanka)
    pub const LCID_TA_LK: u32 = 0x0849;
    /// Sindhi (Pakistan)
    pub const LCID_SD_ARAB_PK: u32 = 0x0859;
    /// Inuktitut (Latin, Canada)
    pub const LCID_IU_LATN_CA: u32 = 0x085D;
    /// Central Atlas Tamazight (Latin, Algeria)
    pub const LCID_TZM_LATN_DZ: u32 = 0x085F;
    /// Nepali (India)
    pub const LCID_NE_IN: u32 = 0x0861;
    /// Fulah (Latin, Senegal)
    pub const LCID_FF_LATN_SN: u32 = 0x0867;
    /// Quichua (Ecuador)
    pub const LCID_QUZ_EC: u32 = 0x086B;
    /// Tigrinya (Eritrea)
    pub const LCID_TI_ER: u32 = 0x0873;
    /// Pseudo (Pseudo Mirrored)
    pub const LCID_QPS_PLOCM: u32 = 0x09FF;
    /// Arabic (Egypt)
    pub const LCID_AR_EG: u32 = 0x0C01;
    /// Chinese (Traditional, Hong Kong SAR)
    pub const LCID_ZH_HK: u32 = 0x0C04;
    /// German (Austria)
    pub const LCID_DE_AT: u32 = 0x0C07;
    /// English (Australia)
    pub const LCID_EN_AU: u32 = 0x0C09;
    /// Spanish (Spain, International Sort)
    pub const LCID_ES_ES: u32 = 0x0C0A;
    /// French (Canada)
    pub const LCID_FR_CA: u32 = 0x0C0C;
    /// Serbian (Cyrillic, Serbia and Montenegro (Former))
    pub const LCID_SR_CYRL_CS: u32 = 0x0C1A;
    /// Sami, Northern (Finland)
    pub const LCID_SE_FI: u32 = 0x0C3B;
    /// Mongolian (Traditional Mongolian, Mongolia)
    pub const LCID_MN_MONG_MN: u32 = 0x0C50;
    /// Dzongkha (Bhutan)
    pub const LCID_DZ_BT: u32 = 0x0C51;
    /// Quechua (Peru)
    pub const LCID_QUZ_PE: u32 = 0x0C6B;
    /// Afar
    pub const LCID_AA: u32 = 0x1000;
    /// Afar (Djibouti)
    pub const LCID_AA_DJ: u32 = 0x1000;
    /// Afar (Eritrea)
    pub const LCID_AA_ER: u32 = 0x1000;
    /// Afar (Ethiopia)
    pub const LCID_AA_ET: u32 = 0x1000;
    /// Afrikaans (Namibia)
    pub const LCID_AF_NA: u32 = 0x1000;
    /// Aghem
    pub const LCID_AGQ: u32 = 0x1000;
    /// Aghem (Cameroon)
    pub const LCID_AGQ_CM: u32 = 0x1000;
    /// Akan
    pub const LCID_AK: u32 = 0x1000;
    /// Akan (Ghana)
    pub const LCID_AK_GH: u32 = 0x1000;
    /// Arabic (World)
    pub const LCID_AR_001: u32 = 0x1000;
    /// Arabic (Djibouti)
    pub const LCID_AR_DJ: u32 = 0x1000;
    /// Arabic (Eritrea)
    pub const LCID_AR_ER: u32 = 0x1000;
    /// Arabic (Israel)
    pub const LCID_AR_IL: u32 = 0x1000;
    /// Arabic (Comoros)
    pub const LCID_AR_KM: u32 = 0x1000;
    /// Arabic (Mauritania)
    pub const LCID_AR_MR: u32 = 0x1000;
    /// Arabic (Palestinian Authority)
    pub const LCID_AR_PS: u32 = 0x1000;
    /// Arabic (Sudan)
    pub const LCID_AR_SD: u32 = 0x1000;
    /// Arabic (Somalia)
    pub const LCID_AR_SO: u32 = 0x1000;
    /// Arabic (South Sudan)
    pub const LCID_AR_SS: u32 = 0x1000;
    /// Arabic (Chad)
    pub const LCID_AR_TD: u32 = 0x1000;
    /// Asu
    pub const LCID_ASA: u32 = 0x1000;
    /// Asu (Tanzania)
    pub const LCID_ASA_TZ: u32 = 0x1000;
    /// Asturian
    pub const LCID_AST: u32 = 0x1000;
    /// Asturian (Spain)
    pub const LCID_AST_ES: u32 = 0x1000;
    /// Basaa
    pub const LCID_BAS: u32 = 0x1000;
    /// Basaa (Cameroon)
    pub const LCID_BAS_CM: u32 = 0x1000;
    /// Bemba
    pub const LCID_BEM: u32 = 0x1000;
    /// Bemba (Zambia)
    pub const LCID_BEM_ZM: u32 = 0x1000;
    /// Bena
    pub const LCID_BEZ: u32 = 0x1000;
    /// Bena (Tanzania)
    pub const LCID_BEZ_TZ: u32 = 0x1000;
    /// Bamanankan
    pub const LCID_BM: u32 = 0x1000;
    /// Bamanankan (Latin, Mali)
    pub const LCID_BM_ML: u32 = 0x1000;
    /// Tibetan (India)
    pub const LCID_BO_IN: u32 = 0x1000;
    /// Bodo
    pub const LCID_BRX: u32 = 0x1000;
    /// Bodo (India)
    pub const LCID_BRX_IN: u32 = 0x1000;
    /// Blin
    pub const LCID_BYN: u32 = 0x1000;
    /// Blin (Eritrea)
    pub const LCID_BYN_ER: u32 = 0x1000;
    /// Catalan (Andorra)
    pub const LCID_CA_AD: u32 = 0x1000;
    /// Catalan (France)
    pub const LCID_CA_FR: u32 = 0x1000;
    /// Catalan (Italy)
    pub const LCID_CA_IT: u32 = 0x1000;
    /// Unknown Language (ccp)
    pub const LCID_CCP: u32 = 0x1000;
    /// Unknown Language (ccp-Cakm)
    pub const LCID_CCP_CAKM: u32 = 0x1000;
    /// Unknown Locale (ccp-Cakm-BD)
    pub const LCID_CCP_CAKM_BD: u32 = 0x1000;
    /// Unknown Locale (ccp-Cakm-IN)
    pub const LCID_CCP_CAKM_IN: u32 = 0x1000;
    /// Chechen (Russia)
    pub const LCID_CE_RU: u32 = 0x1000;
    /// Unknown Language (ceb)
    pub const LCID_CEB: u32 = 0x1000;
    /// Unknown Language (ceb-Latn)
    pub const LCID_CEB_LATN: u32 = 0x1000;
    /// Unknown Locale (ceb-Latn-PH)
    pub const LCID_CEB_LATN_PH: u32 = 0x1000;
    /// Chiga
    pub const LCID_CGG: u32 = 0x1000;
    /// Chiga (Uganda)
    pub const LCID_CGG_UG: u32 = 0x1000;
    /// Church Slavic (Russia)
    pub const LCID_CU_RU: u32 = 0x1000;
    /// Danish (Greenland)
    pub const LCID_DA_GL: u32 = 0x1000;
    /// Taita
    pub const LCID_DAV: u32 = 0x1000;
    /// Taita (Kenya)
    pub const LCID_DAV_KE: u32 = 0x1000;
    /// German (Belgium)
    pub const LCID_DE_BE: u32 = 0x1000;
    /// German (Italy)
    pub const LCID_DE_IT: u32 = 0x1000;
    /// Zarma
    pub const LCID_DJE: u32 = 0x1000;
    /// Zarma (Niger)
    pub const LCID_DJE_NE: u32 = 0x1000;
    /// Duala
    pub const LCID_DUA: u32 = 0x1000;
    /// Duala (Cameroon)
    pub const LCID_DUA_CM: u32 = 0x1000;
    /// Jola-Fonyi
    pub const LCID_DYO: u32 = 0x1000;
    /// Jola-Fonyi (Senegal)
    pub const LCID_DYO_SN: u32 = 0x1000;
    /// Dzongkha
    pub const LCID_DZ: u32 = 0x1000;
    /// Embu
    pub const LCID_EBU: u32 = 0x1000;
    /// Embu (Kenya)
    pub const LCID_EBU_KE: u32 = 0x1000;
    /// Ewe
    pub const LCID_EE: u32 = 0x1000;
    /// Ewe (Ghana)
    pub const LCID_EE_GH: u32 = 0x1000;
    /// Ewe (Togo)
    pub const LCID_EE_TG: u32 = 0x1000;
    /// Greek (Cyprus)
    pub const LCID_EL_CY: u32 = 0x1000;
    /// English (World)
    pub const LCID_EN_001: u32 = 0x1000;
    /// English (Europe)
    pub const LCID_EN_150: u32 = 0x1000;
    /// English (Antigua and Barbuda)
    pub const LCID_EN_AG: u32 = 0x1000;
    /// English (Anguilla)
    pub const LCID_EN_AI: u32 = 0x1000;
    /// English (American Samoa)
    pub const LCID_EN_AS: u32 = 0x1000;
    /// English (Austria)
    pub const LCID_EN_AT: u32 = 0x1000;
    /// English (Barbados)
    pub const LCID_EN_BB: u32 = 0x1000;
    /// English (Belgium)
    pub const LCID_EN_BE: u32 = 0x1000;
    /// English (Burundi)
    pub const LCID_EN_BI: u32 = 0x1000;
    /// English (Bermuda)
    pub const LCID_EN_BM: u32 = 0x1000;
    /// English (Bahamas)
    pub const LCID_EN_BS: u32 = 0x1000;
    /// English (Botswana)
    pub const LCID_EN_BW: u32 = 0x1000;
    /// English (Cocos (Keeling) Islands)
    pub const LCID_EN_CC: u32 = 0x1000;
    /// English (Switzerland)
    pub const LCID_EN_CH: u32 = 0x1000;
    /// English (Cook Islands)
    pub const LCID_EN_CK: u32 = 0x1000;
    /// English (Cameroon)
    pub const LCID_EN_CM: u32 = 0x1000;
    /// English (Christmas Island)
    pub const LCID_EN_CX: u32 = 0x1000;
    /// English (Cyprus)
    pub const LCID_EN_CY: u32 = 0x1000;
    /// English (Germany)
    pub const LCID_EN_DE: u32 = 0x1000;
    /// English (Denmark)
    pub const LCID_EN_DK: u32 = 0x1000;
    /// English (Dominica)
    pub const LCID_EN_DM: u32 = 0x1000;
    /// English (Eritrea)
    pub const LCID_EN_ER: u32 = 0x1000;
    /// English (Finland)
    pub const LCID_EN_FI: u32 = 0x1000;
    /// English (Fiji)
    pub const LCID_EN_FJ: u32 = 0x1000;
    /// English (Falkland Islands)
    pub const LCID_EN_FK: u32 = 0x1000;
    /// English (Micronesia)
    pub const LCID_EN_FM: u32 = 0x1000;
    /// English (Grenada)
    pub const LCID_EN_GD: u32 = 0x1000;
    /// English (Guernsey)
    pub const LCID_EN_GG: u32 = 0x1000;
    /// English (Ghana)
    pub const LCID_EN_GH: u32 = 0x1000;
    /// English (Gibraltar)
    pub const LCID_EN_GI: u32 = 0x1000;
    /// English (Gambia)
    pub const LCID_EN_GM: u32 = 0x1000;
    /// English (Guam)
    pub const LCID_EN_GU: u32 = 0x1000;
    /// English (Guyana)
    pub const LCID_EN_GY: u32 = 0x1000;
    /// English (Israel)
    pub const LCID_EN_IL: u32 = 0x1000;
    /// English (Isle of Man)
    pub const LCID_EN_IM: u32 = 0x1000;
    /// English (British Indian Ocean Territory)
    pub const LCID_EN_IO: u32 = 0x1000;
    /// English (Jersey)
    pub const LCID_EN_JE: u32 = 0x1000;
    /// English (Kenya)
    pub const LCID_EN_KE: u32 = 0x1000;
    /// English (Kiribati)
    pub const LCID_EN_KI: u32 = 0x1000;
    /// English (Saint Kitts and Nevis)
    pub const LCID_EN_KN: u32 = 0x1000;
    /// English (Cayman Islands)
    pub const LCID_EN_KY: u32 = 0x1000;
    /// English (Saint Lucia)
    pub const LCID_EN_LC: u32 = 0x1000;
    /// English (Liberia)
    pub const LCID_EN_LR: u32 = 0x1000;
    /// English (Lesotho)
    pub const LCID_EN_LS: u32 = 0x1000;
    /// English (Madagascar)
    pub const LCID_EN_MG: u32 = 0x1000;
    /// English (Marshall Islands)
    pub const LCID_EN_MH: u32 = 0x1000;
    /// English (Macao SAR)
    pub const LCID_EN_MO: u32 = 0x1000;
    /// English (Northern Mariana Islands)
    pub const LCID_EN_MP: u32 = 0x1000;
    /// English (Montserrat)
    pub const LCID_EN_MS: u32 = 0x1000;
    /// English (Malta)
    pub const LCID_EN_MT: u32 = 0x1000;
    /// English (Mauritius)
    pub const LCID_EN_MU: u32 = 0x1000;
    /// English (Malawi)
    pub const LCID_EN_MW: u32 = 0x1000;
    /// English (Namibia)
    pub const LCID_EN_NA: u32 = 0x1000;
    /// English (Norfolk Island)
    pub const LCID_EN_NF: u32 = 0x1000;
    /// English (Nigeria)
    pub const LCID_EN_NG: u32 = 0x1000;
    /// English (Netherlands)
    pub const LCID_EN_NL: u32 = 0x1000;
    /// English (Nauru)
    pub const LCID_EN_NR: u32 = 0x1000;
    /// English (Niue)
    pub const LCID_EN_NU: u32 = 0x1000;
    /// English (Papua New Guinea)
    pub const LCID_EN_PG: u32 = 0x1000;
    /// English (Pakistan)
    pub const LCID_EN_PK: u32 = 0x1000;
    /// English (Pitcairn Islands)
    pub const LCID_EN_PN: u32 = 0x1000;
    /// English (Puerto Rico)
    pub const LCID_EN_PR: u32 = 0x1000;
    /// English (Palau)
    pub const LCID_EN_PW: u32 = 0x1000;
    /// English (Rwanda)
    pub const LCID_EN_RW: u32 = 0x1000;
    /// English (Solomon Islands)
    pub const LCID_EN_SB: u32 = 0x1000;
    /// English (Seychelles)
    pub const LCID_EN_SC: u32 = 0x1000;
    /// English (Sudan)
    pub const LCID_EN_SD: u32 = 0x1000;
    /// English (Sweden)
    pub const LCID_EN_SE: u32 = 0x1000;
    /// English (St Helena, Ascension, Tristan da Cunha)
    pub const LCID_EN_SH: u32 = 0x1000;
    /// English (Slovenia)
    pub const LCID_EN_SI: u32 = 0x1000;
    /// English (Sierra Leone)
    pub const LCID_EN_SL: u32 = 0x1000;
    /// English (South Sudan)
    pub const LCID_EN_SS: u32 = 0x1000;
    /// English (Sint Maarten)
    pub const LCID_EN_SX: u32 = 0x1000;
    /// English (Swaziland)
    pub const LCID_EN_SZ: u32 = 0x1000;
    /// English (Turks and Caicos Islands)
    pub const LCID_EN_TC: u32 = 0x1000;
    /// English (Tokelau)
    pub const LCID_EN_TK: u32 = 0x1000;
    /// English (Tonga)
    pub const LCID_EN_TO: u32 = 0x1000;
    /// English (Tuvalu)
    pub const LCID_EN_TV: u32 = 0x1000;
    /// English (Tanzania)
    pub const LCID_EN_TZ: u32 = 0x1000;
    /// English (Uganda)
    pub const LCID_EN_UG: u32 = 0x1000;
    /// English (U.S. Outlying Islands)
    pub const LCID_EN_UM: u32 = 0x1000;
    /// English (Saint Vincent and the Grenadines)
    pub const LCID_EN_VC: u32 = 0x1000;
    /// English (British Virgin Islands)
    pub const LCID_EN_VG: u32 = 0x1000;
    /// English (U.S. Virgin Islands)
    pub const LCID_EN_VI: u32 = 0x1000;
    /// English (Vanuatu)
    pub const LCID_EN_VU: u32 = 0x1000;
    /// English (Samoa)
    pub const LCID_EN_WS: u32 = 0x1000;
    /// English (Zambia)
    pub const LCID_EN_ZM: u32 = 0x1000;
    /// Esperanto
    pub const LCID_EO: u32 = 0x1000;
    /// Esperanto (World)
    pub const LCID_EO_001: u32 = 0x1000;
    /// Spanish (Brazil)
    pub const LCID_ES_BR: u32 = 0x1000;
    /// Spanish (Belize)
    pub const LCID_ES_BZ: u32 = 0x1000;
    /// Spanish (Equatorial Guinea)
    pub const LCID_ES_GQ: u32 = 0x1000;
    /// Spanish (Philippines)
    pub const LCID_ES_PH: u32 = 0x1000;
    /// Ewondo
    pub const LCID_EWO: u32 = 0x1000;
    /// Ewondo (Cameroon)
    pub const LCID_EWO_CM: u32 = 0x1000;
    /// Dari (Afghanistan)
    pub const LCID_FA_AF: u32 = 0x1000;
    /// Fulah (Cameroon)
    pub const LCID_FF_CM: u32 = 0x1000;
    /// Fulah (Guinea)
    pub const LCID_FF_GN: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-BF)
    pub const LCID_FF_LATN_BF: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-CM)
    pub const LCID_FF_LATN_CM: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-GH)
    pub const LCID_FF_LATN_GH: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-GM)
    pub const LCID_FF_LATN_GM: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-GN)
    pub const LCID_FF_LATN_GN: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-GW)
    pub const LCID_FF_LATN_GW: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-LR)
    pub const LCID_FF_LATN_LR: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-MR)
    pub const LCID_FF_LATN_MR: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-NE)
    pub const LCID_FF_LATN_NE: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-NG)
    pub const LCID_FF_LATN_NG: u32 = 0x1000;
    /// Unknown Locale (ff-Latn-SL)
    pub const LCID_FF_LATN_SL: u32 = 0x1000;
    /// Fulah (Mauritania)
    pub const LCID_FF_MR: u32 = 0x1000;
    /// Fulah (Nigeria)
    pub const LCID_FF_NG: u32 = 0x1000;
    /// Faroese (Denmark)
    pub const LCID_FO_DK: u32 = 0x1000;
    /// French (Burkina Faso)
    pub const LCID_FR_BF: u32 = 0x1000;
    /// French (Burundi)
    pub const LCID_FR_BI: u32 = 0x1000;
    /// French (Benin)
    pub const LCID_FR_BJ: u32 = 0x1000;
    /// French (Saint Barthélemy)
    pub const LCID_FR_BL: u32 = 0x1000;
    /// French (Central African Republic)
    pub const LCID_FR_CF: u32 = 0x1000;
    /// French (Congo)
    pub const LCID_FR_CG: u32 = 0x1000;
    /// French (Djibouti)
    pub const LCID_FR_DJ: u32 = 0x1000;
    /// French (Algeria)
    pub const LCID_FR_DZ: u32 = 0x1000;
    /// French (Gabon)
    pub const LCID_FR_GA: u32 = 0x1000;
    /// French (French Guiana)
    pub const LCID_FR_GF: u32 = 0x1000;
    /// French (Guinea)
    pub const LCID_FR_GN: u32 = 0x1000;
    /// French (Guadeloupe)
    pub const LCID_FR_GP: u32 = 0x1000;
    /// French (Equatorial Guinea)
    pub const LCID_FR_GQ: u32 = 0x1000;
    /// French (Comoros)
    pub const LCID_FR_KM: u32 = 0x1000;
    /// French (Saint Martin)
    pub const LCID_FR_MF: u32 = 0x1000;
    /// French (Madagascar)
    pub const LCID_FR_MG: u32 = 0x1000;
    /// French (Martinique)
    pub const LCID_FR_MQ: u32 = 0x1000;
    /// French (Mauritania)
    pub const LCID_FR_MR: u32 = 0x1000;
    /// French (Mauritius)
    pub const LCID_FR_MU: u32 = 0x1000;
    /// French (New Caledonia)
    pub const LCID_FR_NC: u32 = 0x1000;
    /// French (Niger)
    pub const LCID_FR_NE: u32 = 0x1000;
    /// French (French Polynesia)
    pub const LCID_FR_PF: u32 = 0x1000;
    /// French (Saint Pierre and Miquelon)
    pub const LCID_FR_PM: u32 = 0x1000;
    /// French (Rwanda)
    pub const LCID_FR_RW: u32 = 0x1000;
    /// French (Seychelles)
    pub const LCID_FR_SC: u32 = 0x1000;
    /// French (Syria)
    pub const LCID_FR_SY: u32 = 0x1000;
    /// French (Chad)
    pub const LCID_FR_TD: u32 = 0x1000;
    /// French (Togo)
    pub const LCID_FR_TG: u32 = 0x1000;
    /// French (Tunisia)
    pub const LCID_FR_TN: u32 = 0x1000;
    /// French (Vanuatu)
    pub const LCID_FR_VU: u32 = 0x1000;
    /// French (Wallis and Futuna)
    pub const LCID_FR_WF: u32 = 0x1000;
    /// French (Mayotte)
    pub const LCID_FR_YT: u32 = 0x1000;
    /// Friulian
    pub const LCID_FUR: u32 = 0x1000;
    /// Friulian (Italy)
    pub const LCID_FUR_IT: u32 = 0x1000;
    /// Swiss German (Switzerland)
    pub const LCID_GSW_CH: u32 = 0x1000;
    /// Swiss German (Liechtenstein)
    pub const LCID_GSW_LI: u32 = 0x1000;
    /// Gusii
    pub const LCID_GUZ: u32 = 0x1000;
    /// Gusii (Kenya)
    pub const LCID_GUZ_KE: u32 = 0x1000;
    /// Manx
    pub const LCID_GV: u32 = 0x1000;
    /// Manx (Isle of Man)
    pub const LCID_GV_IM: u32 = 0x1000;
    /// Hausa (Latin, Ghana)
    pub const LCID_HA_LATN_GH: u32 = 0x1000;
    /// Hausa (Latin, Niger)
    pub const LCID_HA_LATN_NE: u32 = 0x1000;
    /// Interlingua
    pub const LCID_IA: u32 = 0x1000;
    /// Interlingua (World)
    pub const LCID_IA_001: u32 = 0x1000;
    /// Interlingua (France)
    pub const LCID_IA_FR: u32 = 0x1000;
    /// Italian (San Marino)
    pub const LCID_IT_SM: u32 = 0x1000;
    /// Italian (Vatican City)
    pub const LCID_IT_VA: u32 = 0x1000;
    /// Ngomba
    pub const LCID_JGO: u32 = 0x1000;
    /// Ngomba (Cameroon)
    pub const LCID_JGO_CM: u32 = 0x1000;
    /// Machame
    pub const LCID_JMC: u32 = 0x1000;
    /// Machame (Tanzania)
    pub const LCID_JMC_TZ: u32 = 0x1000;
    /// Javanese
    pub const LCID_JV: u32 = 0x1000;
    /// Javanese
    pub const LCID_JV_LATN: u32 = 0x1000;
    /// Javanese (Indonesia)
    pub const LCID_JV_LATN_ID: u32 = 0x1000;
    /// Kabyle
    pub const LCID_KAB: u32 = 0x1000;
    /// Kabyle (Algeria)
    pub const LCID_KAB_DZ: u32 = 0x1000;
    /// Kamba
    pub const LCID_KAM: u32 = 0x1000;
    /// Kamba (Kenya)
    pub const LCID_KAM_KE: u32 = 0x1000;
    /// Makonde
    pub const LCID_KDE: u32 = 0x1000;
    /// Makonde (Tanzania)
    pub const LCID_KDE_TZ: u32 = 0x1000;
    /// Kabuverdianu
    pub const LCID_KEA: u32 = 0x1000;
    /// Kabuverdianu (Cabo Verde)
    pub const LCID_KEA_CV: u32 = 0x1000;
    /// Koyra Chiini
    pub const LCID_KHQ: u32 = 0x1000;
    /// Koyra Chiini (Mali)
    pub const LCID_KHQ_ML: u32 = 0x1000;
    /// Kikuyu
    pub const LCID_KI: u32 = 0x1000;
    /// Kikuyu (Kenya)
    pub const LCID_KI_KE: u32 = 0x1000;
    /// Kako
    pub const LCID_KKJ: u32 = 0x1000;
    /// Kako (Cameroon)
    pub const LCID_KKJ_CM: u32 = 0x1000;
    /// Kalenjin
    pub const LCID_KLN: u32 = 0x1000;
    /// Kalenjin (Kenya)
    pub const LCID_KLN_KE: u32 = 0x1000;
    /// Korean (North Korea)
    pub const LCID_KO_KP: u32 = 0x1000;
    /// Kashmiri (Perso-Arabic)
    pub const LCID_KS_ARAB_IN: u32 = 0x1000;
    /// Shambala
    pub const LCID_KSB: u32 = 0x1000;
    /// Shambala (Tanzania)
    pub const LCID_KSB_TZ: u32 = 0x1000;
    /// Bafia
    pub const LCID_KSF: u32 = 0x1000;
    /// Bafia (Cameroon)
    pub const LCID_KSF_CM: u32 = 0x1000;
    /// Colognian
    pub const LCID_KSH: u32 = 0x1000;
    /// Colognian (Germany)
    pub const LCID_KSH_DE: u32 = 0x1000;
    /// Kurdish (Perso-Arabic, Iran)
    pub const LCID_KU_ARAB_IR: u32 = 0x1000;
    /// Cornish
    pub const LCID_KW: u32 = 0x1000;
    /// Cornish (United Kingdom)
    pub const LCID_KW_GB: u32 = 0x1000;
    /// Langi
    pub const LCID_LAG: u32 = 0x1000;
    /// Langi (Tanzania)
    pub const LCID_LAG_TZ: u32 = 0x1000;
    /// Ganda
    pub const LCID_LG: u32 = 0x1000;
    /// Ganda (Uganda)
    pub const LCID_LG_UG: u32 = 0x1000;
    /// Lakota
    pub const LCID_LKT: u32 = 0x1000;
    /// Lakota (United States)
    pub const LCID_LKT_US: u32 = 0x1000;
    /// Lingala
    pub const LCID_LN: u32 = 0x1000;
    /// Lingala (Angola)
    pub const LCID_LN_AO: u32 = 0x1000;
    /// Lingala (Congo DRC)
    pub const LCID_LN_CD: u32 = 0x1000;
    /// Lingala (Central African Republic)
    pub const LCID_LN_CF: u32 = 0x1000;
    /// Lingala (Congo)
    pub const LCID_LN_CG: u32 = 0x1000;
    /// Northern Luri (Iraq)
    pub const LCID_LRC_IQ: u32 = 0x1000;
    /// Northern Luri (Iran)
    pub const LCID_LRC_IR: u32 = 0x1000;
    /// Luba-Katanga
    pub const LCID_LU: u32 = 0x1000;
    /// Luba-Katanga (Congo DRC)
    pub const LCID_LU_CD: u32 = 0x1000;
    /// Luo
    pub const LCID_LUO: u32 = 0x1000;
    /// Luo (Kenya)
    pub const LCID_LUO_KE: u32 = 0x1000;
    /// Luyia
    pub const LCID_LUY: u32 = 0x1000;
    /// Luyia (Kenya)
    pub const LCID_LUY_KE: u32 = 0x1000;
    /// Masai
    pub const LCID_MAS: u32 = 0x1000;
    /// Masai (Kenya)
    pub const LCID_MAS_KE: u32 = 0x1000;
    /// Masai (Tanzania)
    pub const LCID_MAS_TZ: u32 = 0x1000;
    /// Meru
    pub const LCID_MER: u32 = 0x1000;
    /// Meru (Kenya)
    pub const LCID_MER_KE: u32 = 0x1000;
    /// Morisyen
    pub const LCID_MFE: u32 = 0x1000;
    /// Morisyen (Mauritius)
    pub const LCID_MFE_MU: u32 = 0x1000;
    /// Malagasy
    pub const LCID_MG: u32 = 0x1000;
    /// Malagasy (Madagascar)
    pub const LCID_MG_MG: u32 = 0x1000;
    /// Makhuwa-Meetto
    pub const LCID_MGH: u32 = 0x1000;
    /// Makhuwa-Meetto (Mozambique)
    pub const LCID_MGH_MZ: u32 = 0x1000;
    /// Metaʼ
    pub const LCID_MGO: u32 = 0x1000;
    /// Metaʼ (Cameroon)
    pub const LCID_MGO_CM: u32 = 0x1000;
    /// Mundang
    pub const LCID_MUA: u32 = 0x1000;
    /// Mundang (Cameroon)
    pub const LCID_MUA_CM: u32 = 0x1000;
    /// Mazanderani (Iran)
    pub const LCID_MZN_IR: u32 = 0x1000;
    /// Nama
    pub const LCID_NAQ: u32 = 0x1000;
    /// Nama (Namibia)
    pub const LCID_NAQ_NA: u32 = 0x1000;
    /// Norwegian Bokmål (Svalbard and Jan Mayen)
    pub const LCID_NB_SJ: u32 = 0x1000;
    /// North Ndebele
    pub const LCID_ND: u32 = 0x1000;
    /// North Ndebele (Zimbabwe)
    pub const LCID_ND_ZW: u32 = 0x1000;
    /// Low German
    pub const LCID_NDS: u32 = 0x1000;
    /// Low German (Germany)
    pub const LCID_NDS_DE: u32 = 0x1000;
    /// Low German (Netherlands)
    pub const LCID_NDS_NL: u32 = 0x1000;
    /// Unknown Language (ngo)
    pub const LCID_NGO: u32 = 0x1000;
    /// Unknown Locale (ngo-GN)
    pub const LCID_NGO_GN: u32 = 0x1000;
    /// Dutch (Aruba)
    pub const LCID_NL_AW: u32 = 0x1000;
    /// Dutch (Bonaire, Sint Eustatius and Saba)
    pub const LCID_NL_BQ: u32 = 0x1000;
    /// Dutch (Curaçao)
    pub const LCID_NL_CW: u32 = 0x1000;
    /// Dutch (Suriname)
    pub const LCID_NL_SR: u32 = 0x1000;
    /// Dutch (Sint Maarten)
    pub const LCID_NL_SX: u32 = 0x1000;
    /// Kwasio
    pub const LCID_NMG: u32 = 0x1000;
    /// Kwasio (Cameroon)
    pub const LCID_NMG_CM: u32 = 0x1000;
    /// Ngiemboon
    pub const LCID_NNH: u32 = 0x1000;
    /// Ngiemboon (Cameroon)
    pub const LCID_NNH_CM: u32 = 0x1000;
    /// South Ndebele
    pub const LCID_NR: u32 = 0x1000;
    /// South Ndebele (South Africa)
    pub const LCID_NR_ZA: u32 = 0x1000;
    /// Nuer
    pub const LCID_NUS: u32 = 0x1000;
    /// Nuer (South Sudan)
    pub const LCID_NUS_SD: u32 = 0x1000;
    /// Nuer (South Sudan)
    pub const LCID_NUS_SS: u32 = 0x1000;
    /// Nyankole
    pub const LCID_NYN: u32 = 0x1000;
    /// Nyankole (Uganda)
    pub const LCID_NYN_UG: u32 = 0x1000;
    /// Oromo (Kenya)
    pub const LCID_OM_KE: u32 = 0x1000;
    /// Ossetic
    pub const LCID_OS: u32 = 0x1000;
    /// Ossetic (Georgia)
    pub const LCID_OS_GE: u32 = 0x1000;
    /// Ossetic (Russia)
    pub const LCID_OS_RU: u32 = 0x1000;
    /// Prussian (World)
    pub const LCID_PRG_001: u32 = 0x1000;
    /// Unknown Locale (ps-PK)
    pub const LCID_PS_PK: u32 = 0x1000;
    /// Portuguese (Angola)
    pub const LCID_PT_AO: u32 = 0x1000;
    /// Portuguese (Switzerland)
    pub const LCID_PT_CH: u32 = 0x1000;
    /// Portuguese (Cabo Verde)
    pub const LCID_PT_CV: u32 = 0x1000;
    /// Portuguese (Equatorial Guinea)
    pub const LCID_PT_GQ: u32 = 0x1000;
    /// Portuguese (Guinea-Bissau)
    pub const LCID_PT_GW: u32 = 0x1000;
    /// Portuguese (Luxembourg)
    pub const LCID_PT_LU: u32 = 0x1000;
    /// Portuguese (Macao SAR)
    pub const LCID_PT_MO: u32 = 0x1000;
    /// Portuguese (Mozambique)
    pub const LCID_PT_MZ: u32 = 0x1000;
    /// Portuguese (São Tomé and Príncipe)
    pub const LCID_PT_ST: u32 = 0x1000;
    /// Portuguese (Timor-Leste)
    pub const LCID_PT_TL: u32 = 0x1000;
    /// Rundi
    pub const LCID_RN: u32 = 0x1000;
    /// Rundi (Burundi)
    pub const LCID_RN_BI: u32 = 0x1000;
    /// Rombo
    pub const LCID_ROF: u32 = 0x1000;
    /// Rombo (Tanzania)
    pub const LCID_ROF_TZ: u32 = 0x1000;
    /// Russian (Belarus)
    pub const LCID_RU_BY: u32 = 0x1000;
    /// Russian (Kyrgyzstan)
    pub const LCID_RU_KG: u32 = 0x1000;
    /// Russian (Kazakhstan)
    pub const LCID_RU_KZ: u32 = 0x1000;
    /// Russian (Ukraine)
    pub const LCID_RU_UA: u32 = 0x1000;
    /// Rwa
    pub const LCID_RWK: u32 = 0x1000;
    /// Rwa (Tanzania)
    pub const LCID_RWK_TZ: u32 = 0x1000;
    /// Samburu
    pub const LCID_SAQ: u32 = 0x1000;
    /// Samburu (Kenya)
    pub const LCID_SAQ_KE: u32 = 0x1000;
    /// Sangu
    pub const LCID_SBP: u32 = 0x1000;
    /// Sangu (Tanzania)
    pub const LCID_SBP_TZ: u32 = 0x1000;
    /// Sena
    pub const LCID_SEH: u32 = 0x1000;
    /// Sena (Mozambique)
    pub const LCID_SEH_MZ: u32 = 0x1000;
    /// Koyraboro Senni
    pub const LCID_SES: u32 = 0x1000;
    /// Koyraboro Senni (Mali)
    pub const LCID_SES_ML: u32 = 0x1000;
    /// Sango
    pub const LCID_SG: u32 = 0x1000;
    /// Sango (Central African Republic)
    pub const LCID_SG_CF: u32 = 0x1000;
    /// Tachelhit
    pub const LCID_SHI: u32 = 0x1000;
    /// Tachelhit (Latin)
    pub const LCID_SHI_LATN: u32 = 0x1000;
    /// Tachelhit (Latin, Morocco)
    pub const LCID_SHI_LATN_MA: u32 = 0x1000;
    /// Tachelhit (Tifinagh)
    pub const LCID_SHI_TFNG: u32 = 0x1000;
    /// Tachelhit (Tifinagh, Morocco)
    pub const LCID_SHI_TFNG_MA: u32 = 0x1000;
    /// Shona
    pub const LCID_SN: u32 = 0x1000;
    /// Shona (Latin)
    pub const LCID_SN_LATN: u32 = 0x1000;
    /// Shona (Latin, Zimbabwe)
    pub const LCID_SN_LATN_ZW: u32 = 0x1000;
    /// Somali (Djibouti)
    pub const LCID_SO_DJ: u32 = 0x1000;
    /// Somali (Ethiopia)
    pub const LCID_SO_ET: u32 = 0x1000;
    /// Somali (Kenya)
    pub const LCID_SO_KE: u32 = 0x1000;
    /// Albanian (Macedonia, FYRO)
    pub const LCID_SQ_MK: u32 = 0x1000;
    /// siSwati
    pub const LCID_SS: u32 = 0x1000;
    /// siSwati (Swaziland)
    pub const LCID_SS_SZ: u32 = 0x1000;
    /// siSwati (South Africa)
    pub const LCID_SS_ZA: u32 = 0x1000;
    /// Saho
    pub const LCID_SSY: u32 = 0x1000;
    /// Saho (Eritrea)
    pub const LCID_SSY_ER: u32 = 0x1000;
    /// Sesotho (Lesotho)
    pub const LCID_ST_LS: u32 = 0x1000;
    /// Swedish (Åland Islands)
    pub const LCID_SV_AX: u32 = 0x1000;
    /// Kiswahili (Tanzania)
    pub const LCID_SW_TZ: u32 = 0x1000;
    /// Kiswahili (Uganda)
    pub const LCID_SW_UG: u32 = 0x1000;
    /// Unknown Language (swc)
    pub const LCID_SWC: u32 = 0x1000;
    /// Kiswahili (Congo DRC)
    pub const LCID_SWC_CD: u32 = 0x1000;
    /// Tamil (Malaysia)
    pub const LCID_TA_MY: u32 = 0x1000;
    /// Tamil (Singapore)
    pub const LCID_TA_SG: u32 = 0x1000;
    /// Teso
    pub const LCID_TEO: u32 = 0x1000;
    /// Teso (Kenya)
    pub const LCID_TEO_KE: u32 = 0x1000;
    /// Teso (Uganda)
    pub const LCID_TEO_UG: u32 = 0x1000;
    /// Tigre
    pub const LCID_TIG: u32 = 0x1000;
    /// Tigre (Eritrea)
    pub const LCID_TIG_ER: u32 = 0x1000;
    /// Tongan
    pub const LCID_TO: u32 = 0x1000;
    /// Tongan (Tonga)
    pub const LCID_TO_TO: u32 = 0x1000;
    /// Turkish (Cyprus)
    pub const LCID_TR_CY: u32 = 0x1000;
    /// Tasawaq
    pub const LCID_TWQ: u32 = 0x1000;
    /// Tasawaq (Niger)
    pub const LCID_TWQ_NE: u32 = 0x1000;
    /// Central Atlas Tamazight (Latin, Morocco)
    pub const LCID_TZM_LATN_MA: u32 = 0x1000;
    /// Uzbek (Perso-Arabic)
    pub const LCID_UZ_ARAB: u32 = 0x1000;
    /// Uzbek (Perso-Arabic, Afghanistan)
    pub const LCID_UZ_ARAB_AF: u32 = 0x1000;
    /// Vai
    pub const LCID_VAI: u32 = 0x1000;
    /// Vai (Latin)
    pub const LCID_VAI_LATN: u32 = 0x1000;
    /// Vai (Latin, Liberia)
    pub const LCID_VAI_LATN_LR: u32 = 0x1000;
    /// Vai (Vai)
    pub const LCID_VAI_VAII: u32 = 0x1000;
    /// Vai (Vai, Liberia)
    pub const LCID_VAI_VAII_LR: u32 = 0x1000;
    /// Volapük
    pub const LCID_VO: u32 = 0x1000;
    /// Volapük (World)
    pub const LCID_VO_001: u32 = 0x1000;
    /// Vunjo
    pub const LCID_VUN: u32 = 0x1000;
    /// Vunjo (Tanzania)
    pub const LCID_VUN_TZ: u32 = 0x1000;
    /// Walser
    pub const LCID_WAE: u32 = 0x1000;
    /// Walser (Switzerland)
    pub const LCID_WAE_CH: u32 = 0x1000;
    /// Wolaytta
    pub const LCID_WAL: u32 = 0x1000;
    /// Wolaytta (Ethiopia)
    pub const LCID_WAL_ET: u32 = 0x1000;
    /// Soga
    pub const LCID_XOG: u32 = 0x1000;
    /// Soga (Uganda)
    pub const LCID_XOG_UG: u32 = 0x1000;
    /// Yangben
    pub const LCID_YAV: u32 = 0x1000;
    /// Yangben (Cameroon)
    pub const LCID_YAV_CM: u32 = 0x1000;
    /// Yoruba (Benin)
    pub const LCID_YO_BJ: u32 = 0x1000;
    /// Standard Moroccan Tamazight
    pub const LCID_ZGH: u32 = 0x1000;
    /// Standard Moroccan Tamazight (Tifinagh)
    pub const LCID_ZGH_TFNG: u32 = 0x1000;
    /// Standard Moroccan Tamazight (Tifinagh, Morocco)
    pub const LCID_ZGH_TFNG_MA: u32 = 0x1000;
    /// Arabic (Libya)
    pub const LCID_AR_LY: u32 = 0x1001;
    /// Chinese (Simplified, Singapore)
    pub const LCID_ZH_SG: u32 = 0x1004;
    /// German (Luxembourg)
    pub const LCID_DE_LU: u32 = 0x1007;
    /// English (Canada)
    pub const LCID_EN_CA: u32 = 0x1009;
    /// Spanish (Guatemala)
    pub const LCID_ES_GT: u32 = 0x100A;
    /// French (Switzerland)
    pub const LCID_FR_CH: u32 = 0x100C;
    /// Croatian (Bosnia and Herzegovina)
    pub const LCID_HR_BA: u32 = 0x101A;
    /// Sami, Lule (Norway)
    pub const LCID_SMJ_NO: u32 = 0x103B;
    /// Central Atlas Tamazight (Tifinagh, Morocco)
    pub const LCID_TZM_TFNG_MA: u32 = 0x105F;
    /// Arabic (Algeria)
    pub const LCID_AR_DZ: u32 = 0x1401;
    /// Chinese (Traditional, Macao SAR)
    pub const LCID_ZH_MO: u32 = 0x1404;
    /// German (Liechtenstein)
    pub const LCID_DE_LI: u32 = 0x1407;
    /// English (New Zealand)
    pub const LCID_EN_NZ: u32 = 0x1409;
    /// Spanish (Costa Rica)
    pub const LCID_ES_CR: u32 = 0x140A;
    /// French (Luxembourg)
    pub const LCID_FR_LU: u32 = 0x140C;
    /// Bosnian (Latin, Bosnia and Herzegovina)
    pub const LCID_BS_LATN_BA: u32 = 0x141A;
    /// Sami, Lule (Sweden)
    pub const LCID_SMJ_SE: u32 = 0x143B;
    /// Arabic (Morocco)
    pub const LCID_AR_MA: u32 = 0x1801;
    /// English (Ireland)
    pub const LCID_EN_IE: u32 = 0x1809;
    /// Spanish (Panama)
    pub const LCID_ES_PA: u32 = 0x180A;
    /// French (Monaco)
    pub const LCID_FR_MC: u32 = 0x180C;
    /// Serbian (Latin, Bosnia and Herzegovina)
    pub const LCID_SR_LATN_BA: u32 = 0x181A;
    /// Sami, Southern (Norway)
    pub const LCID_SMA_NO: u32 = 0x183B;
    /// Arabic (Tunisia)
    pub const LCID_AR_TN: u32 = 0x1C01;
    /// English (South Africa)
    pub const LCID_EN_ZA: u32 = 0x1C09;
    /// Spanish (Dominican Republic)
    pub const LCID_ES_DO: u32 = 0x1C0A;
    /// Serbian (Cyrillic, Bosnia and Herzegovina)
    pub const LCID_SR_CYRL_BA: u32 = 0x1C1A;
    /// Sami, Southern (Sweden)
    pub const LCID_SMA_SE: u32 = 0x1C3B;
    /// Arabic (Oman)
    pub const LCID_AR_OM: u32 = 0x2001;
    /// English (Jamaica)
    pub const LCID_EN_JM: u32 = 0x2009;
    /// Spanish (Venezuela)
    pub const LCID_ES_VE: u32 = 0x200A;
    /// French (Réunion)
    pub const LCID_FR_RE: u32 = 0x200C;
    /// Bosnian (Cyrillic, Bosnia and Herzegovina)
    pub const LCID_BS_CYRL_BA: u32 = 0x201A;
    /// Sami, Skolt (Finland)
    pub const LCID_SMS_FI: u32 = 0x203B;
    /// Arabic (Yemen)
    pub const LCID_AR_YE: u32 = 0x2401;
    /// Spanish (Colombia)
    pub const LCID_ES_CO: u32 = 0x240A;
    /// French Congo (DRC)
    pub const LCID_FR_CD: u32 = 0x240C;
    /// Serbian (Latin, Serbia)
    pub const LCID_SR_LATN_RS: u32 = 0x241A;
    /// Sami, Inari (Finland)
    pub const LCID_SMN_FI: u32 = 0x243B;
    /// Arabic (Syria)
    pub const LCID_AR_SY: u32 = 0x2801;
    /// English (Belize)
    pub const LCID_EN_BZ: u32 = 0x2809;
    /// Spanish (Peru)
    pub const LCID_ES_PE: u32 = 0x280A;
    /// French (Senegal)
    pub const LCID_FR_SN: u32 = 0x280C;
    /// Serbian (Cyrillic, Serbia)
    pub const LCID_SR_CYRL_RS: u32 = 0x281A;
    /// Arabic (Jordan)
    pub const LCID_AR_JO: u32 = 0x2C01;
    /// English (Trinidad and Tobago)
    pub const LCID_EN_TT: u32 = 0x2C09;
    /// Spanish (Argentina)
    pub const LCID_ES_AR: u32 = 0x2C0A;
    /// French (Cameroon)
    pub const LCID_FR_CM: u32 = 0x2C0C;
    /// Serbian (Latin, Montenegro)
    pub const LCID_SR_LATN_ME: u32 = 0x2C1A;
    /// Arabic (Lebanon)
    pub const LCID_AR_LB: u32 = 0x3001;
    /// English (Zimbabwe)
    pub const LCID_EN_ZW: u32 = 0x3009;
    /// Spanish (Ecuador)
    pub const LCID_ES_EC: u32 = 0x300A;
    /// French (Côte d’Ivoire)
    pub const LCID_FR_CI: u32 = 0x300C;
    /// Serbian (Cyrillic, Montenegro)
    pub const LCID_SR_CYRL_ME: u32 = 0x301A;
    /// Arabic (Kuwait)
    pub const LCID_AR_KW: u32 = 0x3401;
    /// English (Philippines)
    pub const LCID_EN_PH: u32 = 0x3409;
    /// Spanish (Chile)
    pub const LCID_ES_CL: u32 = 0x340A;
    /// French (Mali)
    pub const LCID_FR_ML: u32 = 0x340C;
    /// Arabic (United Arab Emirates)
    pub const LCID_AR_AE: u32 = 0x3801;
    /// Spanish (Uruguay)
    pub const LCID_ES_UY: u32 = 0x380A;
    /// French (Morocco)
    pub const LCID_FR_MA: u32 = 0x380C;
    /// Arabic (Bahrain)
    pub const LCID_AR_BH: u32 = 0x3C01;
    /// English (Hong Kong SAR)
    pub const LCID_EN_HK: u32 = 0x3C09;
    /// Spanish (Paraguay)
    pub const LCID_ES_PY: u32 = 0x3C0A;
    /// French (Haiti)
    pub const LCID_FR_HT: u32 = 0x3C0C;
    /// Arabic (Qatar)
    pub const LCID_AR_QA: u32 = 0x4001;
    /// English (India)
    pub const LCID_EN_IN: u32 = 0x4009;
    /// Spanish (Bolivia)
    pub const LCID_ES_BO: u32 = 0x400A;
    /// English (Malaysia)
    pub const LCID_EN_MY: u32 = 0x4409;
    /// Spanish (El Salvador)
    pub const LCID_ES_SV: u32 = 0x440A;
    /// English (Singapore)
    pub const LCID_EN_SG: u32 = 0x4809;
    /// Spanish (Honduras)
    pub const LCID_ES_HN: u32 = 0x480A;
    /// English (United Arab Emirates)
    pub const LCID_EN_AE: u32 = 0x4C09;
    /// Spanish (Nicaragua)
    pub const LCID_ES_NI: u32 = 0x4C0A;
    /// Spanish (Puerto Rico)
    pub const LCID_ES_PR: u32 = 0x500A;
    /// Spanish (United States)
    pub const LCID_ES_US: u32 = 0x540A;
    /// Spanish (Cuba)
    pub const LCID_ES_CU: u32 = 0x5C0A;
    /// Bosnian (Cyrillic)
    pub const LCID_BS_CYRL: u32 = 0x641A;
    /// Bosnian (Latin)
    pub const LCID_BS_LATN: u32 = 0x681A;
    /// Serbian (Cyrillic)
    pub const LCID_SR_CYRL: u32 = 0x6C1A;
    /// Serbian (Latin)
    pub const LCID_SR_LATN: u32 = 0x701A;
    /// Sami (Inari)
    pub const LCID_SMN: u32 = 0x703B;
    /// Azerbaijani (Cyrillic)
    pub const LCID_AZ_CYRL: u32 = 0x742C;
    /// Sami (Skolt)
    pub const LCID_SMS: u32 = 0x743B;
    /// Chinese
    pub const LCID_ZH: u32 = 0x7804;
    /// Norwegian Nynorsk
    pub const LCID_NN: u32 = 0x7814;
    /// Bosnian
    pub const LCID_BS: u32 = 0x781A;
    /// Azerbaijani (Latin)
    pub const LCID_AZ_LATN: u32 = 0x782C;
    /// Sami (Southern)
    pub const LCID_SMA: u32 = 0x783B;
    /// Uzbek (Cyrillic)
    pub const LCID_UZ_CYRL: u32 = 0x7843;
    /// Mongolian
    pub const LCID_MN_CYRL: u32 = 0x7850;
    /// Inuktitut (Syllabics)
    pub const LCID_IU_CANS: u32 = 0x785D;
    /// Central Atlas Tamazight (Tifinagh)
    pub const LCID_TZM_TFNG: u32 = 0x785F;
    /// Chinese (Traditional)
    pub const LCID_ZH_HANT: u32 = 0x7C04;
    /// Norwegian Bokmål
    pub const LCID_NB: u32 = 0x7C14;
    /// Serbian
    pub const LCID_SR: u32 = 0x7C1A;
    /// Tajik (Cyrillic)
    pub const LCID_TG_CYRL: u32 = 0x7C28;
    /// Lower Sorbian
    pub const LCID_DSB: u32 = 0x7C2E;
    /// Sami (Lule)
    pub const LCID_SMJ: u32 = 0x7C3B;
    /// Uzbek (Latin)
    pub const LCID_UZ_LATN: u32 = 0x7C43;
    /// Punjabi
    pub const LCID_PA_ARAB: u32 = 0x7C46;
    /// Mongolian (Traditional Mongolian)
    pub const LCID_MN_MONG: u32 = 0x7C50;
    /// Sindhi
    pub const LCID_SD_ARAB: u32 = 0x7C59;
    /// Cherokee
    pub const LCID_CHR_CHER: u32 = 0x7C5C;
    /// Inuktitut (Latin)
    pub const LCID_IU_LATN: u32 = 0x7C5D;
    /// Central Atlas Tamazight (Latin)
    pub const LCID_TZM_LATN: u32 = 0x7C5F;
    /// Fulah
    pub const LCID_FF_LATN: u32 = 0x7C67;
    /// Hausa (Latin)
    pub const LCID_HA_LATN: u32 = 0x7C68;
    /// Central Kurdish
    pub const LCID_KU_ARAB: u32 = 0x7C92;
}
