#![allow(clippy::comparison_to_empty)]
use super::ms_lcid::MsLcid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CultureInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "LCID")]
    pub lcid: u32,
    #[serde(rename = "EnglishName")]
    pub english_name: String,
    #[serde(rename = "TwoLetterISOLanguageName")]
    pub iso639_two_letter: String,
    #[serde(rename = "ThreeLetterISOLanguageName")]
    pub iso639_three_letter: String,
    #[serde(rename = "ThreeLetterWindowsLanguageName")]
    pub windows_three_letter: String,
    #[serde(rename = "ANSICodePage")]
    pub ansi_code_page: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Dump {
    pub culture_info_by_lcid_ok: HashMap<String, CultureInfo>,
    pub culture_info_by_lcid_err: HashMap<String, String>,
    pub culture_info_by_name_ok: HashMap<String, CultureInfo>,
    pub culture_info_by_name_err: HashMap<String, String>,
}

fn to_hex(lcid: u32) -> String {
    format!("0x{:04X}", lcid)
}

fn filter_normal(ms_lcids: &[MsLcid], raw: &mut Dump) {
    for ms_lcid in ms_lcids.iter() {
        match ms_lcid {
            MsLcid::Reserved(name, lcid) => {
                let lcid_hex = to_hex(*lcid);
                raw.culture_info_by_lcid_ok.remove(&lcid_hex);
                raw.culture_info_by_lcid_err.remove(&lcid_hex);
                raw.culture_info_by_name_ok.remove(name);
                raw.culture_info_by_name_err.remove(name);
            }
            MsLcid::ReservedUnknown(lcid)
            | MsLcid::NeitherDefinedNorReserved(lcid)
            | MsLcid::Temporary(lcid) => {
                let lcid_hex = to_hex(*lcid);
                raw.culture_info_by_lcid_ok.remove(&lcid_hex);
                raw.culture_info_by_lcid_err.remove(&lcid_hex);
            }
            MsLcid::Normal(_name, _lcid) => {}
        }
    }

    for (name, ex) in raw.culture_info_by_name_err.iter() {
        match name.as_str() {
            "qps-ploca" => {
                // 0x05FE/qps-ploca missing on server
                assert!(
                    !raw.culture_info_by_lcid_ok.contains_key("0x05FE"),
                    "0x05FE/qps-ploca"
                );
            }
            "qps-plocm" => {
                // 0x09FF/qps-plocm missing on server
                assert!(
                    !raw.culture_info_by_lcid_ok.contains_key("0x09FF"),
                    "0x09FF/qps-plocm"
                );
            }
            _ => panic!("unexpected error `{}`: {}", name, ex),
        }
    }
    raw.culture_info_by_name_err = HashMap::new();

    for (lcid_hex, ex) in raw.culture_info_by_lcid_err.iter() {
        match lcid_hex.as_str() {
            "0x05FE" => {
                // 0x05FE/qps-ploca missing on server
                assert!(
                    !raw.culture_info_by_name_ok.contains_key("qps-ploca"),
                    "0x05FE/qps-ploca"
                );
                assert!(
                    !raw.culture_info_by_name_ok.contains_key("qps-Ploca"),
                    "0x05FE/qps-ploca"
                );
            }
            "0x09FF" => {
                // 0x09FF/qps-plocm missing on server
                assert!(
                    !raw.culture_info_by_lcid_ok.contains_key("qps-plocm"),
                    "0x09FF/qps-plocm"
                );
                assert!(
                    !raw.culture_info_by_lcid_ok.contains_key("qps-Plocm"),
                    "0x09FF/qps-plocm"
                );
            }
            _ => panic!("unexpected error `{}`: {}", lcid_hex, ex),
        }
    }
    raw.culture_info_by_lcid_err = HashMap::new();
}

