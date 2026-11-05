# Sync traits (M6 to M8)

Preferred approach for M6 to M8. Follow unless implementation shows a better way; record any deviation here.

The session, the messages, discovery adverts and the invite link are decisions, in [design/sync.md](../design/sync.md). This file is only about the shape of the traits.

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| Every transport runs the same protocol over `Connection`. A new transport is a new `Transport` implementation, with no protocol change. | This is the seam that keeps other transports possible. | The in-memory and real transports run the same session code |
| `Discovery` stays separate from `Transport`. | Platforms find peers in different ways while the connection stays the same. | Review |
| `peer_transport_key` returns a key the transport has authenticated. | `Hello` binds that key to a member, so it can't just be what the peer claims. | Review |

## Sketch

An illustration, not a spec.

```rust
trait Transport: Send + Sync {
    async fn listen(&self, party: &PartyTransport) -> Result<Incoming>;
    async fn connect(&self, peer: &PeerAddr) -> Result<Box<dyn Connection>>;
}
trait Connection: Send {
    fn peer_transport_key(&self) -> PublicKey;   // authenticated by the transport
    async fn send(&mut self, msg: SyncMsg) -> Result<()>;
    async fn recv(&mut self) -> Result<SyncMsg>;
}
trait Discovery: Send + Sync {                   // separate from Transport on purpose
    async fn advertise(&self, ad: Advert) -> Result<()>;
    fn peers(&self) -> Stream<Item = PeerAddr>;
}
```

## What happened

Nothing yet: M6 has not started. Record each deviation here as it happens, with its reason, and the outcome when the milestone ends.
