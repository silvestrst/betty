# Betty

**The kitty for your poker night.**

Betty keeps the money side of a poker night or betting party on the players' own phones: who bought in, who bet on what, who won, and who owes whom. When the night is over, it works out the fewest payments that settle everyone up and pre-fills each one in the payer's own bank or wallet app. There is no server and no account, and Betty never touches the money.

It is made for groups of friends: a home poker game, a tournament, a darts night, a pub quiz, or bets on the match on TV.

> **Status:** early development. The v1 design is complete and the code is being written. There is no app to install yet. See [Status and roadmap](#status-and-roadmap).

## The idea

On a typical game night one person keeps track on paper or in a notes app. The numbers are only as good as their memory, and the record goes home with them.

Betty gives every phone at the table the same record instead.

- **One person runs the party.** The admin, usually the host, opens games and bets, confirms stakes and enters results. They can play too, or just run things.
- **You sign your own money.** Your buy-ins and bets are signed by your phone and count once the admin confirms them. The admin can't invent a stake for you, and you can't claim one the admin never confirmed.
- **Every phone keeps the full record** and works out every balance itself, so there is no single copy to lose and no server to trust.
- **Nothing is edited or deleted.** A wrong result is fixed with a correction that everyone sees next to the original.
- **Balances always add up to zero.** Payouts come only from the players' own stakes, and nothing goes to a house.

Under the hood it is a signed, chained log shared by the phones in the room, not a blockchain.

## A night with Betty

1. **Create a party.** The admin picks a name, the currency, one of the two modes described below, and the co-admins who can take over if the admin's phone dies. The admin's phone then shows an invite QR code.
2. **Join.** Friends scan the code, or type a short text code if a camera won't cooperate. The admin sees each request with a four-word fingerprint to compare against the joiner's screen, and lets them in. A party holds up to 16 people, the admin included. Someone without a phone can be added too, and the admin enters everything for them, marked as admin-entered.
3. **Play.** The admin opens a cash game and a bet on the match. Players buy in and bet from their own phones, the admin confirms each stake, and every phone updates.
4. **Results.** The admin enters who won. Players accept or dispute, and a mistake gets a visible correction.
5. **Settle up.** Betty works out the fewest payments that settle everyone up, and the players accept the statement. Every phone shows a four-word fingerprint of the final record, so the table can check that everyone holds the same history. Each payer taps to open a pre-filled request, such as 35 EUR to Anna, as a SEPA bank-transfer QR code, a Revolut or PayPal link, or a crypto payment link. Payers mark payments as sent and the people paid mark them as received, so everyone can see who still owes.
6. **Final.** Anyone, including players who went home early, can dispute the settlement for 48 hours by default. After that the party is final.

## What you can run

A party can hold several activities at once, and they all settle up together at the end. Each activity pays out in one of four ways:

| Activity | How it pays out | For example |
| --- | --- | --- |
| Chip count | Chips are bought at a fixed rate, and each player's final count is paid out at the same rate | A poker cash game, or any game whose chips or points convert to money |
| Places | The prize pool is paid by finishing place, from a payout chart that can pay more places as more players enter | A poker tournament, a darts knockout, a pub quiz |
| Pool | Players back an outcome, and the backers of the winner share the whole pool in proportion to their stakes | "Who wins tonight's match?" |
| Head-to-head | Two players put up the same stake, and the winner takes the loser's | "Mark can't finish a pint in 10 seconds" |

**Presets** set up the common games in a tap: a poker cash game, a sit-n-go, a rebuy tournament, a match-winner bet, a side bet and a darts knockout. The admin can adjust a preset, save their own and share it with a QR code.

**Rules** belong to each activity: the buy-in, how many rebuys and add-ons are allowed, a cap per player, and when entries or bets close. The admin can adjust them during the game, for example to stop late entries or to allow one more rebuy, and every phone shows the change. Every phone also checks every stake against the rules, so not even the admin can let through a stake that breaks them.

### Two ways to run a party

The admin picks one when creating the party, and it stays fixed for that party.

- **Admin + majority** suits bigger events and a host who runs everything. Results count as soon as the admin enters them. Players can accept or dispute any result, and a dispute flags it for the admin to correct, without stopping the game. A majority of the party is needed for only three things: locking in the results so far, closing the final settlement, and letting a co-admin take over from a missing admin.
- **Everyone agrees** suits small home games among friends. After a round, each player enters their own chip count, and the counts must add up to the chips in play. The round counts only once every player accepts the result, and a dispute leads to a corrected version that everyone accepts again. A change to an activity's rules works the same way, while opening and closing entries or betting takes effect at once. So that the game never stalls, the admin can close a round by majority if someone stays silent for 15 minutes, or if the same line is still disputed after two corrected versions. The record shows who didn't accept.

## When things go wrong

Betty is built for real nights, where batteries die and people leave early.

- **A player leaves.** They leave from the app with their final chip count, or the admin marks them as gone if their phone died. Their balance carries into the settle-up and their open bets stay in. If a chip count is open, another player can witness their stack so the admin can count it for them.
- **Players who went home** settle with one person, the admin by default, so nobody else has to chase them.
- **The admin disappears.** After 10 minutes of silence a co-admin is offered the chance to take over, and does so with the votes of a majority of the party. The admin can also hand over on purpose at any time. If the old admin comes back, their phone switches over and they carry on as a player.
- **A phone is lost.** The player's new phone creates a new identity, and their seat and balance move to it once the admin and one witness confirm it. The party history comes back from any other phone.
- **Nothing fails silently.** Phones show when a stake hasn't reached the admin yet, when they are missing updates, when a result was an admin ruling, and when a round was closed by majority.
- **Rewritten history is caught.** If the admin ever signs two different versions of the record, every phone shows a red warning and stops at the point where they split. The party continues only when a co-admin takes over from the last point both versions share.

## No server, no accounts

The phones are the system. Each one holds the full record of every party it belongs to and works out every balance itself, so it never needs the network just to show you where you stand.

- **Same Wi-Fi.** Phones on the same network find each other and sync, including on one player's phone hotspot. They sync with each other, not only with the admin, so updates spread even while the admin's phone is in a pocket.
- **QR codes.** Where the Wi-Fi keeps phones apart, as venue and guest networks often do, two phones sync by showing each other QR codes.
- **Private to the party.** Other people on the Wi-Fi can't tell which party is meeting, and nobody receives anything from the record until the admin has let them in.
- **No accounts.** Your identity is a key that your phone creates and keeps in its secure storage. There is no login and no cloud backup.

Phones sync while Betty is open and catch up whenever they meet again.

## What Betty is not

- **Not a bank.** It never holds money and takes no fees. Every payment happens in your own bank or wallet app.
- **Not a bookmaker.** It shows no odds, and every payout comes from the players' own stakes.
- **Not online.** v1 syncs only between phones on the same Wi-Fi or by QR code, so it works where the players are together. The design leaves room for later additions such as a relay for syncing over mobile data, but none of them is planned.

## Status and roadmap

Betty is in early development. The v1 design is complete, and the code is being built in four phases. Each phase ends with a test it must pass before the next one starts.

1. **Core.** The ledger, rules, balances and settlement, tested with scripted parties in a single process. No networking yet.
2. **Sync.** The protocol phones use to exchange records, tested on 1,000 simulated parties with lost messages, delays, network splits and a vanishing admin.
3. **Local networking.** Real sync over Wi-Fi and by QR code, tested with four laptops playing the same party.
4. **The app.** Betty for Android and iPhone, done when a real poker night of 4 to 8 people runs start to finish with no manual fixes.

The core is written in Rust and does all the bookkeeping, identically on every phone. The app around it is built with Tauri 2 and only shows what the core computes.

## Learn more

- For the detailed design, see [docs/design.md](docs/design.md). It is the source of truth for how Betty works.
- For diagrams of each use case, from joining a party to the final settlement, see [docs/diagrams.md](docs/diagrams.md).

## Licence

Betty is free software under the GNU Affero General Public License, version 3 only. See [LICENSE](LICENSE).
