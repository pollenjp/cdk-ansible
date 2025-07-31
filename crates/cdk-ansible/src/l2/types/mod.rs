pub(crate) mod trait_impl;
use crate::HostInventoryVarsGenerator;
use anyhow::Result;
use cdk_ansible_core::core::{
    InventoryChild, InventoryRoot, OptU, Play, PlayOptions, StringOrVecString, Task,
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

/// Define play as lazy
///
/// This is a "L2 feature".
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
    /// impl LazyPlayL2 for SampleLazyPlayL2 {
    ///     fn play_l2(&self) -> BoxFuture<'static, Result<PlayL2>> {
    ///         async move {
    ///             let hosts = get_hosts()?;
    ///             Ok(PlayL2 {
    ///                 name: "sample1".to_string(),
    ///                 hosts: HostsL2::new(vec![
    ///                     Arc::clone(hosts.aaa),
    ///                     Arc::clone(hosts.bbb),
    ///                 ]),
    ///                 options: PlayOptions::default(),
    ///                 tasks: vec![],
    ///             })
    ///         }.boxed()
    ///     }
    /// }
    /// ```
    fn create_play_l2(&self) -> BoxFuture<'static, Result<PlayL2>>;
}

#[derive(Clone)]
pub struct HostsL2(Vec<Arc<dyn HostInventoryVarsGenerator + Send + Sync>>);

impl HostsL2 {
    pub fn new(hosts: Vec<Arc<dyn HostInventoryVarsGenerator + Send + Sync>>) -> Self {
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

    pub fn to_inventory_root(&self) -> Result<InventoryRoot> {
        Ok(InventoryRoot {
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

/// Enum to define the relationship between [`LazyPlayL2`]
///
/// This is a "L2 feature".
///
/// ```rust
/// use cdk_ansible::{prelude::*, Play, PlayOptions, PlayL2, HostsL2, HostInventoryVarsGenerator, HostInventoryVars, LEPSequentialL2, LEPSingleL2, LEPParallelL2, LazyPlayL2, LazyExePlayL2};
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
///     fn create_play_l2(&self) -> BoxFuture<'static, Result<PlayL2>> {
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
/// // Example of creating LazyExePlayL2 simply
/// let _play_exec = LEPSequentialL2(vec![
///     LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
///     LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
///     LEPParallelL2(vec![
///         LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
///         LEPSequentialL2(vec![
///             LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample4"))),
///             LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample5"))),
///         ]),
///     ]),
/// ]);
///
/// // Example of creating LazyExePlayL2 using IntoLazyExePlayL2Parallel and IntoLazyExePlayL2Sequential
/// use cdk_ansible::prelude::*;
///
/// let _play_exec = vec![
///     LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
///     LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
///     vec![
///         LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
///         vec![
///             LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample4"))),
///             LazyExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample5"))),
///         ]
///         .into_exe_play_l2_parallel(),
///     ]
///     .into_exe_play_l2_sequential(),
/// ]
/// .into_exe_play_l2_sequential();
///
/// ```
#[derive(Clone)]
pub enum LazyExePlayL2 {
    /// Sequential execution
    Sequential(Vec<LazyExePlayL2>),
    /// Parallel execution
    Parallel(Vec<LazyExePlayL2>),
    /// Single Play
    Single(Arc<dyn LazyPlayL2 + Send + Sync>),
}

// define alias as 'LEP'
pub use LazyExePlayL2 as LEP;

pub use LazyExePlayL2::Parallel as LEPParallelL2;
pub use LazyExePlayL2::Sequential as LEPSequentialL2;
pub use LazyExePlayL2::Single as LEPSingleL2;

#[cfg(test)]
mod test_exe_play_struct {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_sequential_play_exec() {
        let _play_exec = LEPSequentialL2(vec![
            LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
            LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
            LEPParallelL2(vec![
                LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
                LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample4"))),
            ]),
        ]);
    }
}

impl LazyExePlayL2 {
    /// Experimental feature: Push a play to the end of the execution
    ///
    /// - LEPSingleL2 -> LEPSequentialL2
    /// - LEPSequentialL2 -> LEPSequentialL2
    /// - LEPParallelL2 -> LEPParallelL2
    ///
    /// # Example
    ///
    /// TODO: fill in
    pub fn push(&mut self, p: LazyExePlayL2) {
        match self {
            LazyExePlayL2::Sequential(plays) => plays.push(p),
            LazyExePlayL2::Parallel(plays) => plays.push(p),
            LazyExePlayL2::Single(_) => {
                let p1 = self.clone();
                *self = LEPSequentialL2(vec![p1, p]);
            }
        }
    }
    pub fn push_play(&mut self, p: Arc<dyn LazyPlayL2 + Send + Sync>) {
        match self {
            LazyExePlayL2::Sequential(plays) => plays.push(p.into()),
            LazyExePlayL2::Parallel(plays) => plays.push(p.into()),
            LazyExePlayL2::Single(_) => {
                let p1 = self.clone();
                *self = LEPSequentialL2(vec![p1, p.into()]);
            }
        }
    }
}

impl From<Arc<dyn LazyPlayL2 + Send + Sync>> for LazyExePlayL2 {
    fn from(p: Arc<dyn LazyPlayL2 + Send + Sync>) -> Self {
        LazyExePlayL2::Single(p)
    }
}

#[cfg(test)]
mod test_exe_play_l2_push {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_exe_play_single_push() {
        let mut exe_play = LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample1")));
        exe_play.push(LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample2",
        ))));
        match exe_play {
            LazyExePlayL2::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be LEPSequentialL2"),
        }
    }
    #[test]
    fn test_exe_play_sequential_push() {
        let mut exe_play = LEPSequentialL2(vec![LEPSingleL2(Arc::new(
            SampleLazyPlayL2Helper::new("sample1"),
        ))]);
        exe_play.push(LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample2",
        ))));
        match exe_play {
            LazyExePlayL2::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be LEPSequentialL2"),
        }
    }
    #[test]
    fn test_exe_play_parallel_push() {
        let mut exe_play = LEPParallelL2(vec![LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample1",
        )))]);
        exe_play.push(LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample2",
        ))));
        match exe_play {
            LazyExePlayL2::Parallel(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be LEPParallelL2"),
        }
    }
}

#[cfg(test)]
mod test_exe_play_l2_push_play {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_exe_play_single_push_play() {
        let mut exe_play = LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new("sample1")));
        exe_play.push_play(Arc::new(SampleLazyPlayL2Helper::new("sample2")));
        match exe_play {
            LazyExePlayL2::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be LEPSequentialL2"),
        }
    }
    #[test]
    fn test_exe_play_sequential_push_play() {
        let mut exe_play = LEPSequentialL2(vec![LEPSingleL2(Arc::new(
            SampleLazyPlayL2Helper::new("sample1"),
        ))]);
        exe_play.push_play(Arc::new(SampleLazyPlayL2Helper::new("sample2")));
        match exe_play {
            LazyExePlayL2::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be LEPSequentialL2"),
        }
    }
    #[test]
    fn test_exe_play_parallel_push_play() {
        let mut exe_play = LEPParallelL2(vec![LEPSingleL2(Arc::new(SampleLazyPlayL2Helper::new(
            "sample1",
        )))]);
        exe_play.push_play(Arc::new(SampleLazyPlayL2Helper::new("sample2")));
        match exe_play {
            LazyExePlayL2::Parallel(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be LEPParallelL2"),
        }
    }
}
