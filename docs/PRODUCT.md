# app-cosmic product notes

Platform: **COSMIC Desktop**

## Packages (v2)

| Package | Role |
|---------|------|
| `idlescreen-applet` | This applet (Obsoletes `trance-applet`) |
| `idlescreen-cosmic` | Product meta |

`idlescreen-cosmic` Depends on:

- `idlescreen` (daemon from idle-core)
- `idlescreen-applet` (this repo)
- `idlescreen-savers` (all official `saver-*`)

Recommends: `idlescreen-cli`, `idlescreen-tui`.

Engines stay in idle-core / saver-*; this repo is UI + packaging recipe only.
