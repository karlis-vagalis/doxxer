use git2::{Error, Repository, Tag};
use semver::{BuildMetadata, Prerelease, Version};

use crate::Cli;

pub fn get_latest_semver(
    repo: &Repository,
    prefix: &Option<String>,
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

pub fn handle_version(repo: &Repository, args: &Cli) {
    let latest: Option<Version>;
    match get_latest_semver(repo, &args.prefix) {
        Ok(opt) => {
            match opt {
                Some(version) => {
                    latest = Some(version);
                },
                None => {
                    println!("No SemVer tags found.");
                    latest = None;
                    // You might want to handle this differently, not just a println
                    // For example, setting a default or returning.
                },
            }
        },
        Err(e) => {
            eprintln!("Error getting latest SemVer tag: {}", e);
            latest = None;
            // Similarly, you might want more specific error handling here.
        },
    }

    // Now you can access 'latest_version' here
    dbg!(latest);
}