# Activities

Activities are opened by the admin inside a party. An activity's kind is a payout model rather than a particular game, so one model serves poker, darts or a pub quiz alike, and presets (below) give each game its name and rules. Its parameters say how it pays out; its rules ([rules.md](rules.md)) say who may stake what and when. The parameters are one enum, and `ActivityKind` is the same variants without data, so a kind can never carry another kind's parameters:

```rust
// Activity kinds are payout models, not games; presets give each game its name and rules.
enum ActivityParams {                       // append-only, like Rule
    ChipCount { chip_value: Ratio },        // stakes buy chips; paid by final count x chip value
    Places { payouts: Vec<PayoutTier> },    // pool paid by finishing order; the split depends on players
    Pool { question: String, outcomes: Vec<String> },  // pari-mutuel: backers of the winner share the pool
    HeadToHead { claim: String },           // two players, matched stake; players and stake are rules
}

struct PayoutTier {                         // one row of a Places payout chart, fixed at open
    min_players: u16,                       // applies from this many players up to the next tier
    payout_bps: Vec<u16>,                   // index = place, basis points summing to 10,000
}

struct Activity {                           // fixed at OpenActivity, except rules and windows
    id: ActivityId,                         // hash of the OpenActivity entry: unique, unforgeable
    name: String,                           // "Friday sit-n-go"
    game: Option<String>,                   // "Poker", "Darts": display and cross-party stats only
    params: ActivityParams,
    rules: RuleSet,
    open_windows: BTreeSet<RuleKind>,
}
```

Buy-in amounts, rebuys, add-ons, the head-to-head players and stakes are not parameters: they are rules, so they change through the same actions in every activity.

**Parameters in detail**

- **`ChipCount { chip_value }`**: units per chip as an integer fraction, for poker cash games or any game whose chips or points convert to money at a fixed rate. With a EUR party (units are cents), 1 chip = 0.10 EUR is `Ratio { num: 10, den: 1 }`.
- **`Places { payouts }`**: the payout chart for any ranked competition, such as a poker tournament, a darts knockout or a pub quiz. Each `PayoutTier` gives a split, `payout_bps`, and the number of players from which it applies. In a split, index 0 is first place and each value is that place's share of the prize pool in basis points (1 bp = 0.01%, 10,000 = 100%): `[5000, 3000, 2000]` pays 50/30/20 and `[10000]` is winner takes all. Basis points keep the core free of floats while still allowing splits such as 62.5/37.5 or 33.33/33.33/33.34.
  - The chart is set when the activity opens and is fixed from then on, like every parameter. When the `Result` comes in, every phone uses the last tier whose `min_players` is at most the number of players. A player counts once they have bought in, only once however many times they re-enter, and still counts after leaving; rebuys, add-ons and re-entries only grow the pool. Example chart: 1 to 5 players pay 65/35, 6 to 10 pay 50/30/20, and 11 or more pay 40/25/18/10/7. A chart with one tier is a single fixed split.
  - The prize pool is every buy-in, rebuy and add-on in the activity. Place i gets pool × `payout_bps[i]` / 10,000 from the chosen tier, with the rounding rule in Balances. Six players at 20 make a pool of 120, which the example chart's second tier pays as 60, 36 and 24.
  - If fewer players finish than there are paid places, the unfilled places' shares are split among the paid places in proportion to their shares: 50/30 with two finishers becomes 62.5/37.5. Leaving the party or being departed never changes a player's standing, because Betty never works out a placing by itself: the admin's result places every entrant, including players who have gone home. In poker their stack stays at the table and is blinded off, so they finish wherever it runs out, and a share in the money reaches them through settle-up. A forfeited darts match or a quiz team that leaves early is recorded the same way, as an ordinary result. Ties and deals (two players tied for second, a chopped final table) are not handled in the current design.
- **`Pool { question, outcomes }`**: `question` is the text every phone shows ("Who wins tonight's match?"), and `outcomes` are the possible answers (`["Latvia", "Estonia", "Draw"]`).
  - An outcome is identified by its index (`OutcomeIndex = u16`): a bet stores `outcome: 1`, the `Result` stores the winning index, and a player's backed outcomes are held as indexes. The text is for display only; no rule or balance reads it.
  - Because bets point at indexes, outcomes are fixed once the activity is opened. If one is missing, the admin voids the bet (everyone is refunded) and opens a corrected one.
  - Payout is pari-mutuel: the winners split the whole pool in proportion to their stake on the winning outcome. If nobody backed the winner, or the bet is voided, everyone is refunded. Example: Anna 10 and Juris 30 on Latvia, Mark 20 on Estonia. Latvia wins, so the pool of 60 pays Anna 15 and Juris 45; the nets are +5, +15 and −20.
- **`HeadToHead { claim }`**: display text ("Mark can't finish a pint in 10 seconds"). The `Result` names which of the two allowed players won.

