# LCID-rs: A Rust library for Windows Language Code Identifiers and other language/culture information

[![crates.io](https://img.shields.io/crates/v/lcid.svg)](https://crates.io/crates/lcid) [![docs.rs](https://docs.rs/lcid/badge.svg)](https://docs.rs/lcid/) [![GitHub CI](https://github.com/tobywf/lcid-rs/actions/workflows/check.yaml/badge.svg)](https://github.com/tobywf/lcid-rs/)

[[Repository](https://github.com/tobywf/lcid-rs/)] [[Documentation](https://docs.rs/lcid/)] [[Crate Registry (crates.io)](https://crates.io/crates/lcid)]

---

This crate provides language code identifier parsing and information
according to the [[MS-LCID] Windows Language Code Identifier (LCID) Reference](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f) and [`System.Globalization.CultureInfo` API](https://docs.microsoft.com/en-us/dotnet/api/system.globalization.cultureinfo).

The following information is provided:

* Language Code Identifier/LCID (`lcid`), and lookup by LCID
* Name/IETF language tag (`name`), and lookup by name
* A non-localised, English readable language name (`english_name`)
* ISO 639-1 two-letter code (`iso639_two_letter`)
* ISO 639-2/639-3 three-letter code (`iso639_three_letter`)
* The Windows API three-letter language code (`windows_three_letter`)
* ANSI code page (`ansi_code_page`)

To use this crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
lcid = "0.2"
```

Language identifiers/information can be queried by Language
Code Identifier (LCID, a 32-bit unsigned integer), name (a string, i.e. supported [IETF BCP 47 language tags](https://tools.ietf.org/rfc/bcp/bcp47.txt)), or by directly referring to the language identifier constant:

```rust
use lcid::LanguageId;
use std::convert::TryInto;

fn main() {
    let lang: &LanguageId = 1033.try_into().unwrap();
    println!("Lang is '{}'/{}/'{}'", lang.name, lang.lcid, lang.english_name);

    let lang: &LanguageId = "en-US".try_into().unwrap();
    println!("Lang is '{}'/{}/'{}'", lang.name, lang.lcid, lang.english_name);

    let lang: &LanguageId = lcid::constants::LANG_EN_US;
    println!("Lang is '{}'/{}/'{}'", lang.name, lang.lcid, lang.english_name);
}
```

This prints the following for each:

```
Lang is 'en-US'/1033/'English (United States)'
```

## Project name and status

I struggle to find a good name for this. "locale-info" might be misleading (might imply some kind of POSIX locale support), or "culture-info" implying more than the project offers (like calendar information). In the end, I chose "lcid-rs", because "lcid" is ambiguous/hard to search for, although I named the crate itself "lcid" because in the context of Rust, "lcid" is not ambiguous. It'd be nice if this project was referred to as "lcid-rs" in ambiguous contexts (linking to the repo, blog posts, etc), and "lcid" only in Rust code/configuration.

The maintenance status is "as-is". I'm happy to accept pull requests for corrections (as long as they align with MS-LCID and the Windows API), pull requests for new features, and pull requests for new MS-LCID protocol revisions in the future.

## MS-LCID protocol revision

This library currently tracks the `14.1`/2021-04-07 protocol revision. Future
protocol revisions will may only trigger a minor version bump, so if you need
lookup behaviour of a specific revision, pin this crate accordingly.

## Changelog

### [0.2.1] - 2023-06-09

* Remove `thiserror` dependency
* MSRV is Rust 1.56
* Edition is 2021

### [0.2.0] - 2021-06-08

* Tracks MS-LCID `14.1`/2021-04-07 protocol revision
* Provide ANSI code page information
* Move `LanguageId` constants to a module, to avoid cluttering the crate
  namespace (breaking change)
* Codegen: Sort languages by LCID and name, so the generated code is stable for
  languages that share an LCID (`0x1000` ones)

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

### Protocol revision `14.1`/2021-04-07

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
