use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use lcid_gen::{CultureInfo, Numbered};

fn write_to_file<P, F>(path: P, func: F) -> std::io::Result<()>
where
    P: AsRef<Path>,
    F: FnOnce(&mut BufWriter<File>) -> std::io::Result<()>,
{
    let mut file = BufWriter::new(File::create(path)?);
    func(&mut file)
}

fn name_to_ident(tag: &str) -> String {
    if tag.is_empty() {
        "LANG_INVARIANT".to_owned()
    } else {
        format!("LANG_{}", tag.replace("-", "_").to_ascii_uppercase())
    }
}

fn dump_ci(
    identifiers: &HashMap<String, String>,
    culture_infos: HashMap<String, CultureInfo>,
) -> (String, String) {
    let mut culture_infos: Vec<_> = culture_infos.into_iter().collect();
    culture_infos.sort_by_cached_key(|item| (item.1.lcid, item.1.name.to_owned()));

    let mut ci_dump = String::new();

    let mut lcid_consts = String::new();
    lcid_consts.push_str("pub mod lcid {\n");
    lcid_consts.push_str(
        "    //! Contains constant LCIDs corresponding to the full language information\n",
    );
    lcid_consts.push_str("    //! in the parent module, for easy use in e.g. match statements.\n");

    for (name, ci) in culture_infos {
        let identifier = identifiers.get(&name).expect("Identifier not found");
        ci_dump.push_str(&format!("/// {}\n", ci.english_name));
        ci_dump.push_str(&format!(
            "pub const {}: &LanguageId = &LanguageId {{\n",
            identifier
        ));

        ci_dump.push_str(&format!("    name: \"{}\",\n", ci.name));
        ci_dump.push_str(&format!("    lcid: {:#06X},\n", ci.lcid));
        ci_dump.push_str(&format!("    english_name: \"{}\",\n", ci.english_name));
        ci_dump.push_str(&format!(
            "    iso639_two_letter: \"{}\",\n",
            ci.iso639_two_letter
        ));
        ci_dump.push_str(&format!(
            "    iso639_three_letter: \"{}\",\n",
            ci.iso639_three_letter
        ));
        ci_dump.push_str(&format!(
            "    windows_three_letter: \"{}\",\n",
            ci.windows_three_letter
        ));
        if ci.ansi_code_page == 0 {
            ci_dump.push_str("    ansi_code_page: None,\n");
        } else {
            let cp = match ci.ansi_code_page {
                932 => "ShiftJIS".to_owned(),
                936 => "GB2312".to_owned(),
                949 => "KsC5601".to_owned(),
                950 => "Big5".to_owned(),
                cp => format!("Windows{}", cp),
            };
            ci_dump.push_str(&format!(
                "    ansi_code_page: Some(AnsiCodePage::{}),\n",
                &cp
            ));
        }

        ci_dump.push_str("};\n\n");

        lcid_consts.push_str(&format!("    /// {}\n", ci.english_name));
        lcid_consts.push_str(&format!(
            "    pub const {}: u32 = {:#06X};\n",
            identifier.replace("LANG_", "LCID_"),
            ci.lcid
        ));
    }

    lcid_consts.push_str("}\n");

    (ci_dump, lcid_consts)
}

const NAMED_LCID: u32 = 0x1000;

