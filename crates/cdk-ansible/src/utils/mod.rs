use anyhow::{Context as _, Result};
use cdk_ansible_core::core::Playbook;
use std::path::Path;
use tokio::fs;

pub async fn playbook_dump(playbook: &Playbook, dirpath: &Path) -> Result<()> {
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
