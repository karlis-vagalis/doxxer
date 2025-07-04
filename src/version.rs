use chrono::Utc;
use git2::{Error, ObjectType, Repository};
use once_cell::sync::Lazy;
use semver::{BuildMetadata, Prerelease, Version};
use serde_json::{json, Value};

use crate::{
    cli::{Field, Format},
    settings::Settings,
    template::TemplateVariables,
    Strategy,
};

use regex::Regex;

static SEMVER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?").unwrap()
});

fn get_current_branch_name(repo: &Repository) -> Result<Option<String>, Error> {
    let head = repo.head()?;
    if head.is_branch() {
        let branch_name_shorthand = head.shorthand();
        if let Some(shorthand) = branch_name_shorthand {
            Ok(Some(shorthand.to_string()))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn normalize_branch_name_for_semver(branch_name: &str) -> String {
    let mut normalized_chars: Vec<char> = Vec::new();
    let mut last_char_was_hyphen = false;

    for c in branch_name.chars() {
        if c.is_ascii_alphanumeric() {
            normalized_chars.push(c.to_ascii_lowercase());
            last_char_was_hyphen = false;
        } else if c == '-' {
            if !last_char_was_hyphen {
                normalized_chars.push('-');
                last_char_was_hyphen = true;
            }
        } else {
            if !last_char_was_hyphen {
                normalized_chars.push('-');
                last_char_was_hyphen = true;
            }
        }
    }
    let mut normalized_name: String = normalized_chars.into_iter().collect();
    while normalized_name.starts_with('-') {
        normalized_name.remove(0);
    }
    while normalized_name.ends_with('-') {
        normalized_name.pop();
    }

    normalized_name
}

fn find_tag_name_matching_version(
    repo: &Repository,
    version_string: &str,
    filter: &Regex,
) -> Result<Option<String>, Error> {
    let search_term = format!("{}{}", filter, version_string);
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
    if let Some(tag) = tag_name {
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

fn find_latest_semver(repo: &Repository, filter: &Regex) -> Result<Option<Version>, Error> {
    let mut versions: Vec<Version> = Vec::new();
    let _ = repo.tag_foreach(|_id, name_bytes| {
        if let Ok(name) = String::from_utf8(name_bytes.to_vec()) {
            if let Some(tag_name) = name.strip_prefix("refs/tags/") {
                if filter.is_match(tag_name) {
                    if let Some(captures) = SEMVER_REGEX.find(tag_name) {
                        let matched_str = captures.as_str();
                        if let Ok(version) = Version::parse(matched_str) {
                            versions.push(version);
                        }
                    }
                }
            }
        }
        return true;
    });

    versions.sort();
    versions.reverse();

    Ok(versions.into_iter().next())
}

fn get_inc(pre: &str, identifier: &str) -> usize {
    if pre.is_empty() {
        return 1;
    }

    if pre.starts_with(identifier) {
        // Current pre-release starts with the target identifier
        let suffix = &pre[identifier.len()..];
        if suffix.is_empty() {
            return 2; // Identifier matches, no number, next is 2
        }

        // Try to parse a numeric suffix from the end
        let mut numeric_part = String::new();
        for char in suffix.chars().rev() {
            if char.is_digit(10) {
                numeric_part.insert(0, char);
            } else {
                break; // Stop when a non-digit is encountered
            }
        }

        if !numeric_part.is_empty() {
            if let Ok(n) = numeric_part.parse::<usize>() {
                return n + 1;
            } else {
                // Parsing failed, but we found digits, so assume it was 1 initially
                return 2;
            }
        } else {
            // Identifier matches, but no numeric suffix found
            return 2;
        }
    } else {
        // Identifier does not match, start at 1
        return 1;
    }
}

pub fn next_version(repo: &Repository, strategy: &Strategy, settings: &Settings) -> Version {
    let latest = current_version(repo, &settings.filter.tag);

    let latest_tag_name =
        match find_tag_name_matching_version(repo, &latest.to_string(), &settings.filter.tag) {
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

    let date_time = Utc::now();
    let branch = get_current_branch_name(repo).unwrap().unwrap_or_default();
    let branch = normalize_branch_name_for_semver(&branch);

    let mut next = latest;

    // Set new major/minor/patch versions
    match strategy {
        Strategy::Major(_) | Strategy::PreMajor(_) => {
            next.major += settings.bump.increment;
            next.minor = 0;
            next.patch = 0;
        }
        Strategy::Minor(_) | Strategy::PreMinor(_) => {
            next.minor += settings.bump.increment;
            next.patch = 0;
        }
        Strategy::Patch(_) => {
            if next.pre.is_empty() {
                next.patch += settings.bump.increment;
            }
        }
        Strategy::PrePatch(_) => {
            next.patch += settings.bump.increment;
        }
        Strategy::Prerelease(_) => {}
        Strategy::Dev(_) => {}
    }

    let prerelease_identifier = match &settings.prerelease.identifier {
        Some(identifier) => identifier,
        None => todo!(),
    };

    // Set new prerelease and metadata
    let inc = get_inc(next.pre.as_str(), prerelease_identifier);
    let template_variables = TemplateVariables {
        pre: next.pre.as_str().to_string(),
        inc,
        hash: short_hash,
        distance: commit_count,
        identifier: settings.prerelease.identifier.clone(),
        date_time,
        branch: branch,
    };
    let pre = handle_prerelease(&settings.prerelease.template, &template_variables);
    let build = handle_build_metadata(&settings.build.template, &template_variables);

    next.pre = pre;
    next.build = build;
    next
}

fn handle_prerelease(template: &str, variables: &TemplateVariables) -> Prerelease {
    Prerelease::new(variables.inject(template).as_str()).unwrap()
}

fn handle_build_metadata(template: &str, variables: &TemplateVariables) -> BuildMetadata {
    BuildMetadata::new(variables.inject(template).as_str()).unwrap()
}

pub fn current_version(repo: &Repository, filter: &Regex) -> Version {
    match find_latest_semver(repo, filter) {
        Ok(Some(v)) => v,
        _ => Version::new(0, 0, 0),
    }
}

pub fn format_version(
    field: &Option<Field>,
    version: &Version,
    output_format: &Format,
    output_template: &str,
) {
    let full_version = output_template.replace("{version}", version.to_string().as_str());
    match output_format {
        Format::Plain => match field {
            None => {
                println!("{}", full_version);
            }
            Some(part) => match part {
                Field::Major => println!("{}", version.major),
                Field::Minor => println!("{}", version.minor),
                Field::Patch => println!("{}", version.patch),
                Field::Prerelease => println!("{}", version.pre),
                Field::BuildMetadata => println!("{}", version.build),
            },
        },
        Format::Json => {
            let json_value = match field {
                Some(Field::Major) => json!({ "major": version.major }),
                Some(Field::Minor) => json!({ "minor": version.minor }),
                Some(Field::Patch) => json!({ "patch": version.patch }),
                Some(Field::Prerelease) => json!({ "pre": version.pre.as_str() }),
                Some(Field::BuildMetadata) => json!({ "build": version.build.as_str() }),
                None => {
                    let mut map = serde_json::Map::new();
                    map.insert("major".to_string(), json!(version.major));
                    map.insert("minor".to_string(), json!(version.minor));
                    map.insert("patch".to_string(), json!(version.patch));
                    if !version.pre.is_empty() {
                        map.insert("pre".to_string(), json!(version.pre.as_str()));
                    }
                    if !version.build.is_empty() {
                        map.insert("build".to_string(), json!(version.build.as_str()));
                    }
                    map.insert("full".to_string(), json!(full_version));
                    Value::Object(map)
                }
            };
            // This unwrap is generally safe if json! macro is used correctly.
            // Consider .expect("Failed to serialize to JSON") for clarity.
            println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
        }
    }
}
