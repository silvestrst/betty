//! Betty's ledger: the signed records every phone keeps, their canonical encoding, and the rules
//! that turn them into balances.
//!
//! The crate is pure: no I/O, no async, no clock and no randomness. That keeps it building for
//! every target the app runs on and makes replay give the same bytes and balances everywhere. The
//! design's "Ledger format" section is the contract it implements, and golden vectors in `tests/`
//! pin the encoding.
