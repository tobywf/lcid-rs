#![allow(clippy::comparison_to_empty)]
use super::culture_info::CultureInfo;

pub fn fixup_all(cis: &mut [Option<CultureInfo>]) {
    for ci in cis.iter_mut().flatten() {
        fixup(ci);
    }
}

fn fixup(ci: &mut CultureInfo) {
    match ci.lcid {
        0x0014 => {
            if ci.iso639_two_letter == "no" && ci.iso639_three_letter == "nor" {
                // server -> win10
                ci.iso639_two_letter = "nb".to_string();
                ci.iso639_three_letter = "nob".to_string();
            }
        }
        0x0067 => {
            if ci.english_name == "Fulah (Latin)" {
                // win10 -> server (because "ff" != "ff-Latn")
                ci.english_name = "Fulah".to_string();
            }
        }
        0x006B => {
            // server -> win10
            if ci.iso639_three_letter == "" {
                ci.iso639_three_letter = "qub".to_string();
            }
            // server -> win10
            if ci.english_name == "quz" {
                ci.english_name = "Quechua".to_string();
            }
        }
        0x008C => {
            // server -> win10
            if ci.iso639_three_letter == "" {
                ci.iso639_three_letter = "prs".to_string();
            }
            // server -> win10
            if ci.english_name == "prs" {
                ci.english_name = "Dari".to_string();
            }
        }
        0x0092 => {
            if ci.english_name == "Kurdish" {
                // server -> win10
                ci.english_name = "Central Kurdish".to_string();
            }
        }
        0x0403 => {
            if ci.english_name == "Catalan (Spain)" {
                // server -> win10
                ci.english_name = "Catalan (Catalan)".to_string();
            }
        }
        0x0404 => {
            if ci.english_name == "Chinese (Taiwan)" {
                // server -> win10
                ci.english_name = "Chinese (Traditional, Taiwan)".to_string();
            }
        }
        0x040A => {
            if ci.english_name == "Spanish (Spain, Sort Order=tradnl)" {
                // server -> win10
                ci.english_name = "Spanish (Spain, Traditional Sort)".to_string();
            }
        }
        0x042D => {
            if ci.english_name == "Basque (Spain)" {
                // server -> win10
                ci.english_name = "Basque (Basque)".to_string();
            }
        }
        0x043B => {
            if ci.english_name == "Northern Sami (Norway)" {
                // server -> win10
                ci.english_name = "Sami, Northern (Norway)".to_string();
            }
        }
        0x0445 => {
            if ci.english_name == "Bangla (India)" {
                // server -> win10
                ci.english_name = "Bengali (India)".to_string();
            }
        }
        0x0456 => {
            if ci.english_name == "Galician (Spain)" {
                // server -> win10
                ci.english_name = "Galician (Galician)".to_string();
            }
        }
        0x045D => {
            if ci.english_name == "Inuktitut (Unified Canadian Aboriginal Syllabics, Canada)" {
                // server -> win10
                ci.english_name = "Inuktitut (Syllabics, Canada)".to_string();
            }
        }
        0x0460 => {
            if ci.english_name == "Kashmiri (Arabic)" {
                // server -> win10
                ci.english_name = "Kashmiri (Perso-Arabic)".to_string();
            }
        }
        0x046B => {
            // server -> win10
            if ci.iso639_three_letter == "" {
                ci.iso639_three_letter = "qub".to_string();
            }
            // server -> win10
            if ci.english_name == "quz (Bolivia)" {
                ci.english_name = "Quechua (Bolivia)".to_string();
            }
        }
        0x0471 => {
            if ci.english_name == "Kanuri (Nigeria)" {
                // win10 -> server (because "kr-Latn-NG")
                ci.english_name = "Kanuri (Latin, Nigeria)".to_string();
            }
        }
        0x047C => {
            if ci.english_name == "Mohawk (Canada)" {
                // server -> win10
                ci.english_name = "Mohawk (Mohawk)".to_string();
            }
        }
        0x0484 => {
            if ci.english_name == "Swiss German (France)" {
                // server -> win10
                ci.english_name = "Alsatian (France)".to_string();
            }
        }
        0x048C => {
            // server -> win10
            if ci.iso639_three_letter == "" {
                ci.iso639_three_letter = "prs".to_string();
            }
            // server -> win10
            if ci.english_name == "prs (Afghanistan)" {
                ci.english_name = "Dari (Afghanistan)".to_string();
            }
        }
        0x0492 => {
            if ci.english_name == "Kurdish (Arabic, Iraq)"
                || ci.english_name == "Central Kurdish (Iraq)"
            {
                // merge, because "ku-Arab-IQ"
                ci.english_name = "Central Kurdish (Arabic, Iraq)".to_string();
            }
        }
        0x0501 => {
            if ci.name == "qps-Ploc"
                && ci.iso639_two_letter == "qps"
                && ci.iso639_three_letter == ""
                && ci.english_name == "qps (Ploc)"
            {
                // server -> win10
                ci.name = "qps-ploc".to_string();
                ci.iso639_two_letter = "en".to_string();
                ci.iso639_three_letter = "eng".to_string();
                ci.english_name = "Pseudo (Pseudo)".to_string();
            }
        }
        0x0803 => {
            // server -> win10
            if ci.name == "ca-ES-VALENCIA" {
                ci.name = "ca-ES-valencia".to_string();
            }
            // server -> win10
            if ci.english_name == "Catalan (Spain, Valencian)" {
                ci.english_name = "Valencian (Spain)".to_string();
            }
        }
        0x0804 => {
            if ci.english_name == "Chinese (China)" {
                // server -> win10
                ci.english_name = "Chinese (Simplified, China)".to_string();
            }
        }
        0x081A => {
            if ci.english_name == "Serbian (Latin, Serbia)" {
                // server -> win10-ish
                ci.english_name = "Serbian (Latin, Serbia & Montenegro (Former))".to_string();
            }
            if ci.english_name == "Serbian (Latin, Serbia and Montenegro (Former))" {
                // win10 -> win10-ish
                ci.english_name = "Serbian (Latin, Serbia & Montenegro (Former))".to_string();
            }
        }
        0x083B => {
            if ci.english_name == "Northern Sami (Sweden)" {
                // server -> win10
                ci.english_name = "Sami, Northern (Sweden)".to_string();
            }
        }
        0x0846 => {
            if ci.english_name == "Punjabi (Pakistan)" {
                // win10 -> server, because "pa-Arab-PK"
                ci.english_name = "Punjabi (Arabic, Pakistan)".to_string();
            }
        }
        // 0x0850 => if ci.english_name == "Mongolian (Mongolian, China)" {
        //     // server -> win10
        //     ci.english_name = "Mongolian (Traditional Mongolian, China)".to_string();
        // }
        0x0859 => {
            if ci.english_name == "Sindhi (Pakistan)" {
                // win10 -> server, because "sd-Arab-PK"
                ci.english_name = "Sindhi (Arabic, Pakistan)".to_string();
            }
        }
        0x0860 => {
            if ci.english_name == "Kashmiri (Devanagari)" {
                // win10 -> server, because "ks-Deva-IN"
                ci.english_name = "Kashmiri (Devanagari, India)".to_string();
            }
        }
        0x086B => {
            // server -> win10
            if ci.iso639_three_letter == "" {
                ci.iso639_three_letter = "que".to_string();
            }
            if ci.english_name == "quz (Ecuador)" {
                ci.english_name = "Quechua (Ecuador)".to_string();
            }
        }
        0x0C04 => {
            if ci.english_name == "Chinese (Hong Kong SAR)" {
                // server -> win10
                ci.english_name = "Chinese (Traditional, Hong Kong SAR)".to_string();
            }
        }
        0x0C0A => {
            if ci.english_name == "Spanish (Spain)" {
                // server -> win10
                ci.english_name = "Spanish (Spain, International Sort)".to_string();
            }
        }
        0x0C1A => {
            if ci.english_name == "Serbian (Cyrillic, Serbia)" {
                // server -> win10-ish
                ci.english_name = "Serbian (Cyrillic, Serbia & Montenegro (Former))".to_string();
            }
            if ci.english_name == "Serbian (Cyrillic, Serbia and Montenegro (Former))" {
                // server -> win10-ish
                ci.english_name = "Serbian (Cyrillic, Serbia & Montenegro (Former))".to_string();
            }
        }
        0x0C3B => {
            if ci.english_name == "Northern Sami (Finland)" {
                // server -> win10
                ci.english_name = "Sami, Northern (Finland)".to_string();
            }
        }
        0x0C50 => {
            if ci.english_name == "Mongolian (Mongolian, Mongolia)" {
                // server -> win10
                ci.english_name = "Mongolian (Traditional Mongolian, Mongolia)".to_string();
            }
        }
        0x0C6B => {
            // server -> win10
            if ci.iso639_three_letter == "" {
                ci.iso639_three_letter = "qup".to_string();
            }
            // server -> win10
            if ci.english_name == "quz (Peru)" {
                ci.english_name = "Quechua (Peru)".to_string();
            }
        }
        0x1004 => {
            if ci.english_name == "Chinese (Singapore)" {
                // server -> win10
                ci.english_name = "Chinese (Simplified, Singapore)".to_string();
            }
        }
        0x103B => {
            if ci.english_name == "Lule Sami (Norway)" {
                // server -> win10
                ci.english_name = "Sami, Lule (Norway)".to_string();
            }
        }
        0x1404 => {
            if ci.english_name == "Chinese (Macao SAR)" {
                // server -> win10
                ci.english_name = "Chinese (Traditional, Macao SAR)".to_string();
            }
        }
        0x143B => {
            if ci.english_name == "Lule Sami (Sweden)" {
                // server -> win10
                ci.english_name = "Sami, Lule (Sweden)".to_string();
            }
        }
        0x183B => {
            if ci.english_name == "Southern Sami (Norway)" {
                // server -> win10
                ci.english_name = "Sami, Southern (Norway)".to_string();
            }
        }
        0x1C1A => {
            if ci.english_name == "Serbian (Cyrillic, Bosnia and Herzegovina)" {
                // win10 -> server
                ci.english_name = "Serbian (Cyrillic, Bosnia & Herzegovina)".to_string();
            }
        }
        0x1C3B => {
            if ci.english_name == "Southern Sami (Sweden)" {
                // server -> win10
                ci.english_name = "Sami, Southern (Sweden)".to_string();
            }
        }
        0x201A => {
            if ci.english_name == "Bosnian (Cyrillic, Bosnia and Herzegovina)" {
                // win10 -> server
                ci.english_name = "Bosnian (Cyrillic, Bosnia & Herzegovina)".to_string();
            }
        }
        0x203B => {
            if ci.english_name == "Skolt Sami (Finland)" {
                // server -> win10
                ci.english_name = "Sami, Skolt (Finland)".to_string();
            }
        }
        0x240C => {
            if ci.english_name == "French (Congo [DRC])" {
                // server -> win10
                ci.english_name = "French Congo (DRC)".to_string();
            }
        }
        0x243B => {
            if ci.english_name == "Inari Sami (Finland)" {
                // server -> win10
                ci.english_name = "Sami, Inari (Finland)".to_string();
            }
        }
        0x703B => {
            if ci.english_name == "Inari Sami" {
                // server -> win10
                ci.english_name = "Sami (Inari)".to_string();
            }
        }
        0x743B => {
            if ci.english_name == "Skolt Sami" {
                // server -> win10
                ci.english_name = "Sami (Skolt)".to_string();
            }
        }
        0x783B => {
            if ci.english_name == "Southern Sami" {
                // server -> win10
                ci.english_name = "Sami (Southern)".to_string();
            }
        }
        0x7850 => {
            if ci.english_name == "Mongolian" {
                // win10 -> server, because "mn-Cyrl"
                ci.english_name = "Mongolian (Cyrillic)".to_string();
            }
        }
        0x785D => {
            if ci.english_name == "Inuktitut (Unified Canadian Aboriginal Syllabics)" {
                // server -> win10
                ci.english_name = "Inuktitut (Syllabics)".to_string();
            }
        }
        0x7C04 => {
            if ci.windows_three_letter == "ZHH" {
                // server -> win10
                ci.windows_three_letter = "CHT".to_string();
            }
        }
        0x7C3B => {
            if ci.english_name == "Lule Sami" {
                // server -> win10
                ci.english_name = "Sami (Lule)".to_string();
            }
        }
        0x7C46 => {
            if ci.english_name == "Punjabi" {
                // win10 -> server, because "pa-Arab"
                ci.english_name = "Punjabi (Arabic)".to_string();
            }
        }
        0x7C50 => {
            if ci.english_name == "Mongolian (Mongolian)" {
                // server -> win10
                ci.english_name = "Mongolian (Traditional Mongolian)".to_string();
            }
        }
        0x7C59 => {
            if ci.english_name == "Sindhi" {
                // win10 -> server, because "sd-Arab"
                ci.english_name = "Sindhi (Arabic)".to_string();
            }
        }
        0x7C5C => {
            if ci.english_name == "Cherokee (Cherokee)" {
                // server -> win10
                ci.english_name = "Cherokee".to_string();
            }
        }
        0x7C92 => {
            if ci.english_name == "Kurdish (Arabic)" || ci.english_name == "Central Kurdish" {
                // merge, because "ku-Arab"
                ci.english_name = "Central Kurdish (Arabic)".to_string();
            }
        }
        _ => {}
    }
}
