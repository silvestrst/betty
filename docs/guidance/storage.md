# Store and key custody (M5)

Preferred approach for M5. Follow unless implementation shows a better way; record any deviation here.

Append-only writes and where keys live are decisions, in [design/storage.md](../design/storage.md). This file is only about the shape of the code.

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| `KeyStore` signs with `sign(bytes)` and never hands out the seed. | A later hardware-backed P-256 key fits the same trait. | Compiler: no method on the trait returns the seed |
| Each `KeyStore` backend generates its own seed. The in-memory store is the one exception: it takes a seed, so tests and the simulator can be replayed. | The seed never passes through `engine` or the UI. The mobile plugins can use the platform's random source, and a hardware key is generated inside the hardware anyway. | Review: only the in-memory store's constructor takes a seed |
| A seed or key in memory is wiped as soon as the operation that needs it ends. | A seed left in memory can be read by anything that gets into the app process, or end up in a crash dump. | Review: types that hold a seed or key wipe themselves when dropped and aren't `Copy` or `Debug` |
| `Store` has an in-memory and a SQLite implementation behind one trait. | The simulator and tests use the in-memory one. | The same store tests run against both |
| `state_cache` is only a cache. If it is missing or stale, replay rebuilds it. | Derived state must never be something you can't get back from the log. | M5's check: the state cache rebuilds from scratch identically |
| `peers` holds local hints only, which never enter the log. | Last-seen times and addresses differ per phone, so they can't affect replay. | Review |

## Sketch

An illustration, not a spec.

**Database** (`store`, `rusqlite` with bundled SQLite)

| Table | Contents |
| --- | --- |
| `parties` | party id, name, my role, archived flag, format |
| `entries` | party, epoch, seq, id, encoded bytes; unique on (party, epoch, seq) |
| `objects` | member actions, attestations, marks by id and type, encoded bytes, first-seen time |
| `evidence` | equivocation pairs and quarantined objects with the reason |
| `state_cache` | last replayed head and serialized party state, rebuilt if missing or stale |
| `peers` | transport keys, last addresses, last seen (local hints only) |
| `outbox` | my member actions and attestations not yet receipted |

The `Store` trait also has an in-memory implementation used by the simulator and tests.

**`KeyStore`.** The `KeyStore` exposes `sign(bytes)` rather than handing out the seed, so a later hardware-backed P-256 key fits the same trait. For Ed25519 the seed is the private key, so its life inside a backend has two steps:

- **Once, when the identity is created:** the backend generates the seed and stores it protected. The file store (desktop and CLI) has no platform plugin, so it asks the OS for random bytes; the iOS and Android plugins use the platform's own call.
- **On every `sign`:** the backend decrypts or reads the seed, derives the key from it, signs, and wipes both before returning.

**Keeping exposure short.** FIPS 140-3 calls this zeroisation, and OWASP's mobile testing guide checks app memory for leftover keys.

- In Rust, hold the seed in `zeroize::Zeroizing<[u8; 32]>`. `zeroize` is already in the tree through `ed25519-dalek`'s `zeroize` feature, which wipes its own key when dropped. The wipe is written so the compiler can't remove it.
- Moving or copying a value can leave old bytes on the stack, so pass seeds by reference. Don't derive `Copy`, `Clone` or `Debug` on a type that holds one, and don't keep seeds in a `Vec` that may reallocate.
- On the platform side, overwrite the buffer as soon as the bytes reach Rust (`ByteArray.fill(0)` in Kotlin, `resetBytes` in Swift). A garbage-collected runtime may already have copied them, so the plaintext should spend as little time there as possible.
- For comparison, libsignal doesn't wipe its own Curve25519 identity keys: its `PrivateKey` is `Copy`, which Rust doesn't allow to wipe itself on drop. Betty goes further because it can at little cost.

Until M5, `ledger::primitives::SigningKey` stands in for tests and development, and the in-memory store can wrap it. Its `from_seed` borrows the seed as `&[u8; 32]`, as above, so a store that wraps it can hold the seed in `Zeroizing` and pass it without making a copy.

## What happened

Nothing yet: M5 has not started. Record each deviation here as it happens, with its reason, and the outcome when the milestone ends.