fn add_missing(raw: &mut Dump) {
    // --- 0x05FE/qps-ploca
    if !raw.culture_info_by_lcid_ok.contains_key("0x05FE") {
        // server -> win10
        raw.culture_info_by_lcid_ok.insert(
            "0x05FE".to_string(),
            CultureInfo {
                name: "qps-ploca".to_string(),
                lcid: 1534,
                english_name: "Pseudo (Pseudo Asia)".to_string(),
                iso639_two_letter: "qps".to_string(),
                iso639_three_letter: "jpn".to_string(),
                windows_three_letter: "JPN".to_string(),
                ansi_code_page: 932,
            },
        );
    }
    if !raw.culture_info_by_name_ok.contains_key("qps-ploca") {
        // server -> win10
        raw.culture_info_by_name_ok.insert(
            "qps-ploca".to_string(),
            CultureInfo {
                name: "qps-ploca".to_string(),
                lcid: 1534,
                english_name: "Pseudo (Pseudo Asia)".to_string(),
                iso639_two_letter: "qps".to_string(),
                iso639_three_letter: "jpn".to_string(),
                windows_three_letter: "JPN".to_string(),
                ansi_code_page: 932,
            },
        );
    }
    // --- 0x09FF/qps-plocm
    if !raw.culture_info_by_lcid_ok.contains_key("0x09FF") {
        // server -> win10
        raw.culture_info_by_lcid_ok.insert(
            "0x09FF".to_string(),
            CultureInfo {
                name: "qps-plocm".to_string(),
                lcid: 2559,
                english_name: "Pseudo (Pseudo Mirrored)".to_string(),
                iso639_two_letter: "ar".to_string(),
                iso639_three_letter: "ara".to_string(),
                windows_three_letter: "ARA".to_string(),
                ansi_code_page: 1256,
            },
        );
    }
    if !raw.culture_info_by_name_ok.contains_key("qps-plocm") {
        // server -> win10
        raw.culture_info_by_name_ok.insert(
            "qps-plocm".to_string(),
            CultureInfo {
                name: "qps-plocm".to_string(),
                lcid: 2559,
                english_name: "Pseudo (Pseudo Mirrored)".to_string(),
                iso639_two_letter: "ar".to_string(),
                iso639_three_letter: "ara".to_string(),
                windows_three_letter: "ARA".to_string(),
                ansi_code_page: 1256,
            },
        );
    }
}

fn basic_validation(lcid_hex: &str, name: &str, ci: &CultureInfo) {
    let len = ci.iso639_two_letter.len();
    if !(len == 2 || len == 3) {
        // if len != 2 {
        panic!(
            "LCID `{}`/`{}` ISO 639 two letter length: `{}` == {}",
            lcid_hex, name, ci.iso639_two_letter, len
        );
    }
    let len = ci.iso639_three_letter.len();
    if !(len == 0 || len == 3) {
        // if len != 3 {
        panic!(
            "LCID `{}`/`{}` ISO 639 three letter length: `{}` == {}",
            lcid_hex, name, ci.iso639_three_letter, len
        );
    }
    let len = ci.windows_three_letter.len();
    if len != 3 {
        panic!(
            "LCID `{}`/`{}` Windows three letter length: `{}` == {}",
            lcid_hex, name, ci.windows_three_letter, len
        );
    }
}

