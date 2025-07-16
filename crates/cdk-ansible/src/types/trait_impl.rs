//! Trait implementations for `ExePlay`
//! These traits can be imported with `use cdk_ansible::prelude::*;`

use crate::ExePlay;

/// Convert to sequential execution
pub trait IntoExePlaySequential {
    fn into_exe_play_sequential(self) -> ExePlay;
}

///
/// ```rust
/// use cdk_ansible::{prelude::*, Play, PlayOptions, ExePlay};
///
/// pub fn create_play_helper(name: &str) -> Play {
///     Play {
///         name: name.to_string(),
///         hosts: "localhost".into(),
///         options: PlayOptions::default(),
///         tasks: vec![],
///     }
/// }
///
/// let plays = vec![
///     create_play_helper("sample1").into(),
///     create_play_helper("sample2").into(),
///     create_play_helper("sample3").into(),
/// ];
/// match plays.into_exe_play_sequential() {
///     ExePlay::Sequential(_) => {
///         // OK
///     }
///     _ => unreachable!("exe_play should be ExeSequential"),
/// }
/// ```
impl IntoExePlaySequential for Vec<ExePlay> {
    fn into_exe_play_sequential(self) -> ExePlay {
        ExePlay::Sequential(self)
    }
}

#[cfg(test)]
mod test_into_exe_play_sequential {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_into_exe_play_sequential() {
        let plays = vec![
            create_play_helper("sample1").into(),
            create_play_helper("sample2").into(),
            create_play_helper("sample3").into(),
        ];
        match plays.into_exe_play_sequential() {
            ExePlay::Sequential(_) => {
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequential"),
        }
    }
}

/// Convert to parallel execution
pub trait IntoExePlayParallel {
    fn into_exe_play_parallel(self) -> ExePlay;
}

/// ```rust
/// use cdk_ansible::{prelude::*, Play, PlayOptions, ExePlay};
///
/// pub fn create_play_helper(name: &str) -> Play {
///     Play {
///         name: name.to_string(),
///         hosts: "localhost".into(),
///         options: PlayOptions::default(),
///         tasks: vec![],
///     }
/// }
///
/// let plays = vec![
///     create_play_helper("sample1").into(),
///     create_play_helper("sample2").into(),
///     create_play_helper("sample3").into(),
/// ];
/// match plays.into_exe_play_parallel() {
///     ExePlay::Parallel(_) => {
///         // OK
///     }
///     _ => unreachable!("exe_play should be ExeParallel"),
/// }
/// ```
impl IntoExePlayParallel for Vec<ExePlay> {
    fn into_exe_play_parallel(self) -> ExePlay {
        ExePlay::Parallel(self)
    }
}

#[cfg(test)]
mod test_into_exe_play_parallel {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_into_exe_play_parallel() {
        let plays = vec![
            create_play_helper("sample1").into(),
            create_play_helper("sample2").into(),
            create_play_helper("sample3").into(),
        ];
        match plays.into_exe_play_parallel() {
            ExePlay::Parallel(_) => {
                // OK
            }
            _ => unreachable!("exe_play should be ExeParallel"),
        }
    }
}
