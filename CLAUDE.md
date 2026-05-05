# fledge-plugin-tz

Timezone utility plugin for [fledge](https://github.com/CorvidLabs/fledge). Show current time across zones, convert times between zones, and manage a saved list of preferred zones for distributed teams.

## Build & Test

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Architecture

- `src/main.rs` — Single-file CLI (clap derive). Subcommands: `add`, `rm`, `list`, `now`, `convert`.
- `plugin.toml` — Fledge plugin manifest (declares the `tz` command and binary path).
- State is stored in `state.json` under `$FLEDGE_PLUGIN_DIR` (or `~/.fledge/plugins/fledge-plugin-tz/`).

## Design notes

- Common timezone abbreviations (PST, EST, JST, etc.) are mapped to IANA zone names via `resolve_zone()`.
- Arbitrary IANA names (e.g. `Africa/Cairo`) are validated by parsing with `chrono-tz`.
- `Local` is a special keyword that resolves to the system local timezone.
