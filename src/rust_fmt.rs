//! Hook logic for reminding the agent to run `cargo fmt` after Rust files change.
//!
//! Rather than running the formatter directly, we inject a suggestion into the agent's context
//! via `HookOutput`. The reminder is sent according to the configured `fmt-reminder` policy.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::{config::FormatReminderPolicy, session_state::SessionData};

/// Context passed to the format hook functions.
pub struct RustFmtHookContext {
    /// Directory where the tool ran.
    pub workdir: PathBuf,
}

/// Snapshot the modification times of all `*.rs` files found recursively
/// under `cwd`. Stored in session state at the end of each `PostToolUse`
/// and compared at the start of the next one.
pub fn snapshot_rust_files(cwd: &Path) -> HashMap<PathBuf, SystemTime> {
    let mut mtimes = HashMap::new();
    collect_rust_file_mtimes(cwd, &mut mtimes);
    mtimes
}

/// Walk `dir` recursively, collecting mtimes (modification time) of all `*.rs` files.
fn collect_rust_file_mtimes(dir: &Path, mtimes: &mut HashMap<PathBuf, SystemTime>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entry) => entry,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_file_mtimes(&path, mtimes);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            if let Ok(meta) = std::fs::metadata(&path) {
                if let Ok(mtime) = meta.modified() {
                    mtimes.insert(path, mtime);
                }
            }
        }
    }
}

/// Check whether any `*.rs` files under `cwd` have changed compared to
/// the snapshot stored in session state. Returns `true` if any file was
/// added, removed, or modified.
pub fn rust_files_changed_since(cwd: &Path, previous: &HashMap<PathBuf, SystemTime>) -> bool {
    let current = snapshot_rust_files(cwd);

    // Any new or modified files?
    for (path, mtime) in &current {
        match previous.get(path) {
            Some(prev) if prev != mtime => return true, // modified
            None => return true,                        // new file
            _ => {}
        }
    }

    // Any deleted files?
    for path in previous.keys() {
        if !current.contains_key(path) {
            return true;
        }
    }

    false
}

/// Called at `PostToolUse`. Returns a suggestion string if the agent should
/// be reminded to run `cargo fmt`, or `None` if no reminder is needed.
///
/// A reminder is sent when:
///   - At least one `*.rs` file changed since the last `PostToolUse`, AND
///   - The configured `fmt-reminder` policy allows it.
///
/// The session state is updated regardless — the snapshot is refreshed so
/// subsequent tool uses compare against the latest state.
pub fn maybe_suggest_rust_fmt(
    session: &mut SessionData,
    cwd: &Path,
    policy: &FormatReminderPolicy,
) -> Option<String> {
    let changed = rust_files_changed_since(cwd, &session.rust_file_snapshot);

    // Refresh snapshot for the next tool use regardless of whether we remind the agent to run `cargo fmt`
    session.rust_file_snapshot = snapshot_rust_files(cwd);

    if !changed {
        return None;
    }

    match policy {
        // Reminder is sent at most once per session.
        // TODO: make the threshold configurable (e.g. every N tool uses)
        FormatReminderPolicy::Once => {
            if session.rust_fmt_reminder_sent {
                return None;
            }
            session.rust_fmt_reminder_sent = true;
            Some(rust_fmt_suggestion_text())
        }
        FormatReminderPolicy::Always => Some(rust_fmt_suggestion_text()),
        FormatReminderPolicy::Never => None,
    }
}

