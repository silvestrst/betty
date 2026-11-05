# Guidance

The preferred way to build what [design/](../design/) requires. Follow it by default, and deviate only with a stated reason, recorded in the file.

Guidance never overrides a decision. If a file here contradicts `design/`, the decision wins, and the guidance needs fixing.

## What each file holds

Each file has the same name as the design file it builds on. A design file with nothing to steer, such as `records.md`, has no guidance file.

1. **What must hold:** properties, why they matter, and how to check each one, by the compiler or a test wherever possible. These do the steering.
2. **Sketch:** an illustration of one way to meet the properties, not a spec. The code can differ as long as the properties hold.
3. **What happened:** each deviation and its reason, and the outcome at the end of the milestone.

## Milestone routine

- **At the start,** read the milestone's guidance in plan mode and say where you intend to deviate, before writing code.
- **At the end,** update the guidance file with what actually happened, so it stays a record of intent rather than a stale wish list. Then move the "Current milestone" block in `.claude/CLAUDE.md` on, using the table below.

## Which docs each milestone needs

The decisions are files in [design/](../design/). The acceptance check for each milestone is its row in [phases.md](../design/phases.md), or in [app.md](../design/app.md) for U0 to U7.

| Milestone | Decisions | Guidance |
| --- | --- | --- |
| M1 Ledger kernel | `architecture.md`, for the `ledger` modules; `identity-and-party.md`; `rules.md` and `activities.md`, for the kind enums and record payloads; `ledger-format.md`; `records.md`; stages 1 and 2 in `validation.md` | [ledger-format.md](ledger-format.md), and [architecture.md](architecture.md) for the module layout |
| M2 Party state machine | `rules.md`; `activities.md`; `ledger-format.md`, for the ordering and state layers; stage 3 in `validation.md`; `settlement.md`, for the balance rules and rounding | [rules.md](rules.md), [activities.md](activities.md) |
| M3 Governance | `validation.md` | None: it is all consensus behaviour |
| M4 Settlement | `settlement.md` | [settlement.md](settlement.md) |
| M5 Store, engine and CLI | `architecture.md`; `storage.md`; `activities.md`, for preset files | [architecture.md](architecture.md), [storage.md](storage.md), [activities.md](activities.md) for preset files |
| M6 to M8 Sync, Wi-Fi and invite links | `sync.md` | [sync.md](sync.md) |
| U0 to U5 The app | `app.md` | [app.md](app.md), and [storage.md](storage.md) for `KeyStore` in U1 |
| M9 Everyone agrees core | `validation.md`; `rules.md`; `records.md`; `ledger-format.md`; `settlement.md`; `open-decisions.md`, item 12 | None: it is all consensus behaviour |
| U6 Everyone agrees screens | `app.md`; `validation.md` | [app.md](app.md) |
| M10 Payment links core | `settlement.md`; `identity-and-party.md`, for payout methods; `open-decisions.md`, item 5 | [settlement.md](settlement.md) |
| U7 Payment link screens | `app.md`; `settlement.md` | [app.md](app.md) |
