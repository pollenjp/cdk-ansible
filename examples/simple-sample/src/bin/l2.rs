use anyhow::Result;
use cdk_ansible::{
    AppL2, ExePlayL2, HostInventoryVars, HostInventoryVarsGenerator, HostsL2, LEP, LazyPlayL2,
    OptU, PlayL2, PlayOptions, StackL2, Task, TaskOptions, prelude::*,
};
use chrono::{DateTime, Utc};
use futures::future::{BoxFuture, FutureExt as _};
use std::sync::Arc;

fn main() {
    if let Err(e) = main2() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    AppL2::new(std::env::args().collect())
        .stack(Arc::new(SampleStack::new()))
        .expect("Failed to add sample stack")
        .run()
}

struct SampleStack {
    exe_play: LEP,
}

impl SampleStack {
    fn new() -> Self {
        Self {
            exe_play: LEP::Sequential(vec![
                LEP::Single(Arc::new(Sample1LazyPlayL2Helper::new("sample1"))),
                LEP::Single(Arc::new(Sample2LazyPlayL2Helper::new("sample2"))),
            ]),
        }
    }
}

impl StackL2 for SampleStack {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .expect("Failed to get a stack name")
    }
    fn exe_play(&self) -> &LEP {
        &self.exe_play
    }
}

struct Sample1LazyPlayL2Helper {
    name: String,
}

impl Sample1LazyPlayL2Helper {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl LazyPlayL2 for Sample1LazyPlayL2Helper {
    fn lazy_play_l2(&self) -> BoxFuture<'static, Result<ExePlayL2>> {
        let name = self.name.clone();
        async move {
            let hp = HostPool::new(); // Each hosts are instantiated here!!
            Ok(vec![
                PlayL2 {
                    name: name.clone(),
                    hosts: HostsL2::new(vec![Arc::clone(&hp.localhost) as _]),
                    options: PlayOptions::default(),
                    tasks: create_tasks_helper(Arc::clone(&hp.localhost) as _, 2)?,
                }
                .into(),
                ExePlayL2::Sequential(vec![
                    PlayL2 {
                        name: name.clone(),
                        hosts: HostsL2::new(vec![Arc::clone(&hp.host_a) as _]),
                        options: PlayOptions::default(),
                        tasks: create_tasks_helper(Arc::clone(&hp.host_a) as _, 2)?,
                    }
                    .into(),
                    PlayL2 {
                        name: name.clone(),
                        hosts: HostsL2::new(vec![Arc::clone(&hp.host_a) as _]),
                        options: PlayOptions::default(),
                        tasks: create_tasks_helper(Arc::clone(&hp.host_a) as _, 2)?,
                    }
                    .into(),
                ]),
            ]
            .into_exe_play_l2_parallel())
        }
        .boxed()
    }
}

struct Sample2LazyPlayL2Helper {
    name: String,
}

impl Sample2LazyPlayL2Helper {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl LazyPlayL2 for Sample2LazyPlayL2Helper {
    fn lazy_play_l2(&self) -> BoxFuture<'static, Result<ExePlayL2>> {
        let name = self.name.clone();
        async move {
            let hp = HostPool::new(); // Each hosts are instantiated here!!
            Ok(vec![
                PlayL2 {
                    name: name.clone(),
                    hosts: HostsL2::new(vec![Arc::clone(&hp.localhost) as _]),
                    options: PlayOptions::default(),
                    tasks: create_tasks_helper(Arc::clone(&hp.localhost) as _, 2)?,
                }
                .into(),
                ExePlayL2::Sequential(vec![
                    PlayL2 {
                        name: name.clone(),
                        hosts: HostsL2::new(vec![Arc::clone(&hp.host_a) as _]),
                        options: PlayOptions::default(),
                        tasks: create_tasks_helper(Arc::clone(&hp.host_a) as _, 2)?,
                    }
                    .into(),
                    PlayL2 {
                        name: name.clone(),
                        hosts: HostsL2::new(vec![Arc::clone(&hp.host_a) as _]),
                        options: PlayOptions::default(),
                        tasks: create_tasks_helper(Arc::clone(&hp.host_a) as _, 2)?,
                    }
                    .into(),
                ]),
            ]
            .into_exe_play_l2_parallel())
        }
        .boxed()
    }
}

