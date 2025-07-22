use anyhow::{Context as _, Result};
use cdk_ansible_core::core::Playbook;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::process::Command;

#[cfg(test)]
pub mod test;

pub async fn dump_json(filepath: PathBuf, obj: impl Serialize) -> Result<()> {
    fs::create_dir_all(
        filepath
            .parent()
            .with_context(|| format!("getting parent directory of {}", filepath.display()))?,
    )
    .await?;
    fs::write(&filepath, serde_json::to_string_pretty(&obj)?)
        .await
        .with_context(|| format!("writing to {}", filepath.display()))?;
    Ok(())
}

/// @deprecated
/// Use [`dump_json`] instead.
pub async fn playbook_dump(playbook: Playbook, dirpath: Arc<PathBuf>) -> Result<()> {
    let filepath = dirpath.join(format!("{}.json", playbook.name));
    fs::create_dir_all(
        filepath
            .parent()
            .with_context(|| format!("getting parent directory of {}", filepath.display()))?,
    )
    .await
    .with_context(|| format!("creating directory {}", filepath.display()))?;

    fs::write(&filepath, serde_json::to_string_pretty(&playbook.plays)?)
        .await
        .with_context(|| format!("writing to {}", filepath.display()))?;
    Ok(())
}

/// ```sh
/// yq_script='yq -p json -o yaml "${1}" > "${1%.json}.yaml"'
/// ```
pub async fn json_to_yaml(json_path: PathBuf) -> Result<()> {
    let yaml_path = json_path.with_extension("yaml");
    let output = Command::new("yq")
        .arg("-p")
        .arg("json")
        .arg("-o")
        .arg("yaml")
        .arg(json_path)
        .output();
    let output = output.await?;
    if !output.status.success() {
        anyhow::bail!(
            "command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fs::write(&yaml_path, output.stdout)
        .await
        .with_context(|| format!("writing to {}", yaml_path.display()))?;
    Ok(())
}
