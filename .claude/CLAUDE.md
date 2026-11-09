# Betty

The docs hold three kinds of content, kept apart. Check which one you are reading before acting on it.

- **Decisions are binding:** `docs/design/`. It covers the wire format, consensus rules, architecture, scope and acceptance checks. If it is silent or wrong, ask, then update the design in the same change. An architecturally significant choice, such as structure, dependencies or interfaces, also gets an entry in the "Decisions" section of its design file.
- **Guidance is the preferred direction:** `docs/guidance/`. Follow it by default. Deviate only with a stated reason, recorded in the file. If it contradicts a decision, the decision wins.
- **Ideas are not committed:** `docs/ideas.md`. Never build from it.

To sort new content: if changing it could make two phones disagree or break a promise to users, it is a decision. If a different build gives the same bytes and the same replay, it is guidance.

## What to read before writing code

Before writing code, find the row that fits the work and read only the files it names. If the work spans several kinds, read each row that fits. The map says what to read, not what to build: a row for a later milestone doesn't bring its work into scope. If no row fits, check the index in `docs/design/README.md`, then ask.

- **Where:** crates are in `crates/`.
- **Design:** files in `docs/design/`.
- **Guidance:** files in `docs/guidance/`.

| Kind of code | Where | Design | Guidance |
| --- | --- | --- | --- |
| Encoding, hashing, ids, enum numbering, golden vectors | `ledger` | `ledger-format.md` | `ledger-format.md` |
| Signatures and public keys | `ledger` | `identity-and-party.md`, `ledger-format.md` | `ledger-format.md` |
| Record kinds and their fields | `ledger` | `records.md`, `ledger-format.md` | `ledger-format.md` |
| Kind enums and record payloads for activities and rules | `ledger` | `rules.md`, `activities.md` | `ledger-format.md` |
| Ordering layer, chain rules, entry validity | `ledger` | `ledger-format.md`, `validation.md` | `ledger-format.md` |
| Party, members, joining | `ledger` | `identity-and-party.md` | none |
| Rule checks and stakes | `ledger` | `rules.md`, `validation.md` | `rules.md` |
| Activities and tallies | `ledger` | `activities.md`, `validation.md` | `activities.md` |
| Preset files and the built-in set | `engine` | `activities.md` | `activities.md` |
| Modes, results, quorum, checkpoints, dropouts, handover, takeover | `ledger` | `validation.md` | none |
| Balance rules and rounding | `ledger` | `settlement.md` | none |
| Settlement solver, statement, payment requests, backends | `settle` | `settlement.md` | `settlement.md` |
| Storage and the database | `store` | `storage.md` | `storage.md` |
| Key custody | `engine` | `storage.md`, `identity-and-party.md` | `storage.md` |
| Engine, DTOs, commands, queries, events, CLI | `engine`, `cli` | `architecture.md`, `testing.md` | `architecture.md` |
| Crate boundaries, `ledger` modules and layering | all crates | `architecture.md`, `principles.md` | `architecture.md` |
| Sync, transports, discovery, invite links | `sync`, `net-lan`, `sim` | `sync.md` | `sync.md` |
| App and web UI | `apps/betty/` | `app.md` | `app.md`, `storage.md` |
| Tests and the simulator | `crates/*/tests/`, `sim` | `testing.md` | none |
| Dependencies and licensing | `Cargo.toml`, `deny.toml` | `architecture.md` | none |
| CI and build targets | `.github/workflows/` | `phases.md` | none |
| Milestone scope and acceptance checks | none | `phases.md`, `app.md` | `README.md` |
| An unsettled question | none | `open-decisions.md` | none |
| Decisions, guidance and ideas docs | `docs/` | `README.md` | `README.md` |

When you finish a task, print a list of the documents you used for it, covering both planning and code generation.

## Current milestone: M1, ledger kernel

- **Scope:** validation means stages 1 and 2 in `docs/design/validation.md`.
- **Acceptance check:** the M1 row in `docs/design/phases.md`. CI is host-only for now, so its "all targets" part waits (see "Decisions" in that file).
- **Guidance to review at the start:** `docs/guidance/ledger-format.md`, and `docs/guidance/architecture.md` for the module layout.

`docs/guidance/README.md` maps each milestone to its docs.

## Working on a milestone

- One milestone at a time (M0 to M10). A plan covers only that milestone and ends with its acceptance check from the design.
- At the start, read its guidance in plan mode and say where you intend to deviate, before writing code.
- At the end, update the guidance file with what actually happened, then move "Current milestone" above on.
- Phase 1 (M0 to M5) has no networking and no async.

## Protocol rules (a mistake here compiles, passes tests and still splits phones)

- `ledger` is pure: no I/O, no async.
- Ledger types encode with Borsh only; serde is only for presets. No floats, `usize` or `HashMap`.
- Enum variant numbers are explicit and append-only, never reused or reordered.
- Money is integer minor units, and balances sum to zero.
- Replay is deterministic: no wall clock, randomness or unordered iteration. Time comes from `Accepted.at`.
- The state layer never reads `epoch`, `seq` or `prev`.
- A failing golden vector means the change is wrong. Never regenerate vectors without asking.

## Checks

Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` and `cargo deny check`. Licence is PolyForm Perimeter 1.0.1, so dependencies must be permissive (no GPL family); ask before adding one.
