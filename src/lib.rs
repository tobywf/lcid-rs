//! This crate provides language information and lookups from Microsoft's numeric
//! language code identifiers (LCIDs) or language names (IETF language tags,
//! [`BCP 47`]), similar to - but more limited than - the
//! [`System.Globalization.CultureInfo`] class in .NET. The lookup follows the
//! [`MS-LCID`] "Windows Language Code Identifier Reference" specification.
//! Sort IDs are not supported yet.
//!
//! # Examples
//!
//! ```
//! use lcid::LanguageId;
//! use std::convert::TryInto;
//!
//! let lang: &LanguageId = 1033.try_into().unwrap();
//! assert_eq!(lang.name, "en-US");
//!
//! let lang: &LanguageId = "en-US".try_into().unwrap();
//! assert_eq!(lang.lcid, 1033);
//! ```
//!
//! [`BCP 47`]: https://tools.ietf.org/rfc/bcp/bcp47.txt
//! [`System.Globalization.CultureInfo`]: https://docs.microsoft.com/en-us/dotnet/api/system.globalization.cultureinfo
//! [`MS-LCID`]: https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f
#![deny(
    missing_docs,
    missing_debug_implementations,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused,
    clippy::all,
    clippy::cargo
)]
#[macro_use]
pub mod constants;

use std::convert::TryFrom;
use std::fmt;

/// Errors when looking up a [`LanguageId`] from a numeric ([`u32`]) LCID via
/// [`TryFrom`] or [`TryInto`](std::convert::TryInto).
#[derive(Debug)]
pub enum LcidLookupError {
    /// The LCID has set reserved bits (original value, reserved bits).
    ReservedBits(u32, u32),
    /// The LCID has sort ID bits set (original value, sort ID bits).
    SortIdBits(u32, u32),
    /// The LCID refers to a named reserved language.
    Reserved(u32, &'static str),
    /// The LCID refers to a unknown reserved language.
    ReservedUnknown(u32),
    /// The LCID is neither defined nor reserved.
    NeitherDefinedNorReserved(u32),
    /// The LCID is undefined.
    Undefined(u32),
}

impl fmt::Display for LcidLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReservedBits(lcid, bits) => write!(
                f,
                "LCID {0}/{0:#06x} has reserved bits set: {1}/{1:#x}",
                lcid, bits
            ),
            Self::SortIdBits(lcid, bits) => write!(
                f,
                "LCID {0}/{0:#06x} has sort ID bits set: {1}/{1:#x}",
                lcid, bits
            ),
            Self::Reserved(lcid, language) => {
                write!(f, "LCID {0}/{0:#06x} is reserved (`{1}`)", lcid, language)
            }
            Self::ReservedUnknown(lcid) => {
                write!(f, "LCID {0}/{0:#06x} is reserved (<unknown>)", lcid)
            }
            Self::NeitherDefinedNorReserved(lcid) => {
                write!(f, "LCID {0}/{0:#06x} is neither defined nor reserved", lcid)
            }
            Self::Undefined(lcid) => write!(f, "LCID {0}/{0:#06x} is undefined", lcid),
        }
    }
}

impl std::error::Error for LcidLookupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Errors when looking up a [`LanguageId`] from a named identifier via
/// [`TryFrom`] or [`TryInto`](std::convert::TryInto).
#[derive(Debug)]
pub enum NameLookupError {
    /// The name refers to a reserved language.
    Reserved(&'static str, u32),
    /// The name does not refer to a defined language.
    Undefined(String),
}

impl fmt::Display for NameLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reserved(name, lcid) => {
                write!(f, "Name `{0}` is reserved ({1}/{1:#06x})", name, lcid)
            }
            Self::Undefined(name) => write!(f, "Name `{0}` is undefined", name),
        }
    }
}

