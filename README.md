# idle-cosmic

IdleScreen **product package** for [COSMIC Desktop](https://system76.com/cosmic).

```bash
sudo dnf install idle-cosmic
# or
sudo apt install idle-cosmic
```

## What it installs

```text
idle-cosmic
├── COSMIC panel applet
├── Requires: idle            # daemon (idle-core)
└── Requires: idle-savers     # every idle-saver-*
```

Optional recommends: `idle-cli` (command **`idle`**), `idle-tui`.

| | |
|---|---|
| Core | [idle-core](https://github.com/idlescreen/idle-core) |
| Packages | [idlescreen.github.io/packages](https://idlescreen.github.io/packages/) |

```bash
systemctl --user enable --now idle-daemon
idle status
```

## License

Apache-2.0.
