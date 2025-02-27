use anyhow::Result;
use cdk_ansible::{OptU, Play, PlayOptions, Playbook, Task, TaskOptions};

use crate::playbooks::PlaybookGenArgs;

#[expect(clippy::unnecessary_wraps, reason = "use anyhow::Result interface")]
pub fn playbook1<T: PlaybookGenArgs>(args: &T) -> Result<Playbook> {
    Ok(Playbook {
        // Save as `playbook1.json`
        name: "playbook1".to_owned(),
        plays: vec![Play {
            name: "Debug".to_owned(),
            hosts: vec![args.get_hosts().host_a.fqdn.clone()],
            tasks: vec![
                Task {
                    name: "Debug msg".to_owned(),
                    options: TaskOptions::default(),
                    command: Box::new(cdkam_ansible::builtin::debug::Module {
                        module: cdkam_ansible::builtin::debug::Args {
                            options: cdkam_ansible::builtin::debug::Opt {
                                msg: OptU::Some("msg".to_owned()),
                                ..Default::default()
                            },
                        },
                    }),
                },
                Task {
                    name: "No options' module".to_owned(),
                    options: TaskOptions::default(),
                    command: Box::new(cdkam_ansible::builtin::service_facts::Module {
                        module: cdkam_ansible::builtin::service_facts::Args {
                            options: cdkam_ansible::builtin::service_facts::Opt::default(),
                        },
                    }),
                },
            ],
            options: PlayOptions::default(),
        }],
    })
}
