use git2::Repository;

pub fn get_latest_tag(repo: Repository) {
    println!("{:#?}", repo.path());
}