**Limits checked at `OpenActivity`**, to keep records small, because every phone stores and syncs the full log. Lengths are counted in Unicode characters (scalar values), so every phone gets the same count.

| Field | Limit |
| --- | --- |
| Activity name | 1 to 60 characters |
| `game` label | Optional, 1 to 40 characters; compared case-insensitively for stats |
| `question`, `claim` | 1 to 200 characters |
| `outcomes` | 2 to 16 entries, each 1 to 60 characters, no duplicates |
| `payouts` | 1 to 8 tiers, the first with `min_players` 1 and each later one higher; each tier's `payout_bps` has 1 to 16 places, each value 1 to 10,000, summing to exactly 10,000 |
| `chip_value` | numerator and denominator each 1 to 1,000,000 |

**Activity kinds only declare a schema:** which rules are required, which are allowed, and the template the admin starts from.

| Activity kind (payout model) | Required | Also allowed | Default template |
| --- | --- | --- | --- |
| `ChipCount` | `BuyIn` | `Rebuy`, `MaxPlayers`, `StakeCap`, `AllowedPlayers` | Buy-in range, unlimited rebuys with the window open, registration open |
| `Places` | `BuyIn` (fixed), `MaxEntries` | `Rebuy`, `AddOn`, `MaxPlayers`, `AllowedPlayers` | Fixed buy-in, one entry, no rebuys |
| `Pool` | `BetAmount` | `MaxBets`, `OneOutcomePerPlayer`, `StakeCap`, `AllowedPlayers` | Amount range, one bet per player, one outcome per player |
| `HeadToHead` | `AllowedPlayers` (two), `BetAmount` (fixed), `MaxBets` (1) | none | The two players and the agreed stake |

`AmendRules` or `ProposeRules` is refused if it removes a required rule or adds one the activity kind doesn't allow.

**Running totals.** An activity is never stored as a record of its own. Every phone rebuilds it by replaying the log and keeps running totals beside it, which the rule checks and the balance rules read:

- **For each activity:** its status (`Open`, `Locked`, `Resulted` or `Voided`), the pool (units staked in total), the chips in play (chip games: buy-ins, rebuys and add-ons in chips), the rounds (counts and round results), any voted rule change waiting for its `Close` (Phase 5), and the outcome (winning outcome, places or winner).
- **For each player in it:** entries, rebuys, add-ons and bets so far, units staked (all stakes in this activity), outcomes backed (`Pool`), chips cashed out (`ChipCount`), and whether they are seated: bought in, not cashed out, not left.

| Entry | Effect on the activity |
| --- | --- |
| `OpenActivity` | Creates it with its parameters, starting rules and open windows |
| `Include` of a stake | Runs the rule checks, then updates the player's tally, the pool and the chips in play |
| `AmendRules` | Changes rules or windows; in Everyone agrees mode (Phase 5), only windows |
| `ProposeRules` (Phase 5) | Records a `Set` or `Remove` as pending until its `Close` |
| `Close` (Phase 5) | Applies a voted rule change or closes a round |
| `LockActivity` | Closes every window; status Locked |
| `Result`, `Void`, cash-outs | Set the outcome or the cash-outs; status Resulted or Voided |
| `Correction` | Replay drops the reversed entry's effect and recomputes the totals |

**Presets.** A preset is a named, ready-made activity: a game label, parameters, starting rules and open windows. It is plain data, read into the same types as above, so it can only combine existing payout models and rule kinds and never adds behaviour.

- **Built-in presets** ship with the app: poker cash game, sit-n-go, rebuy tournament, match winner (`Pool`), side bet (`HeadToHead`) and darts knockout (`Places`).
- **Admins** can adjust a preset before opening, save their own, and share presets as a file, sent through the share sheet and imported into Betty.
- **Picking a preset** fills in a normal `OpenActivity`, which then passes the usual schema and limit checks. The log never refers to the preset, so changing or deleting a preset never affects a party.
- **Amounts** are stored with the unit the preset was saved in. Using a preset in a party with a different unit asks the admin to confirm the amounts.
- **Format.** Presets are an import format only, never what is signed or synced.
- **Where.** `engine` parses preset files and holds the built-in set, saving and sharing. "Decisions" below explains why.

## Decisions

### `engine` loads presets

Accepted 2 November 2026.

`engine` parses preset files and holds the built-in set, saving and sharing. `ledger` holds only the types a preset reads into and the checks every `OpenActivity` must pass. A preset can't change replay, because the log never refers to it, so none of this needs to be in the pure crate. `ledger` was the earlier placement, which would have brought a file format into it for something that can't affect consensus. This moved preset loading and the built-in set from M2 to M5, because `engine` arrives in M5.

### Presets are shared as files

Accepted 3 November 2026.

Presets are shared only as files. A file works whether or not the two admins are together and has no size limit. A QR code was the alternative, but a phone can't scan a code it received itself, and a code holds only about 1 to 2 KB reliably, which the largest presets could exceed.
