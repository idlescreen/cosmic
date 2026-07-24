# idle-cosmic

**This is what COSMIC users install.**

```bash
sudo dnf install idle-cosmic
systemctl --user enable --now idle-daemon
idle status
```

Depends on engine packages `idle-daemon` + `idle-savers` (all `idle-saver-*`).  
Do not install the engine alone for a desktop product.

Source: [idlescreen/idle](https://github.com/idlescreen/idle).

## License

Apache-2.0.
