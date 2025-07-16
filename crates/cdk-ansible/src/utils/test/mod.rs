//! Utility for testing

use crate::{OptU, Play, PlayOptions, Task, TaskOptions};

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

pub mod debug {
    use crate::{OptU, PlayOptions, Task, TaskModule, TaskOptions};
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
