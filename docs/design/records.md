# Record type catalogue

The catalogue has 9 member action kinds and 28 entry kinds; this table is the contract for `ledger` and its golden vectors. `ProposeRules`, `CountRequest`, `ProposeRound` and `Close` belong to the Everyone agrees mode, so they keep their numbers but are not implemented until M9.

**Member actions** (signed by the member they concern; included by the admin with `Include`, or by `Admit` for joins)

| Kind | Key fields | Notes |
| --- | --- | --- |
| `JoinRequest` | display name, invite proof (HMAC of the invite secret), transport key | Gives access to nothing until admitted |
| `Profile` | display name, payout methods | Latest included profile wins. Payout methods are empty until Phase 5 |
| `Stake` | activity, action (`BuyIn`, `Rebuy`, `AddOn`, `Bet { outcome }`), amount | Owner must be Active when included |
| `ProposeActivity` | activity kind and parameters | Lets a player suggest a head-to-head or pool bet; the admin opens it |
| `ChipReport` | activity, round, chips | Own count, Unanimous mode or any cash-out |
| `CashOut` | activity, chips | Leaving a chip-count activity (a cash game table) while staying in the party |
| `Leave` | optional final chip counts per open activity | Withdraws that member's pending actions |
| `Withdraw` | action id | Cancels one pending action |
| `Rekey` | old key, new key | Signed by the new key; included only with a witness (lost phone) |

**Entries** (admin of the epoch, except `Takeover`)

| Group | Kind | Key fields |
| --- | --- | --- |
| Party | `Genesis` | format (first field), name, max members, unit, ordering, mode, settings, co-admins, invite commitment |
| Party | `SetSettings` | changed settings (only before the first stake) |
| Party | `SetCoAdmins` | ordered list |
| Party | `RotateInvite` | new invite commitment; old links stop working |
| Membership | `Admit` | embedded `JoinRequest` |
| Membership | `AddManaged` | name, generated key for a no-phone member |
| Membership | `Depart` | member, reason (`Absent`, `Removed`), rejected action ids |
| Inclusion | `Include` | embedded member action; this entry is the delivery receipt |
| Inclusion | `Reject` | action id, reason |
| Inclusion | `OnBehalf` | member, action body, witness attestation embedded (proxy counts, managed members, departed cash-out) |
| Activity | `OpenActivity` | name, game label, parameters (the kind follows from them), starting rules and windows |
| Activity | `LockActivity` | id; closes every rule window. Also AmendRules: activity and a batch of rule changes (set, remove, open window, close window), applied forward only; in Everyone agrees mode it holds only window changes |
| Activity | `ProposeRules` | activity, set and remove changes, the proposal it supersedes; takes effect at a `Close` (Everyone agrees) |
| Activity | `Result` | activity, winning outcome (Pool, HeadToHead) or finishing order (Places) |
| Activity | `Void` | activity; all stakes refunded |
| Activity | `Correction` | reverses entry id, reason |
| Count | `CountRequest` | activity, round |
| Count | `RoundResult` | activity, round, counts, deltas; takes effect when signed (Admin + majority) |
| Count | `ProposeRound` | activity, round, version, supersedes, counts, deltas; takes effect at a `Close` (Everyone agrees) |
| Count | `Close` | target id (a ProposeRound or a ProposeRules), how (`Unanimous`, `MajorityTimeout`, `MajorityRevisions`), embedded attestations |
| Finality | `Checkpoint` | head, balances, electorate, epoch, optional embedded attestations |
| Finality | `Settlement` | settlement statement (see Settlement) |
| Finality | `SettlementRevision` | supersedes, new statement; only before `dispute_until` |
| Authority | `Handover` | to, new epoch |
| Authority | `Takeover` | signed by the candidate: new epoch, base entry |
| Authority | `Assume` | first entry of a takeover epoch, embeds the quorum of `TakeoverVote` attestations |
| Authority | `Reissue` | orphaned entry id from the old epoch, its body re-signed |

**Detached objects:** `Attest` (Accept, Dispute, Witness, TakeoverVote) and `Mark` (Sent, Received, Waived). **Sync messages, not stored as records:** heads, inventories, receipts for detached objects.
