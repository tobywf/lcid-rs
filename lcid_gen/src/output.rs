use super::culture_info::CultureInfo;
use super::ms_lcid::MsLcid;
use std::fmt::Write as _;

fn name_to_ident(tag: &str) -> String {
    if tag.is_empty() {
        "LANG_INVARIANT".to_owned()
    } else {
        format!("LANG_{}", tag.replace('-', "_").to_ascii_uppercase())
    }
}

macro_rules! pushln {
    ($s:ident, $fmt:literal) => {
        $s.push_str(concat!($fmt, "\n"))
    };
    ($s:ident, $fmt:literal,) => {
        $s.push_str(concat!($fmt, "\n"))
    };
    ($s:ident, $fmt:literal, $($arg:expr),+ $(,)?) => {
        writeln!($s, $fmt, $($arg),+).expect("writing to string should be infallible")
    };
}

macro_rules! assert_is_none {
    ($lcid:ident, $maybe_ci:ident) => {{
        if let Some(ci) = $maybe_ci {
            panic!("LCID `0x{:04X}` CI is Some({:#?})", $lcid, ci);
        }
    }};
}

pub fn generate(infos: &[(MsLcid, Option<CultureInfo>)]) -> String {
    let mut ci_dump = String::new();

    let mut lcid_consts = String::new();
    pushln!(lcid_consts, "pub mod lcid {");
    pushln!(
        lcid_consts,
        "    //! Contains constant LCIDs corresponding to the full language information",
    );
    pushln!(
        lcid_consts,
        "    //! in the parent module, for easy use in e.g. match statements."
    );

    let mut lookup_lcid = String::new();
    pushln!(lookup_lcid, "macro_rules! parse_lcid {");
    pushln!(lookup_lcid, "    ($value:expr) => {");
    pushln!(lookup_lcid, "        match $value {");

    let mut lookup_name = String::new();
    pushln!(lookup_name, "macro_rules! parse_name {");
    pushln!(lookup_name, "    ($value:expr) => {");
    pushln!(lookup_name, "        match $value {");

    let indent = "            ";

    for (ms_lcid, maybe_ci) in infos {
        match ms_lcid {
            MsLcid::Temporary(lcid) => {
                assert_is_none!(lcid, maybe_ci);
                lookup_lcid.push_str(indent);
                pushln!(
                    lookup_lcid,
                    "{0:#06X} => Err(Self::Error::Temporary({0:#06X})),",
                    lcid
                );
                // no corresponding name option
            }
            MsLcid::NeitherDefinedNorReserved(lcid) => {
                assert_is_none!(lcid, maybe_ci);
                lookup_lcid.push_str(indent);
                pushln!(
                    lookup_lcid,
                    "{0:#06X} => Err(Self::Error::NeitherDefinedNorReserved({0:#06X})),",
                    lcid
                );
                // no corresponding name option
            }
            MsLcid::ReservedUnknown(lcid) => {
                assert_is_none!(lcid, maybe_ci);
                lookup_lcid.push_str(indent);
                pushln!(
                    lookup_lcid,
                    "{:#06X} => Err(Self::Error::ReservedUnknown({0:#06X})),",
                    lcid
                );
                // no corresponding name option
            }
            MsLcid::Reserved(name, lcid) => {
                assert_is_none!(lcid, maybe_ci);
                lookup_lcid.push_str(indent);
                pushln!(
                    lookup_lcid,
                    "{:#06X} => Err(Self::Error::Reserved({0:#06X}, \"{}\")),",
                    lcid,
                    name
                );
                lookup_name.push_str(indent);
                pushln!(
                    lookup_name,
                    "\"{0}\" => Err(Self::Error::Reserved(\"{0}\", {1})),",
                    name,
                    lcid
                );
            }
            MsLcid::Normal(name, lcid) => {
                let ci = maybe_ci
                    .as_ref()
                    .unwrap_or_else(|| panic!("LCID `0x{:04X}`/`{}` CI is None", lcid, name));
                if ci.lcid != *lcid {
                    panic!(
                        "LCID `0x{:04X}`/`{}` CI LCID `{}` != `{}`",
                        lcid, name, ci.lcid, lcid
                    );
                }
                let identifier = name_to_ident(name);

                lookup_lcid.push_str(indent);
                pushln!(
                    lookup_lcid,
                    "{:#06X} => Ok(constants::{}),",
                    lcid,
                    identifier
                );
                lookup_name.push_str(indent);
                pushln!(
                    lookup_name,
                    "\"{}\" => Ok(constants::{}),",
                    name,
                    identifier
                );

                pushln!(ci_dump, "/// {}", ci.english_name);
                pushln!(
                    ci_dump,
                    "pub const {}: &LanguageId = &LanguageId {{",
                    identifier
                );

                pushln!(ci_dump, "    name: \"{}\",", ci.name);
                pushln!(ci_dump, "    lcid: {:#06X},", ci.lcid);
                pushln!(ci_dump, "    english_name: \"{}\",", ci.english_name);
                pushln!(
                    ci_dump,
                    "    iso639_two_letter: \"{}\",",
                    ci.iso639_two_letter
                );
                pushln!(
                    ci_dump,
                    "    iso639_three_letter: \"{}\",",
                    ci.iso639_three_letter
                );
                pushln!(
                    ci_dump,
                    "    windows_three_letter: \"{}\",",
                    ci.windows_three_letter
                );
                if ci.ansi_code_page == 0 {
                    pushln!(ci_dump, "    ansi_code_page: None,");
                } else {
                    let cp = match ci.ansi_code_page {
                        0 => unreachable!(),
                        932 => "ShiftJIS".to_owned(),
                        936 => "GB2312".to_owned(),
                        949 => "KsC5601".to_owned(),
                        950 => "Big5".to_owned(),
                        cp => format!("Windows{}", cp),
                    };
                    pushln!(ci_dump, "    ansi_code_page: Some(AnsiCodePage::{}),", cp);
                }

                pushln!(ci_dump, "};\n");

                pushln!(lcid_consts, "    /// {}", ci.english_name);
                pushln!(
                    lcid_consts,
                    "    pub const {}: u32 = {:#06X};",
                    identifier.replace("LANG_", "LCID_"),
                    ci.lcid
                );
            }
        }
    }

    pushln!(lcid_consts, "}");

    pushln!(
        lookup_lcid,
        "            undef => Err(Self::Error::Undefined(undef)),"
    );
    pushln!(lookup_lcid, "        }");
    pushln!(lookup_lcid, "    };");
    pushln!(lookup_lcid, "}");
    pushln!(lookup_lcid, "");

    pushln!(
        lookup_name,
        "            undef => Err(Self::Error::Undefined(undef.to_owned())),"
    );
    pushln!(lookup_name, "        }");
    pushln!(lookup_name, "    };");
    pushln!(lookup_name, "}");
    pushln!(lookup_name, "");

    let mut header = String::new();
    pushln!(
        header,
        "//! Contains all defined [`LanguageId`] returned by the lookups."
    );
    pushln!(header, "use crate::{AnsiCodePage, LanguageId};");
    header.push('\n');

    header.push_str(&ci_dump);
    header.push_str(&lookup_lcid);
    header.push_str(&lookup_name);
    header.push_str(&lcid_consts);

    header
}
