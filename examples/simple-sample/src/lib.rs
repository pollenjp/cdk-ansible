use anyhow::Result;
use cdk_ansible::{
    DeployApp, DeployStack, ExPlay, ExSequential, ExSingle, OptU, Play, PlayOptions, TaskOptions,
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
        ]))
    }
}

fn create_play_helper(name: &str) -> Box<Play> {
    Box::new(Play {
        name: name.into(),
        hosts: "localhost".into(),
        options: PlayOptions::default(),
        tasks: vec![::cdk_ansible::Task {
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
        }],
    })
}
