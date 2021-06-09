use lcid::LanguageId;
use std::convert::TryInto;

fn main() {
    let lang: &LanguageId = 1033.try_into().unwrap();
    println!(
        "Lang is '{}'/{}/'{}'",
        lang.name, lang.lcid, lang.english_name
    );

    let lang: &LanguageId = "en-US".try_into().unwrap();
    println!(
        "Lang is '{}'/{}/'{}'",
        lang.name, lang.lcid, lang.english_name
    );

    let lang: &LanguageId = lcid::constants::LANG_EN_US;
    println!(
        "Lang is '{}'/{}/'{}'",
        lang.name, lang.lcid, lang.english_name
    );
}
