use assert_matches::assert_matches;
use lcid::{LanguageId, LcidConversionError};
use lcid_gen::{CultureInfo, Numbered};
use std::collections::HashMap;
use std::convert::TryInto;

struct Fixture {
    numbered: Vec<Numbered>,
    named: Vec<String>,
    culture_infos: HashMap<String, CultureInfo>,
}

fn setup() -> Fixture {
    let numbered = lcid_gen::read_numbered("ms-lcid-14-1-numbered.json");
    let named = lcid_gen::read_named("ms-lcid-14-1-named.json");
    let culture_infos = lcid_gen::read_culture_info("culture-infos.json");

    Fixture {
        numbered,
        named,
        culture_infos,
    }
}

fn assert_lang_eq_ci(lang: &LanguageId, ci: &CultureInfo) {
    assert_eq!(lang.lcid, ci.lcid);
    assert_eq!(lang.name, &ci.name);
    assert_eq!(lang.english_name, &ci.english_name);
    assert_eq!(lang.iso639_two_letter, &ci.iso639_two_letter);
    assert_eq!(lang.iso639_three_letter, &ci.iso639_three_letter);
    assert_eq!(lang.windows_three_letter, &ci.windows_three_letter);
}

#[test]
fn numbered_json() {
    let fixture = setup();

    for value in fixture.numbered {
        match value {
            Numbered::Normal(name, lcid) => {
                let lang: &LanguageId = lcid.try_into().unwrap();
                assert_eq!(lang.lcid, lcid);
                assert_eq!(lang.name, &name);

                let ci = fixture.culture_infos.get(&name).unwrap();
                assert_lang_eq_ci(lang, ci);
            }
            Numbered::Reserved(name, lcid) => {
                let err = TryInto::<&LanguageId>::try_into(lcid).unwrap_err();
                assert_matches!(err, LcidConversionError::Reserved(l, n) if l == lcid && n == &name);
            }
            Numbered::ReservedUnknown(lcid) => {
                let err = TryInto::<&LanguageId>::try_into(lcid).unwrap_err();
                assert_matches!(err, LcidConversionError::ReservedUnknown(l) if l == lcid);
            }
            Numbered::NeitherDefinedNorReserved(lcid) => {
                let err = TryInto::<&LanguageId>::try_into(lcid).unwrap_err();
                assert_matches!(err, LcidConversionError::NeitherDefinedNorReserved(l) if l == lcid);
            }
        }
    }
}

#[test]
fn named_json() {
    let fixture = setup();

    for name in fixture.named {
        let n: &str = &name;
        let lang: &LanguageId = n.try_into().unwrap();
        assert_eq!(lang.name, &name);

        let ci = fixture.culture_infos.get(&name).unwrap();
        assert_lang_eq_ci(lang, ci);
    }
}
