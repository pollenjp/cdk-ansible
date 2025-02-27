use dyn_clone::{DynClone, clone_trait_object};
use erased_serde::serialize_trait_object;
use indexmap::IndexMap;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Inventory {
    pub name: String,
    pub root: InventoryRoot,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryRoot {
    pub all: Child,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Child {
    #[serde(skip_serializing_if = "OptU::is_unset")]
    pub hosts: OptU<IndexMap<String, Option<serde_json::Map<String, serde_json::Value>>>>,
    #[serde(skip_serializing_if = "OptU::is_unset")]
    pub children: OptU<IndexMap<String, Child>>,
    #[serde(skip_serializing_if = "OptU::is_unset")]
    pub vars: OptU<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Clone, Debug)]
pub struct Playbook {
    /// Name of the playbook
    /// The output file name will be `<name>.yaml`
    pub name: String,
    pub plays: Vec<Play>,
}

/// Option for an unset value
///
/// This differs from `Option<T>` in that it has a [`OptU::Unset`], not [`None`]
/// In serializing, [`OptU::Unset`] is skipped, while [`None`] is serialized as `null`.
///
/// ```rust
/// use cdk_ansible_core::core::OptU;
///
/// let x: OptU<i32> = OptU::Unset;
/// ```
#[derive(Clone, Debug, PartialEq, Default, Serialize)]
#[serde(untagged)]
pub enum OptU<T: Serialize> {
    Some(T),
    #[default]
    Unset,
}

/// For skip_serializing_if of serde
impl<T: Serialize> OptU<T> {
    pub fn is_unset(&self) -> bool {
        matches!(self, OptU::Unset)
    }
}

/// Play
/// Optional Values are defined in [`PlayOptions`]
#[derive(Serialize, Clone, Debug)]
pub struct Play {
    /// Identifier. Can be used for documentation, or in tasks/handlers.
    pub name: String,
    /// A list of groups, hosts or host pattern that translates into a list of hosts that are the play's target.
    pub hosts: Vec<String>,
    #[serde(flatten)]
    pub options: PlayOptions,
    /// Main list of tasks to execute in the play, they run after roles and before post_tasks.
    pub tasks: Vec<Task>,
}

/// [playbook keywords (play)](https://docs.ansible.com/ansible/latest/reference_appendices/playbooks_keywords.html#play)
#[derive(Serialize, Default, Clone, Debug)]
pub struct PlayOptions {
    /// The 'action' to execute for a task, it normally translates into a C(module) or action plugin.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub action: OptU<String>,
    /// Force any un-handled task errors on any host to propagate to all hosts and end the play.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub any_errors_fatal: OptU<bool>,
    /// A secondary way to add arguments into a task. Takes a dictionary in which keys map to options and values.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub args: OptU<HashMap<String, serde_json::Value>>,
    /// Run a task asynchronously if the C(action) supports this; the value is the maximum runtime in seconds.
    #[serde(
        rename = "async",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub async_: OptU<i64>,
    /// Boolean that controls if privilege escalation is used or not on Task execution.
    /// Implemented by the become plugin. See Become plugins.
    #[serde(
        rename = "become",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub become_: OptU<bool>,
    /// Path to the executable used to elevate privileges. Implemented by the become plugin. See Become plugins.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub become_exe: OptU<String>,
    /// A string of flag(s) to pass to the privilege escalation program when become is True.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub become_flags: OptU<String>,
    /// Which method of privilege escalation to use (such as sudo or su).
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub become_method: OptU<String>,
    /// User that you 'become' after using privilege escalation. The remote/login user must have permissions to become this user.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub become_user: OptU<String>,
    /// Conditional expression that overrides the task's normal 'changed' status.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub changed_when: OptU<String>,
    /// A boolean that controls if a task is executed in 'check' mode. See Validating tasks: check mode and diff mode.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub check_mode: OptU<bool>,
    /// List of collection namespaces to search for modules, plugins, and roles. See Using collections in a playbook
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub collections: OptU<Vec<String>>,
    /// Allows you to change the connection plugin used for tasks to execute on the target. See Using connection plugins.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub connection: OptU<String>,
    /// Enable debugging tasks based on the state of the task result. See Debugging tasks.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub debugger: OptU<bool>,

    /// Toggle to make tasks return 'diff' information or not.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub diff: OptU<bool>,

    /// A dictionary that gets converted into environment vars to be provided for the task upon execution.
    /// This can ONLY be used with modules. This is not supported for any other type of plugins nor Ansible itself nor its configuration,
    /// it just sets the variables for the code responsible for executing the task.
    /// This is not a recommended way to pass in confidential data.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub environment: OptU<HashMap<String, String>>,

    /// Set the fact path option for the fact gathering plugin controlled by gather_facts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub fact_path: OptU<String>,

    /// Will force notified handler execution for hosts even if they failed during the play.
    /// Will not trigger if the play itself fails.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub force_handlers: OptU<bool>,

    /// A boolean that controls if the play will automatically run the 'setup' task to gather facts for the hosts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub gather_facts: OptU<bool>,

    /// Allows you to pass subset options to the fact gathering plugin controlled by gather_facts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub gather_subset: OptU<Vec<String>>,

    /// Allows you to set the timeout for the fact gathering plugin controlled by gather_facts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub gather_timeout: OptU<i64>,

    /// A section with tasks that are treated as handlers, these won't get executed normally,
    /// only when notified after each section of tasks is complete.
    /// A handler's listen field is not templatable.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub handlers: OptU<Vec<Task>>,

    /// Boolean that allows you to ignore task failures and continue with play. It does not affect connection errors.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ignore_errors: OptU<bool>,

    /// Boolean that allows you to ignore task failures due to an unreachable host and continue with the play.
    /// This does not affect other task errors (see ignore_errors) but is useful for groups of volatile/ephemeral hosts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ignore_unreachable: OptU<bool>,

    /// Can be used to abort the run after a given percentage of hosts in the current batch has failed.
    /// This only works on linear or linear-derived strategies.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub max_fail_percentage: OptU<i64>,

    /// Specifies default parameter values for modules.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub module_defaults: OptU<HashMap<String, serde_json::Value>>,

    /// Boolean that controls information disclosure.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub no_log: OptU<bool>,

    /// Controls the sorting of hosts as they are used for executing the play.
    /// Possible values are inventory (default), sorted, reverse_sorted, reverse_inventory and shuffle.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub order: OptU<String>,

    /// Used to override the default port used in a connection.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub port: OptU<i64>,

    /// A list of tasks to execute after the tasks section.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub post_tasks: OptU<Vec<Task>>,

    /// A list of tasks to execute before roles.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub pre_tasks: OptU<Vec<Task>>,

    /// User used to log into the target via the connection plugin.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub remote_user: OptU<String>,

    /// Boolean that will bypass the host loop, forcing the task to attempt to execute on the first host available
    /// and afterward apply any results and facts to all active hosts in the same batch.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub run_once: OptU<bool>,

    /// Explicitly define how Ansible batches the execution of the current play on the play's target. See Setting the batch size with serial.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub serial: OptU<i64>,

    /// Allows you to choose the strategy plugin to use for the play. See Strategy plugins.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub strategy: OptU<String>,

    /// Tags applied to the task or included tasks, this allows selecting subsets of tasks from the command line.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub tags: OptU<Vec<String>>,

    /// Limit the number of concurrent task runs on task, block and playbook level. This is independent of the forks and serial settings, but cannot be set higher than those limits. For example, if forks is set to 10 and the throttle is set to 15, at most 10 hosts will be operated on in parallel.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub throttle: OptU<i64>,

    /// Time limit for the task action to execute in, if exceeded, Ansible will interrupt the process. Timeout does not include templating or looping.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub timeout: OptU<i64>,

    /// Dictionary/map of variables
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vars: OptU<HashMap<String, serde_json::Value>>,

    /// List of files that contain vars to include in the play.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vars_files: OptU<Vec<String>>,

    /// list of variables to prompt for.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vars_prompt: OptU<Vec<String>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Task {
    /// Identifier. Can be used for documentation, or in tasks/handlers.
    pub name: String,
    #[serde(flatten)]
    pub options: TaskOptions,
    #[serde(flatten)]
    pub command: Box<dyn TaskModule>,
}

/// Task module trait
///
/// If you want to add a new task module, you need to implement this trait
/// https://crates.io/crates/erased-serde
///
/// ```rust
/// use cdk_ansible_core::core::TaskModule;
/// use serde::Serialize;
///
/// #[derive(Serialize, Clone, Debug)]
/// struct SampleTaskModule {
///     x1: String,
/// }
/// impl TaskModule for SampleTaskModule {}
/// ```
pub trait TaskModule: erased_serde::Serialize + DynClone + std::fmt::Debug {}

serialize_trait_object!(TaskModule);
clone_trait_object!(TaskModule);

/// [playbook keyword (task)](https://docs.ansible.com/ansible/latest/reference_appendices/playbooks_keywords.html#task)
#[derive(Serialize, Default, Clone, Debug, PartialEq)]
pub struct TaskOptions {
    /// The 'action' to execute for a task, it normally translates into a C(module) or action plugin.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub action: OptU<String>,
    /// Force any un-handled task errors on any host to propagate to all hosts and end the play.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub any_errors_fatal: OptU<bool>,
    /// A secondary way to add arguments into a task. Takes a dictionary in which keys map to options and values.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub args: OptU<HashMap<String, serde_json::Value>>,
    /// Run a task asynchronously if the C(action) supports this; the value is the maximum runtime in seconds.
    #[serde(
        rename = "async",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub async_: OptU<i64>,
    /// Boolean that controls if privilege escalation is used or not on Task execution.
    /// Implemented by the become plugin. See Become plugins.
    #[serde(
        rename = "become",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub become_: OptU<bool>,
    /// Path to the executable used to elevate privileges. Implemented by the become plugin. See Become plugins.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub become_exe: OptU<String>,
    /// A string of flag(s) to pass to the privilege escalation program when become is True.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub become_flags: OptU<String>,
    /// Which method of privilege escalation to use (such as sudo or su).
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub become_method: OptU<String>,
    /// User that you 'become' after using privilege escalation. The remote/login user must have permissions to become this user.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub become_user: OptU<String>,
    /// Conditional expression that overrides the task's normal 'changed' status.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub changed_when: OptU<String>,
    /// A boolean that controls if a task is executed in 'check' mode. See Validating tasks: check mode and diff mode.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub check_mode: OptU<bool>,
    /// List of collection namespaces to search for modules, plugins, and roles. See Using collections in a playbook
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub collections: OptU<Vec<String>>,
    /// Allows you to change the connection plugin used for tasks to execute on the target. See Using connection plugins.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub connection: OptU<String>,
    /// Enable debugging tasks based on the state of the task result. See Debugging tasks.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub debugger: OptU<bool>,
    ///
    /// delay
    /// Number of seconds to delay between retries. This setting is only used in combination with until.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub delay: OptU<i64>,
    //
    /// delegate_facts
    /// Boolean that allows you to apply facts to a delegated host instead of inventory_hostname.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub delegate_facts: OptU<bool>,
    /// Host to execute task instead of the target (inventory_hostname).
    /// Connection vars from the delegated host will also be used for the task.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub delegate_to: OptU<String>,

    /// Toggle to make tasks return 'diff' information or not.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub diff: OptU<bool>,

    /// A dictionary that gets converted into environment vars to be provided for the task upon execution.
    /// This can ONLY be used with modules. This is not supported for any other type of plugins nor Ansible itself nor its configuration,
    /// it just sets the variables for the code responsible for executing the task.
    /// This is not a recommended way to pass in confidential data.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub environment: OptU<HashMap<String, String>>,
    /// Conditional expression that overrides the task's normal 'failed' status.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub failed_when: OptU<String>,

    /// Boolean that allows you to ignore task failures and continue with play. It does not affect connection errors.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ignore_errors: OptU<bool>,

    /// Boolean that allows you to ignore task failures due to an unreachable host and continue with the play.
    /// This does not affect other task errors (see ignore_errors) but is useful for groups of volatile/ephemeral hosts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ignore_unreachable: OptU<bool>,

    /// Same as action but also implies delegate_to: localhost
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub local_action: OptU<String>,
    /// Takes a list for the task to iterate over, saving each list element into the item variable (configurable via loop_control)
    #[serde(
        rename = "loop",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub loop_: OptU<Vec<serde_json::Value>>,

    /// Several keys here allow you to modify/set loop behavior in a task. See Adding controls to loops.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub loop_control: OptU<HashMap<String, serde_json::Value>>,

    /// Specifies default parameter values for modules.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub module_defaults: OptU<HashMap<String, serde_json::Value>>,

    /// Boolean that controls information disclosure.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub no_log: OptU<bool>,
    /// List of handlers to notify when the task returns a 'changed=True' status.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub notify: OptU<Vec<String>>,

    /// Sets the polling interval in seconds for async tasks (default 10s).
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub poll: OptU<i64>,

    /// Used to override the default port used in a connection.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub port: OptU<i64>,

    /// Name of variable that will contain task status and module return data.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub register: OptU<String>,

    /// User used to log into the target via the connection plugin.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub remote_user: OptU<String>,

    /// Number of retries before giving up in a until loop. This setting is only used in combination with until.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub retries: OptU<i64>,
    /// Boolean that will bypass the host loop, forcing the task to attempt to execute on the first host available
    /// and afterward apply any results and facts to all active hosts in the same batch.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub run_once: OptU<bool>,

    /// Tags applied to the task or included tasks, this allows selecting subsets of tasks from the command line.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub tags: OptU<Vec<String>>,

    /// Limit the number of concurrent task runs on task, block and playbook level.
    /// This is independent of the forks and serial settings, but cannot be set higher than those limits.
    /// For example, if forks is set to 10 and the throttle is set to 15, at most 10 hosts will be operated on in parallel.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub throttle: OptU<i64>,

    /// Time limit for the task action to execute in, if exceeded, Ansible will interrupt the process.
    /// Timeout does not include templating or looping.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub timeout: OptU<i64>,

    /// This keyword implies a 'retries loop' that will go on until the condition supplied here is met or we hit the retries limit.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub until: OptU<String>,

    /// Dictionary/map of variables
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vars: OptU<HashMap<String, serde_json::Value>>,

    /// Conditional expression, determines if an iteration of a task is run or not.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub when: OptU<String>,
    // FIXME: not supported yet!
    // with_<lookup_plugin>
    // The same as loop but magically adds the output of any lookup plugin to generate the item list.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Clone, Debug, PartialEq)]
    struct SampleTaskModule {
        x1: String,
    }

    impl TaskModule for SampleTaskModule {}

    #[test]
    fn test_play_minimum() {
        assert_eq!(
            serde_json::to_string(&Play {
                name: "play1".to_string(),
                hosts: vec!["host1".to_string()],
                tasks: vec![Task {
                    name: "task1".to_string(),
                    options: TaskOptions::default(),
                    command: Box::new(SampleTaskModule {
                        x1: "x1".to_string(),
                    }),
                }],
                options: PlayOptions::default(),
            })
            .expect("failed to serialize"),
            r#"{"name":"play1","hosts":["host1"],"tasks":[{"name":"task1","x1":"x1"}]}"#
        );
    }

    #[test]
    fn test_play_with_all_fields() {
        assert_eq!(
            serde_json::to_string(&Play {
                name: "play1".to_string(),
                hosts: vec!["host1".to_string()],
                tasks: vec![Task {
                    name: "task1".to_string(),
                    options: TaskOptions::default(),
                    command: Box::new(SampleTaskModule {
                        x1: "x1".to_string(),
                    }),
                }],
                options: PlayOptions {
                    action: OptU::Some("action1".to_string()),
                    any_errors_fatal: OptU::Some(true),
                    args: OptU::Some(HashMap::from([(
                        "arg1".to_string(),
                        serde_json::Value::String("value1".to_string())
                    )])),
                    async_: OptU::Some(10),
                    become_: OptU::Some(true),
                    become_exe: OptU::Some("become_exe".to_string()),
                    become_flags: OptU::Some("become_flags".to_string()),
                    become_method: OptU::Some("become_method".to_string()),
                    become_user: OptU::Some("become_user".to_string()),
                    changed_when: OptU::Some("changed_when".to_string()),
                    check_mode: OptU::Some(true),
                    collections: OptU::Some(vec!["collection1".to_string()]),
                    connection: OptU::Some("connection1".to_string()),
                    debugger: OptU::Some(true),
                    diff: OptU::Some(true),
                    environment: OptU::Some(HashMap::from([(
                        "env1".to_string(),
                        "value1".to_string()
                    )])),
                    fact_path: OptU::Some("fact_path".to_string()),
                    force_handlers: OptU::Some(true),
                    gather_facts: OptU::Some(true),
                    gather_subset: OptU::Some(vec!["gather_subset1".to_string()]),
                    gather_timeout: OptU::Some(10),
                    handlers: OptU::Some(vec![Task {
                        name: "handler1".to_string(),
                        options: TaskOptions::default(),
                        command: Box::new(SampleTaskModule {
                            x1: "x1".to_string(),
                        }),
                    }]),
                    ignore_errors: OptU::Some(true),
                    ignore_unreachable: OptU::Some(true),
                    max_fail_percentage: OptU::Some(10),
                    module_defaults: OptU::Some(HashMap::from([(
                        "module1".to_string(),
                        serde_json::Value::String("value1".to_string())
                    )])),
                    no_log: OptU::Some(true),
                    order: OptU::Some("order".to_string()),
                    port: OptU::Some(10),
                    post_tasks: OptU::Some(vec![Task {
                        name: "post_task1".to_string(),
                        options: TaskOptions::default(),
                        command: Box::new(SampleTaskModule {
                            x1: "x1".to_string(),
                        }),
                    }]),
                    pre_tasks: OptU::Some(vec![Task {
                        name: "pre_task1".to_string(),
                        options: TaskOptions::default(),
                        command: Box::new(SampleTaskModule {
                            x1: "x1".to_string(),
                        }),
                    }]),
                    remote_user: OptU::Some("remote_user".to_string()),
                    run_once: OptU::Some(true),
                    serial: OptU::Some(10),
                    strategy: OptU::Some("strategy".to_string()),
                    tags: OptU::Some(vec!["tag1".to_string()]),
                    throttle: OptU::Some(10),
                    timeout: OptU::Some(10),
                    vars: OptU::Some(HashMap::from([(
                        "var1".to_string(),
                        serde_json::Value::String("value1".to_string())
                    )])),
                    vars_files: OptU::Some(vec!["vars_file1".to_string()]),
                    vars_prompt: OptU::Some(vec!["vars_prompt1".to_string()]),
                }
            })
            .expect("failed to serialize"),
            r#"{"name":"play1","hosts":["host1"],"action":"action1","any_errors_fatal":true,"args":{"arg1":"value1"},"async":10,"become":true,"become_exe":"become_exe","become_flags":"become_flags","become_method":"become_method","become_user":"become_user","changed_when":"changed_when","check_mode":true,"collections":["collection1"],"connection":"connection1","debugger":true,"diff":true,"environment":{"env1":"value1"},"fact_path":"fact_path","force_handlers":true,"gather_facts":true,"gather_subset":["gather_subset1"],"gather_timeout":10,"handlers":[{"name":"handler1","x1":"x1"}],"ignore_errors":true,"ignore_unreachable":true,"max_fail_percentage":10,"module_defaults":{"module1":"value1"},"no_log":true,"order":"order","port":10,"post_tasks":[{"name":"post_task1","x1":"x1"}],"pre_tasks":[{"name":"pre_task1","x1":"x1"}],"remote_user":"remote_user","run_once":true,"serial":10,"strategy":"strategy","tags":["tag1"],"throttle":10,"timeout":10,"vars":{"var1":"value1"},"vars_files":["vars_file1"],"vars_prompt":["vars_prompt1"],"tasks":[{"name":"task1","x1":"x1"}]}"#
        );
    }

    #[test]
    /// Test all fields
    fn test_task_options_with_all_fields() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                action: OptU::Some("action1".to_string()),
                any_errors_fatal: OptU::Some(true),
                args: OptU::Some(HashMap::from([(
                    "arg1".to_string(),
                    serde_json::Value::String("value1".to_string())
                )])),
                async_: OptU::Some(10),
                become_: OptU::Some(true),
                become_exe: OptU::Some("become_exe".to_string()),
                become_flags: OptU::Some("become_flags".to_string()),
                become_method: OptU::Some("become_method".to_string()),
                become_user: OptU::Some("become_user".to_string()),
                changed_when: OptU::Some("changed_when".to_string()),
                check_mode: OptU::Some(true),
                collections: OptU::Some(vec!["collection1".to_string()]),
                connection: OptU::Some("connection1".to_string()),
                debugger: OptU::Some(true),
                delay: OptU::Some(10),
                delegate_facts: OptU::Some(true),
                delegate_to: OptU::Some("delegate_to".to_string()),
                diff: OptU::Some(true),
                environment: OptU::Some(HashMap::from([(
                    "env1".to_string(),
                    "value1".to_string()
                )])),
                failed_when: OptU::Some("failed_when".to_string()),
                ignore_errors: OptU::Some(true),
                ignore_unreachable: OptU::Some(true),
                local_action: OptU::Some("local_action".to_string()),
                loop_: OptU::Some(vec![serde_json::Value::String("loop1".to_string())]),
                loop_control: OptU::Some(HashMap::from([(
                    "loop_control1".to_string(),
                    serde_json::Value::String("value1".to_string())
                )])),
                module_defaults: OptU::Some(HashMap::from([(
                    "module1".to_string(),
                    serde_json::Value::String("value1".to_string())
                )])),
                no_log: OptU::Some(true),
                notify: OptU::Some(vec!["notify1".to_string()]),
                poll: OptU::Some(10),
                port: OptU::Some(10),
                register: OptU::Some("register".to_string()),
                remote_user: OptU::Some("remote_user".to_string()),
                retries: OptU::Some(10),
                run_once: OptU::Some(true),
                tags: OptU::Some(vec!["tag1".to_string()]),
                throttle: OptU::Some(10),
                timeout: OptU::Some(10),
                until: OptU::Some("until".to_string()),
                vars: OptU::Some(HashMap::from([(
                    "var1".to_string(),
                    serde_json::Value::String("value1".to_string())
                )])),
                when: OptU::Some("when".to_string()),
            })
            .expect("failed to serialize"),
            r#"{"action":"action1","any_errors_fatal":true,"args":{"arg1":"value1"},"async":10,"become":true,"become_exe":"become_exe","become_flags":"become_flags","become_method":"become_method","become_user":"become_user","changed_when":"changed_when","check_mode":true,"collections":["collection1"],"connection":"connection1","debugger":true,"delay":10,"delegate_facts":true,"delegate_to":"delegate_to","diff":true,"environment":{"env1":"value1"},"failed_when":"failed_when","ignore_errors":true,"ignore_unreachable":true,"local_action":"local_action","loop":["loop1"],"loop_control":{"loop_control1":"value1"},"module_defaults":{"module1":"value1"},"no_log":true,"notify":["notify1"],"poll":10,"port":10,"register":"register","remote_user":"remote_user","retries":10,"run_once":true,"tags":["tag1"],"throttle":10,"timeout":10,"until":"until","vars":{"var1":"value1"},"when":"when"}"#
        );
    }
}
