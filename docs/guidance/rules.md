# Rule checks (M2)

Preferred approach for M2. Follow unless implementation shows a better way; record any deviation here.

Which rule kinds exist, what they allow and what makes a stake valid are decisions, in [design/rules.md](../design/rules.md). This file is only about how the checks are built. Kind enums are in [ledger-format.md](ledger-format.md), and running totals and presets are in [activities.md](activities.md).

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| Rules are stored as a closed enum, never boxed trait objects. | A boxed trait object can't be decoded from the log without a type registry, and an open set would let phones disagree about which rules exist. | Review: `Box<dyn RuleCheck>` appears nowhere |
| Adding a rule kind touches one new type, one enum variant and one match arm, with no macro or extra crate. | Rule kinds stay cheap to add and easy to review. | Review of the diff that adds one |
| A missing variant in the dispatch fails to compile. | A skipped rule would change which stakes count. | Compiler: the match in `as_check` has no wildcard arm |
| A unit test checks that each variant's `kind()` matches its `RuleKind`. | The dispatch and the kind can't be checked against each other by the compiler. | That test |
| Rules can only accept or refuse a stake, never mutate state. | Replay alone changes state, and only after every check passes. | Compiler: a check receives a read-only view |
| A new stake kind does not compile until it names its permitting rule. | Every stake kind has exactly one permitting rule. | Compiler: `permitting_rule` is an exhaustive match |
| `RuleCheck` stays dyn-compatible: methods instead of associated constants, and no generic methods. | `&dyn RuleCheck` is how the enum lends out a rule while a check runs. | Compiler |
| Each rule kind is its own type in its own module, with its own checks and tests. | Types that share data, like `Rebuy` and `AddOn`, can diverge later without touching each other. | Review |
| Shared checks (window open, amount within the rule, count under the limit) live on `StakeCtx`. | Every rule uses the same code for the same kind of check. | Review |
| `StakeCtx` is never stored, signed or synced. | It is an internal, short-lived view, and internal types are never encoded. | Compiler: no Borsh or serde derives |
| `StakeCtx` is defined in `rules` and holds only stake data and counts, never an activity type. | Rules must not depend on activities, and holding `&ActivityState` would make a cycle, because the activity holds the `RuleSet`. | The layer test in [architecture.md](architecture.md) |
| `StakeCtx` and the checks stay `pub(crate)`. Anything above `ledger` asks through a public function that builds the context from replayed state. | A check can never run against a hand-built context. | Compiler |
| Checking a stake runs every rule that applies and collects all violations. | The admin's phone can say exactly why a stake was refused. | Unit test: a stake breaking two rules reports both |
| The same checks run before anything is signed, on the player's and the admin's phones. | A phone can grey out a stake that would fail, and there is no second copy of a rule to drift. | Review: the UI-facing check calls the same code |

## Sketch

An illustration of one way to meet the properties above, not a spec.

### Rule checks

**One type per rule kind, with its checks on that type.** Each rule kind is its own type in its own module and implements one trait. `Rule` is only the closed set that gets signed and stored. To run a check it lends out the inner value as `&dyn RuleCheck` through a single `as_check` match, and every method goes through that match, so adding a rule kind means one new type, one enum variant and one match line, with no macro or extra crate.

```rust
trait RuleCheck {
    fn kind(&self) -> RuleKind;
    fn windowed(&self) -> bool { false }                       // can CloseWindow pause this rule?
    fn applies_to(&self, stake: StakeKind) -> bool;            // e.g. Rebuy applies only to Rebuy stakes
    fn check(&self, ctx: &StakeCtx) -> Result<(), Violation>;  // the stake, the player's counts,
}                                                              // the number of players, open windows

impl RuleCheck for Rebuy {
    fn kind(&self) -> RuleKind { RuleKind::Rebuy }
    fn windowed(&self) -> bool { true }
    fn applies_to(&self, stake: StakeKind) -> bool { stake == StakeKind::Rebuy }
    fn check(&self, ctx: &StakeCtx) -> Result<(), Violation> {
        ctx.require_window(self.kind())?;
        ctx.require_amount(&self.0.amount)?;
        ctx.require_count(StakeKind::Rebuy, self.0.max_count)
    }
}

// The enum is what is stored; &dyn RuleCheck is only borrowed from it to run a check.
impl Rule {
    fn as_check(&self) -> &dyn RuleCheck {
        match self {
            Rule::BuyIn(r) => r,
            Rule::Rebuy(r) => r,
            // one line per variant; a missing variant does not compile
        }
    }
    fn windowed(&self) -> bool { self.as_check().windowed() }
    fn applies_to(&self, stake: StakeKind) -> bool { self.as_check().applies_to(stake) }
    fn check(&self, ctx: &StakeCtx) -> Result<(), Violation> { self.as_check().check(ctx) }
}
```

