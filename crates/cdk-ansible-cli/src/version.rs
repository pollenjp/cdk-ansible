use core::fmt;
use serde::Serialize;

#[derive(Serialize)]
/// Information about the git commit we may have been built from.
pub struct CommitInfo {
    /// The short commit hash
    short_commit_hash: String,
    /// The commit hash
    commit_hash: String,
    /// The commit date
    commit_date: String,
    /// The last tag
    last_tag: Option<String>,
    /// The number of commits since the last tag
    commits_since_last_tag: u32,
    /// Whether the build time repo is dirty
    is_dirty: bool,
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

        if let Some(ci) = self.commit_info.as_ref() {
            if ci.commits_since_last_tag > 0 {
                write!(f, "+{}", ci.commits_since_last_tag)?;
            }
            if ci.is_dirty {
                write!(f, "-dirty")?;
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
#[expect(clippy::single_call_fn, reason = "better readability")]
const fn pkg_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns version information.
pub fn version() -> VersionInfo {
    let version = pkg_version().to_owned();

    // Commit info is pulled from git and set by `build.rs`
    let commit_info = match (
        option_env!("CDK_ANSIBLE_COMMIT_HASH"),
        option_env!("CDK_ANSIBLE_COMMIT_SHORT_HASH"),
        option_env!("CDK_ANSIBLE_COMMIT_DATE"),
        option_env!("CDK_ANSIBLE_LAST_TAG"),
        option_env!("CDK_ANSIBLE_LAST_TAG_DISTANCE"),
        option_env!("CDK_ANSIBLE_LAST_TAG_DISTANCE_DIRTY"),
    ) {
        (
            Some(commit_hash),
            Some(short_commit_hash),
            Some(commit_date),
            Some(last_tag),
            commits_since_last_tag,
            is_dirty,
        ) => Some(CommitInfo {
            short_commit_hash: short_commit_hash.to_owned(),
            commit_hash: commit_hash.to_owned(),
            commit_date: commit_date.to_owned(),
            last_tag: Some(last_tag.to_owned()),
            commits_since_last_tag: commits_since_last_tag
                .map_or(0, |distance| distance.parse::<u32>().unwrap_or(0)),
            is_dirty: is_dirty == Some("1"),
        }),
        _ => None,
    };

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
        let v_ = version();
        assert_eq!(v_.version, env!("CARGO_PKG_VERSION").to_owned());
    }
}
