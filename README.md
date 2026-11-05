# Betty

**The kitty for your poker night.**

Betty keeps the money side of a poker night or betting party on the players' own phones: who bought in, who bet on what, who won, and who owes whom. When the night is over, it works out the fewest payments that settle everyone up and shows who pays whom. After the first beta it will also, where it can, pre-fill each payment in the payer's own payment or wallet app. The beta is intended to work with no server and no accounts, and it doesn't hold the money.

It is made for groups of friends: a home poker game, a tournament, a darts night, a pub quiz, or bets on the match on TV.

> **Status:** early development. The design for the beta is complete and the code is being written. There is no app to install yet. See [Status and roadmap](#status-and-roadmap).

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

1. **Create a party.** The admin picks a name, the currency and the co-admins who can take over if the admin's phone dies. The admin then sends an invite link through WhatsApp or any other chat app.
2. **Join.** On the day of the party, friends tap the link, or paste it into Betty if their chat app doesn't make it tappable, and join once they're on the admin's Wi-Fi. The admin sees each request with a four-word fingerprint to compare against the joiner's screen, and lets them in. A party holds up to 16 people, the admin included. Someone without a phone can be added too, and the admin enters everything for them, marked as admin-entered.
3. **Play.** The admin opens a cash game and a bet on the match. Players buy in and bet from their own phones, the admin confirms each stake, and every phone updates.
4. **Results.** The admin enters who won. Players accept or dispute, and a mistake gets a visible correction.
5. **Settle up.** Betty works out the fewest payments that settle everyone up, and the players accept the statement. Every phone shows a four-word fingerprint of the final record, so the table can check that everyone holds the same history. Betty shows each payer who to pay and how much, and the payer pays their own way, such as in cash. After the first beta, a payer will also be able to tap to open a pre-filled request, such as 35 EUR to Anna, as a Revolut or PayPal link or a crypto payment link. Payers mark payments as sent and the people paid mark them as received, so everyone can see who still owes.
6. **Final.** Anyone, including players who went home early, can dispute the settlement for 48 hours by default. After that the party is final.

## What you can run

A party can hold several activities at once, and they all settle up together at the end. Each activity pays out in one of four ways:

| Activity | How it pays out | For example |
| --- | --- | --- |
| Chip count | Chips are bought at a fixed rate, and each player's final count is paid out at the same rate | A poker cash game, or any game whose chips or points convert to money |
| Places | The prize pool is paid by finishing place, from a payout chart that can pay more places as more players enter | A poker tournament, a darts knockout, a pub quiz |
| Pool | Players back an outcome, and the backers of the winner share the whole pool in proportion to their stakes | "Who wins tonight's match?" |
| Head-to-head | Two players put up the same stake, and the winner takes the loser's | "Mark can't finish a pint in 10 seconds" |

**Presets** set up the common games in a tap: a poker cash game, a sit-n-go, a rebuy tournament, a match-winner bet, a side bet and a darts knockout. The admin can adjust a preset, save their own and share it as a file.

**Rules** belong to each activity: the buy-in, how many rebuys and add-ons are allowed, a cap per player, and when entries or bets close. The admin can adjust them during the game, for example to stop late entries or to allow one more rebuy, and every phone shows the change. Every phone also checks every stake against the rules, so not even the admin can let through a stake that breaks them.

### Two ways to run a party

The first beta has one way, Admin + majority. Everyone agrees follows once the rest of Betty is finished. From then on, the admin picks one when creating the party, and it stays fixed for that party.

- **Admin + majority** suits a host who runs everything, at a home game or a bigger event. Results count as soon as the admin enters them. Players can accept or dispute any result, and a dispute flags it for the admin to correct, without stopping the game. A majority of the party is needed for only three things: locking in the results so far, closing the final settlement, and letting a co-admin take over from a missing admin.
- **Everyone agrees** (after the first beta) suits small home games among friends. After a round, each player enters their own chip count, and the counts must add up to the chips in play. The round counts only once every player accepts the result, and a dispute leads to a corrected version that everyone accepts again. A change to an activity's rules works the same way, while opening and closing entries or betting takes effect at once. So that the game never stalls, the admin can close a round by majority if someone stays silent for 15 minutes, or if the same line is still disputed after two corrected versions. The record shows who didn't accept.

## When things go wrong

Betty is built for real nights, where batteries die and people leave early.

- **A player leaves.** They leave from the app with their final chip count, or the admin marks them as gone if their phone died. Their balance carries into the settle-up and their open bets stay in. If a chip count is open, another player can witness their stack so the admin can count it for them.
- **Players who went home** settle with one person, the admin by default, so nobody else has to chase them.
- **The admin disappears.** After 10 minutes of silence a co-admin is offered the chance to take over, and does so with the votes of a majority of the party. The admin can also hand over on purpose at any time. If the old admin comes back, their phone switches over and they carry on as a player.
- **A phone is lost.** The player's new phone creates a new identity, and their seat and balance move to it once the admin and one witness confirm it. The party history comes back from any other phone.
- **Nothing fails silently.** Phones show when a stake hasn't reached the admin yet, when they are missing updates, and when a result was an admin ruling.
- **Rewritten history is caught.** If the admin ever signs two different versions of the record, every phone shows a red warning and stops at the point where they split. The party continues only when a co-admin takes over from the last point both versions share.

## Servers, accounts and money

The beta is intended to work with no server and no accounts, and it doesn't hold the money. This may change as the project progresses.

## Status and roadmap

Betty is in early development. The design for the beta is complete, and the code is being built in five phases. Each phase ends with a test it must pass before the next one starts.

1. **Core.** The ledger, rules, balances and settlement, tested with scripted parties in a single process. No networking yet.
2. **Sync.** The protocol phones use to exchange records, tested on 1,000 simulated parties with lost messages, delays, network splits and a vanishing admin.
3. **Local networking.** Real sync over Wi-Fi, tested with four laptops playing the same party.
4. **The app.** Betty for Android and iPhone, done when two people on real phones run a party with a pool bet and a head-to-head start to finish with no manual fixes. This is the first beta, with no store listing: a direct download on Android and a TestFlight link on iPhone.
5. **Everyone agrees and payment links.** The second way to run a party, tested on 1,000 simulated parties and a real two-player chip game on real phones, and pre-filled requests for Revolut, PayPal and crypto wallets, tested in the real apps. This is still a beta.

What Betty needs to reach 1.0 hasn't been decided yet.

The core is written in Rust and does all the bookkeeping, identically on every phone. The app around it is built with Tauri 2 and only shows what the core computes.

## Learn more

- For the detailed design, see [docs/design/](docs/design/). It holds the binding decisions, with the reasoning behind the architecturally significant ones, and is the source of truth for how Betty works. Its files also show each use case as a diagram, from joining a party to the final settlement.
- For the preferred way to build it, milestone by milestone, see [docs/guidance/](docs/guidance/).
- For ideas nobody has committed to, see [docs/ideas.md](docs/ideas.md).

## Licence

Betty is free software under the GNU Affero General Public License, version 3 only. See [LICENSE](LICENSE).
