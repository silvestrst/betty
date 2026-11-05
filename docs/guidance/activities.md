# Activities (M2) and preset files (M5)

Preferred approach for activities in M2 and preset files in M5. Follow unless implementation shows a better way; record any deviation here.

Payout models, limits, the schema per kind, what replay tracks and what a preset may do are decisions, in [design/activities.md](../design/activities.md). This file is only about how the code holds them. The checks an activity runs on a stake are in [rules.md](rules.md), and kind enums are in [ledger-format.md](ledger-format.md).

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| `activities` builds each `StakeCtx` and calls the rule checks. Rules never see an activity type. | The dependency runs one way, so rules stay small and testable alone. | The layer test in [architecture.md](architecture.md) |
| Checks about the activity itself live in `activities`: open and not locked, the outcome index exists, the schema and the `OpenActivity` limits. | None of them is a rule kind, and a rule shouldn't need the activity to decide. | Review |
| Checks about the party live in `state`: the owner is Active, and the party's max buy-in. | They read party state, which `activities` can't import. | Review, and the layer test |
| Running totals are rebuilt by replay and never stored as a source of truth. | An activity is never a record of its own. | M2's check: replay is deterministic |
| `engine` parses preset files and holds the built-in set, saving and sharing. `ledger` holds only the types and the checks. | A preset can't change replay, so a file format has no place in the pure crate. | Review: no preset parser or file format in `ledger` |
| Presets read into the same types as the log and pass the same schema and limit checks as any `OpenActivity`. | A preset can only combine existing payout models and rule kinds. | Unit test: an invalid preset is refused |
| serde derives on ledger types sit behind a Cargo feature that only `engine` enables. | The core build stays Borsh-only, so serde can't reach the encoded bytes or a build without presets. | `cargo tree -p ledger -e normal` shows no serde |

## Sketch

An illustration of one way to meet the properties above, not a spec.

### Running totals

`design/activities.md` says what replay tracks. One way to hold it:

```rust
struct ActivityState {
    activity: Activity,
    status: ActivityStatus,                     // Open | Locked | Resulted | Voided
    players: BTreeMap<PublicKey, PlayerTally>,
    pool: u64,                                  // units staked in total
    chips_in_play: u64,                         // chip games: buy-ins, rebuys and add-ons in chips
    rounds: Vec<RoundState>,                    // counts and round results
    pending_amend: Option<EntryId>,             // from M9: a voted rule change waiting for its Close
    outcome: Option<Outcome>,                   // winning outcome, places or winner
}

struct PlayerTally {
    counts: PlayerCounts,                       // from `rules`: what the rule checks read
    cashed_out: Option<u64>,                    // chips, ChipCount
    seated: bool,                               // bought in, not cashed out, not left
}
```

`PlayerCounts` is defined in `rules`, and the tally holds it directly, so handing it to a check copies nothing.

### Preset files

Presets are plain data, which `engine` reads with serde into the same types as the log. YAML, TOML or JSON is chosen when presets are built; the main Rust YAML crate, `serde_yaml`, is no longer maintained. An example in YAML:

```yaml
name: Friday rebuy tournament
game: Poker
params:
  Places:
    payouts:
      - { min_players: 1,  payout_bps: [6500, 3500] }
      - { min_players: 6,  payout_bps: [5000, 3000, 2000] }
      - { min_players: 11, payout_bps: [4000, 2500, 1800, 1000, 700] }
rules:
  - BuyIn: { Fixed: 2000 }
  - MaxEntries: 1
  - Rebuy: { amount: { Fixed: 2000 }, max_count: 2 }
  - AddOn: { amount: { Fixed: 2000 }, max_count: 1 }
open_windows: [BuyIn, Rebuy]
```

serde is not in the workspace yet, so ask before adding it. In `ledger`, the data enums in [ledger-format.md](ledger-format.md) and their kind enums gain the derives behind the feature:

```rust
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", strum_discriminants(derive(Serialize, Deserialize)))]
```

Tried on 2 November 2026 in a throwaway crate, with strum 0.28, borsh 1.8 and serde 1. The Borsh numbers and the decode failure for a number with no variant are the same with the feature on and off, a JSON round trip gives back the same `Rule` and `RuleKind`, and with the feature off serde is not in the dependency tree.

## What happened

Nothing yet: M2 has not started. Record each deviation here as it happens, with its reason, and the outcome when the milestone ends.
