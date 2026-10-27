# Betty

`docs/design.md` (Design 1.0) is the source of truth. Read the sections a task touches. If it is silent or wrong, ask, then update it in the same change.

## Scope

- One milestone at a time (M0 to M8). A plan covers only that milestone and ends with its acceptance check from the design.
- Phase 1 (M0 to M5) has no networking and no async.
- "Possible future extensions" are not to be built.

## Protocol rules (a mistake here compiles, passes tests and still splits phones)

- `betty-ledger` is pure: no I/O, no async, builds for wasm32.
- Ledger types use Borsh only. No floats, `usize` or `HashMap`.
- Enum variant numbers are explicit and append-only, never reused or reordered.
- Money is integer minor units, and balances sum to zero.
- Replay is deterministic: no wall clock, randomness or unordered iteration. Time comes from `Accepted.at`.
- The state layer never reads `epoch`, `seq` or `prev`.
- A failing golden vector means the change is wrong. Never regenerate vectors without asking.

## Checks

Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` and `cargo deny check`. Licence is AGPL-3.0-only, so ask before adding a dependency.
