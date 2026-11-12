//! The base layer: hashes, keys, signatures and the signed envelope every object travels in.
//!
//! It imports no other module of this crate, so every other module can build on it (see the module
//! layout in `docs/guidance/architecture.md`).

// The leaf: it imports no sibling module.
mod encode;

// Built on `encode` only.
mod hash;
mod keys;

// The signed envelope, built from `encode`, `hash` and `keys`.
mod signed;

use std::fmt;

pub use encode::EncodeError;
pub use hash::Hash;
pub use keys::{PublicKey, Signature, SigningKey, VerifyError};
pub use signed::{Object, ObjectType, Signed};

/// Writes `bytes` as lowercase hex, for the `Display` and `Debug` impls in this module.
///
/// Not `hex::encode`, which allocates a `String` on every format: `hex` 0.4 has no `Display`
/// adapter that writes straight into the formatter.
fn write_hex(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    bytes.iter().try_for_each(|byte| write!(f, "{byte:02x}"))
}
