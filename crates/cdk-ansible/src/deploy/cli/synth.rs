use crate::{
    DeployApp, Playbook,
    deploy::{ExePlaybook, cli::GlobalConfig},
    utils::{dump_json, json_to_yaml, playbook_dump},
};
use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::task::JoinSet;

#[derive(Args, Debug, Clone)]
pub struct Synth {}

impl Synth {
    pub async fn run(self, app: &DeployApp, global_config: Arc<GlobalConfig>) -> Result<()> {
        synth(app, &global_config).await?;
        Ok(())
    }
}

pub async fn synth(app: &DeployApp, global_config: &Arc<GlobalConfig>) -> Result<()> {
    let (inv_res, pb_res) = tokio::join!(
        synth_inventory(app, global_config),
        synth_playbooks(app, global_config),
    );
    inv_res?;
    pb_res?;
    Ok(())
}

pub async fn synth_inventory(app: &DeployApp, global_config: &GlobalConfig) -> Result<()> {
    // Reset inventory directory
    if global_config.inventory_dir.exists() {
        tokio::fs::remove_dir_all(&global_config.inventory_dir).await?;
    }

    let mut join_set: JoinSet<Result<()>> = JoinSet::new();
    // Create inventory file
    for (_, inventory) in app.inventories.iter() {
        let inventory_path = global_config
            .inventory_dir
            .join(format!("{}.json", inventory.name));
        let inv_root = inventory.root.clone();
        join_set.spawn(async move {
            dump_json(inventory_path.clone(), inv_root).await?;
            json_to_yaml(inventory_path).await?;
            Ok(())
        });
    }
    while let Some(res) = join_set.join_next().await {
        (res?)?;
    }

    Ok(())
}

pub async fn synth_playbooks(app: &DeployApp, global_config: &GlobalConfig) -> Result<()> {
    // Reset playbook directory
    if global_config.playbook_dir.exists() {
        tokio::fs::remove_dir_all(&global_config.playbook_dir).await?;
    }

    let playbook_dir = Arc::new(global_config.playbook_dir.clone());
    let mut join_set: JoinSet<Result<()>> = JoinSet::new();

    app.exe_playbooks()
        .iter()
        .map(|(_, exe_playbook)| {
            let mut container: Vec<Playbook> = Vec::new();
            recursive_synth(&mut container, exe_playbook.clone());
            container
        })
        .for_each(|container| {
            container.into_iter().for_each(|pb| {
                join_set.spawn(synth_playbook(pb, Arc::clone(&playbook_dir)));
            });
        });
    while let Some(res) = join_set.join_next().await {
        (res?)?;
    }
    Ok(())
}

async fn synth_playbook(pb: Playbook, playbook_dir: Arc<PathBuf>) -> Result<()> {
    let json_path = playbook_dir.join(format!("{}.json", pb.name));
    playbook_dump(pb, playbook_dir).await?;
    json_to_yaml(json_path).await?;
    Ok(())
}

/// Extract Playbooks from ExePlaybook
fn recursive_synth(container: &mut Vec<Playbook>, exe_playbook: ExePlaybook) {
    match exe_playbook {
        ExePlaybook::Single(pb) => {
            container.push(*pb);
        }
        ExePlaybook::Sequential(pbs) | ExePlaybook::Parallel(pbs) => {
            for pb in pbs {
                recursive_synth(container, pb);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OptU;
    use cdk_ansible_core::core::{Play, PlayOptions, Task, TaskModule, TaskOptions};
    use serde::Serialize;
    use tempfile::TempDir;

    mod debug {
        use super::*;

        #[derive(Clone, Debug, Serialize)]
        pub struct Module {
            #[serde(rename = "ansible.builtin.debug")]
            pub module: Args,
        }
        impl TaskModule for Module {}
        #[derive(Clone, Debug, Serialize)]
        pub struct Args {
            #[serde(flatten)]
            pub options: Opt,
        }
        #[derive(Clone, Debug, Default, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub struct Opt {
            #[serde(
                default = "OptU::default",
                skip_serializing_if = "OptU::is_unset",
                rename = "msg"
            )]
            pub msg: OptU<String>,
            #[serde(
                default = "OptU::default",
                skip_serializing_if = "OptU::is_unset",
                rename = "var"
            )]
            pub var: OptU<String>,
            #[serde(
                default = "OptU::default",
                skip_serializing_if = "OptU::is_unset",
                rename = "verbosity"
            )]
            pub verbosity: OptU<crate::IntOrString>,
        }
    }

    fn create_play_helper(name: &str) -> Play {
        Play {
            name: name.to_string(),
            hosts: "localhost".into(),
            options: PlayOptions::default(),
            tasks: vec![Task {
                name: "debug".into(),
                options: TaskOptions::default(),
                command: Box::new(debug::Module {
                    module: debug::Args {
                        options: debug::Opt {
                            msg: OptU::Some("Hello, world!".into()),
                            ..Default::default()
                        },
                    },
                }),
            }],
        }
    }

    fn create_playbook_helper(name: &str) -> Box<Playbook> {
        Box::new(Playbook {
            name: name.to_string(),
            plays: vec![create_play_helper(name)],
        })
    }

    #[tokio::test]
    async fn validate_synth_playbook() {
        let temp_dir = TempDir::new().unwrap();
        let playbook_dir = Arc::new(temp_dir.path().to_path_buf());
        let pb = create_playbook_helper("test");
        synth_playbook(*pb, playbook_dir).await.unwrap();
    }
}
