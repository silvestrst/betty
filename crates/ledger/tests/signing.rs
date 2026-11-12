//! Signing, verification and ids for the signed envelope, checked before any real record exists. A
//! stand-in `Note` body is signed with the key from RFC 8032's first test vector, so the golden
//! vectors pin the whole scheme: object-type tags and domains, the public key, the bytes a
//! signature covers, the signature and the id.

mod golden;

use std::collections::BTreeSet;

use borsh::{BorshDeserialize, BorshSerialize};
use golden::golden_set;
use hex::FromHex;
use ledger::primitives::{
    Object, ObjectType, PublicKey, Signature, Signed, SigningKey, VerifyError,
};

/// The secret key of RFC 8032's first Ed25519 test vector.
const RFC8032_SEED: &str = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
/// The public key of the same test vector.
const RFC8032_PUBLIC_KEY: &str = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";

/// A stand-in body until the real records exist, signed as a mark.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
struct Note {
    text: String,
    amount: u64,
}

impl Object for Note {
    const TYPE: ObjectType = ObjectType::Mark;
}

/// The same bytes as a `Note`, signed as an entry instead.
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
struct NoteAsEntry(Note);

impl Object for NoteAsEntry {
    const TYPE: ObjectType = ObjectType::Entry;
}

fn key() -> SigningKey {
    SigningKey::from_seed(&<[u8; 32]>::from_hex(RFC8032_SEED).unwrap())
}

fn signed_note() -> Signed<Note> {
    let note = Note {
        text: String::from("Anna paid Mark"),
        amount: 3500,
    };
    Signed::sign(note, &key()).unwrap()
}

/// Every object type, found by decoding each tag a byte can hold, so no list here can fall behind
/// the enum.
fn object_types() -> Vec<ObjectType> {
    (0..=u8::MAX)
        .filter_map(|tag| borsh::from_slice(&[tag]).ok())
        .collect()
}

/// Pins the bytes of the signing scheme, from object-type tags to the id.
///
/// Every phone must derive the same signatures and ids, or parties split. Signs a stand-in note
/// with RFC 8032's first key, then compares each type's tag, domain and domain hash, the public
/// key, the signed bytes, the signature, the envelope and the id with `vectors/signing.txt`.
#[test]
fn golden_signing() {
    let mut set = golden_set!("signing");
    for object_type in object_types() {
        let name = object_type.domain().trim_start_matches("betty/v1/");
        set.record(
            &format!("tag/{name}"),
            &borsh::to_vec(&object_type).unwrap(),
        );
        set.record(&format!("domain/{name}"), object_type.domain().as_bytes());
        set.record(&format!("domain_hash/{name}"), object_type.domain_hash());
    }
    let signed = signed_note();
    set.record("public_key", &borsh::to_vec(signed.signer()).unwrap());
    set.record(
        "note/signing_bytes",
        &Signed::signing_bytes(signed.body(), signed.signer()).unwrap(),
    );
    set.record("note/signature", &borsh::to_vec(signed.sig()).unwrap());
    set.record("note/signed", &borsh::to_vec(&signed).unwrap());
    set.record("note/id", signed.id().unwrap().as_bytes());
    set.finish();
}

/// RFC 8032's first secret key gives that test vector's public key.
///
/// It checks key derivation against a published standard, not only against our own vectors.
/// Builds a signing key from the RFC seed and compares its public key with the RFC's.
#[test]
fn seed_gives_the_rfc_8032_public_key() {
    assert_eq!(
        key().public_key(),
        PublicKey::Ed25519(<[u8; 32]>::from_hex(RFC8032_PUBLIC_KEY).unwrap())
    );
}

/// A freshly signed note verifies.
///
/// The failure tests below prove nothing if a good signature fails too. Signs the stand-in note
/// and expects `verify` to return Ok.
#[test]
fn signed_note_verifies() {
    assert_eq!(signed_note().verify(), Ok(()));
}

/// Any one changed byte stops the envelope decoding, or fails verify and gives a new id.
///
/// No part of a signed object may change unnoticed, and other objects refer to it by id. Flips
/// the lowest bit of each byte in turn and skips results that don't decode, as a phone refuses
/// them anyway. Most still decode, as a changed ASCII character is still valid text and the
/// amount, key and signature accept any bytes. Each of those must fail verify, since the signature
/// covers the domain, body and signer, and any error counts, as a changed key may fail as
/// `BadKey`. Its id must also differ from the original's and every other result's, since the id
/// hashes the whole envelope.
#[test]
fn every_changed_byte_fails_verify_and_changes_the_id() {
    let signed = signed_note();
    let bytes = borsh::to_vec(&signed).unwrap();
    let mut ids = BTreeSet::from([signed.id().unwrap()]);
    let mut decoded = 0;
    for i in 0..bytes.len() {
        let mut changed = bytes.clone();
        changed[i] ^= 1;
        let Ok(other) = borsh::from_slice::<Signed<Note>>(&changed) else {
            continue;
        };
        decoded += 1;
        assert!(
            other.verify().is_err(),
            "byte {i} changed and still verifies"
        );
        assert!(
            ids.insert(other.id().unwrap()),
            "byte {i} changed and the id repeats"
        );
    }
    // Only six changes stop the envelope decoding: the four bytes of the text's length, and the
    // algorithm tags of the key and the signature.
    assert_eq!(decoded, bytes.len() - 6);
}

