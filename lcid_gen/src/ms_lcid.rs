use serde::Deserialize;
use std::collections::HashSet;

/// The raw language ID status, from a `ms-lcid-*.json` file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Status {
    /// The language ID has no special status
    None,
    /// The language ID is named, but is reserved
    Reserved,
    /// The language ID is not named, and is reserved
    ReservedUnknown,
    /// The language ID is neither defined nor reserved
    NeitherDefinedNorReserved,
    /// The language ID is a temporary assignment
    Temporary,
}

/// The parsed language ID, from a `ms-lcid-*.json` file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MsLcid {
    /// The language ID has no special status
    Normal(String, u32),
    /// The language ID is named, but is reserved
    Reserved(String, u32),
    /// The language ID is not named, and is reserved
    ReservedUnknown(u32),
    /// The language ID is neither defined nor reserved
    NeitherDefinedNorReserved(u32),
    /// The language ID is a temporary assignment
    Temporary(u32),
}

fn from_hex(lcid_hex: &str) -> u32 {
    lcid_hex
        .strip_prefix("0x")
        .and_then(|s| u32::from_str_radix(s, 16).ok())
        .unwrap_or_else(|| panic!("LCID `{}` not hex", lcid_hex))
}

macro_rules! assert_is_none {
    ($lcid_hex:ident, $name:ident) => {{
        if let Some(name) = $name {
            panic!("LCID `{}` Name is Some(`{}`)", $lcid_hex, name);
        }
    }};
}

pub type MsLcids = Vec<(String, Option<String>, Status)>;