impl std::error::Error for NameLookupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// A known ANSI code page. Some languages can be encoded using one of these
/// code pages. This enum has a [`u32`] representation, and so can be converted
/// to the numeric code page value if needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AnsiCodePage {
    /// windows-874, Thai (Windows)
    Windows874 = 874,
    /// shift_jis, ANSI/OEM Japanese; Japanese (Shift-JIS)
    ShiftJIS = 932,
    /// gb2312, ANSI/OEM Simplified Chinese (PRC, Singapore); Chinese Simplified (GB2312)
    GB2312 = 936,
    /// ks_c_5601-1987, ANSI/OEM Korean (Unified Hangul Code)
    KsC5601 = 949,
    /// big5, ANSI/OEM Traditional Chinese (Taiwan; Hong Kong SAR, PRC); Chinese Traditional (Big5)
    Big5 = 950,
    /// windows-1250, ANSI Central European; Central European (Windows)
    Windows1250 = 1250,
    /// windows-1251, ANSI Cyrillic; Cyrillic (Windows)
    Windows1251 = 1251,
    /// windows-1252, ANSI Latin 1; Western European (Windows)
    Windows1252 = 1252,
    /// windows-1253, ANSI Greek; Greek (Windows)
    Windows1253 = 1253,
    /// windows-1254, ANSI Turkish; Turkish (Windows)
    Windows1254 = 1254,
    /// windows-1255, ANSI Hebrew; Hebrew (Windows)
    Windows1255 = 1255,
    /// windows-1256, ANSI Arabic; Arabic (Windows)
    Windows1256 = 1256,
    /// windows-1257, ANSI Baltic; Baltic (Windows)
    Windows1257 = 1257,
    /// windows-1258, ANSI/OEM Vietnamese; Vietnamese (Windows)
    Windows1258 = 1258,
}

impl From<AnsiCodePage> for u32 {
    fn from(value: AnsiCodePage) -> u32 {
        value as u32
    }
}

/// A language's identifiers and information. Lookups from numeric or named
/// identifiers return a reference to statically defined `LanguageId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageId {
    /// A unique name that identifies the language (IETF language tag).
    pub name: &'static str,
    /// The language code ID for the language. This does not identify a
    /// language uniquely.
    pub lcid: u32,
    /// A non-localized, English, readable name for the language.
    pub english_name: &'static str,
    /// A two-letter ISO-639 code for the language.
    pub iso639_two_letter: &'static str,
    /// A three-letter ISO-639 code for the language.
    pub iso639_three_letter: &'static str,
    /// A three-letter code for the language used in the Windows API.
    pub windows_three_letter: &'static str,
    /// The corresponding ANSI code page that may be used to encode this
    /// language, or [`None`] if the language can't be encoded outside Unicode.
    pub ansi_code_page: Option<AnsiCodePage>,
}

const PRIMARY_LANG_ID_MASK: u32 = 0x3ff;
const SUB_LANG_ID_MASK: u32 = 0x3f;
const RESERVED_BITS_MASK: u32 = 0xfff;
const SORT_ID_BITS_MASK: u32 = 0xf;

const PRIMARY_LANG_ID_SHIFT: u32 = 0;
const SUB_LANG_ID_SHIFT: u32 = 10;
const RESERVED_BITS_SHIFT: u32 = 20;
const SORT_ID_BITS_SHIFT: u32 = 16;

impl LanguageId {
    /// The primary language ID part of the language's LCID
    #[inline]
    pub fn primary_language_id(&self) -> u32 {
        // +----------------+---------------------+
        // | SubLanguage ID | Primary Language ID |
        // +----------------+---------------------+
        // 15           10  9                     0 bit
        // primary language ID is 10 bits (0-9)
        (self.lcid >> PRIMARY_LANG_ID_SHIFT) & PRIMARY_LANG_ID_MASK
    }

    /// The sub language ID part of the language's LCID
    #[inline]
    pub fn sub_language_id(&self) -> u32 {
        // +----------------+---------------------+
        // | SubLanguage ID | Primary Language ID |
        // +----------------+---------------------+
        // 15           10  9                     0 bit
        // sub language ID is 6 bits (10-15)
        (self.lcid >> SUB_LANG_ID_SHIFT) & SUB_LANG_ID_MASK
    }
}

