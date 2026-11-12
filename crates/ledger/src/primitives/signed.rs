use borsh::{BorshDeserialize, BorshSerialize};

use super::Hash;
use super::PublicKey;
use super::Signature;
use super::SigningKey;
use super::VerifyError;
use super::encode::{EncodeError, encode_into};

/// The kind of a signed object.
///
/// Its number is the one-byte tag in front of the object wherever it is stored or synced. The
/// BLAKE3 hash of its domain string starts every signature and id of that kind, so a signature
/// made for one kind is never valid for another. The hash is always 32 bytes, so the bytes after it
/// can never be read as part of it, even where one domain string is a prefix of another. Both are
/// append-only like every ledger enum, and no two kinds share a domain string. The table is in
/// `docs/design/ledger-format.md`, "Object types".
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum ObjectType {
    /// An admin entry in the chain.
    Entry = 0,
    /// Something a member does or asks for, signed by that member.
    MemberAction = 1,
    /// An Accept, Dispute, Witness or `TakeoverVote` note about another object.
    Attest = 2,
    /// A payment mark on a settlement transfer.
    Mark = 3,
}

impl ObjectType {
    /// The domain string of this kind.
    #[must_use]
    pub const fn domain(self) -> &'static str {
        match self {
            Self::Entry => "betty/v1/entry",
            Self::MemberAction => "betty/v1/member_action",
            Self::Attest => "betty/v1/attest",
            Self::Mark => "betty/v1/mark",
        }
    }

    /// `BLAKE3(domain)`, the 32 bytes that start every signature and id of this kind.
    ///
    /// The hashes are precomputed constants, consistent with BIP-340, whose tagged hashes put a
    /// fixed hash of the tag in front so that it is never computed at runtime. The signing tests
    /// check each constant against its domain string.
    #[must_use]
    pub const fn domain_hash(self) -> &'static [u8; 32] {
        match self {
            Self::Entry => &ENTRY_DOMAIN_HASH,
            Self::MemberAction => &MEMBER_ACTION_DOMAIN_HASH,
            Self::Attest => &ATTEST_DOMAIN_HASH,
            Self::Mark => &MARK_DOMAIN_HASH,
        }
    }
}

/// `BLAKE3("betty/v1/entry")`.
const ENTRY_DOMAIN_HASH: [u8; 32] = [
    0x40, 0xd3, 0xf1, 0x1c, 0x57, 0xe1, 0x4b, 0xdf, 0x03, 0x2e, 0x96, 0x08, 0x58, 0x4b, 0x6a, 0xcb,
    0x3f, 0xff, 0xc7, 0xed, 0x22, 0xc7, 0x0b, 0xeb, 0xf5, 0x9a, 0x9c, 0x07, 0xd3, 0x84, 0x68, 0x6c,
];

/// `BLAKE3("betty/v1/member_action")`.
const MEMBER_ACTION_DOMAIN_HASH: [u8; 32] = [
    0xf8, 0x35, 0x69, 0x7c, 0x91, 0x32, 0xdc, 0xc2, 0x0f, 0x4e, 0xff, 0xef, 0x7f, 0x56, 0x9e, 0x35,
    0x81, 0x33, 0x50, 0xd1, 0x52, 0xa2, 0x96, 0x9c, 0x61, 0x02, 0x1b, 0x6f, 0x86, 0xf7, 0x44, 0xe8,
];

/// `BLAKE3("betty/v1/attest")`.
const ATTEST_DOMAIN_HASH: [u8; 32] = [
    0x08, 0x8c, 0xac, 0x85, 0xcf, 0xb4, 0x7f, 0x3f, 0x7e, 0xb0, 0x64, 0x89, 0x81, 0x4f, 0x08, 0xf1,
    0xdb, 0xf2, 0x2b, 0x6d, 0x17, 0x12, 0x0c, 0x01, 0x01, 0x22, 0xe9, 0xa2, 0xb2, 0xef, 0xc8, 0x2f,
];

