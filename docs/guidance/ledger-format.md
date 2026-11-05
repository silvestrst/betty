# Ledger format (M1)

Preferred approach for M1. Follow unless implementation shows a better way; record any deviation here.

The bytes, hashes, signatures and the two layers are decisions, in [design/ledger-format.md](../design/ledger-format.md). This file is only about how the code gets there. [README.md](README.md) lists M1's other decisions, and the module layout is in [architecture.md](architecture.md).

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| The ordering and state layers meet at a module boundary, with one narrow type (`Accepted`) between them. There is one ordering, no trait for it and no plugin system. | The design has one ordering. The split exists so another could be added later, not to support one now. | Review |
| `Accepted` lives in `records`, and `state` never imports `ordering`. | The state layer can't reach chain types by accident, which protects the rule that it never reads `epoch`, `seq` or `prev`. A second ordering would then be a new module beside the admin chain. | The layer test in [architecture.md](architecture.md) |
| State-layer tests feed hand-built `Accepted` streams as well as streams from real chains. | It shows the state layer needs nothing from the chain, which is the point of the split. | The tests exist |
| A kind enum (`RuleKind`, `ActivityKind`, `StakeKind`) is generated from its data enum, not written by hand. | A kind can never carry another kind's data, and the two can't drift apart. | Compiler |
| A generated kind enum has the same explicit variant numbers as its data enum. | `RuleKind` reaches the log, in `open_windows` and `RuleChange`, so the encoding rules apply to it too. | A golden vector for each encoded kind enum |
| M1's ordering layer handles epoch 0. Handover, takeover, `Assume` and equivocation come in M3. | Agreed when M0 and M1 were planned. | Review |
| Enums get explicit numbers in catalogue order now, but only the variants M1 needs are implemented. Later variants fill the gaps. | Agreed when M0 and M1 were planned. | Golden vectors |
| Golden vectors grow with the types: each step adds vectors for the objects it introduces. | Agreed when M0 and M1 were planned, so every step pins its own bytes. | Golden vectors |

## Sketch

An illustration, not a spec. strum is not in the workspace yet, so ask before adding it.

**Kind enums.** strum generates each kind: the same variants without data, plus `From<&Rule> for RuleKind`. `ActivityParams` and `StakeAction` follow the same pattern as `Rule` here.

```rust
#[derive(BorshSerialize, BorshDeserialize, EnumDiscriminants)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
#[strum_discriminants(name(RuleKind))]
#[strum_discriminants(borsh(use_discriminant = true))]
#[strum_discriminants(derive(BorshSerialize, BorshDeserialize, PartialOrd, Ord, Hash, EnumIter))]
enum Rule {
    BuyIn(BuyIn) = 0,
    MaxEntries(MaxEntries) = 1,
    // the other variants, numbered explicitly
}
impl Rule { fn kind(&self) -> RuleKind { RuleKind::from(self) } }
impl ActivityParams { fn kind(&self) -> ActivityKind { ActivityKind::from(self) } }
```

Tried on 1 November 2026 in a throwaway crate, with strum 0.28 and borsh 1.8. strum copies the explicit numbers and `repr` to the generated enum, so `RuleKind` encodes with the same number as `Rule`, and a number with no variant fails to decode. `borsh(use_discriminant = true)` has to be passed through as shown, and leaving it out fails to compile. Repeating `repr(u8)` through `strum_discriminants` also fails, because strum already copies it.

Retried on 2 November 2026 without serde derives, which M1 doesn't need: the numbers still match and a number with no variant still fails to decode. Preset files add serde behind a feature in M5, as [activities.md](activities.md) describes.

## What happened

Nothing yet: M1 has not started. Record each deviation here as it happens, with its reason, and the outcome when the milestone ends.
