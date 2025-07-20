//! Utility for testing
use crate::{
    HostInventoryVars, HostInventoryVarsGenerator, OptU, Play, PlayOptions, Task, TaskOptions,
    l2::types::{HostsL2, LazyPlayL2, PlayL2},
};
use anyhow::Result;
use futures::future::{BoxFuture, FutureExt as _};
use std::sync::Arc;

/// Helper function to create sample play
pub fn create_play_helper(name: &str) -> Play {
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

pub fn create_play_l2_helper(name: &str) -> PlayL2 {
    struct HostA {
        name: String,
    }
    impl HostInventoryVarsGenerator for HostA {
        fn gen_host_vars(&self) -> Result<HostInventoryVars> {
            Ok(HostInventoryVars {
                ansible_host: self.name.clone(),
                inventory_vars: vec![],
            })
        }
    }

    struct HostB {
        name: String,
    }
    impl HostInventoryVarsGenerator for HostB {
        fn gen_host_vars(&self) -> Result<HostInventoryVars> {
            Ok(HostInventoryVars {
                ansible_host: self.name.clone(),
                inventory_vars: vec![],
            })
        }
    }

    let hosts = HostsL2::new(vec![
        Arc::new(HostA {
            name: "host_a".to_string(),
        }),
        Arc::new(HostB {
            name: "host_b".to_string(),
        }),
    ]);
    PlayL2 {
        name: name.to_string(),
        hosts,
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

pub struct SampleLazyPlayL2Helper {
    name: String,
}

impl SampleLazyPlayL2Helper {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

impl LazyPlayL2 for SampleLazyPlayL2Helper {
    fn create_play_l2(&self) -> BoxFuture<'static, Result<PlayL2>> {
        let name = self.name.to_owned();
        async move { Ok(create_play_l2_helper(&name)) }.boxed()
    }
}

pub mod debug {
    use crate::{OptU, TaskModule};
    use serde::Serialize;

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
