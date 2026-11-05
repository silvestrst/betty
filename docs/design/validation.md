# Validation, modes, dropouts and admin handover

Every phone runs the same three-stage check on every object, and replay applies only what passes. Stages 1 and 2 belong to the ordering layer and stage 3 to the state layer (see Ledger format). Objects that fail are kept in a quarantine table for diagnostics, never applied.

1. **Decode and verify.** Known format and type, canonical bytes (re-encoding gives the same bytes), strict signature check, correct `party`.
2. **Authority.** Member actions signed by the member they concern (or, for `OnBehalf`, by the admin with an embedded witness). Entries signed by the holder of their epoch. Attestations signed by a member.
3. **State rules,** checked during replay against the state just before the entry:

**Membership and stakes**

- A stake, chip report or cash-out is valid only if its owner was Active when included, the activity is open and not locked, and the stake satisfies the activity's rules (entry count, rebuy and add-on windows and counts, amounts, per-player caps, allowed players) and the party's max buy-in.
- Pending actions of a member who leaves are withdrawn by `Leave`; those of a departed member are listed as rejected in `Depart`. A stake that was never included never counts.
- `Managed` members' actions arrive only as `OnBehalf` entries and are flagged "admin-entered" everywhere.
- An `Admit` or `AddManaged` is valid only while fewer than `max_members` people have been admitted; a `Rekey` keeps the same member. When the party is full, the admin's phone shows "Party full (16 of 16)" on a new join request instead of Admit.

**Results and corrections**

- `Result`, `Void`, `Correction`, `Depart` and all authority entries are admin-only.
- A `Correction` names the entry it reverses; replay drops that entry's effect and keeps both visible.
- A head-to-head result shows "agreed" when both players accept it, otherwise "admin ruling".
- A `Places` result lists every entrant in finishing order, including players who have left or been departed. Leaving never changes a standing by itself.
- `RoundResult` is valid only in Admin + majority. `ProposeRules`, `CountRequest`, `ProposeRound` and `Close` are valid only in Everyone agrees, where `AmendRules` holds only window changes.
- A second `RoundResult` for the same round is invalid unless a `Correction` has reversed the first, which is how a round is corrected.

**A result and its correction, step by step.** This works the same in both modes, because a bet's result takes effect when the admin signs it.

```mermaid
sequenceDiagram
  autonumber
  participant A as Admin's phone
  participant M as Mark's phone
  participant An as Anna's phone
  A->>A: Sign Result: "Who wins the match" is Red
  A->>M: Sync Result
  A->>An: Sync Result
  M->>M: Recompute balances
  An->>An: Recompute balances
  An-->>A: Attest Accept
  M-->>A: Attest Dispute: "it was Blue"
  A->>A: Sign Correction that reverses the Result
  A->>A: Sign new Result: Blue
  A->>M: Sync Correction and new Result
  A->>An: Sync Correction and new Result
  M-->>A: Attest Accept on the new Result
  An-->>A: Attest Accept (may come late, never blocks)
  Note over A,An: Head-to-head bet: both players accept and it shows "agreed". If one is gone it shows "admin ruling".
  Note over A,An: Two admin entries with the same epoch and seq raise a red equivocation banner, and that epoch stops at the fork until a co-admin takes over.
```

**Chip counts (both modes)**

- Sum check: the counts in a `RoundResult` or `ProposeRound`, or the cash-outs closing a chip-count activity, must equal the chips in play (buy-ins, rebuys and add-ons converted to chips). A gap blocks the round and is shown as "reported 4,850, in play 5,000".
- A proxy count (`OnBehalf` with a `ChipReport`) needs: the player seated in that activity, a `Depart` already recorded, the count still open, and one `Witness` attestation from another Active `App` member. If the player's own report arrives before the round closes, it replaces the proxy. In Admin + majority, a count stays open until the admin signs that round's `RoundResult` or the cash-outs close the activity.

**A player leaving or dropping out, step by step.** A proxy count needs a `Depart` first and one `Witness`.

