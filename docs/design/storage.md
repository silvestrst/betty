# Storage and key custody

Each phone keeps one SQLite database of verified objects and one identity key in the platform keychain; derived state is a cache that can always be rebuilt by replay.

**Database.** Its tables and the `Store` trait are in [guidance/storage.md](../guidance/storage.md). Writes are append-only; the only updates are to caches and hints.

**Encryption at rest.** Betty relies on the OS: iOS Data Protection (complete until first unlock) and Android's app sandbox with file-based encryption.

**Key custody** (`KeyStore` trait; its preferred shape is in [guidance/storage.md](../guidance/storage.md))

- iOS: the Ed25519 seed is a Keychain item, `ThisDeviceOnly`, available after first unlock, excluded from backups.
- Android: the seed is encrypted with an AES key that lives in Android Keystore (StrongBox when available), and only the ciphertext is stored.
- Desktop and tests: a file or in-memory key.

**Lost phone or new phone.** There is no cloud backup of the key. The member's new device creates a new key and a `Rekey` action; the admin includes it with one witness, and the seat and balance move to the new key. Party history comes back by syncing with anyone.
