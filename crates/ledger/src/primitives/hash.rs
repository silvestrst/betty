use std::fmt;

use borsh::{BorshDeserialize, BorshSerialize};

use super::encode::{EncodeError, encode_into};

/// A 32-byte BLAKE3 hash, used for object ids and the links between objects.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct Hash([u8; 32]);

impl Hash {
    /// `BLAKE3(domain_hash || encoded value)`, where `domain_hash` is the precomputed BLAKE3 hash
    /// of a domain string, such as [`ObjectType::domain_hash`](super::ObjectType::domain_hash).
    /// The value is encoded straight into the hasher. To hash several fields, pass them as a
    /// tuple.
    pub(crate) fn of(
        domain_hash: &[u8; 32],
        value: &impl BorshSerialize,
    ) -> Result<Self, EncodeError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(domain_hash);
        encode_into(value, &mut hasher)?;
        Ok(Self(hasher.finalize().into()))
    }

    /// The hash's 32 bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        super::write_hex(f, &self.0)
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({self})")
    }
}
