# idle-cosmic

**This is what COSMIC users install.**

```bash
sudo dnf install idle-cosmic
systemctl --user enable --now idle-daemon
idle status
```

Depends on engine package `idle-daemon`. Recommends official screensaver plugins (`idle-saver-*`) as weak dependencies.  
Do not install the engine alone for a desktop product.

Source: [idlescreen/idle](https://github.com/idlescreen/idle).

## License

Apache-2.0.
