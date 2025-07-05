use ::cdk_ansible::{
    DeployStack, ExeParallel, ExePlay, ExeSequential, ExeSingle, OptU, Play, PlayOptions, Task,
    TaskOptions,
};
use indexmap::IndexMap;

use crate::HostPool;

pub struct SampleStack {
    exe_play: ExePlay,
}

impl SampleStack {
    pub fn new(hp: &HostPool) -> Self {
        let h = hp.localhost.name.as_str();

        Self {
            exe_play: ExeSequential(vec![ExeParallel(vec![ExeSingle(Box::new(Play {
                name: "Debug".to_owned(),
                hosts: vec![h.to_owned()].into(),
                options: PlayOptions {
                    vars: OptU::Some(IndexMap::<String, serde_json::Value>::from([(
                        "var_from_play_option".to_owned(),
                        serde_json::Value::String("VAR_FROM_PLAY_OPTION".to_owned()),
                    )])),
                    ..Default::default()
                },
                tasks: vec![Task {
                    name: "Debug msg".to_owned(),
                    options: TaskOptions::default(),
                    command: Box::new(cdkam::ansible::builtin::debug::Module {
                        module: cdkam::ansible::builtin::debug::Args {
                            options: cdkam::ansible::builtin::debug::Opt {
                                msg: OptU::Some("msg".to_owned()),
                                ..Default::default()
                            },
                        },
                    }),
                }],
            }))])]),
        }
    }
}

impl DeployStack for SampleStack {
    #[expect(clippy::expect_used, reason = "Logical failure")]
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .expect("Failed to get a stack name")
    }

    fn exe_play(&self) -> &ExePlay {
        &self.exe_play
    }
}
