use git2::{Commit, IndexAddOption, Repository};
use std::fs::File;
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
    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();

    let parent_commit = match repo.revparse_single("HEAD") {
        Ok(obj) => Some(obj.into_commit().unwrap()),
        Err(_) => None,
    };
    let mut parents = Vec::new();
    if parent_commit.is_some() {
        parents.push(parent_commit.as_ref().unwrap());
    }

    let signature = repo.signature().unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &parents[..],
    )
    .unwrap();
    repo.head().unwrap().peel_to_commit().unwrap()
}

pub fn add_all(repo: &Repository) {
    let mut index = repo.index().unwrap();
    index
        .add_all(&["."], IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write_tree().unwrap();
    index.write().unwrap();
}

pub fn create_file(dir: &Path, file_name: &str, content: &str) {
    let file_path = dir.join(file_name);
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "{}", content).unwrap();
}

pub fn add_tag(repo: &Repository, tag_name: &str) {
    let obj = repo.revparse_single("HEAD").unwrap();
    repo.tag_lightweight(tag_name, &obj, false).unwrap();
}

pub fn get_short_hash(commit: &Commit) -> String {
    commit.id().to_string()[..7].to_string()
}
