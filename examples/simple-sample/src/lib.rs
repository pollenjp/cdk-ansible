use anyhow::Result;
use cdk_ansible::{
    DeployApp, DeployStack, ExParallel, ExPlay, ExSequential, ExSingle, OptU, Play, PlayOptions,
    TaskOptions,
};

// mod inventory;
// mod playbooks;

// use inventory::get_hosts;
// use playbooks::generate_all;

#[inline]
pub fn main() {
    if let Err(e) = main2() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

pub fn main2() -> Result<()> {
    let mut app = DeployApp::new(std::env::args().collect());
    app.add_stack(Box::new(SampleStack {}))?;
    app.run()
}

struct SampleStack;

impl DeployStack for SampleStack {
    fn name(&self) -> &str {
        "sample"
    }

    fn plays(&self) -> Result<ExPlay> {
        Ok(ExSequential(vec![
            ExSingle(create_play_helper("sample1")),
            ExSingle(create_play_helper("sample2")),
            ExParallel(vec![
                ExSequential(vec![
                    ExSingle(create_play_helper("sample3")),
                    ExSingle(create_play_helper("sample4")),
                    ExSingle(create_play_helper("sample5")),
                ]),
                ExSingle(create_play_helper("sample6")),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample7")),
                    ExSingle(create_play_helper("sample8")),
                    ExSingle(create_play_helper("sample9")),
                ]),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample10")),
                    ExSingle(create_play_helper("sample11")),
                    ExSingle(create_play_helper("sample12")),
                ]),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample13")),
                    ExSingle(create_play_helper("sample14")),
                    ExSingle(create_play_helper("sample15")),
                ]),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample16")),
                    ExSingle(create_play_helper("sample17")),
                    ExSingle(create_play_helper("sample18")),
                ]),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample19")),
                    ExSingle(create_play_helper("sample20")),
                    ExSingle(create_play_helper("sample21")),
                ]),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample22")),
                    ExSingle(create_play_helper("sample23")),
                    ExSingle(create_play_helper("sample24")),
                ]),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample25")),
                    ExSingle(create_play_helper("sample26")),
                    ExSingle(create_play_helper("sample27")),
                ]),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample28")),
                    ExSingle(create_play_helper("sample29")),
                    ExSingle(create_play_helper("sample30")),
                ]),
                ExSequential(vec![
                    ExSingle(create_play_helper("sample31")),
                    ExSingle(create_play_helper("sample32")),
                    ExSingle(create_play_helper("sample33")),
                ]),
            ]),
        ]))
    }
}

fn create_play_helper(name: &str) -> Box<Play> {
    Box::new(Play {
        name: name.into(),
        hosts: "localhost".into(),
        options: PlayOptions::default(),
        tasks: vec![
            ::cdk_ansible::Task {
                name: "debug".into(),
                options: TaskOptions::default(),
                command: Box::new(::sample_cdkam_ansible::builtin::debug::Module {
                    module: ::sample_cdkam_ansible::builtin::debug::Args {
                        options: ::sample_cdkam_ansible::builtin::debug::Opt {
                            msg: OptU::Some("Hello, world!".into()),
                            ..Default::default()
                        },
                    },
                }),
            },
            ::cdk_ansible::Task {
                name: "sleep 10 seconds".into(),
                options: TaskOptions {
                    changed_when: OptU::Some(false.into()),
                    ..Default::default()
                },
                command: Box::new(::sample_cdkam_ansible::builtin::shell::Module {
                    module: ::sample_cdkam_ansible::builtin::shell::Args {
                        options: ::sample_cdkam_ansible::builtin::shell::Opt {
                            cmd: OptU::Some("sleep 3".into()),
                            ..Default::default()
                        },
                    },
                }),
            },
        ],
    })
}
