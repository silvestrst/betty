use std::io;

use borsh::BorshSerialize;

/// A value that Borsh couldn't encode.
///
/// Writing to a `Vec` or a hasher never fails, so in practice this means a collection with more
/// than `u32::MAX` items, whose length doesn't fit Borsh's `u32` prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("the value can't be encoded: a collection has more than u32::MAX items")]
pub struct EncodeError;

/// Writes the Borsh encoding of `value` to `out`, a buffer or a hasher.
pub(super) fn encode_into(
    value: &impl BorshSerialize,
    out: &mut impl io::Write,
) -> Result<(), EncodeError> {
    value.serialize(out).map_err(|_| EncodeError)
}
