#![allow(clippy::expect_used, reason = "fix later")]
#![allow(clippy::unwrap_used, reason = "fix later")]
#![allow(clippy::single_call_fn, reason = "fix later")]
#![allow(clippy::absolute_paths, reason = "fix later")]

use cdk_ansible_static::EnvVars;
use fs_err as fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let workspace_root = Path::new(&std::env::var(EnvVars::CARGO_MANIFEST_DIR).unwrap())
        .parent()
        .expect("CARGO_MANIFEST_DIR should be nested in workspace")
        .parent()
        .expect("CARGO_MANIFEST_DIR should be doubly nested in workspace")
        .to_path_buf();

    commit_info(&workspace_root);
}

fn commit_info(workspace_root: &Path) {
    // If not in a git repository, do not attempt to retrieve commit information
    let git_dir = workspace_root.join(".git");
    if !git_dir.exists() {
        return;
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

    let output = match Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--date=short")
        .arg("--abbrev=9")
        .arg("--format=%H %h %cd")
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return,
    };
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

    if let Ok(output) = Command::new("git")
        .arg("describe")
        .arg("--tags")
        .arg("HEAD")
        .output()
    {
        let describe = String::from_utf8(output.stdout).unwrap();
        let mut describe_parts = describe.split('-');
        println!(
            "cargo:rustc-env={}={}",
            EnvVars::CDK_ANSIBLE_LAST_TAG,
            describe_parts.next().unwrap()
        );
        // If this is the tagged commit, this component will be missing
        println!(
            "cargo:rustc-env={}={}",
            EnvVars::CDK_ANSIBLE_LAST_TAG_DISTANCE,
            describe_parts.next().unwrap_or("0")
        );
    }
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
