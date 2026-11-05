# Rules

Each activity holds a set of modular rules keyed by rule kind, and the same rule checks apply to every stake whatever the activity. The admin sets the rules in `OpenActivity`, starting from a template for the activity's kind ([activities.md](activities.md)), so for example a player cannot buy into the same sit-n-go twice unless a rebuy rule is present. Rules are part of the signed log, so every phone enforces the same limits.

```rust
// The rule kinds and the data each one carries.
struct BuyIn(AmountRule);               // amount of the first entry
struct MaxEntries(u8);                  // entries per player; 1 = no re-entry
struct Rebuy(RepeatRule);
struct AddOn(RepeatRule);
struct MaxPlayers(u16);
struct StakeCap(u64);                   // per player, all stakes in this activity
struct BetAmount(AmountRule);
struct MaxBets(u8);                     // stakes per player on a Pool
struct OneOutcomePerPlayer;
struct AllowedPlayers(Vec<PublicKey>);

// Shared building blocks.
enum AmountRule { Fixed(u64), Range { min: u64, max: u64 } }
struct RepeatRule { amount: AmountRule, max_count: Option<u8> }  // None = unlimited

// The closed set that is signed and stored. RuleKind is the same variants without data.
// Variants are append-only: Borsh writes the variant number, and numbers are never reused.
enum Rule {
    BuyIn(BuyIn), MaxEntries(MaxEntries), Rebuy(Rebuy), AddOn(AddOn),
    MaxPlayers(MaxPlayers), StakeCap(StakeCap), BetAmount(BetAmount),
    MaxBets(MaxBets), OneOutcomePerPlayer(OneOutcomePerPlayer),
    AllowedPlayers(AllowedPlayers),
}

struct RuleSet(BTreeMap<RuleKind, Rule>);  // held by Activity (see activities.md)
```

**Enum with data rather than (key, value) pairs.** The key is derived from the variant, so a `Rebuy` key can never hold an add-on's settings. On the wire the set is a list sorted by kind with no duplicates, and the decoder rejects anything else, so every phone produces the same bytes, which a `HashMap` would not guarantee. In memory it is a map.

**The same actions for every activity.** Rules change only through one admin entry, `AmendRules { activity, changes }`, or `ProposeRules` for a voted change in Everyone agrees (Phase 5), and the possible changes are the same four for every activity:

| Change | Effect |
| --- | --- |
| `Set(Rule)` | Adds the rule, or replaces the one of the same kind |
| `Remove(RuleKind)` | Drops the rule |
| `OpenWindow(RuleKind)` | Opens the time window of a windowed rule: `BuyIn` (registration and late entry), `Rebuy`, `AddOn`, `BetAmount` (betting) |
| `CloseWindow(RuleKind)` | Closes it; `LockActivity` closes every window at once |

Several changes in one entry apply together, from the moment they take effect onward, never to stakes already included. Every phone shows the change. `AmendRules` or `ProposeRules` is refused if it breaks the activity kind's schema, as [activities.md](activities.md) describes.

```rust
struct AmendRules {
    activity: ActivityId,
    changes: Vec<RuleChange>,
}
struct ProposeRules {              // Everyone agrees, from M9
    activity: ActivityId,
    changes: Vec<RuleChange>,      // Set and Remove only
    supersedes: Option<EntryId>,   // a revised proposal after a dispute
}
enum RuleChange { Set(Rule), Remove(RuleKind), OpenWindow(RuleKind), CloseWindow(RuleKind) }
```

**Who approves a change depends on the mode.**

- **Admin + majority:** every change takes effect when the admin signs `AmendRules`. Players can dispute it afterwards, like a result.
- **Everyone agrees (Phase 5):** `Set` and `Remove` change the deal everyone is playing under, so they work like a round result. The admin signs a `ProposeRules`, every Active member accepts or disputes it, and a dispute leads to a revised `ProposeRules` that supersedes it. The change takes effect only at the `Close` entry that records every Active member's acceptance, with the same escape hatches: a majority close after 15 minutes of silence or after two disputed revisions, recorded with who did not accept. Until then the activity carries on under the current rules, so play never waits.
- **Window changes, both modes:** `OpenWindow` and `CloseWindow` take effect when the admin signs them, with no vote. They only time rules everyone already agreed to, and waiting for acceptance before closing betting would let bets in after the match has started. In Everyone agrees mode `AmendRules` holds only window changes, so an `AmendRules` is always immediate and a `ProposeRules` always voted.
- **Starting rules, both modes:** the rules in `OpenActivity` need no vote. A player agrees to them by staking, and nobody is bound before their first stake.

