use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize)]
enum Status {
    None,
    Reserved,
    ReservedUnknown,
    NeitherDefinedNorReserved,
}

#[derive(Debug, PartialEq)]
pub enum Numbered {
    Normal(String, u32),
    Reserved(String, u32),
    ReservedUnknown(u32),
    NeitherDefinedNorReserved(u32),
}

#[derive(Debug, Deserialize, PartialEq)]
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

macro_rules! read_json {
    ($path:expr) => {{
        let json = read_to_string($path).expect("Failed to read JSON");
        serde_json::from_str(&json).expect("Failed to parse JSON")
    }};
}

macro_rules! assert_is_none {
    ($lcid_hex:ident, $name:ident) => {{
        if let Some(_) = $name {
            panic!("Name is not None for '{}'", $lcid_hex);
        }
    }};
}

pub fn read_named<P: AsRef<Path>>(path: P) -> Vec<String> {
    let named: Vec<String> = read_json!(path);
    let mut names = HashSet::new();
    for name in &named {
        if !names.insert(name.clone()) {
            panic!("Duplicate name '{}'", name);
        }
    }
    named
}

pub fn read_culture_info<P: AsRef<Path>>(path: P) -> HashMap<String, CultureInfo> {
    let culture_infos: HashMap<String, CultureInfo> = read_json!(path);

    for (name, ci) in &culture_infos {
        if name != &ci.name {
            panic!("Name mismatch '{}' != '{}'", name, &ci.name);
        }
    }

    culture_infos
}

fn from_hex(lcid_hex: &str) -> u32 {
    // parse from hex
    if !lcid_hex.starts_with("0x") {
        panic!("LCID not hex for '{}'", lcid_hex);
    }
    let hex = lcid_hex.trim_start_matches("0x");
    u32::from_str_radix(hex, 16).unwrap_or_else(|_| panic!("LCID not hex for '{}'", lcid_hex))
}