/// `BLAKE3("betty/v1/mark")`.
const MARK_DOMAIN_HASH: [u8; 32] = [
    0x74, 0xd4, 0x7c, 0xf1, 0x25, 0x7f, 0xe2, 0x92, 0x66, 0x2a, 0x47, 0x06, 0x3a, 0x89, 0xd3, 0x83,
    0x2f, 0x4a, 0x6f, 0xa4, 0xc3, 0x81, 0x4a, 0x1f, 0xa1, 0x3d, 0xb4, 0xae, 0x00, 0x03, 0x2b, 0x1f,
];

/// A body that is signed as one kind of object.
pub trait Object: BorshSerialize {
    /// The kind of object this body is signed as.
    const TYPE: ObjectType;
}

/// An object with its signer and signature.
///
/// Its id is the hash of the whole envelope, so changing one byte of the body, the signer or the
/// signature gives a different id. Decoding doesn't check the signature: call [`Signed::verify`].
///
/// The fields are read-only. [`Signed::sign`] and decoding are the only ways to build one, so code
/// can't change an object after signing or checking it.
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Signed<T> {
    body: T,
    signer: PublicKey,
    sig: Signature,
}

impl<T> Signed<T> {
    /// The object the signature covers.
    #[must_use]
    pub fn body(&self) -> &T {
        &self.body
    }

    /// The key the object claims to be signed with. It is the signer only once
    /// [`Signed::verify`] passes.
    #[must_use]
    pub fn signer(&self) -> &PublicKey {
        &self.signer
    }

    /// The signature over the body and the signer.
    #[must_use]
    pub fn sig(&self) -> &Signature {
        &self.sig
    }

    /// Takes the body out, dropping the signer and the signature.
    #[must_use]
    pub fn into_body(self) -> T {
        self.body
    }
}

impl<T: Object> Signed<T> {
    /// Signs `body` with `key`.
    ///
    /// # Errors
    ///
    /// [`EncodeError`] if the body can't be encoded.
    pub fn sign(body: T, key: &SigningKey) -> Result<Self, EncodeError> {
        let signer = key.public_key();
        let sig = key.sign(&Self::signing_bytes(&body, &signer)?);
        Ok(Self { body, signer, sig })
    }

    /// The bytes a signature covers: the domain hash, then the encoded body and signer. A key
    /// store that never hands out its seed signs these bytes itself.
    ///
    /// # Errors
    ///
    /// [`EncodeError`] if the body can't be encoded.
    pub fn signing_bytes(body: &T, signer: &PublicKey) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = T::TYPE.domain_hash().to_vec();
        encode_into(body, &mut bytes)?;
        encode_into(signer, &mut bytes)?;
        Ok(bytes)
    }

    /// Checks the signature against the body and the signer, in strict mode.
    ///
    /// # Errors
    ///
    /// [`VerifyError::BadKey`] if the signer isn't a valid key, [`VerifyError::BadSignature`]
    /// if the signature doesn't match, including one made for another kind of object, and
    /// [`VerifyError::Encode`] if the body can't be encoded.
    pub fn verify(&self) -> Result<(), VerifyError> {
        self.signer
            .verify(&Self::signing_bytes(&self.body, &self.signer)?, &self.sig)
    }

    /// The object's id: the hash of its domain hash and the whole encoded envelope.
    ///
    /// The id covers the signature bytes, so call [`Signed::verify`] before trusting it. A copy
    /// whose signature is re-encoded, such as `S + L` in place of `S`, has its own id but fails
    /// strict verification. Code that keys, dedups or counts objects by id before verifying them
    /// could therefore see one object under several ids.
    ///
    /// # Errors
    ///
    /// [`EncodeError`] if the envelope can't be encoded.
    pub fn id(&self) -> Result<Hash, EncodeError> {
        Hash::of(T::TYPE.domain_hash(), self)
    }
}
