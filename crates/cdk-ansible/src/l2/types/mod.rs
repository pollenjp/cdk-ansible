pub(crate) mod trait_impl;
use crate::HostInventoryVarsGenerator;
use anyhow::Result;
use cdk_ansible_core::core::{
    Inventory, InventoryChild, InventoryRoot, OptU, Play, PlayOptions, StringOrVecString, Task,
};
use futures::future::BoxFuture;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct PlayL2 {
    pub name: String,
    pub hosts: HostsL2,
    pub options: PlayOptions,
    pub tasks: Vec<Task>,
}

impl PlayL2 {
    pub fn try_play(self) -> Result<Play> {
        Ok(Play {
            name: self.name,
            hosts: self.hosts.try_hosts()?,
            options: self.options,
            tasks: self.tasks,
        })
    }
}

pub trait LazyPlayL2 {
    /// Pseudo code
    ///
    /// ```ignore
    /// use cdk_ansible::{prelude::*, PlayL2, PlayOptions, HostsL2, HostInventoryVarsGenerator, LazyPlayL2};
    /// use std::sync::Arc;
    /// use futures::future::{BoxFuture, FutureExt as _};
    ///
    /// // ...
    ///
    /// impl LazyPlayL2 for SampleLazyPlay {
    ///     async move {
    ///         let hosts = get_hosts()?;
    ///         Ok(PlayL2 {
    ///             name: "sample1".to_string(),
    ///             hosts: HostsL2::new(vec![
    ///                 Arc::clone(hosts.aaa),
    ///                 Arc::clone(hosts.bbb),
    ///             ]),
    ///             options: PlayOptions::default(),
    ///             tasks: vec![],
    ///         })
    ///     }.boxed()
    /// }
    /// ```
    fn exe_play(&self) -> BoxFuture<'static, Result<PlayL2>>;
}

#[derive(Clone)]
pub struct HostsL2(Vec<Arc<dyn HostInventoryVarsGenerator>>);

impl HostsL2 {
    pub fn new(hosts: Vec<Arc<dyn HostInventoryVarsGenerator>>) -> Self {
        Self(hosts)
    }

    pub fn try_hosts(&self) -> Result<StringOrVecString> {
        Ok(self
            .0
            .iter()
            .map(|h| h.gen_host_vars())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|h| h.ansible_host.to_string())
            .collect::<Vec<_>>()
            .into())
    }

    pub fn to_inventory(&self, name: &str) -> Result<Inventory> {
        Ok(Inventory {
            name: name.into(), // generate 'dev.yaml' file
            root: InventoryRoot {
                all: InventoryChild {
                    hosts: OptU::Some(
                        self.0
                            .iter()
                            .map(|h| h.gen_host_vars())
                            .collect::<Result<Vec<_>>>()?
                            .into_iter()
                            .collect(),
                    ),
                    ..Default::default()
                },
            },
        })
    }
}

impl fmt::Debug for PlayL2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayL2")
            .field("name", &self.name)
            .field("hosts", &"hosts (TODO)")
            .field("options", &self.options)
            .field("tasks", &self.tasks)
            .finish()
    }
}

/// Play execution definition
///
/// ```rust
/// use cdk_ansible::{prelude::*, Play, PlayOptions, PlayL2, HostsL2, HostInventoryVarsGenerator, HostInventoryVars, ExeSequentialL2, ExeSingleL2, ExeParallelL2, LazyPlayL2, ExePlayL2};
/// use std::sync::Arc;
/// use anyhow::Result;
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
///     fn exe_play(&self) -> BoxFuture<'static, Result<PlayL2>> {
///         let name = self.name.clone();
///         async move { Ok(PlayL2 {
///             name,
///             hosts: HostsL2::new(vec![Arc::new(HostA { name: "localhost".to_string() })]),
///             options: PlayOptions::default(),
///             tasks: vec![],
///         }) }.boxed()
///     }
/// }
///
/// // Example of creating ExePlayL2 simply
/// let _play_exec = ExeSequentialL2(vec![
///     ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
///     ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
///     ExeParallelL2(vec![
///         ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
///         ExeSequentialL2(vec![
///             ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample4"))),
///             ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample5"))),
///         ]),
///     ]),
/// ]);
///
/// // Example of creating ExePlayL2 using IntoExePlayL2Parallel and IntoExePlayL2Sequential
/// use cdk_ansible::prelude::*;
///
/// let _play_exec = vec![
///     ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
///     ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
///     vec![
///         ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
///         vec![
///             ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample4"))),
///             ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample5"))),
///         ]
///         .into_exe_play_l2_parallel(),
///     ]
///     .into_exe_play_l2_sequential(),
/// ]
/// .into_exe_play_l2_sequential();
///
/// ```
#[derive(Clone)]
pub enum ExePlayL2 {
    /// Sequential execution
    Sequential(Vec<ExePlayL2>),
    /// Parallel execution
    Parallel(Vec<ExePlayL2>),
    /// Single Play
    Single(Arc<dyn LazyPlayL2>),
}

