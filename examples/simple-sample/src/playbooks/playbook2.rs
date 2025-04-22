use crate::playbooks::PlaybookGenArgs;
use anyhow::Result;
use cdk_ansible::{OptU, Play, PlayOptions, Playbook, Task, TaskOptions};

#[expect(clippy::unnecessary_wraps, reason = "use anyhow::Result interface")]
pub fn playbook2<T: PlaybookGenArgs>(args: &T) -> Result<Playbook> {
    Ok(Playbook {
        // Save as `playbook2.json`
        name: "playbook2".to_owned(),
        plays: vec![Play {
            name: "Debug".to_owned(),
            hosts: vec![args.get_hosts().host_a.fqdn.clone()].into(),
            tasks: vec![Task {
                name: "Debug msg".to_owned(),
                options: TaskOptions::default(),
                command: Box::new(sample_cdkam_ansible::builtin::debug::Module {
                    module: sample_cdkam_ansible::builtin::debug::Args {
                        options: sample_cdkam_ansible::builtin::debug::Opt {
                            msg: OptU::Some("msg".to_owned()),
                            ..Default::default()
                        },
                    },
                }),
            }],
            options: PlayOptions::default(),
        }],
    })
}
