# IdleScreen Applet

Optional COSMIC Desktop panel applet for [IdleScreen](https://github.com/idlescreen),
the Wayland-native idle screen and ambient display daemon for Linux.

| | |
|---|---|
| Core | [idlescreen/idlescreen](https://github.com/idlescreen/idle-core) |
| Packages | [idlescreen.github.io/idle-packages](https://idlescreen.github.io/idle-packages/) |
| Brand | [idlescreen/brand](https://github.com/idlescreen/idle-brand) |
| Org | [idlescreen](https://github.com/idlescreen) |

[![CI](https://github.com/idlescreen/idle-cosmic/actions/workflows/ci.yml/badge.svg)](https://github.com/idlescreen/idle-cosmic/actions/workflows/ci.yml)

Not required for GNOME, KDE, Hyprland, or other desktops. Use the CLI and TUI
from the core package there. The shipped package and binary name remain
`trance-applet` for packaging continuity.

## Install

After adding the IdleScreen package repository:

```bash
# Debian / Ubuntu / Pop!_OS (COSMIC)
sudo apt install trance trance-applet

# Fedora
sudo dnf install trance trance-applet
```

Index: [idlescreen.github.io/idle-packages](https://idlescreen.github.io/idle-packages/)

## Build from source

Requires a sibling checkout of the core daemon (path dependencies):

```bash
git clone https://github.com/idlescreen/idle-core.git
git clone https://github.com/idlescreen/idle-cosmic.git
cd idle-cosmic
cargo build --release
```

| Path dependency | Location |
|-----------------|----------|
| `trance-dbus` | `../idlescreen/crates/trance-dbus` |
| `trance-runner` | `../idlescreen/trance-runner` |

System dependencies (Debian/Ubuntu): `libdbus-1-dev libwayland-dev libxkbcommon-dev libssl-dev libegl1-mesa-dev libgl1-mesa-dev pkg-config`

## Releases

Tag `vX.Y.Z` on `master`. Ship `.deb` / `.rpm` through the IdleScreen packages pipeline when configured.

## License

Apache-2.0. See [LICENSE](LICENSE).