pub use ExePlayL2::Parallel as ExeParallelL2;
pub use ExePlayL2::Sequential as ExeSequentialL2;
pub use ExePlayL2::Single as ExeSingleL2;

#[cfg(test)]
mod test_exe_play_struct {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_sequential_play_exec() {
        let _play_exec = ExeSequentialL2(vec![
            ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
            ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
            ExeParallelL2(vec![
                ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
                ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample4"))),
            ]),
        ]);
    }
}

impl ExePlayL2 {
    /// Experimental feature: Push a play to the end of the execution
    ///
    /// - ExeSingleL2 -> ExeSequentialL2
    /// - ExeSequentialL2 -> ExeSequentialL2
    /// - ExeParallelL2 -> ExeParallelL2
    ///
    /// # Example
    ///
    /// TODO: fill in
    pub fn push(&mut self, p: ExePlayL2) {
        match self {
            ExePlayL2::Sequential(plays) => plays.push(p),
            ExePlayL2::Parallel(plays) => plays.push(p),
            ExePlayL2::Single(_) => {
                let p1 = self.clone();
                *self = ExeSequentialL2(vec![p1, p]);
            }
        }
    }
    pub fn push_play(&mut self, p: Arc<dyn LazyPlayL2>) {
        match self {
            ExePlayL2::Sequential(plays) => plays.push(p.into()),
            ExePlayL2::Parallel(plays) => plays.push(p.into()),
            ExePlayL2::Single(_) => {
                let p1 = self.clone();
                *self = ExeSequentialL2(vec![p1, p.into()]);
            }
        }
    }
}

impl From<Arc<dyn LazyPlayL2>> for ExePlayL2 {
    fn from(p: Arc<dyn LazyPlayL2>) -> Self {
        ExePlayL2::Single(p)
    }
}

#[cfg(test)]
mod test_exe_play_l2_push {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_exe_play_single_push() {
        let mut exe_play = ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample1")));
        exe_play.push(ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample2",
        ))));
        match exe_play {
            ExePlayL2::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequentialL2"),
        }
    }
    #[test]
    fn test_exe_play_sequential_push() {
        let mut exe_play = ExeSequentialL2(vec![ExeSingleL2(Arc::new(
            SampleLazyPlayL2Helper::new("sample1"),
        ))]);
        exe_play.push(ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample2",
        ))));
        match exe_play {
            ExePlayL2::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequentialL2"),
        }
    }
    #[test]
    fn test_exe_play_parallel_push() {
        let mut exe_play = ExeParallelL2(vec![ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample1",
        )))]);
        exe_play.push(ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample2",
        ))));
        match exe_play {
            ExePlayL2::Parallel(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeParallelL2"),
        }
    }
}

#[cfg(test)]
mod test_exe_play_l2_push_play {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_exe_play_single_push_play() {
        let mut exe_play = ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample1")));
        exe_play.push_play(Arc::new(SampleLazyPlayL2Helper::new("sample2")));
        match exe_play {
            ExePlayL2::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequentialL2"),
        }
    }
    #[test]
    fn test_exe_play_sequential_push_play() {
        let mut exe_play = ExeSequentialL2(vec![ExeSingleL2(Arc::new(
            SampleLazyPlayL2Helper::new("sample1"),
        ))]);
        exe_play.push_play(Arc::new(SampleLazyPlayL2Helper::new("sample2")));
        match exe_play {
            ExePlayL2::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequentialL2"),
        }
    }
    #[test]
    fn test_exe_play_parallel_push_play() {
        let mut exe_play = ExeParallelL2(vec![ExeSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample1",
        )))]);
        exe_play.push_play(Arc::new(SampleLazyPlayL2Helper::new("sample2")));
        match exe_play {
            ExePlayL2::Parallel(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeParallelL2"),
        }
    }
}
