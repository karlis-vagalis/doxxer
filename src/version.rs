use git2::{Error, ObjectType, Repository};
use once_cell::sync::Lazy;
use semver::{BuildMetadata, Prerelease, Version};
use serde_json::{json, Value};

use crate::{
    cli::{Field, Format, PreReleaseWithBumpArgs, PrereleaseArgs, StandardBumpArgs}, template::TemplateVariables, PrereleaseOptions, Strategy
};

use regex::Regex;

static SEMVER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?").unwrap()
});

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

pub fn next_version(repo: &Repository, filter: &Regex, strategy: &Strategy) -> Version {
    let latest = current_version(repo, filter);

    let latest_tag_name = match find_tag_name_matching_version(repo, &latest.to_string(), filter) {
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

    let mut pre = Prerelease::EMPTY;
    let mut build = BuildMetadata::EMPTY;

    // Set new major/minor/patch versions
    match strategy {
        Strategy::Major(StandardBumpArgs{bump_options,..})
        | Strategy::PreMajor(PreReleaseWithBumpArgs{bump_options, ..}) => {
            next.major += bump_options.increment;
            next.minor = 0;
            next.patch = 0;
        }
        Strategy::Minor(StandardBumpArgs{bump_options,..})
        | Strategy::PreMinor(PreReleaseWithBumpArgs{bump_options, ..}) => {
            next.minor += bump_options.increment;
            next.patch = 0;
        }
        Strategy::Patch(StandardBumpArgs{bump_options,..})
        | Strategy::PrePatch(PreReleaseWithBumpArgs{bump_options, ..}) => {
            next.patch += bump_options.increment;
        }
        Strategy::Prerelease(_) => {}
        Strategy::Dev(_) => {}
    }

    // Set new prerelease and metadata
    match strategy {
        Strategy::Prerelease(PrereleaseArgs{prerelease_options, build_metadata_options, ..})
        | Strategy::PreMajor(PreReleaseWithBumpArgs{prerelease_options, build_metadata_options, ..})
        | Strategy::PreMinor(PreReleaseWithBumpArgs{prerelease_options, build_metadata_options, ..})
        | Strategy::PrePatch(PreReleaseWithBumpArgs{prerelease_options, build_metadata_options, ..}) => {
            let inc = get_inc(next.pre.as_str(), prerelease_options.identifier.as_str());
            let template_variables = TemplateVariables {
                pre: next.pre.as_str().to_string(),
                inc,
                hash: short_hash,
                distance: commit_count,
                identifier: prerelease_options.identifier.clone(),
            };
            pre = handle_prerelease(prerelease_options, &template_variables);
            if let Some(template) = &build_metadata_options.template {
                build = handle_build_metadata(&template, &template_variables);
            }
        }
        _ => {}
    }

    next.pre = pre;
    next.build = build;
    next
}

fn handle_prerelease(options: &PrereleaseOptions, variables: &TemplateVariables) -> Prerelease {
    Prerelease::new(variables.inject(&options.prerelease_template).as_str()).unwrap()
}

fn handle_build_metadata(
    template: &str,
    variables: &TemplateVariables,
) -> BuildMetadata {
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
    output_template: &str
) {
    match output_format {
        Format::Plain => match field {
            None => {
                println!(
                    "{}",
                    output_template.replace("{version}", version.to_string().as_str())
                );
            }
            Some(part) => match part {
                Field::Major => println!("{}", version.major),
                Field::Minor => println!("{}", version.minor),
                Field::Patch => println!("{}", version.patch),
                Field::Pre => println!("{}", version.pre),
                Field::Build => println!("{}", version.build),
            },
        },
        Format::Json => {
            let json_value = match field {
                Some(Field::Major) => json!({ "major": version.major }),
                Some(Field::Minor) => json!({ "minor": version.minor }),
                Some(Field::Patch) => json!({ "patch": version.patch }),
                Some(Field::Pre) => json!({ "pre": version.pre.as_str() }),
                Some(Field::Build) => json!({ "build": version.build.as_str() }),
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
                    map.insert("full".to_string(), json!(version.to_string()));
                    Value::Object(map)
                }
            };
            // This unwrap is generally safe if json! macro is used correctly.
            // Consider .expect("Failed to serialize to JSON") for clarity.
            println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
        }
    }
}
