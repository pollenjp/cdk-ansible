use cdk_ansible_static::*;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub(crate) struct CommitInfo {
    short_commit_hash: String,
    commit_hash: String,
    commit_date: String,
    last_tag: Option<String>,
    commits_since_last_tag: u32,
}

#[derive(Serialize)]
pub struct VersionInfo {
    /// version, such as "1.2.3"
    version: String,
    /// Information about the git commit we may have been built from.
    ///
    /// `None` if not built from a git repo or if retrieval failed.
    commit_info: Option<CommitInfo>,
}

impl fmt::Display for VersionInfo {
    /// Formatted version information: "<version>[+<commits>] (<commit> <date>)"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)?;

        if let Some(ref ci) = self.commit_info {
            if ci.commits_since_last_tag > 0 {
                write!(f, "+{}", ci.commits_since_last_tag)?;
            }
            write!(f, " ({} {})", ci.short_commit_hash, ci.commit_date)?;
        }

        Ok(())
    }
}

/// Return the application version.
///
/// note: this function returns the version of `cdk-ansible-cli` crate
///       and need to be the same as the version of `cdk-ansible` crate.
pub fn pkg_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns version information.
pub(crate) fn version() -> VersionInfo {
    let version = pkg_version().to_string();

    // Commit info is pulled from git and set by `build.rs`
    let commit_info = CDK_ANSIBLE_COMMIT_HASH.map(|commit_hash| CommitInfo {
        short_commit_hash: CDK_ANSIBLE_COMMIT_SHORT_HASH.unwrap().to_string(),
        commit_hash: commit_hash.to_string(),
        commit_date: CDK_ANSIBLE_COMMIT_DATE.unwrap().to_string(),
        last_tag: CDK_ANSIBLE_LAST_TAG.map(|tag| tag.to_string()),
        commits_since_last_tag: CDK_ANSIBLE_LAST_TAG_DISTANCE
            .map_or(0, |distance| distance.parse::<u32>().unwrap_or(0)),
    });

    VersionInfo {
        version,
        commit_info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        assert_eq!(version().to_string(), env!("CARGO_PKG_VERSION").to_string());
    }
}
