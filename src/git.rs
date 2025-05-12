use git2::{Error, Repository, Tag};
use semver::{BuildMetadata, Prerelease, Version};

use crate::Cli;

fn find_latest_semver(
    repo: &Repository,
    prefix: Option<&str>,
) -> Result<Option<Version>, Error> {
    let mut versions: Vec<Version> = Vec::new();
    repo.tag_foreach(|id, name_bytes| {
        if let Ok(name) = String::from_utf8(name_bytes.to_vec()) {
            if let Some(tag_name) = name.strip_prefix("refs/tags/") {
                let tag_name = if let Some(p) = prefix {
                    if tag_name.starts_with(p) {
                        tag_name.trim_start_matches(p)
                    } else {
                        tag_name
                    }
                } else {
                    tag_name
                };
                if let Ok(version) = Version::parse(tag_name) {
                    versions.push(version);
                }
            }
        }
        return true;
    });

    versions.sort();
    versions.reverse();

    Ok(versions.into_iter().next())
}

fn get_commit_count_since_tag(repo: &Repository, tag_name: Option<&str>) -> Result<usize, Error> {
    let revspec = if let Some(tag) = tag_name {
        format!("{}..HEAD", tag)
    } else {
        "HEAD".to_string()
    };
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    if let Some(tag) = tag_name {
        if let Ok(obj) = repo.revparse_single(tag) {
            revwalk.hide(obj.id())?;
        }
    }

    Ok(revwalk.count())
}

pub fn bump(repo: &Repository, tag: Option<&str>) {
    
}

pub fn get_semver(repo: &Repository, prefix: Option<&str>) -> Version {
    match find_latest_semver(repo, prefix) {
        Ok(Some(v)) => v,
        _ => Version::new(0, 0, 0),
    }
}