fn fixup_really_broken(raw: &mut Dump) {
    // --- fix up really broken entries

    // 0x0004/zh-Hans broken on win10
    if let Some(ci) = raw.culture_info_by_lcid_ok.get_mut("0x0004") {
        // win10
        if ci.name == "zh-CHS" {
            ci.name = "zh-Hans".to_string();
            ci.english_name = "Chinese (Simplified)".to_string();
        }
    }

    // 0x7C04/zh-Hant broken on win10
    if let Some(ci) = raw.culture_info_by_lcid_ok.get_mut("0x7C04") {
        // win10
        if ci.name == "zh-CHT" {
            ci.name = "zh-Hant".to_string();
            ci.english_name = "Chinese (Traditional)".to_string();
        }
    }

    // 0x0086/qut is badly broken on server (kinda garbage on both)
    if let Some(ci) = raw.culture_info_by_lcid_ok.get_mut("0x0086") {
        // server
        if ci.english_name == "Kʼicheʼ" {
            ci.english_name = "K'iche'".to_string();
        }
    }
    if let Some(ci) = raw.culture_info_by_name_ok.get_mut("qut") {
        // server
        if ci.lcid == 4096 {
            ci.lcid = 134;
        }
        // server
        if ci.name == "qut" {
            ci.name = "quc".to_string();
        }
        // server
        if ci.iso639_two_letter == "qut" {
            ci.iso639_two_letter = "quc".to_string();
        }
        // server
        if ci.iso639_three_letter == "" {
            ci.iso639_three_letter = "quc".to_string();
        }
        // server
        if ci.windows_three_letter == "ZZZ" {
            ci.windows_three_letter = "QUC".to_string();
        }
        // server
        if ci.english_name == "qut" {
            ci.english_name = "K'iche'".to_string();
        }
    }

    // 0x0467/ff-Latn-NG broken LCID on server
    if let Some(ci) = raw.culture_info_by_lcid_ok.get_mut("0x0467") {
        // server
        if ci.lcid == 4096 {
            ci.lcid = 1127;
        }
    }
    if let Some(ci) = raw.culture_info_by_name_ok.get_mut("ff-Latn-NG") {
        // server
        if ci.lcid == 4096 {
            ci.lcid = 1127;
        }
    }

    // 0x0471/kr-Latn-NG broken LCID on server
    if let Some(ci) = raw.culture_info_by_lcid_ok.get_mut("0x0471") {
        // server
        if ci.lcid == 4096 {
            ci.lcid = 1137;
        }
    }
    if let Some(ci) = raw.culture_info_by_name_ok.get_mut("kr-Latn-NG") {
        // server
        if ci.lcid == 4096 {
            ci.lcid = 1137;
        }
    }

    // 0x0476/la-VA is a mess, and also maps to "la-001"
    if let Some(ci) = raw.culture_info_by_lcid_ok.get_mut("0x0476") {
        // server and win10
        if ci.name == "la-001" {
            ci.name = "la-VA".to_string();
        }
        // server and win10
        if ci.english_name == "Latin (World)" {
            ci.english_name = "Latin (Vatican City)".to_string();
        }
    }
    if let Some(ci) = raw.culture_info_by_name_ok.get_mut("la-VA") {
        // server and win10
        if ci.lcid == 4096 {
            ci.lcid = 1142;
        }
        // win10 only
        if ci.english_name == "Unknown Locale (la-VA)" {
            ci.english_name = "Latin (Vatican City)".to_string();
        }
    }

    // 0x4C09/en-AE broken LCID on server
    if let Some(ci) = raw.culture_info_by_lcid_ok.get_mut("0x4C09") {
        // server
        if ci.lcid == 4096 {
            ci.lcid = 19465;
        }
    }
    if let Some(ci) = raw.culture_info_by_name_ok.get_mut("en-AE") {
        // server
        if ci.lcid == 4096 {
            ci.lcid = 19465;
        }
    }
}

pub fn parse(mut raw: Dump, ms_lcids: &[MsLcid]) -> Vec<Option<CultureInfo>> {
    filter_normal(ms_lcids, &mut raw);
    add_missing(&mut raw);

    for (lcid_hex, ci) in raw.culture_info_by_lcid_ok.iter() {
        let name = ci.name.as_str();
        basic_validation(lcid_hex, name, ci);
    }

    for (name, ci) in raw.culture_info_by_name_ok.iter() {
        let lcid_hex = to_hex(ci.lcid);
        basic_validation(&lcid_hex, name, ci);
    }

    fixup_really_broken(&mut raw);

    ms_lcids
        .iter()
        .map(|ms_lcid| {
            match ms_lcid {
                MsLcid::Normal(name, lcid) => {
                    let lcid = *lcid;
                    let lcid_hex = &to_hex(lcid);

                    let key = match (lcid_hex.as_str(), name.as_str()) {
                        // MS-LCID misspelling
                        ("0x0C6B", "quz-PE") => "0x0C6b",
                        (other, _) => other,
                    };
                    let ci_lcid = raw.culture_info_by_lcid_ok.remove(key).unwrap_or_else(|| {
                        panic!("LCID `{}`/`{}` not found in LCID", lcid_hex, name)
                    });

                    let key = name.as_str();
                    let ci_name = raw.culture_info_by_name_ok.remove(key).unwrap_or_else(|| {
                        panic!("LCID `{}`/`{}` not found in Name", lcid_hex, name)
                    });

                    assert_eq!(ci_lcid.lcid, lcid, "LCID `{}`/`{}` (LCID)", lcid_hex, name);
                    assert_eq!(ci_name.lcid, lcid, "LCID `{}`/`{}` (Name)", lcid_hex, name);
                    assert_eq!(ci_lcid, ci_name, "LCID `{}`/`{}`", lcid_hex, name);
                    if &ci_lcid.name != name {
                        println!(
                            "LCID `{}`/`{}` Name `{}` != `{}`",
                            lcid_hex, name, ci_lcid.name, name
                        );
                    }
                    Some(ci_lcid)
                }
                _ => None,
            }
        })
        .collect()
}
