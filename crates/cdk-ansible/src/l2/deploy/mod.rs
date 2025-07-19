mod cli;
mod stack_container;
// use crate::{ExePlay, ExePlaybook, Inventory};
use crate::types::{ExePlay, ExePlaybook};
use anyhow::Result;
use cdk_ansible_core::core::Inventory;
use cli::Cli;
use indexmap::IndexMap;
use stack_container::StackContainer;
use std::{fmt, ops::DerefMut, sync::Arc};

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
#[derive(Debug)]
pub struct AppL2 {
    inner: Arc<AppL2Inner>,
}

#[derive(Debug)]
struct AppL2Inner {
    args: Vec<String>,
    stack_container: StackContainer,
    /// key is only used for check duplication.
    inventories: IndexMap<String, Inventory>,
}

impl AppL2 {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            inner: Arc::new(AppL2Inner {
                args,
                inventories: IndexMap::new(),
                stack_container: StackContainer::new(),
            }),
        }
    }

    fn into_inner(self) -> AppL2Inner {
        // Arc::new(self.inner)
        match Arc::try_unwrap(self.inner) {
            Ok(inner) => inner,
            Err(arc) => AppL2Inner {
                args: arc.args.clone(),
                stack_container: arc.stack_container.clone(),
                inventories: arc.inventories.clone(),
            },
        }
    }

    pub fn inventory(self, inventory: Inventory) -> Result<Self> {
        let mut inner = self.into_inner();
        let old_inventory = inner.inventories.insert(inventory.name.clone(), inventory);
        if let Some(old_inventory) = old_inventory {
            anyhow::bail!("conflicting inventory name: {}", old_inventory.name);
        }
        Ok(AppL2 {
            inner: Arc::new(inner),
        })
    }

    pub fn stack(self, stack: Arc<dyn StackL2>) -> Result<Self> {
        let inner = self.into_inner();
        Ok(AppL2 {
            inner: Arc::new(AppL2Inner {
                stack_container: inner.stack_container.stack(stack)?,
                ..inner
            }),
        })
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
}

/// 副作用の無いコードを書くこと
pub trait StackL2 {
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

        impl StackL2 for SampleStack {
            fn name(&self) -> &str {
                ::std::any::type_name::<Self>()
            }
            fn exe_play(&self) -> &ExePlay {
                &self.exe_play
            }
        }

        let _app = AppL2::new(vec!["help".to_string()])
            .stack(Arc::new(SampleStack::new()))
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
        impl StackL2 for SampleStack1 {
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

        impl StackL2 for SampleStack2 {
            fn name(&self) -> &str {
                &self.name
            }
            fn exe_play(&self) -> &ExePlay {
                &self.exe_play
            }
        }

        let _app = AppL2::new(vec!["help".to_string()])
            .stack(Arc::new(SampleStack1::new("sample")))
            .expect("Failed to add sample stack")
            .stack(Arc::new(SampleStack2::new("sample")))
            .expect_err("should be duplicated error");
    }
}
