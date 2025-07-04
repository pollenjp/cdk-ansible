use dyn_clone::{DynClone, clone_trait_object};
use erased_serde::serialize_trait_object;
use indexmap::IndexMap;
use serde::Serialize;

mod types;
pub use types::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Inventory {
    pub name: String,
    pub root: InventoryRoot,
}

impl Inventory {
    pub fn dump_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self.root)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryRoot {
    pub all: InventoryChild,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryChild {
    #[serde(skip_serializing_if = "OptU::is_unset")]
    pub hosts: OptU<InventoryHosts>,
    #[serde(skip_serializing_if = "OptU::is_unset")]
    pub children: OptU<IndexMap<String, InventoryChild>>,
    #[serde(skip_serializing_if = "OptU::is_unset")]
    pub vars: OptU<InventoryVars>,
}

pub type InventoryHosts = IndexMap<String, Option<InventoryVars>>;
pub type InventoryVars = IndexMap<String, serde_json::Value>;

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
    pub hosts: StringOrVecString,
    #[serde(flatten)]
    pub options: PlayOptions,
    /// Main list of tasks to execute in the play, they run after roles and before post_tasks.
    pub tasks: Vec<Task>,
}

/// [playbook keywords (play)](https://docs.ansible.com/ansible/latest/reference_appendices/playbooks_keywords.html#play)
#[derive(Serialize, Default, Clone, Debug)]
pub struct PlayOptions {
    /// Force any un-handled task errors on any host to propagate to all hosts and end the play.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub any_errors_fatal: OptU<BoolOrString>,
    /// Boolean that controls if privilege escalation is used or not on Task execution.
    /// Implemented by the become plugin. See Become plugins.
    #[serde(
        rename = "become",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub become_: OptU<BoolOrString>,
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
    /// A boolean that controls if a task is executed in 'check' mode. See Validating tasks: check mode and diff mode.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub check_mode: OptU<BoolOrString>,
    /// List of collection namespaces to search for modules, plugins, and roles. See Using collections in a playbook
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub collections: OptU<Vec<String>>,
    /// Allows you to change the connection plugin used for tasks to execute on the target. See Using connection plugins.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub connection: OptU<String>,
    /// Enable debugging tasks based on the state of the task result. See Debugging tasks.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub debugger: OptU<BoolOrString>,
    /// Toggle to make tasks return 'diff' information or not.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub diff: OptU<BoolOrString>,
    /// A dictionary that gets converted into environment vars to be provided for the task upon execution.
    /// This can ONLY be used with modules. This is not supported for any other type of plugins nor Ansible itself nor its configuration,
    /// it just sets the variables for the code responsible for executing the task.
    /// This is not a recommended way to pass in confidential data.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub environment: OptU<IndexMap<String, String>>,
    /// Set the fact path option for the fact gathering plugin controlled by gather_facts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub fact_path: OptU<String>,
    /// Will force notified handler execution for hosts even if they failed during the play.
    /// Will not trigger if the play itself fails.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub force_handlers: OptU<BoolOrString>,
    /// A boolean that controls if the play will automatically run the 'setup' task to gather facts for the hosts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub gather_facts: OptU<BoolOrString>,
    /// Allows you to pass subset options to the fact gathering plugin controlled by gather_facts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub gather_subset: OptU<Vec<String>>,
    /// Allows you to set the timeout for the fact gathering plugin controlled by gather_facts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub gather_timeout: OptU<IntOrString>,
    /// A section with tasks that are treated as handlers, these won't get executed normally,
    /// only when notified after each section of tasks is complete.
    /// A handler's listen field is not templatable.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub handlers: OptU<Vec<Task>>,
    /// Boolean that allows you to ignore task failures and continue with play. It does not affect connection errors.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ignore_errors: OptU<BoolOrString>,
    /// Boolean that allows you to ignore task failures due to an unreachable host and continue with the play.
    /// This does not affect other task errors (see ignore_errors) but is useful for groups of volatile/ephemeral hosts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ignore_unreachable: OptU<BoolOrString>,
    /// Can be used to abort the run after a given percentage of hosts in the current batch has failed.
    /// This only works on linear or linear-derived strategies.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub max_fail_percentage: OptU<IntOrString>,
    /// Specifies default parameter values for modules.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub module_defaults: OptU<IndexMap<String, serde_json::Value>>,
    /// Boolean that controls information disclosure.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub no_log: OptU<BoolOrString>,
    /// Controls the sorting of hosts as they are used for executing the play.
    /// Possible values are inventory (default), sorted, reverse_sorted, reverse_inventory and shuffle.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub order: OptU<String>,
    /// Used to override the default port used in a connection.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub port: OptU<IntOrString>,
    /// A list of tasks to execute after the tasks section.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub post_tasks: OptU<Vec<Task>>,
    /// A list of tasks to execute before roles.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub pre_tasks: OptU<Vec<Task>>,
    /// User used to log into the target via the connection plugin.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub remote_user: OptU<String>,
    /// List of roles to be imported into the play
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub roles: OptU<Vec<String>>,
    /// Boolean that will bypass the host loop, forcing the task to attempt to execute on the first host available
    /// and afterward apply any results and facts to all active hosts in the same batch.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub run_once: OptU<BoolOrString>,
    /// Explicitly define how Ansible batches the execution of the current play on the play's target. See Setting the batch size with serial.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub serial: OptU<IntOrString>,
    /// Allows you to choose the strategy plugin to use for the play. See Strategy plugins.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub strategy: OptU<String>,
    /// Tags applied to the task or included tasks, this allows selecting subsets of tasks from the command line.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub tags: OptU<Vec<String>>,
    /// Limit the number of concurrent task runs on task, block and playbook level. This is independent of the forks and serial settings, but cannot be set higher than those limits. For example, if forks is set to 10 and the throttle is set to 15, at most 10 hosts will be operated on in parallel.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub throttle: OptU<IntOrString>,
    /// Time limit for the task action to execute in, if exceeded, Ansible will interrupt the process. Timeout does not include templating or looping.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub timeout: OptU<IntOrString>,
    /// Dictionary/map of variables
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vars: OptU<IndexMap<String, serde_json::Value>>,
    /// List of files that contain vars to include in the play.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vars_files: OptU<Vec<String>>,
    /// List of variables to prompt for.
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
pub trait TaskModule: erased_serde::Serialize + DynClone + std::fmt::Debug + Send + Sync {}

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
    pub any_errors_fatal: OptU<BoolOrString>,
    /// A secondary way to add arguments into a task. Takes a dictionary in which keys map to options and values.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub args: OptU<IndexMap<String, serde_json::Value>>,
    /// Run a task asynchronously if the C(action) supports this; the value is the maximum runtime in seconds.
    #[serde(
        rename = "async",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub async_: OptU<IntOrString>,
    /// Boolean that controls if privilege escalation is used or not on Task execution.
    /// Implemented by the become plugin. See Become plugins.
    #[serde(
        rename = "become",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub become_: OptU<BoolOrString>,
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
    ///
    /// The ansible original type allows `Array of strings`.
    /// But we use `String` for now, because all conditions are expressed as a single string.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub changed_when: OptU<BoolOrStringOrVecString>,
    /// A boolean that controls if a task is executed in 'check' mode. See Validating tasks: check mode and diff mode.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub check_mode: OptU<BoolOrString>,
    /// List of collection namespaces to search for modules, plugins, and roles. See Using collections in a playbook
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub collections: OptU<Vec<String>>,
    /// Allows you to change the connection plugin used for tasks to execute on the target. See Using connection plugins.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub connection: OptU<String>,
    /// Enable debugging tasks based on the state of the task result. See Debugging tasks.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub debugger: OptU<BoolOrString>,
    ///
    /// delay
    /// Number of seconds to delay between retries. This setting is only used in combination with until.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub delay: OptU<IntOrString>,
    //
    /// delegate_facts
    /// Boolean that allows you to apply facts to a delegated host instead of inventory_hostname.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub delegate_facts: OptU<BoolOrString>,
    /// Host to execute task instead of the target (inventory_hostname).
    /// Connection vars from the delegated host will also be used for the task.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub delegate_to: OptU<String>,

