# LCID-rs: A Rust library for Windows Language Code Identifiers and other language/culture information

[![crates.io](https://img.shields.io/crates/v/lcid.svg)](https://crates.io/crates/lcid) [![docs.rs](https://docs.rs/lcid/badge.svg)](https://docs.rs/lcid/) [![GitHub CI](https://github.com/tobywf/lcid-rs/actions/workflows/check.yaml/badge.svg)](https://github.com/tobywf/lcid-rs/)

[[Repository](https://github.com/tobywf/lcid-rs/)] [[Documentation](https://docs.rs/lcid/)] [[Crate Registry (crates.io)](https://crates.io/crates/lcid)]

---

This crate provides language code identifier parsing and information according to the [[MS-LCID] Windows Language Code Identifier (LCID) Reference](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f) and [`System.Globalization.CultureInfo` API](https://docs.microsoft.com/en-us/dotnet/api/system.globalization.cultureinfo).

The following information is provided:

* Language Code Identifier/LCID (`lcid`), and lookup by LCID
* Name/IETF language tag (`name`), and lookup by name
* A non-localised, English readable language name (`english_name`)
* ISO 639-1 two-letter code (`iso639_two_letter`) - note this is not always two letters
* ISO 639-2/639-3 three-letter code (`iso639_three_letter`)
* The Windows API three-letter language code (`windows_three_letter`)
* ANSI code page (`ansi_code_page`), if available

To use this crate, add the following to your `Cargo.toml`:

```toml
[dependencies]
lcid = "0.3"
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

This library currently tracks the `15.0`/2021-06-25 protocol revision. Future protocol revisions will may only trigger a minor version bump, so if you need lookup behaviour of a specific revision, pin this crate accordingly.

## Changelog

### [0.3.0] - 2023-06-15

* Tracks MS-LCID `15.0`/2021-05-25 protocol revision
* Breaking change: As the spec no longer enumerates "Locale Names without LCIDs", these are no longer supported
* Codegen: Sort order is now as specified in the MS-LCID spec

### [0.2.1] - 2023-06-10

* Remove `thiserror` dependency
* MSRV is Rust 1.56
* Edition is 2021
* Add `PartialEq`, `Eq`, and `Hash` traits to `AnsiCodePage` and `LanguageId`

### [0.2.0] - 2021-06-08

* Tracks MS-LCID `14.1`/2021-04-07 protocol revision
* Provide ANSI code page information
* Move `LanguageId` constants to a module, to avoid cluttering the crate namespace (breaking change)
* Codegen: Sort languages by LCID and name, so the generated code is stable for languages that share an LCID (`0x1000` ones)

### [0.1.0] - 2021-06-06

* Initial release

## How the information was generated

First, information was extracted from the [MS-LCID PDF](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f) corresponding to the tracked protocol revision, and from the HTML table of the [associated LCIDs](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/63d3d639-7fd2-4afb-abbe-0d5b5551eef8). This was then manually cleaned, converted to JSON, and compared.

The `GetCultureInfo.ps1` script was run on a Windows Server 2022 machine (Build 20348, locale "en-us") and a Windows 10 (Build 19045, locale "en-us") to gather further information from the `System.Globalization.CultureInfo` API, based on the language IDs in MS-LCID. The values returned by the API do not always match the information in MS-LCID, so some fix-up were applied. For details, please see [`lcid_gen`](lcid_gen/src/). Since there were differences between the output on Windows Server 2022 and Windows 10, additional fix-ups were applied so that the information matches. Many of these are listed in the errata section.

Finally, the `lcid_gen` crate was invoked to generate code for the `lcid` crate. The generated code is committed to the repository. This is done to avoid having a build-time dependency on the JSON files.

## MS-LCID/CultureInfo errata

### Protocol revision `15.0`/2021-06-25

* The download link for the diff file is [incorrect](https://winprotocoldoc.blob.core.windows.net/productionwindowsarchives/MS-LCID/%5bMS-LCID%5d-210625-diff.pdf) and points to `[MS-LCID]-210625-diff.pdf`; the [correct link](https://winprotocoldoc.blob.core.windows.net/productionwindowsarchives/MS-LCID/%5bMS-LCID%5d-diff.pdf) points to `[MS-LCID]-diff.pdf`.
* The language ID for "quz-PE" is misprinted as `0x0C6b`. It should be `0x0C6B`, as all other language IDs are upper-cased hexadecimal. This does no affect lcid-rs.
* On some versions (Windows 10 only?), the culture information's name for "zh-Hans"/`0x0004` is returned as "zh-CHS", and the name for "zh-Hant"/`0x7C04` is returned as "zh-CHT". These are legacy names. This is [a known problem](https://social.msdn.microsoft.com/Forums/en-US/8b93c07b-93bd-465f-b48f-0fff544c06d8/), which [Microsoft acknowledges](https://learn.microsoft.com/en-us/dotnet/api/system.globalization.cultureinfo):
  > There are two culture names that contradict this rule. The cultures Chinese (Simplified), named `zh-Hans`, and Chinese (Traditional), named `zh-Hant`, are neutral cultures. The culture names represent the current standard and should be used unless you have a reason for using the older names `zh-CHS` and `zh-CHT`.

  lcid-rs uses the names "zh-Hans"/"zh-Hant", and the English Names "Chinese (Simplified)"/"Chinese (Traditional)" (without the suffix "Legacy"). However, lcid-rs uses the Windows API three letter language code "CHT" instead of the sometimes used "ZHH" for "zh-Hant".
* The culture information for "qut"/`0x0086` is quite broken. On Windows Server 2022, the LCID, ISO 639, and English Name are wrong or incomplete. On Windows 10, the culture information returned seems to be for "quc"/`0x0093`, which is reserved. This also means the culture information name does not match the MS-LCID name. lcid-rs v0.2 used to change this, but lcid-rs v0.3 uses the culture information as returned on Windows 10 when it was built, even though this seems to violate MS-LCID.
* The MS-LCID spec specified "ff-NG, ff-Latn-NG" for `0x0467`. The culture information returned has the name "ff-Latn-NG". lcid-rs uses "ff-Latn-NG".
* The culture information for "la-VA"/`0x0476` is a mess. When queried by LCID, the name is "la-001", and the English Name is "Latin (World)" (instead of "Latin (Vatican City)"). When queried by name, the LCID is incorrect (`0x1000`), and sometimes the English Name also. lcid-rs uses "la-VA" and "Latin (Vatican City)", as this is what is returned when queried by name. This also matches MS-LCID, which does not specify "la-001".
* The culture information name for "es-ES_tradnl"/`0x040A` is "es-ES". However, the LCID, English Name, and Windows API three letter language code will be different from "es-ES"/`0x0C0A`. lcid-rs does not change this.
* The ISO 639 two and three letter language codes for "no"/`0x0014` are confusing. On Windows Server 2022, they are "no"/"nor". On Windows 10, they seem to be "nb"/"nob" for "Bokmål". If you are Norwegian, please weigh in. lcid-rs uses "nb"/"nob".
* Further small fix-ups to some English Names are documented in [`lcid_gen/src/fixup.rs`](lcid_gen/src/fixup.rs). Generally, a preference was given to the values returned by Windows 10.

### Protocol revision `14.1`/2021-04-07

* "es-CU" is listed twice. Once as `0x5C0A` in the "Language ID" table, and once in the "Locale Names without LCIDs" table as `0x1000`. The former LCID was used.
* "ff-Latn-GM" is misprinted as "ff-latn-GM" (lower-case "l"). This was corrected.
* Many more culture information errata/fix-ups.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
