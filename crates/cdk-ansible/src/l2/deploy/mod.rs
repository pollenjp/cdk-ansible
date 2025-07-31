mod cli;
mod stack_container;
use crate::l2::types::LazyExePlayL2;
use anyhow::Result;
use cli::Cli;
use stack_container::StackContainer;
use std::rc::Rc;
use std::sync::Arc;

/// Main entry point for the cdk-ansible CLI.
///
/// ```rust
/// use anyhow::Result;
/// use cdk_ansible::{AppL2, StackL2, LazyExePlayL2, ExeSingle, Play, PlayOptions, LazyPlayL2, PlayL2, HostsL2, HostInventoryVarsGenerator, HostInventoryVars};
/// use std::rc::Rc;
/// use std::sync::Arc;
/// use futures::future::{BoxFuture, FutureExt as _};
///
/// struct HostA {
///     name: String,
/// }
///
/// impl HostInventoryVarsGenerator for HostA {
///     fn gen_host_vars(&self) -> Result<HostInventoryVars> {
///         Ok(HostInventoryVars {
///             ansible_host: self.name.clone(),
///             inventory_vars: vec![],
///         })
///     }
/// }
///
/// struct SampleLazyPlayL2Helper {
///     name: String,
/// }
///
/// impl SampleLazyPlayL2Helper {
///     pub fn new(name: &str) -> Self {
///         Self { name: name.to_string() }
///     }
/// }
///
/// impl LazyPlayL2 for SampleLazyPlayL2Helper {
///     fn create_play_l2(&self) -> BoxFuture<'static, Result<PlayL2>> {
///         let name = self.name.clone();
///         async move { Ok(PlayL2 {
///             name,
///             hosts: HostsL2::new(vec![
///                 Arc::new(HostA { name: "localhost".to_string() }),
///             ]),
///             options: PlayOptions::default(),
///             tasks: vec![],
///         }) }.boxed()
///     }
/// }
///
/// struct SampleStack {
///     exe_play: LazyExePlayL2,
/// }
///
/// impl SampleStack {
///     fn new() -> Self {
///         Self {
///             exe_play: LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample"))),
///         }
///     }
/// }
///
/// impl StackL2 for SampleStack {
///     fn name(&self) -> &str {
///         ::std::any::type_name::<Self>()
///     }
///     fn exe_play(&self) -> &LazyExePlayL2 {
///         &self.exe_play
///     }
/// }
///
/// let _app = AppL2::new(vec!["help".to_string()])
///     .stack(Arc::new(SampleStack::new()))
///     .expect("Failed to add sample stack");
/// ```
#[derive(Debug)]
pub struct AppL2 {
    inner: Rc<AppL2Inner>,
}

#[derive(Debug)]
struct AppL2Inner {
    args: Vec<String>,
    stack_container: StackContainer,
}

impl AppL2 {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            inner: Rc::new(AppL2Inner {
                args,
                stack_container: StackContainer::new(),
            }),
        }
    }

    fn into_inner(self) -> AppL2Inner {
        match Rc::try_unwrap(self.inner) {
            Ok(inner) => inner,
            Err(arc) => AppL2Inner {
                args: arc.args.clone(),
                stack_container: arc.stack_container.clone(),
            },
        }
    }

    pub fn stack(self, stack: Arc<dyn StackL2>) -> Result<Self> {
        let inner = self.into_inner();
        Ok(AppL2 {
            inner: Rc::new(AppL2Inner {
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
    fn exe_play(&self) -> &LazyExePlayL2;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_sample_stack() {
        struct SampleStack {
            exe_play: LazyExePlayL2,
        }

        impl SampleStack {
            fn new() -> Self {
                Self {
                    exe_play: LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new(
                        "sample",
                    ))),
                }
            }
        }

        impl StackL2 for SampleStack {
            fn name(&self) -> &str {
                ::std::any::type_name::<Self>()
            }
            fn exe_play(&self) -> &LazyExePlayL2 {
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
            exe_play: LazyExePlayL2,
        }

        impl SampleStack1 {
            fn new(n: &str) -> Self {
                Self {
                    name: n.to_string(),
                    exe_play: LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new(
                        "sample1",
                    ))),
                }
            }
        }
        impl StackL2 for SampleStack1 {
            fn name(&self) -> &str {
                &self.name
            }
            fn exe_play(&self) -> &LazyExePlayL2 {
                &self.exe_play
            }
        }

        struct SampleStack2 {
            name: String,
            exe_play: LazyExePlayL2,
        }
        impl SampleStack2 {
            fn new(n: &str) -> Self {
                Self {
                    name: n.to_string(),
                    exe_play: LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new(
                        "sample2",
                    ))),
                }
            }
        }

        impl StackL2 for SampleStack2 {
            fn name(&self) -> &str {
                &self.name
            }
            fn exe_play(&self) -> &LazyExePlayL2 {
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
