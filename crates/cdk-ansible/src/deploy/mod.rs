mod cli;
use crate::{
    Inventory,
    types::{ExePlay, ExePlaybook, StackName},
};
use anyhow::Result;
use cli::Cli;
use indexmap::IndexMap;

/// Main entry point for the cdk-ansible CLI.
///
/// ```rust
/// use anyhow::Result;
/// use cdk_ansible::{App, Stack, ExePlay, ExeSingle, Play, PlayOptions};
///
/// fn create_play_helper(name: &str) -> Box<Play> {
///     Box::new(Play {
///         name: name.to_string(),
///         hosts: "localhost".into(),
///         options: PlayOptions::default(),
///         tasks: vec![],
///     })
/// }
///
/// struct SampleStack {
///     exe_play: ExePlay,
/// }
///
/// impl SampleStack {
///   pub fn new() -> Self {
///     Self {
///       exe_play: ExeSingle(create_play_helper("sample")),
///     }
///   }
/// }
///
/// impl Stack for SampleStack {
///     fn name(&self) -> &str {
///         "sample"
///     }
///     fn exe_play(&self) -> &ExePlay {
///         &self.exe_play
///     }
/// }
///
/// let mut app = App::new(vec!["help".to_string()]);
/// app.add_stack(Box::new(SampleStack::new()))
///     .expect("Failed to add sample stack");
/// ```
pub struct App {
    args: Vec<String>,
    /// key is an unique name of stack. Forbidden to be duplicated.
    stacks: IndexMap<StackName, Box<dyn Stack>>,
    /// key is only used for check duplication.
    inventories: IndexMap<String, Inventory>,
    /// Don't use this directly. Use [`App::exe_playbooks`] method.
    /// Memoization of ExePlaybooks.
    /// key is an unique name of stack. Forbidden to be duplicated.
    #[doc(hidden)]
    exe_playbooks: IndexMap<StackName, ExePlaybook>,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            inventories: IndexMap::new(),
            stacks: IndexMap::new(),
            exe_playbooks: IndexMap::new(),
        }
    }

    pub fn add_inventory(&mut self, inventory: Inventory) -> Result<()> {
        let old_inventory = self.inventories.insert(inventory.name.clone(), inventory);
        if let Some(old_inventory) = old_inventory {
            anyhow::bail!("conflicting inventory name: {}", old_inventory.name);
        }
        Ok(())
    }

    pub fn add_stack(&mut self, stack: Box<dyn Stack>) -> Result<()> {
        // Memoization of ExePlaybook
        let old_exe_playbook = self.exe_playbooks.insert(
            stack.name().into(),
            ExePlaybook::from_exe_play(stack.name(), stack.exe_play().clone()),
        );
        if let Some(old_exe_playbook) = old_exe_playbook {
            anyhow::bail!(
                "conflicting stack name: {} ({:?})",
                stack.name(),
                old_exe_playbook
            );
        }

        // Store a stack
        let old_stack = self.stacks.insert(stack.name().into(), stack);
        if let Some(old_stack) = old_stack {
            anyhow::bail!("conflicting stack name: {}", old_stack.name());
        }

        Ok(())
    }

    /// Main entry point for end users
    pub fn run(&self) -> Result<()> {
        let nprocs = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or_default();
        let threads = nprocs; // TODO: use env var
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(threads)
            .build()?
            .block_on(Cli::run(self))
    }

    fn exe_playbooks(&self) -> &IndexMap<StackName, ExePlaybook> {
        &self.exe_playbooks
    }
}

/// 副作用の無いコードを書くこと
pub trait Stack {
    fn name(&self) -> &str;
    fn exe_play(&self) -> &ExePlay;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_sample_stack() {
        struct SampleStack {
            exe_play: ExePlay,
        }

        impl SampleStack {
            fn new() -> Self {
                Self {
                    exe_play: create_play_helper("sample").into(),
                }
            }
        }

        impl Stack for SampleStack {
            fn name(&self) -> &str {
                ::std::any::type_name::<Self>()
            }
            fn exe_play(&self) -> &ExePlay {
                &self.exe_play
            }
        }

        let mut app = App::new(vec!["help".to_string()]);
        app.add_stack(Box::new(SampleStack::new()))
            .expect("Failed to add sample stack");
    }

    #[test]
    fn test_stack_name_confliction() {
        struct SampleStack1 {
            name: String,
            exe_play: ExePlay,
        }

        impl SampleStack1 {
            fn new(n: &str) -> Self {
                Self {
                    name: n.to_string(),
                    exe_play: create_play_helper("sample1").into(),
                }
            }
        }
        impl Stack for SampleStack1 {
            fn name(&self) -> &str {
                &self.name
            }
            fn exe_play(&self) -> &ExePlay {
                &self.exe_play
            }
        }

        struct SampleStack2 {
            name: String,
            exe_play: ExePlay,
        }
        impl SampleStack2 {
            fn new(n: &str) -> Self {
                Self {
                    name: n.to_string(),
                    exe_play: create_play_helper("sample2").into(),
                }
            }
        }

        impl Stack for SampleStack2 {
            fn name(&self) -> &str {
                &self.name
            }
            fn exe_play(&self) -> &ExePlay {
                &self.exe_play
            }
        }

        let mut app = App::new(vec!["help".to_string()]);
        app.add_stack(Box::new(SampleStack1::new("sample")))
            .expect("Failed to add sample stack");
        app.add_stack(Box::new(SampleStack2::new("sample")))
            .expect_err("should be duplicated error");
    }
}
