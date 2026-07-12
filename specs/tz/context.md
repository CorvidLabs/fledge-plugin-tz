---
spec: tz.spec.md
---

## Context

Distributed teams need a compact offline utility for comparing local times without relying on a network service. The plugin stores a small user-selected zone list and performs conversions against the bundled timezone database.

## Related Modules

- The Fledge plugin manifest exposes the `tz` command implemented by this module.

## Design Decisions

- Persist canonical IANA names so aliases remain stable across later invocations.
- Reject ambiguous daylight-saving instants rather than silently choosing one.
- Keep all normal display and conversion behavior offline.
