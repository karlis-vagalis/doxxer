use git2::{Repository, Signature, Commit, Oid};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use tempfile::{TempDir, tempdir};

pub fn initialize_repository() -> (TempDir, Repository) {
    let td = tempdir().unwrap();
    let repo = Repository::init(td.path()).unwrap();
    {
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();
    }
    (td, repo)
}

pub fn add_commit<'repo>(
    repo: &'repo Repository,
    message: &str
) -> Commit<'repo> {
    let signature = repo.signature().unwrap();
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[],
    )
    .unwrap();
    repo.head().unwrap().peel_to_commit().unwrap()
}

pub fn add_tag(repo: &Repository, commit: &Commit, tag_name: &str) -> Oid {
    let signature = Signature::now("Test User", "test@example.com").unwrap();
    repo.tag(
        tag_name,
        &commit.as_object(),
        &signature,
        "", // No tag message for lightweight tags
        false, // Not forcing
    )
    .unwrap()
}

// Helper to create a dummy file in the repo, needed for commits to have content
#[allow(dead_code)] // This might be used by specific tests later
pub fn create_dummy_file(repo_path: &Path, file_name: &str, content: &str) {
    let file_path = repo_path.join(file_name);
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "{}", content).unwrap();

    // Stage the file
    let mut index = Repository::open(repo_path).unwrap().index().unwrap();
    index.add_path(Path::new(file_name)).unwrap();
    index.write().unwrap();
}

// Helper to append to a dummy file in the repo, needed for commits to have content
#[allow(dead_code)] // This might be used by specific tests later
pub fn append_to_dummy_file(repo_path: &Path, file_name: &str, content: &str) {
    let file_path = repo_path.join(file_name);
    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
    writeln!(file, "{}", content).unwrap();

    // Stage the file
    let mut index = Repository::open(repo_path).unwrap().index().unwrap();
    index.add_path(Path::new(file_name)).unwrap();
    index.write().unwrap();
}