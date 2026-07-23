<h1 align="center">
  IdleScreen Applet
</h1>

<p align="center">
  <b>Optional COSMIC Desktop panel applet for IdleScreen (Wayland idle / ambient screensaver daemon).</b>
</p>

<p align="center">
  Part of <a href="https://github.com/idlescreen">IdleScreen</a>
  · Core: <a href="https://github.com/idlescreen/idlescreen">idlescreen/idlescreen</a>
  · Packages: <a href="https://idlescreen.github.io/packages/">idlescreen.github.io/packages</a>
  · Brand: <a href="https://github.com/idlescreen/brand">idlescreen/brand</a>
</p>

<p align="center">
  <a href="https://github.com/idlescreen/idlescreen-applet/actions/workflows/ci.yml"><img src="https://github.com/idlescreen/idlescreen-applet/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/idlescreen/idlescreen-applet/security/advisories"><img src="https://img.shields.io/badge/security-private%20reporting-blue" alt="Security"></a>
</p>

---

Not required for GNOME, KDE, Hyprland, or other desktops — use `trance-tui` / `trance-cli` from the core package there. The applet package name remains **`trance-applet`** (binary `trance-applet`) for packaging continuity.

### Install (native packages)

**Debian / Ubuntu / Pop!_OS (COSMIC):**

```bash
# After adding the IdleScreen apt source (see packages index)
sudo apt install trance trance-applet
```

**Fedora:**

```bash
sudo dnf install trance trance-applet
```

Package index: [idlescreen.github.io/packages](https://idlescreen.github.io/packages/)

---

### Build from source

Requires a sibling checkout of the core daemon (path deps):

```bash
git clone https://github.com/idlescreen/idlescreen.git
git clone https://github.com/idlescreen/idlescreen-applet.git
cd idlescreen-applet
cargo build --release
```

| Path dependency | Location |
|-----------------|----------|
| `trance-dbus` | `../idlescreen/crates/trance-dbus` |
| `trance-runner` | `../idlescreen/trance-runner` |

System deps (Debian/Ubuntu): `libdbus-1-dev libwayland-dev libxkbcommon-dev libssl-dev libegl1-mesa-dev libgl1-mesa-dev pkg-config`

---

### Releases

Tag `vX.Y.Z` on `master`. Ship `.deb` / `.rpm` via the IdleScreen packages pipeline when configured.

---

### License

Apache-2.0. See [LICENSE](LICENSE).
