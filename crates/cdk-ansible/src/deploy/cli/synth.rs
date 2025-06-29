use crate::utils::playbook_dump;
use crate::{
    DeployApp, Playbook,
    deploy::{ExPlay, cli::GlobalArgs},
};
use anyhow::Result;
use clap::Args;
use futures::future::{BoxFuture, FutureExt};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::task::JoinSet;

#[derive(Args, Debug, Clone)]
pub struct Synth {}

impl Synth {
    pub async fn run(self, app: &DeployApp, global_args: GlobalArgs) -> Result<()> {
        let playbook_dir = global_args.app_dir.join("playbooks");

        // Reset playbook directory
        tokio::fs::remove_dir_all(&playbook_dir).await?;

        let playbook_dir = Arc::new(playbook_dir);
        for (name, play_ex) in app.stacks.iter() {
            recursive_synth(name.to_owned(), play_ex.clone(), Arc::clone(&playbook_dir)).await?;
            // recursive_synth(play_ex.clone(), &playbook_dir).await?;
        }
        Ok(())
    }
}

fn recursive_synth(
    name: String,
    ex_play: ExPlay,
    playbook_dir: Arc<PathBuf>,
) -> BoxFuture<'static, Result<()>> {
    async move {
        match ex_play {
            ExPlay::Single(play) => {
                playbook_dump(
                    &Playbook {
                        name: format!("{name}_{}", play.name.to_lowercase().replace(' ', "_")),
                        plays: vec![*play],
                    },
                    &playbook_dir,
                )
                .await?;
            }
            ExPlay::Sequential(plays) => {
                for (i, play) in plays.into_iter().enumerate() {
                    recursive_synth(format!("{name}_seq{i}"), play, Arc::clone(&playbook_dir))
                        .await?;
                }
            }
            ExPlay::Parallel(plays) => {
                let mut set: JoinSet<Result<()>> = JoinSet::new();
                for (i, play) in plays.into_iter().enumerate() {
                    set.spawn(recursive_synth(
                        format!("{name}_par{i}"),
                        play,
                        Arc::clone(&playbook_dir),
                    ));
                }
                while let Some(res) = set.join_next().await {
                    (res?)?;
                }
            }
        }
        Ok(())
    }
    .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OptU;
    use cdk_ansible_core::core::{Play, PlayOptions, Task, TaskModule, TaskOptions};
    use serde::Serialize;
    use std::path::PathBuf;

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

    fn create_play_helper(name: &str) -> Box<Play> {
        Box::new(Play {
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
        })
    }

    #[tokio::test]
    async fn test_recursive_synth_single() {
        let play = create_play_helper("test_single");
        let app_dir = PathBuf::from("target/test/synth");
        recursive_synth(
            "SampleStack".to_owned(),
            ExPlay::Single(play),
            Arc::new(app_dir),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_recursive_synth_sequential() {
        let plays = vec![
            ExPlay::Single(create_play_helper("test_seq1")),
            ExPlay::Single(create_play_helper("test_seq2")),
        ];
        let app_dir = PathBuf::from("target/test/synth");
        recursive_synth(
            "SampleStack".to_owned(),
            ExPlay::Sequential(plays),
            Arc::new(app_dir),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_recursive_synth_parallel() {
        let plays = vec![
            ExPlay::Single(create_play_helper("test_par1")),
            ExPlay::Single(create_play_helper("test_par2")),
        ];
        let app_dir = PathBuf::from("target/test/synth");
        recursive_synth(
            "SampleStack".to_owned(),
            ExPlay::Parallel(plays),
            Arc::new(app_dir),
        )
        .await
        .unwrap();
    }
}
