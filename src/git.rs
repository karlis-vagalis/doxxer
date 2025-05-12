use git2::{Error, Repository, ObjectType};
use semver::{BuildMetadata, Prerelease, Version};

use crate::Cli;

fn find_tag_name_matching_version(repo: &Repository, version_string: &str, tag_prefix: Option<&str>) -> Result<Option<String>, Error> {
    let search_term = format!("{}{}", tag_prefix.unwrap_or(""), version_string);
    let mut matching_tag_name: Option<String> = None;
    for tag_name_result in repo.tag_names(None)?.iter() {
        if let Some(tag_name) = tag_name_result {
            if tag_name == search_term {
                matching_tag_name = Some(format!("refs/tags/{}", tag_name));
                break;
            }
        }
    }
    Ok(matching_tag_name)
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

fn get_short_head_hash(repo: &Repository) -> Result<String, Error> {
    let head = repo.head()?;
    let commit = head.peel(ObjectType::Commit)?;
    let id = commit.id();
    Ok(id.to_string()[..7].to_string())
}

fn find_latest_semver(repo: &Repository, prefix: Option<&str>) -> Result<Option<Version>, Error> {
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

pub fn next_version(repo: &Repository, tag_prefix: Option<&str>) -> Version {

    let latest = current_version(repo, tag_prefix);

    let latest_tag_name = match find_tag_name_matching_version(repo, &latest.to_string(), tag_prefix) {
        Ok(tag) => tag,
        Err(_) => None,
    };
    let commit_count = match get_commit_count_since_tag(repo, latest_tag_name.as_deref()) {
        Ok(count) => count,
        Err(_) => 0,
    };
    let short_hash = match get_short_head_hash(repo) {
        Ok(hash) => hash,
        Err(_) => String::from(""),
    };

    let mut next = latest;
    let mut pre = String::new();
    if !next.pre.is_empty() {
        pre = format!("{}.", next.pre.as_str())
    }
    next.pre = Prerelease::new(&format!("{pre}dev.{commit_count}")).unwrap();
    next.build = BuildMetadata::new(&format!("{}", short_hash)).unwrap();
    next
}

pub fn current_version(repo: &Repository, tag_prefix: Option<&str>) -> Version {
    match find_latest_semver(repo, tag_prefix) {
        Ok(Some(v)) => v,
        _ => Version::new(0, 0, 0),
    }
}
