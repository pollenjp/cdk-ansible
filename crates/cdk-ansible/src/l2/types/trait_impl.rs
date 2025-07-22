//! Trait implementations for `ExePlayL2`
//! These traits can be imported with `use cdk_ansible::prelude::*;`

use crate::l2::types::ExePlayL2;

/// Convert to sequential execution
pub trait IntoExePlayL2Sequential {
    fn into_exe_play_l2_sequential(self) -> ExePlayL2;
}

///
/// ```rust
/// use cdk_ansible::{prelude::*, PlayL2, PlayOptions, ExePlayL2, HostsL2, HostInventoryVarsGenerator, HostInventoryVars, LazyPlayL2};
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
/// let plays = vec![
///     ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
///     ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
///     ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
/// ];
/// match plays.into_exe_play_l2_sequential() {
///     ExePlayL2::Sequential(_) => {
///         // OK
///     }
///     _ => unreachable!("exe_play should be ExeSequential"),
/// }
/// ```
impl IntoExePlayL2Sequential for Vec<ExePlayL2> {
    fn into_exe_play_l2_sequential(self) -> ExePlayL2 {
        ExePlayL2::Sequential(self)
    }
}

#[cfg(test)]
mod test_into_exe_play_l2_sequential {
    use super::*;
    use crate::utils::test::*;
    use std::sync::Arc;

    #[test]
    fn test_into_exe_play_l2_sequential() {
        let plays = vec![
            ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
            ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
            ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
        ];
        match plays.into_exe_play_l2_sequential() {
            ExePlayL2::Sequential(_) => {
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequential"),
        }
    }
}

/// Convert to parallel execution
pub trait IntoExePlayL2Parallel {
    fn into_exe_play_l2_parallel(self) -> ExePlayL2;
}

/// ```rust
/// use cdk_ansible::{prelude::*, PlayL2, PlayOptions, ExePlayL2, HostsL2, HostInventoryVarsGenerator, HostInventoryVars, LazyPlayL2};
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
/// let plays = vec![
///     ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
///     ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
///     ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
/// ];
/// match plays.into_exe_play_l2_parallel() {
///     ExePlayL2::Parallel(_) => {
///         // OK
///     }
///     _ => unreachable!("exe_play should be ExeParallel"),
/// }
/// ```
impl IntoExePlayL2Parallel for Vec<ExePlayL2> {
    fn into_exe_play_l2_parallel(self) -> ExePlayL2 {
        ExePlayL2::Parallel(self)
    }
}

#[cfg(test)]
mod test_into_exe_play_l2_parallel {
    use super::*;
    use crate::utils::test::*;
    use std::sync::Arc;

    #[test]
    fn test_into_exe_play_l2_parallel() {
        let plays = vec![
            ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample1"))),
            ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample2"))),
            ExePlayL2::Single(Arc::new(SampleLazyPlayL2Helper::new("sample3"))),
        ];
        match plays.into_exe_play_l2_parallel() {
            ExePlayL2::Parallel(_) => {
                // OK
            }
            _ => unreachable!("exe_play should be ExeParallel"),
        }
    }
}
