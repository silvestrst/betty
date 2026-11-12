# Ledger modules and the engine (M1 and M5)

Preferred approach for the `ledger` module layout (M1) and the engine (M5). Follow unless implementation shows a better way; record any deviation here.

The crates, their dependencies, the engine's role and the choice of one layered `ledger` crate are decisions, in [design/architecture.md](../design/architecture.md). This file is only about the shape of the code.

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| Each `ledger` module imports only the modules listed for it in the layout below. | Cycles between layers can't form, and `rules` can never come to depend on `activities`. | The layer test, from M1 |
| `rules` could move to its own crate without changing its code, apart from visibility and import paths. | That keeps the crate split in "Decisions" a mechanical change if it is ever needed. | Follows from the layer test |
| The engine's DTOs are plain records and enums, with no generics or lifetimes. | The same API can be exposed through Tauri commands now and through UniFFI to Kotlin and Swift later. | Review of the engine's public types |

## Sketch

An illustration, not a spec.

### `ledger` modules

```text
crates/ledger/src/
  primitives/     ids, keys, signatures, hashing, encoding
  rules/          Rule, RuleKind, RuleChange, Stake, StakeAction, StakeKind,
                  RuleCheck, StakeCtx, one file per rule kind
  activities/     ActivityParams, ActivityKind, Activity, schema and limits, running totals
  records/        record catalogue, Accepted, Head
  ordering/
    admin_chain/  the only ordering
  state/          party, membership, results, governance, balances
  replay.rs       feeds the ordering layer's Accepted records into the state layer
```

| Module | May import |
| --- | --- |
| `primitives` | no other module |
| `rules` | `primitives` |
| `activities` | `primitives`, `rules` |
| `records` | `primitives`, `rules`, `activities` |
| `ordering` | `primitives`, `records` |
| `state` | `primitives`, `rules`, `activities`, `records` |
| `replay` | any |

Activities sit below `records`, not inside `state`, because `OpenActivity` carries `ActivityParams` and `state` reads `records`. Nesting them under `state` would make a cycle. `Stake` and `StakeAction` sit in `rules` because `StakeKind` is generated from `StakeAction` (see [ledger-format.md](ledger-format.md)) and the checks read them.

**The layer test.** `crates/ledger/tests/layers.rs` reads each module's files and fails on `::<module>` for any module not allowed above, which catches `crate::state` and `super::super::state` alike. Grouped imports such as `use crate::{state, ...}` would hide a name from that scan, so it also refuses `crate::{` and `super::{` in those files. It needs no new dependency and runs on the host only, since it reads the source tree. It scans text, so it catches ordinary imports rather than a determined workaround such as a renamed path.

**Inside a module.** The files in one module directory are siblings, so the tree doesn't show which file builds on which. The `mod` lines in its `mod.rs` show it instead. Group them by level, lowest first, with a one-line comment on each group saying what it builds on, as `curve25519-dalek` does in its `lib.rs`. A file that builds on its siblings stays a named sibling and doesn't move into `mod.rs`. When a lower level grows past a handful of files, move it into its own submodule so each directory holds one level, as `curve25519-dalek` does with `backend/`. Put a type in `mod.rs` only when the other files are its parts and import it from there, as `serde_json` does with `Value`. The layer test doesn't check order inside a module, so update the comments whenever the imports change.

### Engine

`engine` exposes plain commands (`create_party`, `approve_join`, `place_bet`, `attest`, `settle`), queries that return view structs (`PartyView`, `BalancesView`, `InboxView`, `SettlementView`) and a stream of events (`StateChanged`, `PeerSeen`, `SyncProgress`, `MissingUpdates`, `Equivocation`). The DTOs are plain records and enums with no generics or lifetimes, so the same API can be exposed through Tauri commands now and through UniFFI to Kotlin and Swift later.

## What happened

Nothing yet: M1 has not started. Record each deviation here as it happens, with its reason, and the outcome when each milestone ends.
