# Open decisions

These are not settled yet. The design proceeds on the recommended option for each, and each can be settled during the phase where it appears. When one is settled, move it into the design, as [README.md](README.md) describes, and remove its row. The licence, signature scheme and party size cap were settled on 26 October 2026: see "Decisions" in [architecture.md](architecture.md) and [identity-and-party.md](identity-and-party.md).

| # | Decision | Recommended | Alternative | Needed by |
| --- | --- | --- | --- | --- |
| 1 | Places model (tournaments) in the beta | Include (places and payout percentages) | Ship ChipCount, Pool and HeadToHead first, Places later | M2 |
| 2 | No-phone members | Include as `Managed`, every action flagged "admin-entered" | Leave them out | M2 |
| 3 | Default settings | Majority quorum, 48 h dispute window, 10 min takeover hint | Any other values | M3 |
| 4 | Equivocation handling | Freeze that epoch past the fork; continue only by co-admin takeover | Follow the first-seen branch | M3 |
| 5 | Crypto payment requests | Offer a crypto URI only when the party unit is that asset; no exchange rates in the app | Let the group enter a fixed rate at settle-up | M10 |
| 6 | Same-Wi-Fi transport | Decide after the M7 spike: iroh with relays off, or quinn | Either; both sit behind `Transport` | M7 |
| 8 | Web UI framework | Svelte 5 with Vite and TypeScript | React or Solid | U0 |
| 9 | Lost or replaced phone | Witnessed `Rekey` only, no key export | Encrypted key export or cloud backup | U2 |
| 10 | Minimum OS versions | Tauri 2's minimums, raised if the Bonjour or NSD plugin needs it | A higher floor | U0 |
| 11 | Cross-party groups and stats | Local history list only | `Group` objects with shared stats | U5 |
| 12 | Closing without everyone's acceptance in Everyone agrees | Majority close after 15 min of silence or 2 disputed revisions, with the settlement using the same timeout | Other values, or another way to handle a silent player, including in a two-person party, which can't reach a majority close | M9 |
