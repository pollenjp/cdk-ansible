use ::anyhow::Result;
use ::cdk_ansible::{OptU, Play, PlayOptions, StringOrVecString, Task, TaskOptions};

pub fn create_tasks_helper(n: usize) -> Result<Vec<Task>> {
    let mut tasks = vec![::cdk_ansible::Task {
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
    }];

    // Don't sleep in CI
    if std::env::var("CI_JOB").is_err() {
        tasks.extend((0..n).map(|_| ::cdk_ansible::Task {
            name: "sleep 2 seconds".into(),
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

    // tasks.push(::cdk_ansible::Task {
    //     name: "interrupt play".into(),
    //     options: TaskOptions {
    //         changed_when: OptU::Some(false.into()),
    //         ..Default::default()
    //     },
    //     command: Box::new(::sample_cdkam_ansible::builtin::shell::Module {
    //         module: ::sample_cdkam_ansible::builtin::shell::Args {
    //             options: ::sample_cdkam_ansible::builtin::shell::Opt {
    //                 cmd: OptU::Some("exit 1".into()),
    //                 ..Default::default()
    //             },
    //         },
    //     }),
    // });
    Ok(tasks)
}

pub fn create_play_helper(name: &str, hosts: StringOrVecString, n: usize) -> Result<Play> {
    Ok(Play {
        name: name.into(),
        hosts,
        options: PlayOptions::default(),
        tasks: create_tasks_helper(n)?,
    })
}
