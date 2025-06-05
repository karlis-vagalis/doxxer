mod common;

use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_current_no_git_repo() {
    let temp_dir = tempfile::tempdir().unwrap();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("current")
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!("Issue opening repository: could not find repository at '{}'!", temp_dir.path().to_str().unwrap())));
}
