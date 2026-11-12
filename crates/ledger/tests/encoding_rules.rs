//! The encoding rules from the design's "Ledger format" section, pinned on the borsh crate before
//! any ledger type exists. If a borsh upgrade changes one of these bytes, or starts accepting input
//! it rejects today, a test here fails.

mod golden;

use std::collections::BTreeMap;
use std::fmt::Debug;

use borsh::{BorshDeserialize, BorshSerialize};
use golden::{GoldenSet, golden_set};

/// Struct fields are written in declaration order, with no names, tags or padding.
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct SampleStruct {
    flag: bool,
    count: u32,
    label: String,
}

/// Explicit variant numbers, as the record catalogue will use them. Numbers 1 and 3 to 6 are
/// gaps: never used yet, or used by a newer version of Betty.
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
enum SampleEnum {
    Empty = 0,
    Amount(u64) = 2,
    Named { name: String } = 7,
}

/// Records the encoding of `value` in `set`, and checks that those bytes decode back to the same
/// value.
fn pin<T>(set: &mut GoldenSet, name: &str, value: &T)
where
    T: BorshSerialize + BorshDeserialize + PartialEq + Debug,
{
    let bytes = borsh::to_vec(value).expect("encoding into a Vec cannot fail");
    set.record(name, &bytes);
    let decoded: T =
        borsh::from_slice(&bytes).unwrap_or_else(|e| panic!("{name} does not decode: {e}"));
    assert_eq!(
        &decoded, value,
        "{name} does not decode back to the same value"
    );
}

/// Pins the bytes of each value shape the ledger uses to the design's encoding rules.
///
/// Ids and signatures cover these bytes, so a borsh upgrade that moved one would split phones.
/// Encodes a sample of each shape, decodes it back, and compares the bytes with
/// `vectors/encoding.txt`.
#[test]
fn golden_encoding_rules() {
    let mut set = golden_set!("encoding");
    // Integers are fixed-width little-endian.
    pin(&mut set, "u8", &0xab_u8);
    pin(&mut set, "u16_little_endian", &0x1234_u16);
    pin(&mut set, "u32_fixed_width", &1_u32);
    pin(&mut set, "u64_max", &u64::MAX);
    pin(&mut set, "i64_minus_one", &-1_i64);
    // A bool is one byte, 0 or 1.
    pin(&mut set, "bool_false", &false);
    pin(&mut set, "bool_true", &true);
    // A string is UTF-8 after a u32 length that counts bytes, not characters.
    pin(&mut set, "string_ascii", &String::from("Betty"));
    pin(&mut set, "string_utf8", &String::from("Līga"));
    // An Option is a 0 or 1 tag, then the value if there is one.
    pin(&mut set, "option_none", &None::<u16>);
    pin(&mut set, "option_some", &Some(7_u16));
    // A Vec has a u32 length first. A fixed array, such as a key or a signature, is written as
    // it is.
    pin(&mut set, "vec_u16", &Vec::from([1_u16, 2]));
    pin(&mut set, "array_u8_4", &[0xde_u8, 0xad, 0xbe, 0xef]);
    // A map is a u32 count, then its entries in ascending key order, whatever order they were
    // inserted in.
    pin(
        &mut set,
        "btreemap_ascending",
        &BTreeMap::from([(2_u8, 20_u16), (1, 10)]),
    );
    pin(
        &mut set,
        "struct_fields_in_order",
        &SampleStruct {
            flag: true,
            count: 3,
            label: String::from("pot"),
        },
    );
    // An enum writes its explicit variant number as one byte, then the variant's fields.
    pin(&mut set, "enum_unit_variant", &SampleEnum::Empty);
    pin(&mut set, "enum_tuple_variant", &SampleEnum::Amount(500));
    pin(
        &mut set,
        "enum_struct_variant",
        &SampleEnum::Named {
            name: String::from("Anna"),
        },
    );
    set.finish();
}

/// Asserts that `bytes` don't decode as `T`, for the reason `fragment` names in borsh's error
/// message.
fn assert_rejected<T: BorshDeserialize + Debug>(bytes: &[u8], fragment: &str) {
    match borsh::from_slice::<T>(bytes) {
        Ok(value) => {
            panic!("{bytes:02x?} decoded to {value:?}; expected an error mentioning {fragment:?}")
        }
        Err(e) => assert!(
            e.to_string().contains(fragment),
            "{bytes:02x?} was rejected with {e:?}; expected an error mentioning {fragment:?}"
        ),
    }
}

/// A bool byte other than 0 or 1 fails to decode.
///
/// If 2 also decoded as true, one value would have two encodings, and so two ids. Decodes `[2]`
/// and expects borsh's bool error.
#[test]
fn rejects_bool_other_than_0_or_1() {
    assert_rejected::<bool>(&[2], "bool");
}

/// An Option tag other than 0 or 1 fails to decode.
///
/// A second tag meaning Some would give one value two encodings, and so two ids. Decodes tag 2
/// and a byte as `Option<u8>`, and expects borsh's Option error.
#[test]
fn rejects_option_tag_other_than_0_or_1() {
    assert_rejected::<Option<u8>>(&[2, 0], "Option");
}

/// Input with bytes left over after the value fails to decode.
///
/// If extra bytes were ignored, every value would have endless encodings, each with its own id.
/// Decodes a u16 followed by one extra byte and expects borsh's leftover-bytes error.
#[test]
fn rejects_trailing_bytes() {
    assert_rejected::<u16>(&[0x34, 0x12, 0], "Not all bytes read");
}

/// Input shorter than its type fails to decode.
///
/// Bytes cut short in transit must be refused cleanly, never padded or panicked on. Decodes three
/// bytes as a u32 and expects borsh's length error.
#[test]
fn rejects_truncated_input() {
    assert_rejected::<u32>(&[1, 0, 0], "Unexpected length");
}

/// A string whose bytes aren't valid UTF-8 fails to decode.
///
/// The design checks strings on decode, so no invalid text reaches the log. Decodes a one-byte
/// string holding 0xff and expects borsh's UTF-8 error.
#[test]
fn rejects_invalid_utf8() {
    assert_rejected::<String>(&[1, 0, 0, 0, 0xff], "utf-8");
}

/// A variant number in a gap of the explicit numbering fails to decode.
///
/// A phone must refuse a number it doesn't know, such as one from a newer Betty, rather than
/// guess. Decodes `[1]`, a gap in `SampleEnum`, and expects borsh's variant error.
#[test]
fn rejects_unknown_enum_variant() {
    assert_rejected::<SampleEnum>(&[1], "variant");
}

/// A map whose keys aren't in ascending order fails to decode.
///
/// Without borsh's `de_strict_order` feature they are sorted silently, so two encodings, with two
/// ids, would give one map. Decodes two entries, key 2 before key 1, and expects the
/// ascending-order error.
#[test]
fn rejects_map_keys_out_of_order() {
    assert_rejected::<BTreeMap<u8, u16>>(&[2, 0, 0, 0, 2, 20, 0, 1, 10, 0], "ascending order");
}

/// A map with the same key twice fails to decode.
///
/// Without `de_strict_order`, borsh keeps the last repeat silently, so two encodings would give
/// one map. Decodes two entries with key 1 and expects the ascending-order error.
#[test]
fn rejects_repeated_map_keys() {
    assert_rejected::<BTreeMap<u8, u16>>(&[2, 0, 0, 0, 1, 10, 0, 1, 20, 0], "ascending order");
}
