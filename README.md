# fledge-plugin-tz

Timezone utility for [fledge](https://github.com/CorvidLabs/fledge) — show, convert, and manage saved zones across distributed teams.

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-tz
```

## Usage

```bash
fledge tz                      # show current time in saved zones
fledge tz add PST              # save a zone (alias or IANA name)
fledge tz add Asia/Tokyo
fledge tz rm PST
fledge tz list
fledge tz now UTC EST JST      # one-off display, ignores saved list
fledge tz convert "3pm PST" EST UTC
fledge tz convert "2026-04-25 09:30 UTC" Local PST
```

### Recognised shortcuts

`UTC`, `GMT`, `Local`, `PST`/`PDT`/`PT`, `MST`/`MDT`/`MT`, `CST`/`CDT`/`CT`, `EST`/`EDT`/`ET`, `AKST`/`AKDT`, `HST`, `BST`, `CET`/`CEST`, `EET`/`EEST`, `IST`, `JST`, `KST`, `AEST`/`AEDT`, `NZST`/`NZDT`.

Anything else is treated as an IANA zone name (e.g. `Africa/Cairo`, `America/Sao_Paulo`).

## License

MIT