pub static VAR_NAME_INSTANTIATED_AT: &str = "var_instantiated_at";

pub struct HostPool {
    pub localhost: Arc<LocalHost>,
    pub host_a: Arc<HostA>,
}

impl HostPool {
    pub fn new() -> Self {
        Self {
            localhost: Arc::new(LocalHost::new()),
            host_a: Arc::new(HostA::new()),
        }
    }
}

pub struct LocalHost {
    common_field: CommonField,
}

impl LocalHost {
    pub fn new() -> Self {
        Self {
            common_field: CommonField {
                name: "localhost".into(),
                instantiated_at: Utc::now(),
            },
        }
    }
}

impl Host for LocalHost {
    fn common_field(&self) -> &CommonField {
        &self.common_field
    }
}

impl HostInventoryVarsGenerator for LocalHost {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.common_field.name.clone(),
            inventory_vars: vec![
                ("ansible_connection".to_string(), "local".into()),
                (
                    VAR_NAME_INSTANTIATED_AT.to_string(),
                    self.common_field.instantiated_at.to_rfc3339().into(),
                ),
            ],
        })
    }
}

pub struct HostA {
    common_field: CommonField,
}

impl HostA {
    pub fn new() -> Self {
        Self {
            common_field: CommonField {
                name: "host_a".into(),
                instantiated_at: Utc::now(),
            },
        }
    }
}

impl Host for HostA {
    fn common_field(&self) -> &CommonField {
        &self.common_field
    }
}

impl HostInventoryVarsGenerator for HostA {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.common_field.name.clone(),
            inventory_vars: vec![
                // If you want to connect to a remote host, you can change "local" to "ssh" or remove this line.
                ("ansible_connection".to_string(), "local".into()),
                (
                    VAR_NAME_INSTANTIATED_AT.to_string(),
                    self.common_field.instantiated_at.to_rfc3339().into(),
                ),
            ],
        })
    }
}

pub struct CommonField {
    pub name: String,
    pub instantiated_at: DateTime<Utc>,
}

pub trait Host: Send + Sync {
    fn common_field(&self) -> &CommonField;
}

fn create_tasks_helper(h: Arc<dyn Host>, n: usize) -> Result<Vec<Task>> {
    let mut tasks = vec![::cdk_ansible::Task {
        name: "debug".into(),
        options: TaskOptions::default(),
        command: Box::new(::sample_cdkam_ansible::builtin::debug::Module {
            module: ::sample_cdkam_ansible::builtin::debug::Args {
                options: ::sample_cdkam_ansible::builtin::debug::Opt {
                    msg: OptU::Some(format!(
                        "Hello '{}'! Instantiated at '{{{{ {} | default('N/A') }}}}'",
                        h.common_field().name.clone(),
                        VAR_NAME_INSTANTIATED_AT
                    )),
                    ..Default::default()
                },
            },
        }),
    }];

    // Don't sleep in CI
    if std::env::var("CI_JOB").is_err() {
        tasks.extend((0..n).map(|_| ::cdk_ansible::Task {
            name: "sleep".into(),
            options: TaskOptions {
                changed_when: OptU::Some(false.into()),
                ..Default::default()
            },
            command: Box::new(::sample_cdkam_ansible::builtin::command::Module {
                module: ::sample_cdkam_ansible::builtin::command::Args {
                    options: ::sample_cdkam_ansible::builtin::command::Opt {
                        cmd: OptU::Some("sleep 3".into()),
                        ..Default::default()
                    },
                },
            }),
        }));
    }

    Ok(tasks)
}
