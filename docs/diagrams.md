# Betty architecture diagrams

Mermaid source for the overview and each use case. They started from `assessments/party-ledger-assessment.md` (part c and Follow-ups 2 to 4) and were brought in line with the v1 technical design on 26 October 2026, which also shows the seven use-case diagrams next to the sections they explain. Solid arrows are signed records or messages, dashed arrows are attestations (Accept, Dispute, Witness, TakeoverVote), receipts or refusals.

## The whole picture

Every phone keeps a full copy of the signed log, and only the admin writes the official chain. New players join by scanning the admin's invite QR, and the admin approves each one. Phones sync over the same Wi-Fi or by QR tap-to-sync. A relay for mobile data, share-sheet links and browser joiners is a possible later addition. At the end each payer opens a pre-filled request in their own bank or wallet app, so money never passes through Betty.

```mermaid
flowchart LR
  subgraph Party["Party: Friday poker"]
    A["Admin's phone<br/>writes the chain<br/>full signed log"]
    P1["Anna's phone<br/>full signed log"]
    P2["Mark's phone<br/>full signed log"]
    P3["Juris's phone<br/>full signed log"]
  end
  N["Liga's phone<br/>wants to join"]
  QR{{"Invite QR or text code<br/>party id, invite secret,<br/>admin key and addresses"}}
  R[("Relay, possible later<br/>forwards encrypted traffic<br/>stores nothing")]
  BR["Browser joiner, possible later<br/>can play and bet only"]
  subgraph Pay["Settle-up, outside Betty"]
    BA["Mark's bank or wallet app"]
    AA["Anna's account"]
  end
  A -->|shows| QR
  N -->|scans| QR
  N -->|JoinRequest| A
  A -->|"approves: Admit entry"| N
  A <-->|"Wi-Fi sync or QR tap-to-sync"| P1
  A <--> P2
  A <--> P3
  P1 <--> P2
  P2 <--> P3
  P3 -.->|mobile data| R
  BR -.->|always via relay| R
  R -.->|end-to-end encrypted| A
  P2 -->|"opens pre-filled request<br/>35 EUR to Anna"| BA
  BA -->|transfer| AA
```

## Create a party and join it

The admin signs the Genesis entry and shows an invite QR, with a short text code as the fallback. The invite only lets someone ask to join: the joiner sees nothing from the log until the admin has checked the name and key fingerprint and signed an `Admit` entry that embeds the request. A full party (16 members in v1) refuses new requests.

```mermaid
sequenceDiagram
  autonumber
  participant AP as Admin's phone
  participant NP as Liga's phone
  participant MP as Members' phones
  AP->>AP: Sign Genesis (unit, mode, ordering, settings, co-admins, max members). Its hash is the party id
  AP->>AP: Make invite: party id, invite secret, admin transport key and addresses
  alt Camera works
    AP-->>NP: Liga scans the invite QR
  else Camera broken
    AP-->>NP: Liga types the short text code
  end
  NP->>NP: First launch: create identity keypair, kept in the keychain
  NP->>AP: JoinRequest over the same Wi-Fi (name, invite proof, transport key)
  Note over NP: Receives nothing from the log yet
  AP->>AP: Admin sees "Liga wants to join" and the four-word fingerprint
  alt Admin approves and the party is not full
    AP->>AP: Sign Admit entry embedding the JoinRequest
    AP->>NP: Admitted, then a full log sync
    AP->>MP: Sync the Admit entry
    MP->>MP: Verify signatures, add Liga as Active
  else Admin rejects, the party is full, or the invite expired or was rotated
    AP-->>NP: Refused, nothing shared
  end
```

## Buy in or place a bet

Anything that touches a player's money carries two signatures. The player signs a `Stake` member action, and the admin embeds it in an `Include` entry with a second signature. A receipt shows it arrived; only the `Include` makes it count.

```mermaid
sequenceDiagram
  autonumber
  participant P as Anna's phone
  participant A as Admin's phone
  participant O as Other phones
  P->>P: Sign Stake action: BuyIn 500 units, or Bet 50 on Red
  P->>A: MemberAction. Anna's phone shows "not delivered yet"
  A-->>P: Signed receipt. Anna's phone shows "delivered"
  A->>A: Check: Anna is Active, the activity is open, the stake passes its rules
  alt Admin includes it
    A->>A: Sign Include entry: seq, prev hash, the embedded stake
    A->>P: Sync the Include entry. The stake now counts
    A->>O: Sync the Include entry
    O->>O: Verify both signatures, recompute balances
  else Admin rejects it
    A-->>P: Reject entry with a reason. The stake never counts
  end
  opt Anna cancels before it is included
    P->>A: Withdraw action naming the stake
    Note over P,A: A Leave withdraws every pending action, and a Depart lists them as rejected
  end
  Note over P,O: The admin cannot invent a stake for Anna. Anna cannot claim a stake the admin never included.
```

