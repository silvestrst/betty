# Settlement backends (M4 and M10)

Preferred approach for M4, which adds the trait and a manual backend with plain requests, and M10, which adds payment links. Follow unless implementation shows a better way; record any deviation here.

The solver, the statement, the finality rule and what the manual backend builds are decisions, in [design/settlement.md](../design/settlement.md). This file is only about the shape of the backend trait.

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| A backend only turns a transfer into something the payer completes in their own app. It can't change balances, transfers or the statement. | The app never holds money, and every phone must compute the same statement. | Compiler: the trait's methods take shared references |
| A new backend is a new implementation of the trait, with no change to `ledger`. | This is the seam that keeps other settlement methods possible. | Review |
| `observe` defaults to `None`, so the manual backend needs no code for it. | The design only builds payment requests and reads no chain. | Review |

## Sketch

An illustration, not a spec.

```rust
trait SettlementBackend {
    fn id(&self) -> BackendId;
    fn methods_for(&self, payee: &Profile) -> Vec<PaymentMethod>;
    fn request(&self, t: &Transfer, unit: &Unit, m: &PaymentMethod) -> PaymentRequest;
    fn observe(&self, t: &Transfer) -> Option<MarkEvidence> { None } // escrow: read chain state
}
```

## What happened

Nothing yet: M4 has not started. Record each deviation here as it happens, with its reason, and the outcome when the milestone ends.
