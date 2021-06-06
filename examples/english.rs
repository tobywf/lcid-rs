use lcid::LanguageId;
use std::convert::TryInto;

fn main() {
    let lcid = 1033;
    let lang: &LanguageId = lcid.try_into().unwrap();
    println!("LCID {} is '{}'/'{}'", lcid, lang.name, lang.english_name);

    let name = "en-US";
    let lang: &LanguageId = name.try_into().unwrap();
    println!("Name '{}' is {}/'{}'", name, lang.lcid, lang.english_name);
}
