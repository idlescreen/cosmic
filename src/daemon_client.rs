// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! D-Bus / systemd helpers for talking to `idle-daemon` from the panel applet.

use std::process::Command;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use idle_dbus::{DaemonStatus, TranceClient, daemon_available};

pub fn is_running() -> bool {
    daemon_available()
}

/// Start the user unit and enable it so it returns after login/upgrades.
///
/// Falls back to spawning `idle-daemon daemon` only if systemctl is unusable
/// (unusual on a COSMIC session). Legacy `trance-daemon` binary is tried last.
pub fn start_daemon_service() -> Result<()> {
    for unit in ["idle-daemon.service", "trance-daemon.service"] {
        let status = Command::new("systemctl")
            .args(["--user", "enable", "--now", unit])
            .status()
            .with_context(|| format!("systemctl --user enable --now {unit}"))?;

        if status.success() {
            wait_until_running(Duration::from_secs(3))?;
            return Ok(());
        }
        tracing::warn!(
            "systemctl enable --now {unit} failed (exit {:?})",
            status.code()
        );
    }

    tracing::warn!("systemctl enable --now failed; trying direct spawn");
    for bin in ["idle-daemon", "idlescreen-daemon", "trance-daemon"] {
        if Command::new(bin).arg("daemon").spawn().is_ok() {
            wait_until_running(Duration::from_secs(3))?;
            return Ok(());
        }
    }
    bail!("could not start idle-daemon via systemctl or direct spawn")
}

/// Stop the running user unit (does **not** disable — keeps login autostart).
pub fn stop_daemon_service() -> Result<()> {
    for unit in ["idle-daemon.service", "trance-daemon.service"] {
        let status = Command::new("systemctl")
            .args(["--user", "stop", unit])
            .status()
            .with_context(|| format!("systemctl --user stop {unit}"))?;

        if status.success() {
            return Ok(());
        }
    }

    // Fallback: SIGTERM via PID file if the unit is unmanaged.
    // Read with O_NOFOLLOW so a planted symlink cannot redirect us at a
    // different process, and refuse to send SIGTERM to a pid whose argv0
    // does not match idle-daemon (defense vs pidfile tampering).
    let runtime = std::env::var("XDG_RUNTIME_DIR").ok();
    for name in ["idle-daemon.pid", "trance-daemon.pid"] {
        let pid_path = if let Some(ref runtime_dir) = runtime {
            std::path::PathBuf::from(runtime_dir).join(name)
        } else {
            std::env::temp_dir().join(name)
        };
        let pid = crate::pidfile::read_pidfile_safely(&pid_path);
        if let Some(pid) = pid {
            if !crate::pidfile::pid_targets_idle_daemon(pid) {
                tracing::warn!("refusing to SIGTERM pid {pid} — not idle-daemon");
                continue;
            }
            // SAFETY: kill with SIGTERM on a process we verified is idle-daemon.
            unsafe {
                libc::kill(pid, libc::SIGTERM);
            }
            return Ok(());
        }
    }

    bail!("could not stop idle-daemon via systemctl or PID file")
}

#[tracing::instrument]
pub fn fetch_status() -> Result<DaemonStatus> {
    let client = TranceClient::connect().context("failed to connect to idle daemon")?;
    client.get_status().context("failed to fetch daemon status")
}

pub fn set_idle_enabled(enabled: bool) -> Result<()> {
    let client = TranceClient::connect().context("failed to connect to idle daemon")?;
    if enabled {
        client.enable().context("failed to enable idle activation")
    } else {
        client
            .disable()
            .context("failed to disable idle activation")
    }
}

pub fn set_timeout(minutes: u32) -> Result<()> {
    TranceClient::connect()
        .context("failed to connect to idle daemon")?
        .set_timeout(minutes)
        .context("failed to set idle timeout")
}

pub fn set_active_saver(name: Option<&str>) -> Result<()> {
    TranceClient::connect()
        .context("failed to connect to idle daemon")?
        .set_saver(name.unwrap_or(""))
        .context("failed to set active screensaver")
}

pub fn set_show_fps_overlay(enabled: bool) -> Result<()> {
    TranceClient::connect()
        .context("failed to connect to idle daemon")?
        .set_show_fps_overlay(enabled)
        .context("failed to set FPS overlay")
}

#[tracing::instrument]
pub fn list_savers() -> Result<Vec<String>> {
    TranceClient::connect()
        .context("failed to connect to idle daemon")?
        .list_savers()
        .context("failed to list installed screensavers")
}

pub fn set_render_scale(scale: f32) -> Result<()> {
    TranceClient::connect()
        .context("failed to connect to idle daemon")?
        .set_render_scale(scale)
        .context("failed to set render scale")
}

/// Preview a saver: prefer the daemon D-Bus path (layer-shell overlay).
///
/// If the daemon is down, try to start it first. As a last resort, run the
/// packaged `idle-daemon run-plugin <name>` fullscreen helper.
#[tracing::instrument]
pub fn preview_saver(name: &str) -> Result<()> {
    if !is_running() {
        tracing::info!("daemon offline; starting before preview");
        if let Err(e) = start_daemon_service() {
            tracing::warn!("could not start daemon for preview: {e:#}");
        }
    }

    if is_running() {
        match TranceClient::connect()
            .context("connect for preview")
            .and_then(|c| c.preview(name).context("D-Bus preview"))
        {
            Ok(()) => return Ok(()),
            Err(e) => tracing::warn!("D-Bus preview failed: {e:#}; trying run-plugin fallback"),
        }
    }

    for bin in ["idle-daemon", "idlescreen-daemon", "trance-daemon"] {
        if Command::new(bin).args(["run-plugin", name]).spawn().is_ok() {
            return Ok(());
        }
    }
    bail!("could not spawn idle-daemon run-plugin for preview")
}

fn wait_until_running(budget: Duration) -> Result<()> {
    let deadline = std::time::Instant::now() + budget;
    while std::time::Instant::now() < deadline {
        if is_running() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    if is_running() {
        Ok(())
    } else {
        bail!("idle-daemon did not become reachable on the session bus within {budget:?}")
    }
}

/// Launch the IdleScreen TUI in a terminal (requires a real TTY).
pub fn open_tui_dashboard() -> bool {
    let programs: &[&[&str]] = &[
        &["idle-tui"],
        &["idlescreen-tui"],
        &["idlescreen", "tui"],
        &["idle", "tui"],
    ];
    // cosmic-term: `-e` / `--` then program + args
    let terminals: &[(&str, &[&str])] = &[
        ("cosmic-term", &["-e"]),
        ("cosmic-term", &["--"]),
        ("kgx", &["-e"]),
        ("gnome-terminal", &["--"]),
        ("alacritty", &["-e"]),
        ("kitty", &[]),
        ("foot", &["-e"]),
        ("xterm", &["-e"]),
    ];

    for (term, term_flags) in terminals {
        for prog in programs {
            let mut cmd = Command::new(term);
            cmd.args(*term_flags);
            for p in *prog {
                cmd.arg(p);
            }
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                // SAFETY: start a new session so the terminal outlives the applet.
                unsafe {
                    cmd.pre_exec(|| {
                        libc::setsid();
                        Ok(())
                    });
                }
            }
            if cmd.spawn().is_ok() {
                return true;
            }
        }
    }

    for desktop_id in ["io.github.idlescreen.tui", "idlescreen"] {
        if Command::new("gtk-launch").arg(desktop_id).spawn().is_ok() {
            return true;
        }
    }
    false
}
