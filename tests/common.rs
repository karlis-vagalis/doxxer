use git2::{Commit, IndexAddOption, Oid, Repository, Signature};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn initialize_repository(path: &Path) -> Repository {
    let repo = Repository::init(path).unwrap();
    {
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();
    }
    repo
}

pub fn add_commit<'repo>(repo: &'repo Repository, message: &str) -> Commit<'repo> {
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let signature = repo.signature().unwrap();
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
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
        "",    // No tag message for lightweight tags
        false, // Not forcing
    )
    .unwrap()
}

pub fn add_all(repo: &Repository) {
    let mut index = repo.index().unwrap();
    index
        .add_all(&["."], IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();
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
