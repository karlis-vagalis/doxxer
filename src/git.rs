use git2::{Error, ObjectType, Repository};
use semver::{BuildMetadata, Prerelease, Version};
use std::{slice::Iter};

fn find_tag_name_matching_version(
    repo: &Repository,
    version_string: &str,
    tag_prefix: &str,
) -> Result<Option<String>, Error> {
    let search_term = format!("{}{}", tag_prefix, version_string);
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

fn find_latest_semver(repo: &Repository, prefix: &str) -> Result<Option<Version>, Error> {
    let mut versions: Vec<Version> = Vec::new();
    repo.tag_foreach(|id, name_bytes| {
        if let Ok(name) = String::from_utf8(name_bytes.to_vec()) {
            if let Some(tag_name) = name.strip_prefix("refs/tags/") {
                let tag_name = {
                    if tag_name.starts_with(prefix) {
                        tag_name.trim_start_matches(prefix)
                    } else {
                        tag_name
                    }
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

fn inject_variables(template: &str, old_pre: &str, commit_count: usize, short_hash: &String) -> String {
    let mut template = String::from(template);
    for variable in TemplateVariables::iterator() {
        match variable {
            TemplateVariables::Hash => template = template.replace(variable.as_str(), short_hash.as_str()),
            TemplateVariables::Distance => template = template.replace(variable.as_str(), commit_count.to_string().as_str()),
            TemplateVariables::OldPre => template = template.replace(variable.as_str(), old_pre),
        }
    }
    template = match template.strip_prefix(".") {
        Some(s) => s.to_string(),
        None => template
    };
    template = match template.strip_suffix(".") {
        Some(s) => s.to_string(),
        None => template
    };
    template
}

pub fn next_version(
    repo: &Repository,
    tag_prefix: &str,
    pre_template: &str,
    build_template: &str,
) -> Version {
    let latest = current_version(repo, tag_prefix);

    let latest_tag_name =
        match find_tag_name_matching_version(repo, &latest.to_string(), tag_prefix) {
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

    let pre = inject_variables(pre_template, next.pre.as_str(), commit_count, &short_hash);
    let build = inject_variables(build_template, next.pre.as_str(), commit_count, &short_hash);

    next.pre = Prerelease::new(pre.as_str()).unwrap();
    next.build = BuildMetadata::new(build.as_str()).unwrap();
    next
}

pub fn current_version(repo: &Repository, tag_prefix: &str) -> Version {
    match find_latest_semver(repo, tag_prefix) {
        Ok(Some(v)) => v,
        _ => Version::new(0, 0, 0),
    }
}

#[derive(Debug)]
enum TemplateVariables {
    OldPre,
    Hash,
    Distance,
}
impl TemplateVariables {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemplateVariables::Hash => "{hash}",
            TemplateVariables::Distance => "{distance}",
            TemplateVariables::OldPre => "{old_pre}",
        }
    }
    pub fn iterator() -> Iter<'static, TemplateVariables> {
        static TEMPLATE_VARIABLES: [TemplateVariables; 3] = [TemplateVariables::Hash, TemplateVariables::Distance, TemplateVariables::OldPre];
        TEMPLATE_VARIABLES.iter()
    }
}
