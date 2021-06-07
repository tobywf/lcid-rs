# LCID-rs: A Rust library for Windows Language Code Identifiers

[![crates.io](https://img.shields.io/crates/v/lcid.svg)](https://crates.io/crates/lcid)

This library provides language code identifier parsing and information
according to the [MS-LCID "Windows Language Code Identifier (LCID) Reference"](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f).

```toml
[dependencies]
lcid = "0.1"
```

Language identifiers can be queried from a 32-bit unsigned integer (Language
Code Identifier, or LCID) or a string (name, i.e. supported [IETF BCP 47 language tag](https://tools.ietf.org/rfc/bcp/bcp47.txt)):

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

The information provided in `LanguageId` is:

* Language Code Identifier/LCID (`lcid`)
* Name/IETF language tag (`name`)
* A non-localised, English readable language name (`english_name`)
* ISO 639-1 two-letter code (`iso639_two_letter`)
* ISO 639-2/639-3 three-letter code (`iso639_three_letter`)
* The Windows API three-letter language code (`windows_three_letter`)

It currently tracks the `14.1`/2021-07-04 protocol revision. Future protocol
revisions will may only trigger a minor version bump, so if you need a specific
revision, pin this crate accordingly.

## Changelog

### [0.1.0] - 2021-06-06

* Initial release

## How the information was generated

First, information was extracted from the [MS-LCID PDF](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f),
and from both HTML tables of the [associated LCIDs](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/63d3d639-7fd2-4afb-abbe-0d5b5551eef8)
("numbered") and the [unassociated LCIDs](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/926e694f-1797-4418-a922-343d1c5e91a6)
("named"). This was then manually cleaned, converted to JSON, and compared.

Then, the `GetCultureInfo.ps1` script was run on a Windows Server 2019 machine
(Build 17763) to gather further information from the
`System.Globalization.CultureInfo` API. The values returned by the API do not
always match the information in MS-LCID, so some fix-up were applied. For
details, please [see the script](lcid_gen/GetCultureInfo.ps1).

Finally, the `lcid-gen` crate generates code for the `lcid` crate
(`src/gen.rs`). This is done to avoid having a build-time dependency on the
JSON files.

## MS-LCID errata

* "es-CU" is listed twice. Once as `0x5C0A` in the "Language ID" table, and
  once in the "Locale Names without LCIDs" table as `0x1000`. The former LCID
  was used.
* "ff-Latn-GM" is misprinted as "ff-latn-GM" (lower-case "l"). This was
  corrected.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
