# Design

These files hold Betty's decisions, and everything in them is binding. They describe Betty's beta, through Gate 5; 1.0 isn't defined yet (see "Decisions" in [phases.md](phases.md)). Change them only deliberately, as "Changing a decision" below describes. The preferred way to build what they require is in [guidance/](../guidance/), and what the current design doesn't commit to is parked in [ideas.md](../ideas.md). Decisions, guidance and ideas are kept apart because, when one file held all three, a sketch could be read as binding and a consensus rule as a suggestion.

## Files

Read only the file a task needs. `.claude/CLAUDE.md` maps kinds of code and directories to these files.

| File | Holds |
| --- | --- |
| [principles.md](principles.md) | The rules every choice follows, platform constraints, the whole picture and the scope of the beta |
| [architecture.md](architecture.md) | Crates, their layers and dependencies, the `ledger` modules, the engine |
| [identity-and-party.md](identity-and-party.md) | Keys, the party, members, joining |
| [rules.md](rules.md) | Rule kinds, rule changes and who approves them, stakes and the rules that check them |
| [activities.md](activities.md) | Activity kinds and parameters, limits, the rules each kind allows, running totals, presets |
| [ledger-format.md](ledger-format.md) | Objects, encoding, hashing, signing, ordering, heads, the ordering and state layers |
| [records.md](records.md) | The catalogue of member actions, entries and detached objects |
| [validation.md](validation.md) | Validation stages, modes, results, dropouts, quorum, handover, takeover, time |
| [settlement.md](settlement.md) | Balance rules, rounding, the solver, the statement, finality |
| [sync.md](sync.md) | The sync session, transports, discovery, invite links |
| [storage.md](storage.md) | The database, encryption at rest, key custody |
| [phases.md](phases.md) | Phases 1 to 5: milestones M0 to M10, acceptance checks, gates |
| [app.md](app.md) | The Tauri 2 app: milestones U0 to U5 (Phase 4), and U6 and U7 (Phase 5) |
| [testing.md](testing.md) | The testing strategy |
| [open-decisions.md](open-decisions.md) | Questions not settled yet |

## Sorting new content

- Could changing it make two phones disagree (bytes, hashes, validity, balances), or break a promise to users? It is a decision. When unsure, it stays a decision.
- Would a different build give the same bytes and the same replay? It is guidance.
- Does the current design not build it at all? It is an idea. A seam the design keeps for it, such as a reserved enum variant or a trait, is still a decision.

## Keeping the files and the map current

- A new section gets its own file and a row in the table above and in the map in `.claude/CLAUDE.md`. A file that grows too large for one read gets split the same way, so a task only has to load its own section.
- A new crate or guidance file updates the map in the same change.
- The map loads in every session, so it stays short and leaves the content in the files it points at. It says what to read, not what to build, so scope stays with the current milestone.
- The map prompts a read but can't force one. Anything that must never go wrong still needs a check that fails, such as a golden vector, the compiler or a test.

## Changing a decision

1. Ask first, unless the change is already agreed.
2. Update the design file in the same change, so it stays the current statement of what is decided.
3. If the choice is architecturally significant, such as structure, dependencies or interfaces, add an entry to the "Decisions" section at the end of that file. Keep it short: its date, the context if it isn't obvious, the decision, and the alternatives or consequences worth remembering. Otherwise, put the reason in the commit message.
4. When a recorded decision changes, update its entry, date the change, and keep the replaced choice as an alternative, so the reasoning isn't lost.

The questions still open are in [open-decisions.md](open-decisions.md). When one is settled, it moves into the design the same way and its row is removed.