pub fn parse(raw: MsLcids) -> Vec<MsLcid> {
    let mut lcids = HashSet::new();
    let mut names = HashSet::new();

    raw.into_iter()
        .map(|(lcid_hex, name, status)| {
            let lcid = from_hex(&lcid_hex);

            match status {
                Status::None => {
                    let name = name.unwrap_or_else(|| panic!("LCID `{}` Name is None`", lcid_hex));

                    if !lcids.insert(lcid) {
                        panic!("LCID `{}` duplicate LCID", lcid_hex);
                    }
                    if !names.insert(name.clone()) {
                        panic!("LCID `{}` duplicate Name", lcid_hex);
                    }

                    MsLcid::Normal(name, lcid)
                }
                Status::Reserved => {
                    let name = name.unwrap_or_else(|| panic!("LCID `{}` Name is None`", lcid_hex));

                    if !lcids.insert(lcid) {
                        panic!("LCID `{}` duplicate LCID", lcid_hex);
                    }
                    if !names.insert(name.clone()) {
                        panic!("LCID `{}` duplicate Name", lcid_hex);
                    }

                    MsLcid::Reserved(name, lcid)
                }
                Status::NeitherDefinedNorReserved => {
                    assert_is_none!(lcid_hex, name);

                    if !lcids.insert(lcid) {
                        panic!("LCID `{}` duplicate LCID", lcid_hex);
                    }

                    MsLcid::NeitherDefinedNorReserved(lcid)
                }
                Status::ReservedUnknown => {
                    assert_is_none!(lcid_hex, name);

                    if !lcids.insert(lcid) {
                        panic!("LCID `{}` duplicate LCID", lcid_hex);
                    }

                    MsLcid::ReservedUnknown(lcid)
                }
                Status::Temporary => {
                    assert_is_none!(lcid_hex, name);

                    if !lcids.insert(lcid) {
                        panic!("LCID `{}` duplicate LCID", lcid_hex);
                    }

                    MsLcid::Temporary(lcid)
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! from_json {
        ($json:literal $(,)?) => {
            serde_json::from_str::<MsLcids>($json).unwrap()
        };
    }

    #[test]
    fn parse_succeeds() {
        let raw = from_json!(
            r#"[
            ["0x1", "normal", "None"],
            ["0x2", "reserved", "Reserved"],
            ["0x3", null, "NeitherDefinedNorReserved"],
            ["0x4", null, "ReservedUnknown"],
            ["0x5", null, "Temporary"]
        ]"#
        );
        let lcids = parse(raw);
        assert_eq!(
            lcids,
            vec![
                MsLcid::Normal("normal".to_owned(), 1),
                MsLcid::Reserved("reserved".to_owned(), 2),
                MsLcid::NeitherDefinedNorReserved(3),
                MsLcid::ReservedUnknown(4),
                MsLcid::Temporary(5),
            ]
        );
    }

    #[test]
    #[should_panic(expected = "LCID `1` not hex")]
    fn parse_not_hex_normal() {
        let raw = from_json!(r#"[["1", "normal", "None"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `1` not hex")]
    fn parse_not_hex_reserved() {
        let raw = from_json!(r#"[["1", "reserved", "Reserved"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `1` not hex")]
    fn parse_not_hex_ndnr() {
        let raw = from_json!(r#"[["1", null, "NeitherDefinedNorReserved"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `1` not hex")]
    fn parse_not_hex_runk() {
        let raw = from_json!(r#"[["1", null, "ReservedUnknown"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `1` not hex")]
    fn parse_not_hex_temp() {
        let raw = from_json!(r#"[["1", null, "Temporary"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0xz` not hex")]
    fn parse_invalid_hex_normal() {
        let raw = from_json!(r#"[["0xz", "normal", "None"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0xz` not hex")]
    fn parse_invalid_hex_reserved() {
        let raw = from_json!(r#"[["0xz", "reserved", "Reserved"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0xz` not hex")]
    fn parse_invalid_hex_ndnr() {
        let raw = from_json!(r#"[["0xz", null, "NeitherDefinedNorReserved"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0xz` not hex")]
    fn parse_invalid_hex_runk() {
        let raw = from_json!(r#"[["0xz", null, "ReservedUnknown"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0xz` not hex")]
    fn parse_invalid_hex_temp() {
        let raw = from_json!(r#"[["0xz", null, "Temporary"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` Name is None")]
    fn parse_name_is_none_normal() {
        let raw = from_json!(r#"[["0x1", null, "None"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` Name is None")]
    fn parse_name_is_none_reserved() {
        let raw = from_json!(r#"[["0x1", null, "Reserved"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` Name is Some(`foo`)")]
    fn parse_name_is_not_none_ndnr() {
        let raw = from_json!(r#"[["0x1", "foo", "NeitherDefinedNorReserved"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` Name is Some(`foo`)")]
    fn parse_name_is_not_none_runk() {
        let raw = from_json!(r#"[["0x1", "foo", "ReservedUnknown"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` Name is Some(`foo`)")]
    fn parse_name_is_not_none_temp() {
        let raw = from_json!(r#"[["0x1", "foo", "Temporary"]]"#);
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` duplicate LCID")]
    fn parse_duplicate_lcid_normal() {
        let raw = from_json!(
            r#"[
            ["0x1", "foo", "None"],
            ["0x1", "bar", "None"]
        ]"#
        );
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` duplicate LCID")]
    fn parse_duplicate_lcid_reserved() {
        let raw = from_json!(
            r#"[
            ["0x1", "foo", "None"],
            ["0x1", "bar", "Reserved"]
        ]"#
        );
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` duplicate LCID")]
    fn parse_duplicate_lcid_ndnr() {
        let raw = from_json!(
            r#"[
            ["0x1", "foo", "None"],
            ["0x1", null, "NeitherDefinedNorReserved"]
        ]"#
        );
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x1` duplicate LCID")]
    fn parse_duplicate_lcid_runk() {
        let raw = from_json!(
            r#"[
            ["0x1", "foo", "None"],
            ["0x1", null, "ReservedUnknown"]
        ]"#
        );
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x2` duplicate Name")]
    fn parse_duplicate_name_normal() {
        let raw = from_json!(
            r#"[
            ["0x1", "foo", "None"],
            ["0x2", "foo", "None"]
        ]"#
        );
        parse(raw);
    }

    #[test]
    #[should_panic(expected = "LCID `0x2` duplicate Name")]
    fn parse_duplicate_name_reserved() {
        let raw = from_json!(
            r#"[
            ["0x1", "foo", "None"],
            ["0x2", "foo", "Reserved"]
        ]"#
        );
        parse(raw);
    }
}
