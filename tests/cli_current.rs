mod common;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use crate::common::{add_all, add_commit, create_file, initialize_repository};

#[test]
fn test_current_no_git_repo() {
    let temp_dir = tempfile::tempdir().unwrap();
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("current")
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Issue opening repository: could not find repository at '{}'!",
            temp_dir.path().to_str().unwrap()
        )));
}

#[test]
fn test_current_repo_no_tags() {
    let mut td = tempfile::tempdir().unwrap();
    td.disable_cleanup(true);
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    add_commit(&repo, "Initial commit");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("current")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No tags found in the repository"));
}
