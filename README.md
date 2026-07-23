# idlescreen-applet

Optional **COSMIC Desktop** panel applet for [IdleScreen](https://github.com/idlescreen/idlescreen)
(the Wayland idle/screensaver daemon, package name `trance`).

Not required for GNOME, KDE, Hyprland, or other desktops — use `trance-tui` / `trance-cli` there.

## Build

```bash
git clone https://github.com/idlescreen/idlescreen.git
git clone https://github.com/idlescreen/idlescreen-applet.git
cd idlescreen-applet
cargo build --release
```

Sibling path deps: `../idlescreen/crates/trance-dbus` and `../idlescreen/trance-runner`.

## Install

```bash
sudo apt install trance-applet   # or dnf, after IdleScreen package repo
```

## License

Apache-2.0.
