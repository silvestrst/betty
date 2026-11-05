# Sync protocol and transports

One transport-agnostic sync protocol runs over any `Connection`; Betty ships a same-Wi-Fi transport, and a relay, if one is ever added, would be a second `Transport` implementation with no protocol change.

**Traits.** `sync` defines `Transport`, `Connection` and `Discovery`. Their preferred shape is in [guidance/sync.md](../guidance/sync.md).

**Session, in order**

1. `Hello`: party tag, protocol version, own transport key and a signature by the member's identity key over it (binding transport key to member). The other side refuses anyone not in its member list, except a `JoinRequest` with a valid invite proof, which may send only that request and receive only an `Admitted` or `Refused` answer.
2. `Heads`: each side's best admin head plus any equivocation evidence it holds.
3. `Want` / `Entries`: the side behind asks for `seq` ranges and verifies each entry before applying it.
4. `Inventory` / `Objects`: sorted id lists of detached objects (pending member actions, attestations, marks) since the last frozen checkpoint, and the missing ones exchanged. Parties are small, so a plain list beats set-reconciliation tricks.
5. `Receipt`: the admin signs a receipt for each member action or dispute it received, so the sender's phone moves from "not delivered yet" to "delivered" even before inclusion.
6. The session stays open while both apps are in the foreground and pushes new objects as they appear.

Every phone syncs with every phone, not only the admin, so updates spread even while the admin's app is in the background. Per-member rate limits and a cap on pending join requests protect the admin's battery.

**Transport: same Wi-Fi**

- **Connection:** QUIC with the per-party transport key as the TLS identity. The first M7 task is a spike comparing iroh with relays and public discovery switched off (then the relay is later a configuration change) against plain `quinn`. Both sit behind `Transport`, so the choice is reversible.
- **Discovery:** DNS-SD service `_betty._udp`. The advert carries only a blinded party tag (`HMAC(party id, "discovery" || day)`) so other people on the Wi-Fi can't tell which party is meeting. On iOS it goes through Bonjour (`NWBrowser`, `NWListener`) and on Android through `NsdManager`, both in a small Tauri plugin, because raw multicast on iOS needs a special Apple entitlement. Desktop and tests use the `mdns-sd` crate.
- **Direct address:** the invite link also carries the admin's IP addresses and port, so joining works even where multicast is blocked but unicast is not.
- **Hotspot:** if the venue Wi-Fi isolates clients, one phone turns on its hotspot and everyone joins it; it is just another LAN.

**Invite link.** A `betty://` link carrying the party id, invite secret (only its HMAC proof ever travels), admin transport key, admin addresses and party name. Expires at party close or after 12 h; `RotateInvite` kills old links.

- **Sending:** the admin sends it through the share sheet, to WhatsApp or any other chat app, in a pre-filled message that names the party, says Betty is needed, links to its install page and says when the link expires. Inside Betty the invite appears as the party name with share and copy buttons, never as raw text.
- **Opening:** the joiner taps the link, or pastes it into Betty's join screen if their chat app doesn't make it tappable. Pasting always works.
- **Joining:** `JoinRequest` still needs the same network as the admin. A phone that isn't on it yet waits and says so.

## Decisions

### No QR ledger sync

Accepted 3 November 2026.

Betty syncs the ledger only over a network connection, never by QR code. Phones scanning each other's codes in turn is too inconvenient to be a fallback anyone would use.

**Alternative considered.** A QR bundle mode for networks where phones can't reach each other: a `Bundle` of entries and detached objects, compressed and split into fountain-coded frames.

**Consequences.** A phone hotspot is now the only fallback when Wi-Fi keeps phones apart. Without a shared network, each phone still shows its own copy of the log but can't sync until phones can reach each other. `Bundle` is gone from the protocol. No golden vector changes, because bundles were never ledger records. A mailbox server, if ever built, would have to add a bundle mode.

### Invites are `betty://` links

Accepted 3 November 2026. Settles the open decision on joining.

Invites are `betty://` links sent through any chat app, not QR codes, because people expect to invite friends through the group chat. Betty no longer scans QR codes for anything, so the `qr` crate, the barcode-scanner plugin and the camera permission are gone. Invite links live in `sync`, and M8 is now the invite link.

**Alternatives considered.**

- Https links with universal and app links are reliably tappable and can open a store page when Betty is missing, but they need a domain and static hosting. They stay parked in [ideas.md](../ideas.md).
- An invite QR with a typed text code needs the joiner in front of the admin's phone. The text code couldn't be short either: the party id and admin key alone are about 103 base32 characters.

**Consequences.**

- The link is roughly 200 to 250 characters. Chat apps show all of it, because they can't hide a link behind a label, so the message text around it does the explaining.
- A chat app may not make `betty://` tappable, and the link does nothing if Betty isn't installed. That is why pasting must always work, why the message says Betty is needed, and why U1 tests the link in WhatsApp on both platforms.
- Another app could register the same scheme and receive the link, and a forwarded link reaches whoever gets it. Either way, the admin's approval with the fingerprint check still decides who joins.
- With the 12-hour expiry, the link has to be sent on the day of the party.
- No golden vector changes, because the invite commitment and proof stay the same.