```mermaid
sequenceDiagram
  autonumber
  participant J as Juris's phone
  participant A as Admin's phone
  participant W as Anna (witness)
  participant O as Other phones
  alt Juris leaves on purpose
    J->>A: Leave action with final chip count 1,350. Pending actions are withdrawn
    A->>A: Include the Leave. Juris is now Left
    A->>O: Sync
  else Juris vanishes (battery dead, went home)
    Note over A: App shows "Juris last seen 25 min ago". The admin decides, nothing is automatic
    A->>A: Sign Depart (reason Absent), listing pending actions as rejected
    opt A count is open and Juris is still seated
      A->>W: Ask Anna to witness a count of 1,350
      W-->>A: Witness attestation: "I saw the stack"
      A->>A: Sign OnBehalf entry: ChipReport 1,350 with the witness embedded, then the sum check
      Note over A,W: Shown as "counted by admin, witnessed by Anna"
      opt Juris returns before the round closes
        J->>A: Own ChipReport replaces the proxy count
      end
    end
    A->>O: Sync
  end
  Note over J,O: Juris keeps the balance. It carries into settle-up and open bets stay in.
  J->>A: Later: reconnect, exchange heads
  A->>J: Every entry after Juris's head
  J-->>A: Accept, or Dispute anything not yet frozen, until the dispute window ends
```

**The two modes in plain words.** A party runs in one of two modes, chosen in Genesis and fixed for the party. Only Admin + majority exists until Phase 5 adds Everyone agrees (see "Decisions" in [phases.md](phases.md)).

- **Admin + majority** (`AdminMajority`) suits bigger events, a host who runs everything, and sweepstakes. It is also how home games run in the first beta.
  - The admin enters results and chip counts. A result takes effect the moment the admin signs it; nobody has to approve it first.
  - Players can Accept or Dispute any result, but that never blocks the party. A dispute flags the result for the admin, who fixes it with a `Correction`. Changes to an activity's rules also take effect as soon as the admin signs them.
  - A majority (the admin plus more than half of the Active members) is needed for only three things: freezing a checkpoint so earlier records can no longer be disputed, closing the final settlement, and letting a co-admin take over if the admin disappears.
- **Everyone agrees** (`Unanimous`, from Phase 5) suits small home games among friends.
  - After a round, the admin opens a count and each player enters their own chip count. The counts must add up to the chips in play before a result can be proposed.
  - The admin proposes the round result, and it closes only when every Active member accepts the latest version. A dispute leads to a corrected new version that everyone accepts again. Changing an activity's rules mid-game (Set or Remove) works the same way; opening and closing windows takes effect at once.
  - So it never halts: if someone is silent for 15 minutes, or the same line is still disputed after two revisions, the admin may close the round by majority. The close is recorded with who did not accept, and they can still accept or dispute later.

In both modes, only the admin writes the official log, dropouts and admin takeover follow the same rules, and bet outcomes are entered by the admin.

**Mode rules**

|  | Admin + majority | Everyone agrees (Phase 5) |
| --- | --- | --- |
| Who enters chip counts | Admin, or players via `ChipReport` | Each player via `ChipReport` after a `CountRequest` |
| Round or result takes effect | When the admin signs it | When every Active member accepts the latest `ProposeRound` version, recorded by `Close` |
| Rule change (`Set`, `Remove`) takes effect | When the admin signs `AmendRules` | When every Active member accepts the latest `ProposeRules` version, recorded by `Close` |
| Window change (`OpenWindow`, `CloseWindow`) | When the admin signs it | Same, no vote |
| Escape hatch | Not needed | `Close` by majority after the timeout (15 min) or after the same line is disputed past 2 revisions, recorded with who did not accept |
| Checkpoint frozen | Admin plus a quorum of its electorate | Same |
| Settlement closed | Admin plus quorum | Every Active member, or majority after the timeout |

**An Everyone agrees chip count, step by step (Phase 5).** The round counts only once a `Close` entry is in the chain.

