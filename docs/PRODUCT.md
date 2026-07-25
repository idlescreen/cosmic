# idle-cosmic product

Platform: **COSMIC Desktop**

## One-command install

Prefer the OS/DE-aware installer (pulls modular packages + applet on COSMIC):

```bash
curl -fsSL https://idlescreen.github.io/packages/install.sh | sh
```

Or install the applet package alone after the engine is present:

```bash
sudo dnf install idle-cosmic   # Fedora
sudo apt install idle-cosmic   # Debian/Pop
```

## Package: `idle-cosmic`

Shipped package name is **`idle-cosmic`** (crate may still be named `idle-applet` internally).

| Role | Package |
|------|---------|
| Ships | COSMIC panel applet (`idlescreen-applet` binary) |
| Requires | `idle-daemon` |
| Soft | `idle-savers`, `idle-cli`, `idle-tui` (via install script / recommends) |

**Provides** (transitional upgrades): `app-cosmic`, `idlescreen-applet`, `trance-applet`, `idlescreen-cosmic`.

## Out of scope

Daemon, CLI, and saver plugins live in [idle](https://github.com/idlescreen/idle) and `idle-saver-*` repos. This package is the COSMIC panel surface only.

## Config

Shared with the daemon: prefer `~/.config/idle/config.yaml` (legacy `~/.config/trance/` still read).
