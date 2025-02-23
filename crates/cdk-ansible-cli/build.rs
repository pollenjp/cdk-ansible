#![allow(clippy::expect_used, reason = "fix later")]
#![allow(clippy::unwrap_used, reason = "fix later")]
#![allow(clippy::single_call_fn, reason = "fix later")]
#![allow(clippy::absolute_paths, reason = "fix later")]

use anyhow::bail;
use anyhow::{Context as _, Result};
use cdk_ansible_static::EnvVars;
use core::fmt;
use fs_err as fs;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;

static TAG_PREFIX: &str = "cdk-ansible-cli";

fn main() -> Result<()> {
    let workspace_root = Path::new(&std::env::var(EnvVars::CARGO_MANIFEST_DIR).unwrap())
        .parent()
        .context("CARGO_MANIFEST_DIR should be nested in workspace")?
        .parent()
        .context("CARGO_MANIFEST_DIR should be doubly nested in workspace")?
        .to_path_buf();

    if let Err(e) = commit_info(&workspace_root) {
        println!("cargo:warning={e}");
    }
    Ok(())
}

fn commit_info(workspace_root: &Path) -> Result<()> {
    // If not in a git repository, do not attempt to retrieve commit information
    let git_dir = workspace_root.join(".git");
    if !git_dir.exists() {
        bail!("not in a git directory")
    }

    if let Some(git_head_path) = git_head(&git_dir) {
        println!("cargo:rerun-if-changed={}", git_head_path.display());

        let git_head_contents = fs::read_to_string(git_head_path);
        if let Ok(git_head_contents) = git_head_contents {
            // The contents are either a commit or a reference in the following formats
            // - "<commit>" when the head is detached
            // - "ref <ref>" when working on a branch
            // If a commit, checking if the HEAD file has changed is sufficient
            // If a ref, we need to add the head file for that ref to rebuild on commit
            let mut git_ref_parts = git_head_contents.split_whitespace();
            git_ref_parts.next();
            if let Some(git_ref) = git_ref_parts.next() {
                let git_ref_path = git_dir.join(git_ref);
                println!("cargo:rerun-if-changed={}", git_ref_path.display());
            }
        }
    }

    // git log -1 --date=short --abbrev=9 --format=%H %h %cd
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--date=short")
        .arg("--abbrev=9")
        .arg("--format=%H %h %cd")
        .output()
        .context("failed to run git log -1 --date=short --abbrev=9 --format=%H %h %cd")?;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut parts = stdout.split_whitespace();
    let mut next = || parts.next().unwrap();
    println!(
        "cargo:rustc-env={}={}",
        EnvVars::CDK_ANSIBLE_COMMIT_HASH,
        next()
    );
    println!(
        "cargo:rustc-env={}={}",
        EnvVars::CDK_ANSIBLE_COMMIT_SHORT_HASH,
        next()
    );
    println!(
        "cargo:rustc-env={}={}",
        EnvVars::CDK_ANSIBLE_COMMIT_DATE,
        next()
    );

    let latest_tag = git_latest_tag()?;
    println!(
        "cargo:rustc-env={}={}",
        EnvVars::CDK_ANSIBLE_LAST_TAG,
        latest_tag.name
    );
    let distance = git_tag_distance(&latest_tag.name)?;
    println!(
        "cargo:rustc-env={}={}",
        EnvVars::CDK_ANSIBLE_LAST_TAG_DISTANCE,
        distance,
    );
    println!(
        "cargo:rustc-env={}={}",
        EnvVars::CDK_ANSIBLE_LAST_TAG_DISTANCE_DIRTY,
        if git_is_dirty()? { "1" } else { "0" }
    );

    Ok(())
}

struct LatestTag {
    /// tag name
    name: String,
    /// semantic version
    semver: SemVer,
}

/// Get the latest tag from the git repository
fn git_latest_tag() -> Result<LatestTag> {
    // git tag --sort=-committerdate
    let output = Command::new("git")
        .arg("tag")
        .arg("--sort=-committerdate")
        .output()
        .context("failed to run git tag --sort=-committerdate")?;
    let stdout = String::from_utf8(output.stdout).context("failed to read git tag output")?;
    let re = Regex::new(&format!(
        "^{}-v{}$",
        TAG_PREFIX, r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)"
    ))
    .context("failed to create regex")?;
    let tag_cap = stdout
        .lines()
        .find_map(|line| re.captures(line))
        .context("no tag found")?;
    let semver = SemVer {
        major: tag_cap
            .name("major")
            .context("failed to get major")?
            .as_str()
            .parse()
            .context("failed to parse major")?,
        minor: tag_cap
            .name("minor")
            .context("failed to get minor")?
            .as_str()
            .parse()
            .context("failed to parse minor")?,
        patch: tag_cap
            .name("patch")
            .context("failed to get patch")?
            .as_str()
            .parse()
            .context("failed to parse patch")?,
    };
    let tag_name = format!("{TAG_PREFIX}-v{semver}");
    Ok(LatestTag {
        name: tag_name,
        semver,
    })
}

/// Get the distance between the current HEAD and the given tag
fn git_tag_distance(tag_name: &str) -> Result<u32> {
    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(format!("{tag_name}..HEAD"))
        .output()
        .context("failed to run git rev-list --count")?;
    let stdout = String::from_utf8(output.stdout).context("failed to read git rev-list output")?;
    let distance = stdout
        .trim()
        .parse()
        .context("failed to parse git rev-list output")?;
    Ok(distance)
}

// dirty check
fn git_is_dirty() -> Result<bool> {
    // git diff-index --quiet HEAD
    // exit code 1 if dirty
    let status = Command::new("git")
        .arg("diff-index")
        .arg("--quiet")
        .arg("HEAD")
        .status()
        .context("failed to run git diff-index --quiet HEAD")?;
    Ok(!status.success())
}

fn git_head(git_dir: &Path) -> Option<PathBuf> {
    let git_head_path = git_dir.join("HEAD");
    if git_head_path.exists() {
        return Some(git_head_path);
    }
    if !git_dir.is_file() {
        return None;
    }
    let contents = fs::read_to_string(git_dir).ok()?;
    let (label, worktree_path) = contents.split_once(':')?;
    if label != "gitdir" {
        return None;
    }
    let worktree_path = worktree_path.trim();
    Some(PathBuf::from(worktree_path))
}

/// Semantic versioning
struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
