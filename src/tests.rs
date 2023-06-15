use super::*;
use std::convert::TryInto;

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
        match err {
            LcidLookupError::ReservedBits(v, r) if v == reserved && r == reserved_bits => {}
            e => panic!(
                "Expected ReservedBits({}, {}), but got {:?}",
                reserved, reserved_bits, e
            ),
        }
    }
}

#[test]
fn lcid_conv_sort_id() {
    for sort_id_bits in 1..=SORT_ID_BITS_MASK {
        let sort_id = sort_id_bits << SORT_ID_BITS_SHIFT;
        let err = TryInto::<&LanguageId>::try_into(sort_id).unwrap_err();
        match err {
            LcidLookupError::SortIdBits(v, r) if v == sort_id && r == sort_id_bits => {}
            e => panic!(
                "Expected SortIdBits({}, {}), but got {:?}",
                sort_id, sort_id_bits, e
            ),
        }
    }
}

#[test]
fn ansi_code_page_to_u32() {
    assert_eq!(AnsiCodePage::Windows1252 as u32, 1252u32);
    let value: u32 = AnsiCodePage::Windows1252.into();
    assert_eq!(value, 1252u32);
}
