# Packaging

**Product package name:** `app-cosmic`

Built from the crate root via `cargo deb` / `cargo generate-rpm` using
`[package.metadata.deb]` / `[package.metadata.generate-rpm]` in `Cargo.toml`.

```bash
# with idle-core checked out as ../idle-core
cargo build --release
cargo deb
cargo generate-rpm
```

The package:

- Installs the COSMIC applet binaries
- **Requires** `idlescreen` (idle-core daemon)
- **Requires** `idlescreen-savers` (all official `saver-*` plugins)

Mirrored product notes live under
[idlescreen/packages/metapackages/app-cosmic](https://github.com/idlescreen/packages/tree/master/metapackages/app-cosmic).
