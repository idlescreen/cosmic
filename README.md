# app-cosmic — IdleScreen for COSMIC

COSMIC **product package** for [IdleScreen](https://github.com/idlescreen/idle-core).

| | |
|---|---|
| Core (daemon) | [idlescreen/idle-core](https://github.com/idlescreen/idle-core) → package **`idlescreen`** |
| Packages host | [idlescreen.github.io/packages](https://idlescreen.github.io/packages/) |
| Brand | [idlescreen/idle-brand](https://github.com/idlescreen/idle-brand) |
| Org | [idlescreen](https://github.com/idlescreen) |

[![CI](https://github.com/idlescreen/app-cosmic/actions/workflows/ci.yml/badge.svg)](https://github.com/idlescreen/app-cosmic/actions/workflows/ci.yml)

## What `app-cosmic` installs

```text
app-cosmic
├── this package          → COSMIC panel applet (idlescreen-applet binary)
├── Requires: idlescreen  → idle-core daemon + user service
└── Requires: idlescreen-savers
    └── every official saver-* plugin
```

Optional (recommends): `idlescreen-cli`, `idlescreen-tui`.

Not needed on GNOME/KDE/Hyprland — use `idlescreen` + `idlescreen-cli` there.

## Install

After adding the IdleScreen package repository:

```bash
# Fedora
sudo curl -fsSL https://idlescreen.github.io/packages/rpm/crateria.repo \
  -o /etc/yum.repos.d/idlescreen.repo
sudo dnf install app-cosmic

# Debian / Ubuntu / Pop!_OS (COSMIC)
sudo apt install app-cosmic
```

Then:

```bash
systemctl --user enable --now idlescreen-daemon
idlescreen status   # if idlescreen-cli was pulled in
```

Add the **IdleScreen** applet in COSMIC panel settings if it does not appear automatically.

Index: [idlescreen.github.io/packages](https://idlescreen.github.io/packages/)

## Build from source

Requires a sibling checkout of idle-core (path dependencies):

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

## License

Apache-2.0. See [LICENSE](LICENSE).