fn rust_fmt_suggestion_text() -> String {
    "One or more Rust source files were modified.\n\
     Please run `cargo fmt` to keep the code consistently formatted."
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
        thread,
        time::{Duration, SystemTime},
    };

    use tempfile::tempdir;

    use super::*;
    use crate::config::FormatReminderPolicy;
    use crate::session_state::SessionData;

    const MTIME_SLEEP_MS: u64 = 10;

    fn touch(path: &Path) {
        fs::write(path, b"fn main() {}").unwrap();
    }

    fn make_session_with_snapshot(root: &Path) -> SessionData {
        SessionData {
            rust_file_snapshot: snapshot_rust_files(root),
            rust_fmt_reminder_sent: false,
            ..Default::default()
        }
    }

    fn make_temp_rs(root: &Path, name: &str) -> PathBuf {
        let p = root.join(name);
        touch(&p);
        p
    }

    fn modify_file(path: &Path) {
        // Sleep to ensure the mtime actually changes on filesystems
        // with low-resolution timestamps.
        thread::sleep(Duration::from_millis(MTIME_SLEEP_MS));
        fs::write(path, b"fn main() { println!(\"changed\"); }").unwrap();
    }

    #[test]
    fn snapshot_rust_files_collects_only_rs_files_recursively() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let subdir = root.join("sub");
        fs::create_dir(&subdir).unwrap();

        let rs1 = make_temp_rs(root, "a.rs");
        let rs2 = make_temp_rs(&subdir, "b.rs");
        let txt = root.join("ignore.txt");
        fs::write(&txt, b"not rust").unwrap();

        let snapshot = snapshot_rust_files(root);

        assert!(snapshot.contains_key(&rs1));
        assert!(snapshot.contains_key(&rs2));
        assert!(!snapshot.contains_key(&txt));
    }

    #[test]
    fn collect_rust_file_mtimes_ignores_unreadable_directories() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let rs1 = make_temp_rs(root, "a.rs");

        let mut mtimes: HashMap<PathBuf, SystemTime> = HashMap::new();
        collect_rust_file_mtimes(root, &mut mtimes);

        assert!(mtimes.contains_key(&rs1));
    }

    #[test]
    fn rust_files_changed_since_is_false_when_nothing_changes() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let _rs1 = make_temp_rs(root, "a.rs");
        let snapshot = snapshot_rust_files(root);

        assert!(!rust_files_changed_since(root, &snapshot));
    }

    #[test]
    fn rust_files_changed_since_detects_new_file() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let snapshot = snapshot_rust_files(root);
        assert!(snapshot.is_empty());

        let _rs1 = make_temp_rs(root, "a.rs");

        assert!(rust_files_changed_since(root, &snapshot));
    }

    #[test]
    fn rust_files_changed_since_detects_modified_file() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let rs1 = make_temp_rs(root, "a.rs");
        let snapshot = snapshot_rust_files(root);

        modify_file(&rs1);

        assert!(rust_files_changed_since(root, &snapshot));
    }

    #[test]
    fn rust_files_changed_since_detects_deleted_file() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let rs1 = make_temp_rs(root, "a.rs");
        let snapshot = snapshot_rust_files(root);

        fs::remove_file(&rs1).unwrap();

        assert!(rust_files_changed_since(root, &snapshot));
    }

    #[test]
    fn maybe_suggest_rust_fmt_returns_none_when_no_files_changed() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let mut session = make_session_with_snapshot(root);
        let policy = FormatReminderPolicy::Once;

        assert!(maybe_suggest_rust_fmt(&mut session, root, &policy).is_none());
    }

    #[test]
    fn maybe_suggest_rust_fmt_once_sends_only_on_first_change() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let rs1 = root.join("a.rs");
        let mut session = make_session_with_snapshot(root);
        let policy = FormatReminderPolicy::Once;

        // First change — should remind
        touch(&rs1);
        let suggestion1 = maybe_suggest_rust_fmt(&mut session, root, &policy);
        assert!(suggestion1.is_some());
        assert!(session.rust_fmt_reminder_sent);

        // Second change — snapshot was refreshed after first call,
        // so we need to modify the file again to trigger a change.
        // But policy is Once so reminder should not be sent again.
        modify_file(&rs1);
        let suggestion2 = maybe_suggest_rust_fmt(&mut session, root, &policy);
        assert!(suggestion2.is_none());
    }

    #[test]
    fn maybe_suggest_rust_fmt_always_sends_on_every_change() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let rs1 = root.join("a.rs");
        let mut session = make_session_with_snapshot(root);
        let policy = FormatReminderPolicy::Always;

        touch(&rs1);
        assert!(maybe_suggest_rust_fmt(&mut session, root, &policy).is_some());

        // Snapshot refreshed — modify again to trigger second change
        modify_file(&rs1);
        assert!(maybe_suggest_rust_fmt(&mut session, root, &policy).is_some());
    }

    #[test]
    fn maybe_suggest_rust_fmt_never_sends_even_when_files_change() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let rs1 = root.join("a.rs");
        let mut session = make_session_with_snapshot(root);
        let policy = FormatReminderPolicy::Never;

        touch(&rs1);
        assert!(maybe_suggest_rust_fmt(&mut session, root, &policy).is_none());
    }

    #[test]
    fn rust_fmt_suggestion_text_has_expected_content() {
        let msg = rust_fmt_suggestion_text();
        assert!(msg.contains("One or more Rust source files were modified."));
        assert!(msg.contains("cargo fmt"));
    }
}
