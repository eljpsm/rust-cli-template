//! End-to-end tests of the binary: exit codes, stdout and stderr contracts.

use std::path::PathBuf;
use std::process::{Command, Output};

struct TempTree {
    root: PathBuf,
}

impl TempTree {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!("renameme-cli-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        TempTree { root }
    }

    fn write(&self, rel: &str, contents: &str) -> PathBuf {
        let path = self.root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, contents).unwrap();
        path
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_renameme"))
            .args(args)
            .current_dir(&self.root)
            .output()
            .unwrap()
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn code(output: &Output) -> i32 {
    output.status.code().unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn a_bare_invocation_prints_usage_and_exits_two() {
    let tree = TempTree::new("bare");
    let out = tree.run(&[]);
    assert_eq!(code(&out), 2);
    assert_eq!(stdout(&out), "");
    assert!(
        stderr(&out).contains("Usage:"),
        "stderr was: {}",
        stderr(&out)
    );
}

#[test]
fn greet_defaults_to_the_world() {
    let tree = TempTree::new("greet");
    let out = tree.run(&["greet"]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out), "hello, world\n");
    assert_eq!(stderr(&out), "");
}

#[test]
fn greet_shouts_a_name() {
    let tree = TempTree::new("shout");
    let out = tree.run(&["greet", "--shout", "crab"]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out), "HELLO, CRAB\n");
}

#[test]
fn count_prints_the_line_count() {
    let tree = TempTree::new("count");
    tree.write("notes.txt", "one\ntwo\nthree\n");
    let out = tree.run(&["count", "notes.txt"]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out), "3\n");
}

#[test]
fn count_on_a_missing_file_fails_with_a_prefixed_error() {
    let tree = TempTree::new("missing");
    let out = tree.run(&["count", "missing.txt"]);
    assert_eq!(code(&out), 1);
    assert_eq!(stdout(&out), "");
    let err = stderr(&out);
    assert!(
        err.starts_with("renameme: read missing.txt"),
        "stderr was: {err}"
    );
}