/// A note signed as a mark fails verify when read as an entry, and gets a different id.
///
/// The domain prefix must stop a signature for one object type passing as another. Decodes the
/// same bytes as an entry type that differs only in its domain, and expects `BadSignature`.
#[test]
fn a_signature_for_one_object_type_fails_as_another() {
    let signed = signed_note();
    let bytes = borsh::to_vec(&signed).unwrap();
    let as_entry = borsh::from_slice::<Signed<NoteAsEntry>>(&bytes).unwrap();
    assert_eq!(&as_entry.body().0, signed.body());
    assert_eq!(as_entry.verify(), Err(VerifyError::BadSignature));
    assert_ne!(as_entry.id().unwrap(), signed.id().unwrap());
}

/// The same signature in a second form, its scalar plus the group order, fails verify.
///
/// Verification must stop anyone but the signer giving a signed object a second id. RFC 8032
/// section 8.4 rules out this second form by requiring the scalar to be below the group order.
/// Checks the original verifies, so the altered copy fails for its scalar alone, then adds the
/// group order to the signature's scalar and expects `BadSignature` and a new id.
#[test]
fn a_signature_with_the_group_order_added_fails_verify() {
    let signed = signed_note();
    assert_eq!(signed.verify(), Ok(()));
    let mut bytes = borsh::to_vec(&signed).unwrap();
    // The signature comes last in the envelope, and its scalar is its last 32 bytes.
    add_group_order(bytes.last_chunk_mut::<32>().unwrap());
    let altered = borsh::from_slice::<Signed<Note>>(&bytes).unwrap();
    assert_eq!(altered.body(), signed.body());
    assert_eq!(altered.signer(), signed.signer());
    assert_ne!(altered.sig(), signed.sig());
    assert_ne!(altered.id().unwrap(), signed.id().unwrap());
    assert_eq!(altered.verify(), Err(VerifyError::BadSignature));
}

/// A small-order signer key fails verify as `BadKey`.
///
/// Under the identity point as the key, the signature with R the identity point and S zero passes
/// plain Ed25519 verification for every message, so anyone could forge objects for that key. Only
/// strict checking refuses it, and it should say the key is bad, not the signature. Builds a note
/// that names the identity key with that signature and expects `BadKey`.
#[test]
fn a_small_order_key_fails_as_bad_key() {
    // The identity point, y = 1 in little-endian.
    let mut identity = [0; 32];
    identity[0] = 1;
    // R is the identity point and S is zero.
    let mut sig = [0; 64];
    sig[0] = 1;
    let bytes = borsh::to_vec(&(
        signed_note().into_body(),
        PublicKey::Ed25519(identity),
        Signature::Ed25519(sig),
    ))
    .unwrap();
    let forged = borsh::from_slice::<Signed<Note>>(&bytes).unwrap();
    assert_eq!(forged.verify(), Err(VerifyError::BadKey));
}

/// Every domain starts with `betty/v1/`, its precomputed hash is its BLAKE3 hash, and no two
/// object types share a domain hash.
///
/// The domain hashes are constants in the source, so a typo or a wrong paste would change every
/// signature and id of that type. Two types with one domain hash would sign and hash the same
/// bytes, so a signature for one would pass as the other. The domains are fixed strings, so
/// checking their hashes once here rules that out without relying on BLAKE3 having no collisions.
/// Hashes each domain with `blake3` directly, compares it with the constant, and checks the
/// constants are distinct, for every object type found by decoding each possible tag byte.
#[test]
fn every_object_type_has_its_own_domain_hash() {
    let types = object_types();
    for object_type in &types {
        let domain = object_type.domain();
        assert!(domain.starts_with("betty/v1/"), "{object_type:?}: {domain}");
        let expected: [u8; 32] = blake3::hash(domain.as_bytes()).into();
        assert_eq!(
            object_type.domain_hash(),
            &expected,
            "{object_type:?}: the domain hash should be {expected:#04x?}"
        );
    }
    let hashes: BTreeSet<_> = types.iter().map(|t| t.domain_hash()).collect();
    assert_eq!(
        hashes.len(),
        types.len(),
        "two object types share a domain hash"
    );
}

/// The signed bytes start with the BLAKE3 hash of the domain.
///
/// A fixed-size hash in front means a body's bytes can never be read as part of the domain, even
/// where one domain is a prefix of another, so one signature can't fit two object types. Hashes
/// the domain with `blake3` directly, not through the crate, and compares it with the start of the
/// note's signed bytes.
#[test]
fn signed_bytes_start_with_the_domain_hash() {
    let signed = signed_note();
    let bytes = Signed::signing_bytes(signed.body(), signed.signer()).unwrap();
    let domain_hash = blake3::hash(Note::TYPE.domain().as_bytes());
    assert_eq!(&bytes[..32], domain_hash.as_bytes());
}

/// The id is the BLAKE3 hash of the domain hash, then the encoded envelope.
///
/// The crate encodes the envelope straight into the hasher, and the golden vector pins only the
/// result, so this checks the result against the formula in the design. Rebuilds it with `blake3`
/// directly from the encoded envelope and compares it with the note's id.
#[test]
fn id_hashes_the_domain_hash_then_the_envelope() {
    let signed = signed_note();
    let mut hasher = blake3::Hasher::new();
    hasher.update(blake3::hash(Note::TYPE.domain().as_bytes()).as_bytes());
    hasher.update(&borsh::to_vec(&signed).unwrap());
    assert_eq!(
        signed.id().unwrap().as_bytes(),
        hasher.finalize().as_bytes()
    );
}

/// Adds the Ed25519 group order to a little-endian 32-byte scalar.
fn add_group_order(scalar: &mut [u8]) {
    const ORDER: [u8; 32] = [
        0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde,
        0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
    ];
    let mut carry = 0;
    for (byte, order) in scalar.iter_mut().zip(ORDER) {
        let [low, high] = (u16::from(*byte) + u16::from(order) + carry).to_le_bytes();
        *byte = low;
        carry = u16::from(high);
    }
}
