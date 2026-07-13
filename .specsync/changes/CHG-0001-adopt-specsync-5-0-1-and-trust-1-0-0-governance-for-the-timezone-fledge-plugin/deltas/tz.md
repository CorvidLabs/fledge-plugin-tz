## MODIFIED
### SPEC SECTION Change Log
| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing timezone display, conversion, persistence, and validation behavior. |
| 2 | 2026-07-13 | Reconciled existing timezone-resolution documentation and stable requirement IDs for SpecSync 5.0.1 governance; runtime behavior is unchanged. |

### REQUIREMENT REQ-tz-001
The plugin SHALL display current time for saved, explicitly supplied, local, UTC, alias, and valid IANA zones.

Acceptance Criteria
- Existing unit tests verify documented PST and JST aliases resolve and explicit valid IANA zone names pass through unchanged.