fn parse_ms_lcid<P: AsRef<Path>>(
    numbered: Vec<Numbered>,
    named: Vec<String>,
    culture_infos: HashMap<String, CultureInfo>,
    out_path: P,
) {
    let identifiers: HashMap<String, String> = culture_infos
        .iter()
        .map(|(key, _value)| (key.to_owned(), name_to_ident(key)))
        .collect();

    let mut lookup_lcid = String::new();
    lookup_lcid.push_str("macro_rules! parse_lcid {\n");
    lookup_lcid.push_str("    ($value:expr) => {\n");
    lookup_lcid.push_str("        match $value {\n");

    let mut lookup_name = String::new();
    lookup_name.push_str("macro_rules! parse_name {\n");
    lookup_name.push_str("    ($value:expr) => {\n");
    lookup_name.push_str("        match $value {\n");

    let indent = "            ";

    for value in numbered {
        lookup_lcid.push_str(indent);
        match value {
            Numbered::NeitherDefinedNorReserved(lcid) => {
                lookup_lcid.push_str(&format!(
                    "{0:#06X} => Err(Self::Error::NeitherDefinedNorReserved({0:#06X})),\n",
                    lcid
                ));
                // no corresponding name option
            }
            Numbered::ReservedUnknown(lcid) => {
                lookup_lcid.push_str(&format!(
                    "{:#06X} => Err(Self::Error::ReservedUnknown({0:#06X})),\n",
                    lcid
                ));
                // no corresponding name option
            }
            Numbered::Reserved(name, lcid) => {
                lookup_lcid.push_str(&format!(
                    "{:#06X} => Err(Self::Error::Reserved({0:#06X}, \"{}\")),\n",
                    lcid, name
                ));
                lookup_name.push_str(indent);
                lookup_name.push_str(&format!(
                    "\"{0}\" => Err(Self::Error::Reserved(\"{0}\", {1})),\n",
                    name, lcid
                ));
            }
            Numbered::Normal(name, lcid) => {
                // Some extra validation
                let culture_info = culture_infos
                    .get(&name)
                    .unwrap_or_else(|| panic!("Culture info for '{}' not found", &name));
                if culture_info.name != name {
                    panic!("Culture info name '{}' != '{}", culture_info.name, name);
                }
                if culture_info.lcid != lcid {
                    panic!("Culture info LCID '{}' != '{}", culture_info.lcid, lcid);
                }
                let identifier = identifiers.get(&name).expect("Identifier not found");

                lookup_lcid.push_str(&format!(
                    "{:#06X} => Ok(constants::{}),\n",
                    lcid, identifier
                ));
                lookup_name.push_str(indent);
                lookup_name.push_str(&format!("\"{}\" => Ok(constants::{}),\n", name, identifier));
            }
        }
    }

    lookup_lcid.push_str("            undef => Err(Self::Error::Undefined(undef)),\n");
    lookup_lcid.push_str("        }\n");
    lookup_lcid.push_str("    };\n");
    lookup_lcid.push_str("}\n");

    for name in named {
        // Some extra validation
        let culture_info = culture_infos
            .get(&name)
            .unwrap_or_else(|| panic!("Culture info for '{}' not found", &name));
        if culture_info.name != name {
            panic!("Culture info name '{}' != '{}", culture_info.name, name);
        }
        if culture_info.lcid != NAMED_LCID {
            panic!(
                "Culture info LCID '{}' != '{}",
                culture_info.lcid, NAMED_LCID
            );
        }
        let identifier = identifiers.get(&name).expect("Identifier not found");

        lookup_name.push_str(indent);
        lookup_name.push_str(&format!("\"{}\" => Ok(constants::{}),\n", name, identifier));
    }

    lookup_name.push_str("            undef => Err(Self::Error::Undefined(undef.to_owned())),\n");
    lookup_name.push_str("        }\n");
    lookup_name.push_str("    };\n");
    lookup_name.push_str("}\n");

    let (ci_dump, lcid_consts) = dump_ci(&identifiers, culture_infos);

    let mut header = String::new();
    header.push_str("//! Contains all defined [`LanguageId`] returned by the lookups.\n");
    header.push_str("use crate::{AnsiCodePage, LanguageId};\n");

    write_to_file(out_path, |file| {
        write!(
            file,
            "{}\n{}{}\n{}\n{}",
            &header, &ci_dump, &lookup_lcid, &lookup_name, &lcid_consts
        )
    })
    .expect("Failed to write string");
}

fn main() {
    let numbered = lcid_gen::read_numbered("lcid_gen/ms-lcid-14-1-numbered.json");
    let named = lcid_gen::read_named("lcid_gen/ms-lcid-14-1-named.json");
    let culture_infos = lcid_gen::read_culture_info("lcid_gen/culture-infos.json");

    parse_ms_lcid(numbered, named, culture_infos, "src/constants.rs");
}
