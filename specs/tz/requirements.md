---
spec: tz.spec.md
---

## Requirements

### REQ-tz-001

The plugin SHALL display current time for saved, explicitly supplied, local, UTC, alias, and valid IANA zones.

Acceptance Criteria
- Existing unit tests verify documented PST and JST aliases resolve and explicit valid IANA zone names pass through unchanged.

### REQ-tz-002

The plugin SHALL convert supported time forms to one or more target zones without changing the represented instant.

### REQ-tz-003

The plugin SHALL add and remove canonical saved zones idempotently in its JSON state file.

### REQ-tz-004

The plugin SHALL reject invalid zones, invalid time forms, and ambiguous or nonexistent local instants with a non-zero result and actionable error.

## Constraints

- Normal operation must not require network access.
- State must remain isolated under the configured Fledge plugin directory.

## Out of Scope

- Downloading timezone database updates.
- Scheduling meetings or modifying calendar data.
