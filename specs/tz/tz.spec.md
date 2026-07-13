---
module: tz
version: 2
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Timezone Plugin

## Purpose

Provide a deterministic command-line interface for displaying, converting, saving, and removing timezone preferences across distributed teams.

## Public API

The `fledge-tz` binary exposes `add`, `rm`, `list`, `now`, and `convert` subcommands. With no subcommand it displays saved zones, or local time and UTC when no zones have been saved.

## Invariants

1. Common timezone abbreviations resolve to the documented IANA timezone while explicit valid IANA names pass through unchanged.
2. Saved timezone state is JSON under `FLEDGE_PLUGIN_DIR`, or the plugin's default Fledge data directory when that variable is absent.
3. Adding a saved zone is idempotent and removing an absent zone does not corrupt or rewrite unrelated state.
4. Invalid zones, invalid times, and ambiguous or nonexistent local times return a non-zero result with actionable context.

## Behavioral Examples

```
Given a developer runs `fledge tz convert "3pm PST" EST UTC`
When the plugin resolves and converts the supplied instant
Then it displays the same instant in America/New_York and UTC
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Unknown timezone | An alias or IANA timezone cannot be resolved | Exit non-zero and suggest accepted timezone forms |
| Invalid time | A time does not match a supported 12-hour, 24-hour, or dated form | Exit non-zero and show supported examples |
| Invalid local instant | A local time is ambiguous or nonexistent at a daylight-saving boundary | Exit non-zero instead of guessing |
| State I/O failure | The state directory or JSON state cannot be read or written | Exit non-zero with filesystem context |

## Dependencies

- Rust 1.89 or newer
- `chrono` and `chrono-tz` for timezone-aware conversion
- `clap` for command parsing
- `serde` and `serde_json` for persisted state

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing timezone display, conversion, persistence, and validation behavior. |
| 2 | 2026-07-13 | Reconciled existing timezone-resolution documentation and stable requirement IDs for SpecSync 5.0.1 governance; runtime behavior is unchanged. |
| 2026-07-13 | CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-timezone-fledge-plugin: Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the timezone Fledge plugin |