- `Rebuy` and `AddOn` share `RepeatRule` as data but are separate types, so each has its own checks and can diverge later without touching the other.
- The shared checks (window open, amount within the rule, count under the limit) live on `StakeCtx`, so every rule uses the same code for the same kind of check.
- The newtypes cost nothing in the log: Borsh writes a one-field struct as just its field, so the bytes are the same as a plain enum.
- Rules are stored as the closed enum, never as `Box<dyn RuleCheck>`. A boxed trait object can't be decoded from the log without a hand-written type registry, an open set would let phones disagree about which rules exist, a `match` on the enum fails to compile when a variant is missing, and the enum gets serialize, compare, clone and display from derives. `&dyn RuleCheck` is only ever a short borrow from the enum while a check runs, so the trait must stay dyn-compatible: methods instead of associated constants, and no generic methods. A unit test checks that every variant's `as_check().kind()` equals its `RuleKind`.

Checking a stake runs every rule that applies to it and collects all violations, so the admin's phone can say exactly why a stake was refused ("Juris already has 1 entry, maximum 1").

**What this pattern is called.** Two established names describe it:

- **Specification pattern** (Domain-Driven Design, Evans and Fowler): each rule is a small object that answers "does this candidate satisfy me?", and a candidate is valid when every specification passes. Each `RuleCheck` type is a specification, and the candidate is a stake with its `StakeCtx`.
- **Consensus rules** (a ledger term; some chains say validity predicates): rules that every node evaluates identically to accept or reject an entry. Betty's rule checks are consensus rules because every phone runs them during replay, which is why a stake that breaks one is invalid even when the admin included it.
- It is not a **rules engine** in the usual sense, where rules are data that can change without recompiling (as in Drools). See "Decisions" in [design/rules.md](../design/rules.md).

### Permitting rules

The mapping itself is a decision, in the table in `design/rules.md`. An exhaustive match is one way to make sure a new stake kind can't skip it.

```rust
impl StakeKind {
    // Exhaustive: a new stake kind does not compile until it names its permitting rule.
    fn permitting_rule(self) -> RuleKind {
        match self {
            StakeKind::BuyIn => RuleKind::BuyIn,
            StakeKind::Rebuy => RuleKind::Rebuy,
            StakeKind::AddOn => RuleKind::AddOn,
            StakeKind::Bet => RuleKind::BetAmount,
        }
    }
}
```

### `StakeCtx`

`StakeCtx` is what every rule check receives: a short-lived, read-only view of the stake being checked and the counts its rules read. It is defined in `rules` and names no activity type.

```rust
struct StakeCtx<'a> {
    member: &'a PublicKey,               // who made the stake (the action's signer)
    stake: &'a Stake,                    // activity id, action (kind and outcome), amount
    player: &'a PlayerCounts,            // this player's counts so far in this activity
    players: u16,                        // players bought in so far, for MaxPlayers
    open_windows: &'a BTreeSet<RuleKind>,
}

struct PlayerCounts {
    entries: u8, rebuys: u8, add_ons: u8, bets: u8,
    staked: u64,                         // units, all stakes in this activity
    outcomes_backed: BTreeSet<OutcomeIndex>,  // Pool
}
// Shared helpers every rule uses: require_window, require_amount, require_count, ...
```

1. The state layer receives an accepted `Stake` (currently unwrapped by the ordering layer from an `Include` entry) and checks what belongs to the party: the owner is Active, and the party's max buy-in.
2. `activities` looks up the `ActivityState` and the member's `PlayerTally` (empty on their first stake) as replay has built them up to this entry, and checks what belongs to the activity: it is open and not locked, and a bet's outcome exists.
3. It borrows the counts into a `StakeCtx`, checks the permitting rule, runs every rule whose `applies_to` matches, and drops the view.
4. Only if every check passes does replay update the tally, the pool and the chips in play.

The same checks run before anything is signed: the player's phone uses them to grey out a stake that would fail, and the admin's phone to show why a stake can't be included. `StakeCtx` is never stored, signed or synced, and because it is read-only a rule can only accept or refuse a stake, never change state.

## What happened

Nothing yet: M2 has not started. Record each deviation here as it happens, with its reason, and the outcome when the milestone ends.