pub fn read_numbered<P: AsRef<Path>>(path: P) -> Vec<Numbered> {
    let numbered_hex: Vec<(String, Option<String>, Status)> = read_json!(path);
    let mut lcids = HashSet::new();
    let mut names = HashSet::new();

    numbered_hex
        .into_iter()
        .map(|(lcid_hex, name, status)| {
            let lcid = from_hex(&lcid_hex);

            match status {
                Status::None => {
                    let name = name.unwrap_or_else(|| panic!("Name is None for '{}'", lcid_hex));

                    if !lcids.insert(lcid) {
                        panic!("Duplicate LCID for '{}'", lcid_hex);
                    }
                    if !names.insert(name.clone()) {
                        panic!("Duplicate name for '{}'", lcid_hex);
                    }

                    Numbered::Normal(name, lcid)
                }
                Status::Reserved => {
                    let name = name.unwrap_or_else(|| panic!("Name is None for '{}'", lcid_hex));

                    if !lcids.insert(lcid) {
                        panic!("Duplicate LCID for '{}'", lcid_hex);
                    }
                    if !names.insert(name.clone()) {
                        panic!("Duplicate name for '{}'", lcid_hex);
                    }

                    Numbered::Reserved(name, lcid)
                }
                Status::NeitherDefinedNorReserved => {
                    assert_is_none!(lcid_hex, name);

                    if !lcids.insert(lcid) {
                        panic!("Duplicate LCID for '{}'", lcid_hex);
                    }

                    Numbered::NeitherDefinedNorReserved(lcid)
                }
                Status::ReservedUnknown => {
                    assert_is_none!(lcid_hex, name);

                    if !lcids.insert(lcid) {
                        panic!("Duplicate LCID for '{}'", lcid_hex);
                    }

                    Numbered::ReservedUnknown(lcid)
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_json(json: &str) -> NamedTempFile {
        let mut tmpfile = NamedTempFile::new().unwrap();
        write!(tmpfile, "{}", json).unwrap();
        tmpfile
    }

    #[test]
    fn read_named_succeeds() {
        let file = write_json(r#"["foo", "bar"]"#);
        let named = read_named(file.path());
        assert_eq!(named, vec!["foo".to_owned(), "bar".to_owned()]);
    }

    #[test]
    #[should_panic(expected = "Duplicate name 'foo'")]
    fn read_named_duplicates() {
        let file = write_json(r#"["foo", "foo"]"#);
        read_named(file.path());
    }

    #[test]
    fn read_culture_info_succeeds() {
        let file = write_json(
            r#"{
            "foo": {
                "Name": "foo",
                "LCID": 4096,
                "EnglishName": "FooBar",
                "TwoLetterISOLanguageName": "fo",
                "ThreeLetterISOLanguageName": "foo",
                "ThreeLetterWindowsLanguageName": "FOO",
                "ANSICodePage": 1252
            }
        }"#,
        );
        let ci: Vec<(String, CultureInfo)> = read_culture_info(file.path()).into_iter().collect();
        assert_eq!(
            ci,
            vec![(
                "foo".to_owned(),
                CultureInfo {
                    name: "foo".to_owned(),
                    lcid: 0x1000,
                    english_name: "FooBar".to_owned(),
                    iso639_two_letter: "fo".to_owned(),
                    iso639_three_letter: "foo".to_owned(),
                    windows_three_letter: "FOO".to_owned(),
                    ansi_code_page: 1252,
                }
            )]
        );
    }

    #[test]
    #[should_panic(expected = "Name mismatch 'foo' != 'bar'")]
    fn read_culture_info_mismatch() {
        let file = write_json(
            r#"{
            "foo": {
                "Name": "bar",
                "LCID": 4096,
                "EnglishName": "FooBar",
                "TwoLetterISOLanguageName": "fo",
                "ThreeLetterISOLanguageName": "foo",
                "ThreeLetterWindowsLanguageName": "FOO",
                "ANSICodePage": 1252
            }
        }"#,
        );
        read_culture_info(file.path());
    }

    #[test]
    fn read_numbered_succeeds() {
        let file = write_json(
            r#"[
            ["0x1", "normal", "None"],
            ["0x2", "reserved", "Reserved"],
            ["0x3", null, "NeitherDefinedNorReserved"],
            ["0x4", null, "ReservedUnknown"]
        ]"#,
        );
        let numbered = read_numbered(file.path());
        assert_eq!(
            numbered,
            vec![
                Numbered::Normal("normal".to_owned(), 1),
                Numbered::Reserved("reserved".to_owned(), 2),
                Numbered::NeitherDefinedNorReserved(3),
                Numbered::ReservedUnknown(4),
            ]
        );
    }

    #[test]
    #[should_panic(expected = "LCID not hex for '1'")]
    fn read_numbered_not_hex_normal() {
        let file = write_json(r#"[["1", "normal", "None"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "LCID not hex for '1'")]
    fn read_numbered_not_hex_reserved() {
        let file = write_json(r#"[["1", "reserved", "Reserved"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "LCID not hex for '1'")]
    fn read_numbered_not_hex_ndnr() {
        let file = write_json(r#"[["1", null, "NeitherDefinedNorReserved"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "LCID not hex for '1'")]
    fn read_numbered_not_hex_runk() {
        let file = write_json(r#"[["1", null, "ReservedUnknown"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "LCID not hex for '0xz'")]
    fn read_numbered_invalid_hex_normal() {
        let file = write_json(r#"[["0xz", "normal", "None"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "LCID not hex for '0xz'")]
    fn read_numbered_invalid_hex_reserved() {
        let file = write_json(r#"[["0xz", "reserved", "Reserved"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "LCID not hex for '0xz'")]
    fn read_numbered_invalid_hex_ndnr() {
        let file = write_json(r#"[["0xz", null, "NeitherDefinedNorReserved"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "LCID not hex for '0xz'")]
    fn read_numbered_invalid_hex_runk() {
        let file = write_json(r#"[["0xz", null, "ReservedUnknown"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Name is None for '0x1'")]
    fn read_numbered_name_is_none_normal() {
        let file = write_json(r#"[["0x1", null, "None"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Name is None for '0x1'")]
    fn read_numbered_name_is_none_reserved() {
        let file = write_json(r#"[["0x1", null, "Reserved"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Name is not None for '0x1'")]
    fn read_numbered_name_is_not_none_ndnr() {
        let file = write_json(r#"[["0x1", "foo", "NeitherDefinedNorReserved"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Name is not None for '0x1'")]
    fn read_numbered_name_is_not_none_runk() {
        let file = write_json(r#"[["0x1", "foo", "ReservedUnknown"]]"#);
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Duplicate LCID for '0x1'")]
    fn read_numbered_duplicate_lcid_normal() {
        let file = write_json(
            r#"[
            ["0x1", "foo", "None"],
            ["0x1", "bar", "None"]
        ]"#,
        );
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Duplicate LCID for '0x1'")]
    fn read_numbered_duplicate_lcid_reserved() {
        let file = write_json(
            r#"[
            ["0x1", "foo", "None"],
            ["0x1", "bar", "Reserved"]
        ]"#,
        );
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Duplicate LCID for '0x1'")]
    fn read_numbered_duplicate_lcid_ndnr() {
        let file = write_json(
            r#"[
            ["0x1", "foo", "None"],
            ["0x1", null, "NeitherDefinedNorReserved"]
        ]"#,
        );
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Duplicate LCID for '0x1'")]
    fn read_numbered_duplicate_lcid_runk() {
        let file = write_json(
            r#"[
            ["0x1", "foo", "None"],
            ["0x1", null, "ReservedUnknown"]
        ]"#,
        );
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Duplicate name for '0x2'")]
    fn read_numbered_duplicate_name_normal() {
        let file = write_json(
            r#"[
            ["0x1", "foo", "None"],
            ["0x2", "foo", "None"]
        ]"#,
        );
        read_numbered(file.path());
    }

    #[test]
    #[should_panic(expected = "Duplicate name for '0x2'")]
    fn read_numbered_duplicate_name_reserved() {
        let file = write_json(
            r#"[
            ["0x1", "foo", "None"],
            ["0x2", "foo", "Reserved"]
        ]"#,
        );
        read_numbered(file.path());
    }
}
