# app-cosmic product notes

Platform: **COSMIC Desktop**

## Metapackage target (when published)

`idlescreen-cosmic` (name TBD) should Depend on:

- core daemon + CLI (`trance` / idle-core packages)
- this applet (`trance-applet`)
- official savers (`trance-plugins-all` or each `trance-plugin-*`)

Recommends: `trance-tui` from [idle-tui](https://github.com/idlescreen/idle-tui).

Engines stay in idle-core / saver-*; this repo is UI + packaging recipe only.
