# The Tauri 2 app (U0 to U7)

Preferred approach for U0 to U7. Follow unless implementation shows a better way; record any deviation here.

That the UI holds no business logic, the milestones and the copy rules for describing Betty are decisions, in [design/app.md](../design/app.md). The web UI framework is still open decision 8 in [design/open-decisions.md](../design/open-decisions.md).

## What must hold

| Property | Why | How to check |
| --- | --- | --- |
| Tauri commands map one-to-one to engine commands and queries, and engine events are forwarded to the web view. | The Rust layer stays thin, so the logic stays in `engine`. | Review |
| TypeScript bindings are generated from the Rust DTOs, never written by hand. | The UI cannot drift from the core. | Review: no hand-written DTO types in the web UI |
| The UI renders rules generically, with one editor per config type (amount, count, player list). | A new rule kind needs at most one new editor. | Review of the diff that adds a rule kind |

## Sketch

An illustration, not a spec.

- `apps/betty/src-tauri`: a thin Rust layer that owns one `engine`, registers Tauri commands that map one-to-one to engine commands and queries, and forwards engine events to the web view.
- TypeScript bindings are generated from the Rust DTOs (`specta` with `tauri-specta`), so the UI cannot drift from the core.
- Web UI: a single-page app built with Vite and TypeScript, no server rendering. Framework recommendation: Svelte 5 (small bundles, simple reactivity); React or Solid work equally well with this design.

## What happened

Nothing yet: U0 has not started. Record each deviation here as it happens, with its reason, and the outcome when the milestone ends.
