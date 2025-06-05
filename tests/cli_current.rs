mod common;

use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_current_no_git_repo() {
    let temp_dir = tempfile::tempdir().unwrap();
    let cmd = Command::cargo_bin("doxxer")
        .unwrap()
        .current_dir(temp_dir)
        .arg("current")
        .assert()
        .failure();
}
