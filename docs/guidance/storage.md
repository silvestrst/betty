# Store and key custody (M5)

Preferred approach for M5. Follow unless implementation shows a better way; record any deviation here.

Append-only writes and where keys live are decisions, in [design/storage.md](../design/storage.md). This file is only about the shape of the code.

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| `KeyStore` signs with `sign(bytes)` and never hands out the seed. | A later hardware-backed P-256 key fits the same trait. | Compiler: no method on the trait returns the seed |
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

**`KeyStore`.** The `KeyStore` exposes `sign(bytes)` rather than handing out the seed, so a later hardware-backed P-256 key fits the same trait.

## What happened

Nothing yet: M5 has not started. Record each deviation here as it happens, with its reason, and the outcome when the milestone ends.
