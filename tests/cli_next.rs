mod common;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

use common::{add_all, add_commit, add_tag, create_file, get_short_hash, initialize_repository};

#[test]
fn test_next_patch_repo_975() {
    let td = tempfile::tempdir().unwrap();
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    let commit = add_commit(&repo, "Initial commit");
    add_tag(&repo, "version-9.7.5");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("next")
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "9.7.5-dev.1+{}",
            get_short_hash(&commit)
        )));
}

#[test]
fn test_next_minor_env() {
    let td = tempfile::tempdir().unwrap();
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    add_commit(&repo, "Initial commit");
    add_tag(&repo, "v4.86.7");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("next")
        .arg("minor")
        .env("DOXXER__NEXT__MINOR__INCREMENT", "3")
        .assert()
        .success()
        .stdout(predicate::str::contains("4.89.0"));
}

#[test]
fn test_next_patch_with_pre() {
    let td = tempfile::tempdir().unwrap();
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    add_commit(&repo, "Initial commit");
    add_tag(&repo, "9.0.0-beta.4");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("next")
        .arg("patch")
        .assert()
        .success()
        .stdout(predicate::str::contains("9.0.0"));
}

#[test]
fn test_next_patch() {
    let td = tempfile::tempdir().unwrap();
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    add_commit(&repo, "Initial commit");
    add_tag(&repo, "9.0.0");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("next")
        .arg("patch")
        .assert()
        .success()
        .stdout(predicate::str::contains("9.0.1"));
}

#[test]
fn test_next_minor_autodetect_identifier() {
    let td = tempfile::tempdir().unwrap();
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    add_commit(&repo, "Initial commit");
    add_tag(&repo, "5.1.2-alpha.5");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("next")
        .arg("prerelease")
        .assert()
        .success()
        .stdout(predicate::str::contains("5.1.2-alpha.6"));
}

#[test]
fn test_next_minor_autodetect_identifier_dev() {
    let td = tempfile::tempdir().unwrap();
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    add_commit(&repo, "Initial commit");
    add_tag(&repo, "5.1.2-alpha.5");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("next")
        .assert()
        .success()
        .stdout(predicate::str::contains("5.1.2-alpha.5.dev.0"));
}

#[test]
fn test_next_minor_autodetect_identifier_nonstandard() {
    let td = tempfile::tempdir().unwrap();
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    add_commit(&repo, "Initial commit");
    add_tag(&repo, "5.1.2-alpha5");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("next")
        .arg("prerelease")
        .assert()
        .success()
        .stdout(predicate::str::contains("5.1.2-alpha.6"));
}

#[test]
fn test_next_minor_override_identifier() {
    let td = tempfile::tempdir().unwrap();
    let td = td.path();

    let repo = initialize_repository(td);
    create_file(td, "file.txt", "initial content");
    add_all(&repo);
    add_commit(&repo, "Initial commit");
    add_tag(&repo, "5.1.2-alpha.5");

    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir(td)
        .arg("next")
        .arg("prerelease")
        .arg("beta")
        .assert()
        .success()
        .stdout(predicate::str::contains("5.1.2-beta.1"));
}
