# Testing strategy

Correctness of balances and convergence of every phone are the two things that must never fail, so most of the test effort goes there, in Phases 1 to 3, before any UI exists.

| Layer | Technique | What it proves |
| --- | --- | --- |
| Encoding | Golden vectors run on Android, iOS and desktop in CI; `cargo-fuzz` on every decoder | Identical bytes and hashes on every platform; malformed input never panics |
| Ledger rules | Unit tests per entry kind; property tests with `proptest` | Balances sum to zero; replay is deterministic; a correction fully reverses; no stake without its owner's signature counts |
| Scenarios | One test per diagram sequence, written as scripts for `cli` | Join, stake, results, Everyone agrees count (from M9), leave and depart, handover and takeover, settlement, all behave as designed |
| Settlement | Exact solver checked against brute force for up to 10 members; payment payloads checked against BIP-21, EIP-681 and Solana Pay examples (from M10) | Fewest transfers really is fewest; payment apps accept the requests |
| Convergence | `sim`: random parties of 2 to 16 phones with message loss, delay, partitions, app suspension and admin loss | All honest phones reach the same head and balances; equivocation is always detected |
| Devices | Manual test matrix in M7, U1 and U5: two Android brands, two iPhone generations, home Wi-Fi, guest Wi-Fi with client isolation, phone hotspot | Discovery and sync work on home Wi-Fi and a phone hotspot; where client isolation blocks them, the app says so and suggests the hotspot |
| UI | Component tests in the web UI; end-to-end on the desktop build of the Tauri app via WebDriver | Screens send the right commands and render view structs correctly |