```mermaid
sequenceDiagram
  autonumber
  participant A as Admin's phone
  participant An as Anna
  participant M as Mark
  participant J as Juris
  A->>An: CountRequest entry (cash game, round 3)
  A->>M: CountRequest
  A->>J: CountRequest
  An->>A: ChipReport action 2,450 (signed by Anna)
  M->>A: ChipReport action 1,100 (signed by Mark)
  J->>A: ChipReport action 1,300 (signed by Juris)
  A->>A: Include each report. Sum check: reported 4,850, in play 5,000
  alt Counts do not balance
    A-->>M: "150 missing", the round cannot be proposed yet
    M->>A: ChipReport 1,250 after a recount
  end
  A->>A: Sum check passes: 5,000
  A->>An: ProposeRound v1
  A->>M: ProposeRound v1
  A->>J: ProposeRound v1
  M-->>A: Accept v1
  J-->>A: Dispute v1, Juris line: "1,350, and Anna has 2,400"
  An->>A: ChipReport 2,400
  J->>A: ChipReport 1,350
  A->>A: Sum check passes again
  A->>An: ProposeRound v2 supersedes v1
  A->>M: ProposeRound v2 ("unchanged for you")
  A->>J: ProposeRound v2
  An-->>A: Accept v2
  M-->>A: Accept v2 (one tap)
  alt Juris accepts v2
    J-->>A: Accept v2
    A->>A: Sign Close (Unanimous) embedding every Accept. Replay applies the round
  else Juris silent for 15 min, or the same line disputed after 2 revisions
    A->>A: Sign Close (MajorityTimeout or MajorityRevisions), recorded as "not accepted by Juris"
    Note over A,J: Juris can still accept or dispute later
  end
```

**Quorum.** The electorate is the Active member list written into the latest checkpoint (or Genesis plus admissions before the first one). Quorum is more than half of it, admin included. It is used only to freeze a checkpoint, close a settlement and approve a takeover.

**Attesting what you hold.** A phone refuses to sign an Accept on a checkpoint or settlement unless it holds the full chain up to the head named in it and its own replay gives the same balances. Attestations carry that head, so a stale phone cannot be tricked into signing.

**Admin epochs**

- `Handover { to, epoch + 1 }` takes effect at once, no vote.
- `Takeover { candidate, epoch + 1, base }` is signed by the candidate. Members attest it with `TakeoverVote`, at most one per member per epoch; the phone enforces this. With a quorum of the last electorate (the missing admin counts as a no), the candidate writes `Assume` embedding the votes and continues from `base`.
- When the old admin returns, their phone switches to the higher epoch. Member actions embedded in their orphaned entries are re-included automatically; admin-only orphans are shown to the new admin for one-tap `Reissue`.
- Known limit: a two-person party cannot reach a takeover quorum, so the app offers a pre-signed `Handover` when a party has two members.

**Handover and takeover, step by step.** With four members the quorum is 3, and the missing admin counts as a no.

```mermaid
sequenceDiagram
  autonumber
  participant OA as Admin (epoch 0)
  participant C as Anna (co-admin)
  participant M as Mark
  participant J as Juris
  alt Planned handover
    OA->>OA: Sign Handover to Anna, epoch 1
    OA->>C: Sync
    OA->>M: Sync
    OA->>J: Sync
    Note over C: Anna is admin at once, no vote needed
  else Admin vanished
    Note over OA: Silent for 10+ min. The app offers "Take over"
    C->>C: Sign Takeover: candidate Anna, epoch 1, base is the last admin entry Anna holds
    C->>M: Ask for a TakeoverVote
    C->>J: Ask for a TakeoverVote
    M-->>C: TakeoverVote (at most one per member per epoch)
    J-->>C: TakeoverVote
    C->>C: Quorum: 3 of 4 (the missing admin counts as no)
    C->>C: Sign Assume embedding the votes, continue from base, include pending member actions
  end
  opt The old admin comes back
    OA->>C: Sync
    C->>OA: Takeover and Assume with the higher epoch
    OA->>OA: Switch to epoch 1, now a normal member
    OA->>C: Entries written offline after base
    C->>C: Re-include their member actions automatically, offer admin-only entries for one-tap Reissue
  end
  Note over OA,J: In a two-person party a takeover cannot reach quorum. A Handover signed in advance covers it.
```

**Time.** Entry timestamps come from the admin's clock and must not go backwards. Timeouts are measured from the time the ordering layer gives accepted records (currently the admin entry's timestamp), never from arrival times. The settlement's `dispute_until` is checked against the entry timestamp of any `SettlementRevision`; phones additionally show a revision that arrives long after the window as "late".