    /// Toggle to make tasks return 'diff' information or not.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub diff: OptU<BoolOrString>,

    /// A dictionary that gets converted into environment vars to be provided for the task upon execution.
    /// This can ONLY be used with modules. This is not supported for any other type of plugins nor Ansible itself nor its configuration,
    /// it just sets the variables for the code responsible for executing the task.
    /// This is not a recommended way to pass in confidential data.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub environment: OptU<IndexMap<String, String>>,
    /// Conditional expression that overrides the task's normal 'failed' status.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub failed_when: OptU<BoolOrStringOrVecString>,

    /// Boolean that allows you to ignore task failures and continue with play. It does not affect connection errors.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ignore_errors: OptU<BoolOrString>,

    /// Boolean that allows you to ignore task failures due to an unreachable host and continue with the play.
    /// This does not affect other task errors (see ignore_errors) but is useful for groups of volatile/ephemeral hosts.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ignore_unreachable: OptU<BoolOrString>,

    /// Same as action but also implies delegate_to: localhost
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub local_action: OptU<String>,
    /// Takes a list for the task to iterate over, saving each list element into the item variable (configurable via loop_control)
    #[serde(
        rename = "loop",
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset"
    )]
    pub loop_: OptU<StringOrVec>,

    /// Several keys here allow you to modify/set loop behavior in a task. See Adding controls to loops.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub loop_control: OptU<IndexMap<String, serde_json::Value>>,

    /// Specifies default parameter values for modules.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub module_defaults: OptU<IndexMap<String, serde_json::Value>>,

    /// Boolean that controls information disclosure.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub no_log: OptU<BoolOrString>,
    /// List of handlers to notify when the task returns a 'changed=True' status.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub notify: OptU<Vec<String>>,

    /// Sets the polling interval in seconds for async tasks (default 10s).
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub poll: OptU<IntOrString>,

    /// Used to override the default port used in a connection.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub port: OptU<IntOrString>,

    /// Name of variable that will contain task status and module return data.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub register: OptU<String>,

    /// User used to log into the target via the connection plugin.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub remote_user: OptU<String>,

    /// Number of retries before giving up in a until loop. This setting is only used in combination with until.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub retries: OptU<IntOrString>,
    /// Boolean that will bypass the host loop, forcing the task to attempt to execute on the first host available
    /// and afterward apply any results and facts to all active hosts in the same batch.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub run_once: OptU<BoolOrString>,

    /// Tags applied to the task or included tasks, this allows selecting subsets of tasks from the command line.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub tags: OptU<Vec<String>>,

    /// Limit the number of concurrent task runs on task, block and playbook level.
    /// This is independent of the forks and serial settings, but cannot be set higher than those limits.
    /// For example, if forks is set to 10 and the throttle is set to 15, at most 10 hosts will be operated on in parallel.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub throttle: OptU<IntOrString>,

    /// Time limit for the task action to execute in, if exceeded, Ansible will interrupt the process.
    /// Timeout does not include templating or looping.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub timeout: OptU<IntOrString>,

    /// This keyword implies a 'retries loop' that will go on until the condition supplied here is met or we hit the retries limit.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub until: OptU<String>,

    /// Dictionary/map of variables
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vars: OptU<IndexMap<String, serde_json::Value>>,

    /// Conditional expression, determines if an iteration of a task is run or not.
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub when: OptU<BoolOrStringOrVecString>,
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
                hosts: vec!["host1".to_string()].into(),
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
                hosts: vec!["host1".to_string()].into(),
                tasks: vec![Task {
                    name: "task1".to_string(),
                    options: TaskOptions::default(),
                    command: Box::new(SampleTaskModule {
                        x1: "x1".to_string(),
                    }),
                }],
                options: PlayOptions {
                    any_errors_fatal: OptU::Some(true.into()),
                    become_: OptU::Some(true.into()),
                    become_exe: OptU::Some("become_exe".to_string()),
                    become_flags: OptU::Some("become_flags".to_string()),
                    become_method: OptU::Some("become_method".to_string()),
                    become_user: OptU::Some("become_user".to_string()),
                    check_mode: OptU::Some(true.into()),
                    collections: OptU::Some(vec!["collection1".to_string()]),
                    connection: OptU::Some("connection1".to_string()),
                    debugger: OptU::Some(true.into()),
                    diff: OptU::Some(true.into()),
                    environment: OptU::Some(IndexMap::from([(
                        "env1".to_string(),
                        "value1".to_string()
                    )])),
                    fact_path: OptU::Some("fact_path".to_string()),
                    force_handlers: OptU::Some(true.into()),
                    gather_facts: OptU::Some(true.into()),
                    gather_subset: OptU::Some(vec!["gather_subset1".to_string()]),
                    gather_timeout: OptU::Some(10.into()),
                    handlers: OptU::Some(vec![Task {
                        name: "handler1".to_string(),
                        options: TaskOptions::default(),
                        command: Box::new(SampleTaskModule {
                            x1: "x1".to_string(),
                        }),
                    }]),
                    ignore_errors: OptU::Some(true.into()),
                    ignore_unreachable: OptU::Some(true.into()),
                    max_fail_percentage: OptU::Some(10.into()),
                    module_defaults: OptU::Some(IndexMap::from([(
                        "module1".to_string(),
                        serde_json::Value::String("value1".to_string())
                    )])),
                    no_log: OptU::Some(true.into()),
                    order: OptU::Some("order".to_string()),
                    port: OptU::Some(10.into()),
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
                    roles: OptU::Some(vec!["role1".to_string()]),
                    run_once: OptU::Some(true.into()),
                    serial: OptU::Some(10.into()),
                    strategy: OptU::Some("strategy".to_string()),
                    tags: OptU::Some(vec!["tag1".to_string()]),
                    throttle: OptU::Some(10.into()),
                    timeout: OptU::Some(10.into()),
                    vars: OptU::Some(IndexMap::from([(
                        "var1".to_string(),
                        serde_json::Value::String("value1".to_string())
                    )])),
                    vars_files: OptU::Some(vec!["vars_file1".to_string()]),
                    vars_prompt: OptU::Some(vec!["vars_prompt1".to_string()]),
                }
            })
            .expect("failed to serialize"),
            String::new()
                + "{"
                + r#""name":"play1","#
                + r#""hosts":["host1"],"#
                + r#""any_errors_fatal":true,"#
                + r#""become":true,"#
                + r#""become_exe":"become_exe","#
                + r#""become_flags":"become_flags","#
                + r#""become_method":"become_method","#
                + r#""become_user":"become_user","#
                + r#""check_mode":true,"#
                + r#""collections":["collection1"],"#
                + r#""connection":"connection1","#
                + r#""debugger":true,"#
                + r#""diff":true,"#
                + r#""environment":{"env1":"value1"},"#
                + r#""fact_path":"fact_path","#
                + r#""force_handlers":true,"#
                + r#""gather_facts":true,"#
                + r#""gather_subset":["gather_subset1"],"#
                + r#""gather_timeout":10,"#
                + r#""handlers":[{"name":"handler1","x1":"x1"}],"#
                + r#""ignore_errors":true,"#
                + r#""ignore_unreachable":true,"#
                + r#""max_fail_percentage":10,"#
                + r#""module_defaults":{"module1":"value1"},"#
                + r#""no_log":true,"#
                + r#""order":"order","#
                + r#""port":10,"#
                + r#""post_tasks":[{"name":"post_task1","x1":"x1"}],"#
                + r#""pre_tasks":[{"name":"pre_task1","x1":"x1"}],"#
                + r#""remote_user":"remote_user","#
                + r#""roles":["role1"],"#
                + r#""run_once":true,"#
                + r#""serial":10,"#
                + r#""strategy":"strategy","#
                + r#""tags":["tag1"],"#
                + r#""throttle":10,"#
                + r#""timeout":10,"#
                + r#""vars":{"var1":"value1"},"#
                + r#""vars_files":["vars_file1"],"#
                + r#""vars_prompt":["vars_prompt1"],"#
                + r#""tasks":[{"name":"task1","x1":"x1"}]"#
                + r#"}"#
        );
    }

    #[test]
    /// Test all fields
    fn test_task_options_with_all_fields() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                action: OptU::Some("action1".to_string()),
                any_errors_fatal: OptU::Some(true.into()),
                args: OptU::Some(IndexMap::from([(
                    "arg1".to_string(),
                    serde_json::Value::String("value1".to_string())
                )])),
                async_: OptU::Some(10.into()),
                become_: OptU::Some(true.into()),
                become_exe: OptU::Some("become_exe".to_string()),
                become_flags: OptU::Some("become_flags".to_string()),
                become_method: OptU::Some("become_method".to_string()),
                become_user: OptU::Some("become_user".to_string()),
                changed_when: OptU::Some("changed_when".to_string().into()),
                check_mode: OptU::Some(true.into()),
                collections: OptU::Some(vec!["collection1".to_string()]),
                connection: OptU::Some("connection1".to_string()),
                debugger: OptU::Some(true.into()),
                delay: OptU::Some(
                    // random filter is useful to avoid congestion
                    // Include `IntOrString`'s test
                    "{{ 10 | ansible.builtin.random(seed=inventory_hostname) }}".into(),
                ),
                delegate_facts: OptU::Some(true.into()),
                delegate_to: OptU::Some("delegate_to".to_string()),
                diff: OptU::Some(true.into()),
                environment: OptU::Some(IndexMap::from([(
                    "env1".to_string(),
                    "value1".to_string()
                )])),
                failed_when: OptU::Some("failed_when".to_string().into()),
                ignore_errors: OptU::Some(true.into()),
                ignore_unreachable: OptU::Some(true.into()),
                local_action: OptU::Some("local_action".to_string()),
                loop_: OptU::Some(vec!["loop1".into()].into()),
                loop_control: OptU::Some(IndexMap::from([(
                    "loop_control1".to_string(),
                    serde_json::Value::String("value1".to_string())
                )])),
                module_defaults: OptU::Some(IndexMap::from([(
                    "module1".to_string(),
                    serde_json::Value::String("value1".to_string())
                )])),
                no_log: OptU::Some(true.into()),
                notify: OptU::Some(vec!["notify1".to_string()]),
                poll: OptU::Some(10.into()),
                port: OptU::Some(10.into()),
                register: OptU::Some("register".to_string()),
                remote_user: OptU::Some("remote_user".to_string()),
                retries: OptU::Some(10.into()),
                run_once: OptU::Some(true.into()),
                tags: OptU::Some(vec!["tag1".to_string()]),
                throttle: OptU::Some(10.into()),
                timeout: OptU::Some(10.into()),
                until: OptU::Some("until".to_string()),
                vars: OptU::Some(IndexMap::from([(
                    "var1".to_string(),
                    serde_json::Value::String("value1".to_string())
                )])),
                when: OptU::Some("when".to_string().into()),
            })
            .expect("failed to serialize"),
            String::new()
                + r#"{""#
                + r#"action":"action1","#
                + r#""any_errors_fatal":true,"#
                + r#""args":{"arg1":"value1"},"#
                + r#""async":10,"#
                + r#""become":true,"#
                + r#""become_exe":"become_exe","#
                + r#""become_flags":"become_flags","#
                + r#""become_method":"become_method","#
                + r#""become_user":"become_user","#
                + r#""changed_when":"changed_when","#
                + r#""check_mode":true,"#
                + r#""collections":["collection1"],"#
                + r#""connection":"connection1","#
                + r#""debugger":true,"#
                + r#""delay":"{{ 10 | ansible.builtin.random(seed=inventory_hostname) }}","#
                + r#""delegate_facts":true,"#
                + r#""delegate_to":"delegate_to","#
                + r#""diff":true,"#
                + r#""environment":{"env1":"value1"},"#
                + r#""failed_when":"failed_when","#
                + r#""ignore_errors":true,"#
                + r#""ignore_unreachable":true,"#
                + r#""local_action":"local_action","#
                + r#""loop":["loop1"],"#
                + r#""loop_control":{"loop_control1":"value1"},"#
                + r#""module_defaults":{"module1":"value1"},"#
                + r#""no_log":true,"#
                + r#""notify":["notify1"],"#
                + r#""poll":10,"#
                + r#""port":10,"#
                + r#""register":"register","#
                + r#""remote_user":"remote_user","#
                + r#""retries":10,"#
                + r#""run_once":true,"#
                + r#""tags":["tag1"],"#
                + r#""throttle":10,"#
                + r#""timeout":10,"#
                + r#""until":"until","#
                + r#""vars":{"var1":"value1"},"#
                + r#""when":"when""#
                + r#"}"#
        );
    }

    #[test]
    fn test_changed_when_bool() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                changed_when: OptU::Some(true.into()),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"changed_when":true}"#
        );
    }

    #[test]
    fn test_changed_when_string() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                changed_when: OptU::Some("changed_when".to_string().into()),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"changed_when":"changed_when"}"#
        );
    }
    #[test]
    fn test_changed_when_vec_string() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                changed_when: OptU::Some(
                    vec!["changed_when1".to_string(), "changed_when2".to_string()].into()
                ),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"changed_when":["changed_when1","changed_when2"]}"#
        );
    }

    #[test]
    fn test_failed_when_bool() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                failed_when: OptU::Some(true.into()),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"failed_when":true}"#
        );
    }

    #[test]
    fn test_failed_when_string() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                failed_when: OptU::Some("failed_when".to_string().into()),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"failed_when":"failed_when"}"#
        );
    }

    #[test]
    fn test_failed_when_vec_string() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                failed_when: OptU::Some(
                    vec!["failed_when1".to_string(), "failed_when2".to_string()].into()
                ),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"failed_when":["failed_when1","failed_when2"]}"#
        );
    }

    #[test]
    fn test_when_bool() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                when: OptU::Some(true.into()),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"when":true}"#
        );
    }

    #[test]
    fn test_when_string() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                when: OptU::Some("1 == 1".to_string().into()),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"when":"1 == 1"}"#
        );
    }

    #[test]
    fn test_when_vec_string() {
        assert_eq!(
            serde_json::to_string(&TaskOptions {
                when: OptU::Some(vec!["1 == 1".to_string(), "2 == 2".to_string()].into()),
                ..Default::default()
            })
            .expect("failed to serialize"),
            String::new() + r#"{"when":["1 == 1","2 == 2"]}"#
        );
    }
}
