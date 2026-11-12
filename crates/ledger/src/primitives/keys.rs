use std::fmt;

use borsh::{BorshDeserialize, BorshSerialize};
use ed25519_dalek::Signer as _;

/// A public key, tagged with its algorithm. Another algorithm, such as a hardware-backed P-256
/// key, would be a new variant, and keys already in the log would keep their bytes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum PublicKey {
    Ed25519([u8; 32]) = 0,
}

/// A signature, tagged with its algorithm like [`PublicKey`].
#[derive(Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum Signature {
    Ed25519([u8; 64]) = 0,
}

/// Why a signature was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum VerifyError {
    /// The signer's key is not a valid public key: it isn't a curve point, or it has small order,
    /// which would let anyone forge a signature for it.
    #[error("the signer's key is not a valid public key")]
    BadKey,
    /// The signature doesn't match the signed bytes, or isn't in its one strict form.
    #[error("the signature does not match the signed bytes")]
    BadSignature,
    /// The object couldn't be encoded, so there were no signed bytes to check.
    #[error(transparent)]
    Encode(#[from] super::encode::EncodeError),
}

/// An Ed25519 key that signs in memory, made from a 32-byte seed the caller supplies.
///
/// This is a stand-in for testing and development. In M5 the `KeyStore` trait in `engine` will
/// likely take its place for real signing: it never hands out its seed and signs
/// [`Signed::signing_bytes`](super::Signed::signing_bytes) itself, so a hardware-backed key fits
/// too. The ledger never makes keys, so it needs no randomness.
pub struct SigningKey(ed25519_dalek::SigningKey);

impl SigningKey {
    /// The key for `seed`, as RFC 8032 derives it. The seed is borrowed, so passing it makes no
    /// copy that would be left on the stack unwiped.
    #[must_use]
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self(ed25519_dalek::SigningKey::from_bytes(seed))
    }

    /// The public key that checks this key's signatures.
    #[must_use]
    pub fn public_key(&self) -> PublicKey {
        PublicKey::Ed25519(self.0.verifying_key().to_bytes())
    }

    /// Signs `message` as it is. [`Signed`](super::Signed) builds the message with its domain.
    pub(crate) fn sign(&self, message: &[u8]) -> Signature {
        Signature::Ed25519(self.0.sign(message).to_bytes())
    }
}

impl PublicKey {
    /// Checks `sig` over `message` in strict mode: the signature's scalar must be fully reduced and
    /// neither point may have small order. Nobody but the signer can then turn one valid signature
    /// into another, so an object can't be given a second id.
    pub(crate) fn verify(&self, message: &[u8], sig: &Signature) -> Result<(), VerifyError> {
        match (self, sig) {
            (Self::Ed25519(key), Signature::Ed25519(sig)) => {
                let key = ed25519_dalek::VerifyingKey::from_bytes(key)
                    .map_err(|_| VerifyError::BadKey)?;
                // `verify_strict` refuses a small-order key too, but as a bad signature.
                if key.is_weak() {
                    return Err(VerifyError::BadKey);
                }
                key.verify_strict(message, &ed25519_dalek::Signature::from_bytes(sig))
                    .map_err(|_| VerifyError::BadSignature)
            }
        }
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ed25519(key) => {
                f.write_str("ed25519:")?;
                super::write_hex(f, key)
            }
        }
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ed25519(sig) => {
                f.write_str("ed25519:")?;
                super::write_hex(f, sig)
            }
        }
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
