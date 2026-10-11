# cosmic [ARCHIVED / CONSOLIDATED]

> [!WARNING]
> **Repository Archived & Deprecated**
>
> This standalone repository is archived and deprecated. COSMIC Desktop integration has been consolidated directly into the main [IdleScreen mono-repo](https://github.com/idlescreen/idlescreen).
>
> - **Autosetup on Install**: The installer (`packages/install.sh`) automatically detects COSMIC Desktop (`DE_ID=cosmic`) and provisions all required integration without manual repository configuration:
>   ```sh
>   curl -fsSL https://idlescreen.github.io/install.sh | sh
>   ```
> - **Zero Friction Configuration**: Use the unified terminal UI to preview, configure, and manage screensavers, idle timeouts, and display behaviors:
>   ```sh
>   idlescreen tui
>   ```
> - **CLI Controls**:
>   ```sh
>   idlescreen preview ascii    # Interactive terminal preview with tri-rotation
>   idlescreen set ascii        # Set default screensaver
>   idlescreen status           # Check daemon & display state
>   ```

<div align="center">

| Security Pillar | Verification Badge |
| :--- | :---: |
| **Platform Standard** | [![secured by studio2201][b-studio]][u-home] |
| **Credential Defense** | [![snip][b-snip]][u-snip] |
| **Supply Chain Surface** | [![vigil][b-vigil]][u-vigil] |
| **Post-Quantum Cryptography** | [![aegis][b-aegis]][u-aegis] |
| **Build Provenance & SLSA** | [![proven][b-proven]][u-proven] |
| **Repository Governance** | [![boneyard][b-boneyard]][u-boneyard] |

[b-studio]: https://img.shields.io/badge/secured%20by-studio2201-2f6f5e?logo=shield
[u-home]: https://studio2201.com
[b-snip]: https://img.shields.io/badge/snip-0%20secrets-2f6f5e?logo=shield
[u-snip]: https://studio2201.com/snip
[b-vigil]: https://img.shields.io/badge/vigil-0%20dependencies-2f6f5e?logo=shield
[u-vigil]: https://studio2201.com/vigil
[b-aegis]: https://img.shields.io/badge/aegis-PQC%20compliant-2f6f5e?logo=shield
[u-aegis]: https://studio2201.com/aegis
[b-proven]: https://img.shields.io/badge/proven-ML--DSA--65%20verified-2f6f5e?logo=shield
[u-proven]: https://studio2201.com/proven
[b-boneyard]: https://img.shields.io/badge/boneyard-maintained-2f6f5e?logo=shield
[u-boneyard]: https://studio2201.com/boneyard

</div>

COSMIC panel applet for the IdleScreen daemon — applet state, quick
actions, daemon handshake. Part of
[IdleScreen](https://idlescreen.github.io) — universal Wayland screensavers
for Linux.

## Install

On COSMIC Desktop, the web installer detects your desktop environment and configures IdleScreen automatically:

```sh
curl -fsSL https://idlescreen.github.io/install.sh | sh
```

Or install the package directly:
```sh
sudo dnf install idle-cosmic       # Fedora COSMIC / Pop!_OS
```

Configure any screensaver or timeout at any time via:
```sh
idlescreen tui
```

## License

Apache-2.0 · © 2026 IdleScreen
