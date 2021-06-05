/// This crate is based on MS-LCID "Windows Language Code Identifier Reference".
/// Sort IDs are not supported (yet).
///
/// PDF:
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f
/// Numbered Table:
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/63d3d639-7fd2-4afb-abbe-0d5b5551eef8
/// Named Table:
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/926e694f-1797-4418-a922-343d1c5e91a6
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LcidConversionError {
    #[error("LCID {0}/{0:#06x} has reserved bits set: {1}/{1:#x}")]
    ReservedBits(u32, u32),
    #[error("LCID {0}/{0:#06x} has sort ID bits set: {1}/{1:#x}")]
    SortIdBits(u32, u32),
    #[error("LCID {0}/{0:#06x} is reserved ('{1}')")]
    Reserved(u32, &'static str),
    #[error("LCID {0}/{0:#06x} is reserved (<unknown>)")]
    ReservedUnknown(u32),
    #[error("LCID {0}/{0:#06x} is undefined")]
    Undefined(u32),
    #[error("LCID {0}/{0:#06x} is neither defined no reserved")]
    NeitherDefinedNorReserved(u32),
}

#[derive(Error, Debug)]
pub enum NameConversionError {
    #[error("Name '{0}' is reserved ({1}/{1:#06x})")]
    Reserved(&'static str, u32),
    #[error("Name '{0}' is undefined")]
    Undefined(String),
}

#[derive(Clone, Debug)]
pub struct LanguageId {
    /// The language identifier's name/IETF tag
    pub name: &'static str,
    /// The language identifier's combined language code ID
    pub lcid: u32,
    pub english_name: &'static str,
    pub iso639_two_letter: &'static str,
    pub iso639_three_letter: &'static str,
    pub windows_three_letter: &'static str,
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
    /// The language identifier's primary language code ID
    #[inline]
    pub fn primary_language_id(&self) -> u32 {
        // +----------------+---------------------+
        // | SubLanguage ID | Primary Language ID |
        // +----------------+---------------------+
        // 15           10  9                     0 bit
        // primary language ID is 10 bits (0-9)
        (self.lcid >> PRIMARY_LANG_ID_SHIFT) & PRIMARY_LANG_ID_MASK
    }

    /// The language identifier's sub language code ID
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

include!(concat!(env!("OUT_DIR"), "/ms-lcid-14-1.rs"));

impl TryFrom<u32> for &'static LanguageId {
    type Error = LcidConversionError;

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
    type Error = NameConversionError;

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
            assert_matches!(err, LcidConversionError::ReservedBits(v, r) if v == reserved && r == reserved_bits);
        }
    }

    #[test]
    fn lcid_conv_sort_id() {
        for sort_id_bits in 1..=SORT_ID_BITS_MASK {
            let sort_id = sort_id_bits << SORT_ID_BITS_SHIFT;
            let err = TryInto::<&LanguageId>::try_into(sort_id).unwrap_err();
            assert_matches!(err, LcidConversionError::SortIdBits(v, r) if v == sort_id && r == sort_id_bits);
        }
    }
}
