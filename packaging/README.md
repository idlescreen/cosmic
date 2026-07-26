# Packaging

**Product package name:** `idle-cosmic`

Built from the crate root via `cargo deb` / `cargo generate-rpm` using
`[package.metadata.deb]` / `[package.metadata.generate-rpm]` in `Cargo.toml`.

```bash
# Local path dep: symlink or checkout idlescreen/idle as ./idle
ln -sfn ../idle idle   # from a sibling workspace checkout
cargo build --release
cargo deb
cargo generate-rpm
```

The package:

- Installs the COSMIC applet binary (`idlescreen-applet`; desktop `Exec=` uses this name)
- **Requires** `idle-daemon` and `idle-tui`
- **Recommends** savers / CLI as configured in metadata
- **Provides** transitional package names: `app-cosmic`, `idlescreen-applet`, `trance-applet`, `idlescreen-cosmic`

Prefer the org installer for a full stack:

```bash
curl -fsSL https://idlescreen.github.io/packages/install.sh | sh
```

See [docs/PRODUCT.md](../docs/PRODUCT.md) and packages [docs/MIGRATION.md](https://github.com/idlescreen/packages/blob/master/docs/MIGRATION.md).