**Example: a rebuy tournament.**

1. `OpenActivity` (filled in from the Rebuy tournament preset) sets a fixed buy-in of 20, one entry per player, up to 2 rebuys of 20 and one add-on, with the registration and rebuy windows open.
2. Cards are dealt: `AmendRules [CloseWindow(BuyIn)]` stops late entries.
3. The group wants a third rebuy: `AmendRules [Set(Rebuy(RepeatRule { amount: Fixed(20), max_count: Some(3) }))]`. With Admin + majority it applies at once. With Everyone agrees the same change is a `ProposeRules`, and it applies once every Active member has accepted it.
4. Break after level 4: `AmendRules [CloseWindow(Rebuy), OpenWindow(AddOn)]` ends rebuys and opens add-ons in one entry.
5. After the break: `AmendRules [CloseWindow(AddOn)]`.

For a pool bet, `CloseWindow(BetAmount)` stops betting when the match starts. `LockActivity` closes every window at once.

**Rule checks are consensus rules.** Every phone evaluates them identically during replay to accept or reject an entry. A stake that breaks a rule is invalid during replay even if the admin includes it, so the admin cannot bypass the rules either. The preferred way to build the checks is in [guidance/rules.md](../guidance/rules.md).

**Stakes and the rules that check them.** A stake kind is what a player does with money; a rule kind is a limit the admin sets. They are separate enums on purpose. The link between them is many-to-many, and each enum grows on its own in the log, so a new limit never creates a stake nobody can make and a new stake never creates a rule with nothing to check.

```rust
// What a Stake action does. StakeKind is the same variants without data.
enum StakeAction {                          // append-only, like Rule
    BuyIn, Rebuy, AddOn,
    Bet { outcome: OutcomeIndex },
}
struct Stake { activity: ActivityId, action: StakeAction, amount: u64 }
```

- **Every stake kind has exactly one permitting rule.** It allows the stake, sets its amount and keys its window: `OpenWindow(Rebuy)` opens rebuys and `CloseWindow(BetAmount)` stops betting. The names match for buy-ins, rebuys and add-ons; a bet's permitting rule is `BetAmount`.
- **A stake is checked in two steps.** First the activity must hold the permitting rule for the stake's kind, or the stake is refused as not allowed ("this activity has no rebuys"). Then every rule that applies to the stake's kind runs its check, as the table below lists.

Which rules check each stake kind:

| Stake kind | Permitting rule | Also checked by |
| --- | --- | --- |
| `BuyIn` | `BuyIn` | `MaxEntries`, `MaxPlayers`, `StakeCap`, `AllowedPlayers` |
| `Rebuy` | `Rebuy` | `StakeCap`, `AllowedPlayers` |
| `AddOn` | `AddOn` | `StakeCap`, `AllowedPlayers` |
| `Bet` | `BetAmount` | `MaxBets`, `OneOutcomePerPlayer`, `StakeCap`, `AllowedPlayers` |

**What a missing rule means.** A missing permitting rule means "not allowed" (no `Rebuy` rule, no rebuys). A missing limit means "no limit" (no `StakeCap`, no cap). This is fixed per rule kind, so replay never guesses.

**New rule kinds later.** Each rule kind records the format version that introduced it. A phone that meets a rule kind it doesn't know shows "update Betty" for that party rather than skipping the rule, because skipping would change which stakes count.

One thing stays with the admin: the app can't tell whether a player has really run out of chips before a rebuy, so that is the admin's call when including it.

**Rule logic is not configurable.** Rule kinds and payout models are compiled in, and new ones come only with a new format version. "Decisions" below explains why.

## Decisions

### Rules are a closed set, and rule logic is not configurable

Accepted 26 October 2026.

Each activity holds a set of rules keyed by rule kind, from a closed enum that is signed and stored. An open set would let phones disagree about which rules exist. Rule and payout behaviour is compiled in, not defined in config files.

**Alternatives considered.** A fixed struct of optional fields, the earlier draft, is simpler, but every new rule changes it and activities can't opt in and out cleanly. A small expression language ("rebuys at most 3 while the window is open") is the most flexible, but it is hard to validate, to explain on screen and to keep identical across app versions. The enum-keyed rule set sits between the two.

**Why rule logic is not configurable.** Defining new rule or payout behaviour in config files was considered and rejected for now. The definition would have to travel in the signed log, making its interpreter part of the protocol; payout formulas from a file could break the sum-to-zero guarantee; the UI could not explain arbitrary logic; and every phone would still need an app update to understand it.

An expression-based rule variant remains possible, as a note in [ideas.md](../ideas.md).
