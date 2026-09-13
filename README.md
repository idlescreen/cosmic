# cosmic

COSMIC panel applet for the IdleScreen daemon — applet state, quick
actions, daemon handshake. Part of
[IdleScreen](https://idlescreen.github.io) — modular Wayland screensavers
for Linux.

The installed binary is **`idlescreen-applet`** (package `idle-cosmic`).

## Use

On COSMIC Desktop the applet is seated by the installer; add it from
COSMIC Settings → Panel → Applets. Status also surfaces via
`idlescreen cosmic`.

## Develop

Path dependency: a `runtime/` checkout inside this repo (or a symlink to a
sibling clone) provides `idle-dbus`. libcosmic deps are pinned git revs.

```sh
sudo dnf install libdbus-1-devel wayland-devel libxkbcommon-devel \
    fontconfig-devel freetype-devel openssl-devel libudev-devel \
    pkgconf-pkg-config                                       # apt: -dev names
git clone https://github.com/idlescreen/cosmic.git && cd cosmic
git clone https://github.com/idlescreen/runtime runtime    # path dep
cargo build --workspace && cargo test --workspace
```

## License

Apache-2.0 · © 2026 IdleScreen
