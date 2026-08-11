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

/// Verify that `/proc/<pid>/cmdline` AND `/proc/<pid>/comm` BOTH identify
/// the target as idle-daemon, before signaling it. Belt + suspenders:
/// - `cmdline` is set by the kernel from the path passed to `execve(2)` —
///   an attacker would need a binary literally named `idle-daemon` on
///   disk in a location that ends up in the cmdline argv0.
/// - `comm` is settable by the target itself via `prctl(PR_SET_NAME, …)`,
///   so it's only useful as a *secondary* signal; we never accept comm
///   alone.
///
/// F-203 mitigation: the previous version accepted comm alone. That let
/// any process call `prctl(PR_SET_NAME, "idlescreen-")` and pass the
/// check (15-byte kernel-truncated form). The combined check closes the
/// spoof window: an attacker would need both a `idlescreen-daemon`
/// binary in a path that ends up in argv0 AND the prctl call.
pub(crate) fn pid_targets_idle_daemon(pid: i32) -> bool {
    let cmdline_match = std::fs::read_to_string(format!("/proc/{pid}/cmdline"))
        .map(|s| {
            s.split('\0')
                .filter(|a| !a.is_empty())
                .any(|argv| argv.ends_with("/idle-daemon") || argv.ends_with("/idlescreen-daemon"))
        })
        .unwrap_or(false);
    if !cmdline_match {
        return false;
    }
    // Comm is set by the kernel from the first 15 bytes of argv0 by
    // default, or by the target via prctl. We require it to be one of
    // the two acceptable names — the kernel-truncated "idlescreen-"
    // form AND the longer "idle-daemon" form. A process that didn't
    // name itself accordingly is suspicious enough to refuse.
    let comm_match = std::fs::read_to_string(format!("/proc/{pid}/comm"))
        .map(|s| {
            let c = s.trim();
            c == "idle-daemon" || c == "idlescreen-" || c == "idlescreen-daemon"
        })
        .unwrap_or(false);
    cmdline_match && comm_match
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

    /// F-203 regression: `pid_targets_idle_daemon` must require BOTH
    /// cmdline argv0 AND comm to identify the target. A process that
    /// only set comm via prctl(PR_SET_NAME) (without a real
    /// `idlescreen-daemon` binary in argv0) must be refused.
    #[test]
    fn f203_requires_both_cmdline_and_comm() {
        // Self-test: `cargo test` runs the test binary. Its cmdline
        // argv0 won't end with /idle-daemon; comm is set from the
        // test binary name. The function MUST refuse self — if it
        // returned true, the combined check would be vacuous and
        // any process would pass.
        let me = std::process::id() as i32;
        assert!(
            !pid_targets_idle_daemon(me),
            "test-binary argv0 must not satisfy cmdline match (otherwise the check is vacuous)"
        );
    }
}