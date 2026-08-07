// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Defensive pidfile reader for the applet stop path.
//!
//! The daemon-side `idle-daemon/src/daemon/pidfile.rs` writes with
//! `O_NOFOLLOW|O_CREAT|O_EXCL` (see F-008). The consumer side must read
//! with the same defense in depth: `O_NOFOLLOW` on the open and an identity
//! check on the parsed pid before sending any signal.

use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;

/// Read a pidfile via `O_NOFOLLOW`. Returns the parsed pid if the file is a
/// regular file containing a parseable integer. Symlink paths return `None`.
pub(crate) fn read_pidfile_safely(path: &std::path::Path) -> Option<i32> {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    buf.trim().parse::<i32>().ok()
}

/// Verify that `/proc/<pid>/cmdline` or `/proc/<pid>/comm` indicates this is
/// an idle-daemon process before signaling it.
pub(crate) fn pid_targets_idle_daemon(pid: i32) -> bool {
    let cmdline = std::fs::read_to_string(format!("/proc/{pid}/cmdline")).unwrap_or_default();
    if cmdline
        .split('\0')
        .filter(|s| !s.is_empty())
        .any(|argv| argv.ends_with("/idle-daemon") || argv.ends_with("/idlescreen-daemon"))
    {
        return true;
    }
    if let Ok(comm) = std::fs::read_to_string(format!("/proc/{pid}/comm")) {
        let c = comm.trim();
        // Kernel truncates comm to 15 bytes; accept either form.
        if c == "idle-daemon" || c == "idlescreen-" {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_symlink_target() {
        let dir = std::env::temp_dir().join(format!("idle-pf-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let real = dir.join("real.pid");
        let link = dir.join("link.pid");
        std::fs::write(&real, "99999\n").unwrap();
        // Try to symlink. Skip if not supported (Windows etc).
        if std::os::unix::fs::symlink(&real, &link).is_ok() {
            assert!(
                read_pidfile_safely(&link).is_none(),
                "O_NOFOLLOW must refuse the symlink; got pid"
            );
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn reads_regular_pidfile() {
        let dir = std::env::temp_dir().join(format!("idle-pf-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let f = dir.join("plain.pid");
        std::fs::write(&f, "1234\n").unwrap();
        assert_eq!(read_pidfile_safely(&f), Some(1234));
        let _ = std::fs::remove_dir_all(&dir);
    }
}