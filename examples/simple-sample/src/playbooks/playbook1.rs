use anyhow::Result;
use cdk_ansible::{OptionUnset, Play, PlayOptions, Playbook, Task, TaskOptions};

use crate::playbooks::PlaybookGenArgs;

pub fn playbook1(args: &impl PlaybookGenArgs) -> Result<Playbook> {
    Ok(Playbook {
        // Save as `playbook1.json`
        name: "playbook1".to_string(),
        plays: vec![Play {
            name: "Debug".to_string(),
            hosts: vec![args.get_hosts().host_a.fqdn.clone()],
            tasks: vec![Task {
                name: "Debug msg".to_string(),
                options: TaskOptions {
                    ..Default::default()
                },
                command: Box::new(cdkam_ansible::builtin::debug::Module {
                    module: cdkam_ansible::builtin::debug::Args {
                        options: cdkam_ansible::builtin::debug::Opt {
                            msg: OptionUnset::Some("msg".to_string()),
                            ..Default::default()
                        },
                    },
                }),
            }],
            options: PlayOptions::default(),
        }],
    })
}