impl TryFrom<u32> for &'static LanguageId {
    type Error = LcidLookupError;

    /// Try to identify a [`LanguageId`] from a numeric LCID. This returns an
    /// error if:
    /// * The LCID has any reserved bits set
    /// * The LCID has a sort ID
    /// * The LCID maps to an unknown, reserved, or neither defined nor reserved
    ///   language
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        // +----------+---------+-------------+
        // | Reserved | Sort ID | Language ID |
        // +----------+---------+-------------+
        // 31      20 19     16 15            0 bit
        // reserved is 12 bits (20-31)
        let reserved_bits = (value >> RESERVED_BITS_SHIFT) & RESERVED_BITS_MASK;
        if reserved_bits != 0 {
            return Err(Self::Error::ReservedBits(value, reserved_bits));
        }
        // sort ID is 4 bits (16-19)
        let sort_id_bits = (value >> SORT_ID_BITS_SHIFT) & SORT_ID_BITS_MASK;
        if sort_id_bits != 0 {
            return Err(Self::Error::SortIdBits(value, sort_id_bits));
        }
        // language ID is 16 bits (0-15), primary and sub language ID combined

        // Generated from JSON
        parse_lcid!(value)
    }
}

impl TryFrom<&str> for &'static LanguageId {
    type Error = NameLookupError;

    /// Try to identify a [`LanguageId`] from a name. This returns an
    /// error if the name is unknown or reserved.
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        // Generated from JSON
        parse_name!(value)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn bit_mask_size() {
        assert_eq!(PRIMARY_LANG_ID_MASK.count_ones(), 10);
        assert_eq!(SUB_LANG_ID_MASK.count_ones(), 6);
        assert_eq!(RESERVED_BITS_MASK.count_ones(), 12);
        assert_eq!(SORT_ID_BITS_MASK.count_ones(), 4);
    }

    #[test]
    fn bit_mask_pos() {
        let primary_lang_id: u32 = 0b0000001111111111;
        let sub_lang_id: u32 = 0b1111110000000000;
        let reserved_bits: u32 = 0b1111111111110000 << 16;
        let sort_id_bits: u32 = 0b0000000000001111 << 16;

        assert_eq!(
            PRIMARY_LANG_ID_MASK.count_ones(),
            primary_lang_id.count_ones()
        );
        assert_eq!(SUB_LANG_ID_MASK.count_ones(), sub_lang_id.count_ones());
        assert_eq!(RESERVED_BITS_MASK.count_ones(), reserved_bits.count_ones());
        assert_eq!(SORT_ID_BITS_MASK.count_ones(), sort_id_bits.count_ones());

        assert_eq!(
            PRIMARY_LANG_ID_MASK << PRIMARY_LANG_ID_SHIFT,
            primary_lang_id
        );
        assert_eq!(SUB_LANG_ID_MASK << SUB_LANG_ID_SHIFT, sub_lang_id);
        assert_eq!(RESERVED_BITS_MASK << RESERVED_BITS_SHIFT, reserved_bits);
        assert_eq!(SORT_ID_BITS_MASK << SORT_ID_BITS_SHIFT, sort_id_bits);

        let all_bits = reserved_bits | sort_id_bits | sub_lang_id | primary_lang_id;
        assert_eq!(all_bits, u32::MAX);
        let all_count = reserved_bits.count_ones()
            + sort_id_bits.count_ones()
            + sub_lang_id.count_ones()
            + primary_lang_id.count_ones();
        assert_eq!(all_count, u32::MAX.count_ones());
    }

    #[test]
    fn lcid_conv_reserved() {
        for reserved_bits in 1..=RESERVED_BITS_MASK {
            let reserved = reserved_bits << RESERVED_BITS_SHIFT;
            let err = TryInto::<&LanguageId>::try_into(reserved).unwrap_err();
            assert_matches!(err, LcidLookupError::ReservedBits(v, r) if v == reserved && r == reserved_bits);
        }
    }

    #[test]
    fn lcid_conv_sort_id() {
        for sort_id_bits in 1..=SORT_ID_BITS_MASK {
            let sort_id = sort_id_bits << SORT_ID_BITS_SHIFT;
            let err = TryInto::<&LanguageId>::try_into(sort_id).unwrap_err();
            assert_matches!(err, LcidLookupError::SortIdBits(v, r) if v == sort_id && r == sort_id_bits);
        }
    }

    #[test]
    fn ansi_code_page_to_u32() {
        assert_eq!(AnsiCodePage::Windows1252 as u32, 1252u32);
        let value: u32 = AnsiCodePage::Windows1252.into();
        assert_eq!(value, 1252u32);
    }
}
