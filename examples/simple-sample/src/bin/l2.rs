use anyhow::Result;
use cdk_ansible::{
    AppL2, ExePlayL2, HostInventoryVars, HostInventoryVarsGenerator, HostsL2, LazyPlayL2, PlayL2,
    PlayOptions, StackL2,
};
use futures::future::{BoxFuture, FutureExt as _};
use simple_sample::create_tasks_helper;
use std::sync::Arc;

pub fn main() {
    if let Err(e) = main2() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

pub fn main2() -> Result<()> {
    AppL2::new(std::env::args().collect())
        .stack(Arc::new(SampleStack::new()))
        .expect("Failed to add sample stack")
        .run()
}

struct HostA {
    name: String,
}

impl HostInventoryVarsGenerator for HostA {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.name.clone(),
            inventory_vars: vec![("ansible_connection".to_string(), "local".into())],
        })
    }
}

struct SampleLazyPlayL2Helper {
    name: String,
}

impl SampleLazyPlayL2Helper {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl LazyPlayL2 for SampleLazyPlayL2Helper {
    fn create_play_l2(&self) -> BoxFuture<'static, Result<PlayL2>> {
        let name = self.name.clone();
        async move {
            Ok(PlayL2 {
                name,
                hosts: HostsL2::new(vec![Arc::new(HostA {
                    name: "localhost".to_string(),
                })]),
                options: PlayOptions::default(),
                tasks: create_tasks_helper(2)?,
            })
        }
        .boxed()
    }
}

struct SampleStack {
    exe_play: ExePlayL2,
}

impl SampleStack {
    fn new() -> Self {
        Self {
            exe_play: ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample"))),
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
    fn exe_play(&self) -> &ExePlayL2 {
        &self.exe_play
    }
}