## Results and corrections

Only the admin writes results, and in both modes a bet's result takes effect when the admin signs it. Players attest with Accept or Dispute, which never blocks the party. A mistake is fixed with a `Correction` that reverses the old entry, so nothing is edited or deleted.

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

## Chip count in Everyone agrees mode

Each player reports their own chips, and the app checks that the counts add up to the chips in play before the admin can propose a result. The round counts only once a `Close` entry is in the chain: with every Active member's Accept on the latest version, or by majority after a timeout or a repeated dispute, so the round never halts.

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
  A->>An: RoundResult v1
  A->>M: RoundResult v1
  A->>J: RoundResult v1
  M-->>A: Accept v1
  J-->>A: Dispute v1, Juris line: "1,350, and Anna has 2,400"
  An->>A: ChipReport 2,400
  J->>A: ChipReport 1,350
  A->>A: Sum check passes again
  A->>An: RoundResult v2 supersedes v1
  A->>M: RoundResult v2 ("unchanged for you")
  A->>J: RoundResult v2
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

## A player leaves or drops out

A player who leaves on purpose signs a `Leave` with their final chip count. A player who simply vanishes is marked Departed by the admin. If they are still seated in an open count, the admin may enter a proxy count as an `OnBehalf` entry, but only with one `Witness`, and the player can dispute it later.

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

## Admin handover and takeover

Only one key may write the chain at a time, numbered by epoch. A leaving admin hands over with one `Handover` entry. If the admin vanishes, a co-admin signs a `Takeover`, collects `TakeoverVote`s from a majority of the last checkpoint's electorate (the missing admin counts as a no), and starts the new epoch with an `Assume` entry.

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

## Checkpoint and final settlement

A checkpoint freezes everything before it once the admin and a quorum accept it. The settlement adds the fewest-transfers list and a 48-hour dispute window. Each payer then opens a pre-filled payment request in their own bank or wallet app and marks it sent.

```mermaid
sequenceDiagram
  autonumber
  participant A as Admin's phone
  participant An as Anna
  participant M as Mark
  participant J as Juris (gone home)
  participant B as Mark's bank or wallet app
  A->>An: Checkpoint: head, balances, electorate, epoch
  A->>M: Checkpoint
  An->>An: Holds the full chain up to the head? Same balances on replay?
  An-->>A: Accept, naming the head it holds
  M-->>A: Accept
  Note over A,M: Admin plus a quorum: frozen. Earlier entries can no longer be disputed
  A->>A: Settlement: final balances, fewest transfers, dispute window 48h
  A->>An: Settlement
  A->>M: Settlement
  Note over A,M: Each phone shows the head fingerprint as four words, and the table compares them
  An-->>A: Accept
  M-->>A: Accept: "I owe Anna 35 EUR"
  Note over A,J: Admin plus quorum closes it (Everyone agrees: every Active member, or a majority after the timeout). Juris shows "awaiting acceptance"
  M->>B: Open pre-filled request: 35 EUR to Anna (EPC QR, Revolut.me, PayPal.me or a crypto URI)
  B->>An: Transfer, done outside Betty
  M->>A: Mark Sent, synced to every phone
  An->>A: Mark Received
  alt Dispute inside the window
    J-->>A: Dispute, after syncing at home
    A->>A: Sign SettlementRevision that supersedes it
    A->>An: Revision to accept again
    A->>M: Revision to accept again
  else Window passes
    Note over A,J: Final. Any later entry other than a Mark is invalid
  end
```

## Sync over the relay

The relay is a possible later second transport, not part of v1. It forwards encrypted traffic and can see only metadata. Head gossip and delivery receipts make withholding visible, and same-Wi-Fi or QR sync always remain as fallbacks.

```mermaid
sequenceDiagram
  autonumber
  participant M as Mark's phone (mobile data)
  participant R as Betty relay
  participant A as Admin's phone
  M->>R: Connect with the per-party transport key (relay URL pinned in the invite)
  R->>A: Forward over QUIC, end-to-end encrypted
  A->>A: Is this key in the member list? Otherwise refuse
  M->>A: My head: epoch 1, seq 41, hash
  A->>M: My head: epoch 1, seq 47, hash
  M->>M: Show "missing updates"
  A->>M: Entries 42 to 47
  M->>M: Verify signatures and the hash chain
  M->>A: Dispute on entry 45, shown as "not delivered yet"
  A->>M: Signed receipt, now "delivered"
  Note over M,A: After setup iroh usually finds a direct path, and the relay only sees that a connection existed
  alt Relay withholds or is down
    M->>M: Still "missing updates" or "not delivered"
    M->>A: Fall back to Wi-Fi sync or QR tap-to-sync
  end
  Note over R: Cannot read, forge or reorder entries. Can see who and when, delay, or go down
```
