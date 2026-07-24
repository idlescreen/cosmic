# IdleScreen Applet

COSMIC **platform app** for [IdleScreen](https://github.com/idlescreen/idle-core). Ships the panel applet; a metapackage (via [packages](https://github.com/idlescreen/packages)) should pull `idle-core` (+ CLI), official `saver-*` effects, and this applet. Optional: [idle-tui](https://github.com/idlescreen/idle-tui).


Optional COSMIC Desktop panel applet for [IdleScreen](https://github.com/idlescreen),
the Wayland-native idle screen and ambient display daemon for Linux.

| | |
|---|---|
| Core | [idlescreen/idle-core](https://github.com/idlescreen/idle-core) |
| Packages | [idlescreen.github.io/packages](https://idlescreen.github.io/packages/) |
| Brand | [idlescreen/idle-brand](https://github.com/idlescreen/idle-brand) |
| Org | [idlescreen](https://github.com/idlescreen) |

[![CI](https://github.com/idlescreen/app-cosmic/actions/workflows/ci.yml/badge.svg)](https://github.com/idlescreen/app-cosmic/actions/workflows/ci.yml)

Not required for GNOME, KDE, Hyprland, or other desktops. Use the CLI and TUI
from the core package there. The shipped package and binary name remain
`idlescreen-applet` (legacy `trance-applet` still provided for upgrades).

## Install

After adding the IdleScreen package repository:

```bash
# Debian / Ubuntu / Pop!_OS (COSMIC)
sudo apt install idlescreen idlescreen-applet

# Fedora
sudo dnf install idlescreen idlescreen-applet
```

Index: [idlescreen.github.io/packages](https://idlescreen.github.io/packages/)

## Build from source

Requires a sibling checkout of the core daemon (path dependencies):

```bash
git clone https://github.com/idlescreen/idle-core.git
git clone https://github.com/idlescreen/app-cosmic.git
cd app-cosmic
cargo build --release
```

| Path dependency | Location |
|-----------------|----------|
| `trance-dbus` | `../idle-core/crates/trance-dbus` |
| `trance-runner` | `../idle-core/trance-runner` |

System dependencies (Debian/Ubuntu): `libdbus-1-dev libwayland-dev libxkbcommon-dev libssl-dev libegl1-mesa-dev libgl1-mesa-dev pkg-config`

## Releases

Tag `vX.Y.Z` on `master`. Ship `.deb` / `.rpm` through the IdleScreen packages pipeline when configured.

## License

Apache-2.0. See [LICENSE](LICENSE).

## Product metapackage

`idlescreen-cosmic` depends on `idlescreen`, `idlescreen-applet`, and `idlescreen-savers`.
