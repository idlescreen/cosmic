# app-cosmic product

Platform: **COSMIC Desktop**

## One-command install

```bash
sudo dnf install app-cosmic   # Fedora
sudo apt install app-cosmic   # Debian/Pop
```

## Package: `app-cosmic`

This repo’s **shipped package name is `app-cosmic`**.

| Pulls | Package | Role |
|-------|---------|------|
| Ships | (this RPM/DEB) | COSMIC panel applet binaries |
| Requires | `idlescreen` | idle-core daemon + systemd user unit |
| Requires | `idlescreen-savers` | **all** official `saver-*` plugins |
| Recommends | `idlescreen-cli` | `idlescreen` CLI |
| Recommends | `app-tui` | live TUI |

**Provides / Obsoletes** (transitional): `idlescreen-applet`, `trance-applet`, `idlescreen-cosmic`.

## Out of scope

Engines and plugin content stay in [idle-core](https://github.com/idlescreen/idle) and [saver-\*](https://github.com/orgs/idlescreen/repositories?q=saver-). This repo is the COSMIC product surface only.
