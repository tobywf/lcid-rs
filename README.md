# LCID-rs: A Rust library for Windows Language Code Identifiers

This library provides language code identifier parsing and information according to [MS-LCID](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f).

It currently tracks the `14.1`/2021-07-04 protocol revision. If you require that specific revision, please pin to version `0.1` of this crate.

Language identifiers can be queried from a 32-bit unsigned integer (Language Code Identifier, or LCID) or a string (name, i.e. supported [IETF BCP 47 language tag](https://tools.ietf.org/rfc/bcp/bcp47.txt)):

```rust
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
```

prints

```
LCID 1033 is 'en-US'/'English (United States)'
Name 'en-US' is 1033/'English (United States)'
```

## MS-LCID specification errata

* "es-CU" is listed as `0x5C0A` in the Language ID table, and in the Locale Names without LCIDs table as `0x1000`.
* "ff-Latn-GM" is misprinted as "ff-latn-GM" (lower-case "l").

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](lcid/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](lcid/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
