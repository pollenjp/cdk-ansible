pub mod trait_impl;
use crate::{Play, Playbook};

/// Play execution definition
///
/// ```rust
/// use cdk_ansible::{Play, PlayOptions, ExeSequential, ExeSingle, ExeParallel};
///
/// /// Helper function to create sample play
/// fn create_play_helper(name: &str) -> Play {
///     Play {
///         name: name.to_string(),
///         hosts: "localhost".into(),
///         options: PlayOptions::default(),
///         tasks: vec![],
///     }
/// }
///
/// // Example of creating ExePlay simply
/// let _play_exec = ExeSequential(vec![
///     ExeSingle(Box::new(create_play_helper("sample1"))),
///     ExeSingle(Box::new(create_play_helper("sample2"))),
///     ExeParallel(vec![
///         ExeSingle(Box::new(create_play_helper("sample3"))),
///         ExeSequential(vec![
///             ExeSingle(Box::new(create_play_helper("sample4"))),
///             ExeSingle(Box::new(create_play_helper("sample5"))),
///         ]),
///     ]),
/// ]);
///
/// // Example of creating ExePlay using IntoExePlayParallel and IntoExePlaySequential
/// use cdk_ansible::prelude::*;
///
/// let _play_exec = vec![
///     create_play_helper("sample1").into(),
///     create_play_helper("sample2").into(),
///     vec![
///         create_play_helper("sample3").into(),
///         vec![
///             create_play_helper("sample4").into(),
///             create_play_helper("sample5").into(),
///         ]
///         .into_exe_play_parallel(),
///     ]
///     .into_exe_play_sequential(),
/// ]
/// .into_exe_play_sequential();
///
/// ```
#[derive(Debug, Clone)]
pub enum ExePlay {
    /// Sequential execution
    Sequential(Vec<ExePlay>),
    /// Parallel execution
    Parallel(Vec<ExePlay>),
    /// Single Play
    Single(Box<Play>),
}

pub use ExePlay::Parallel as ExeParallel;
pub use ExePlay::Sequential as ExeSequential;
pub use ExePlay::Single as ExeSingle;

#[cfg(test)]
mod test_exe_play_struct {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_sequential_play_exec() {
        let _play_exec = ExeSequential(vec![
            ExeSingle(Box::new(create_play_helper("sample1"))),
            ExeSingle(Box::new(create_play_helper("sample2"))),
            ExeParallel(vec![
                ExeSingle(Box::new(create_play_helper("sample3"))),
                ExeSingle(Box::new(create_play_helper("sample4"))),
            ]),
        ]);
    }
}

impl ExePlay {
    /// Experimental feature: Push a play to the end of the execution
    ///
    /// - ExeSingle -> ExeSequential
    /// - ExeSequential -> ExeSequential
    /// - ExeParallel -> ExeParallel
    ///
    /// # Example
    ///
    /// TODO: fill in
    pub fn push(&mut self, p: ExePlay) {
        match self {
            ExePlay::Sequential(plays) => plays.push(p),
            ExePlay::Parallel(plays) => plays.push(p),
            ExePlay::Single(_) => {
                let p1 = self.clone();
                *self = ExeSequential(vec![p1, p]);
            }
        }
    }
    pub fn push_play(&mut self, p: Play) {
        match self {
            ExePlay::Sequential(plays) => plays.push(p.into()),
            ExePlay::Parallel(plays) => plays.push(p.into()),
            ExePlay::Single(_) => {
                let p1 = self.clone();
                *self = ExeSequential(vec![p1, p.into()]);
            }
        }
    }
}

#[cfg(test)]
mod test_exe_play {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_exe_play_single_push() {
        let mut exe_play = ExeSingle(create_play_helper("sample1").into());
        exe_play.push(create_play_helper("sample2").into());
        match exe_play {
            ExePlay::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequential"),
        }
    }
    #[test]
    fn test_exe_play_sequential_push() {
        let mut exe_play = ExeSequential(vec![create_play_helper("sample1").into()]);
        exe_play.push(create_play_helper("sample2").into());
        match exe_play {
            ExePlay::Sequential(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequential"),
        }
    }
    #[test]
    fn test_exe_play_parallel_push() {
        let mut exe_play = ExeParallel(vec![create_play_helper("sample1").into()]);
        exe_play.push(create_play_helper("sample2").into());
        match exe_play {
            ExePlay::Parallel(plays) => {
                assert_eq!(plays.len(), 2);
                // OK
            }
            _ => unreachable!("exe_play should be ExeParallel"),
        }
    }
}

impl From<Play> for ExePlay {
    fn from(play: Play) -> Self {
        ExePlay::Single(Box::new(play))
    }
}

impl From<Box<Play>> for ExePlay {
    fn from(play: Box<Play>) -> Self {
        ExePlay::Single(play)
    }
}

impl From<Vec<ExePlay>> for ExePlay {
    fn from(plays: Vec<ExePlay>) -> Self {
        ExePlay::Sequential(plays)
    }
}

#[cfg(test)]
mod test_exe_play_from_impl {
    use super::*;
    use crate::utils::test::*;

    #[test]
    fn test_exe_play_from_play() {
        let play = create_play_helper("sample");
        let exe_play: ExePlay = play.into();
        match exe_play {
            ExePlay::Single(_) => {
                // OK
            }
            _ => unreachable!("exe_play should be ExeSingle"),
        }
    }
    #[test]
    fn test_exe_play_from_play_vec() {
        let plays = vec![
            create_play_helper("sample1").into(),
            create_play_helper("sample2").into(),
            create_play_helper("sample3").into(),
        ];
        let exe_play: ExePlay = plays.into();
        match exe_play {
            ExePlay::Sequential(_) => {
                // OK
            }
            _ => unreachable!("exe_play should be ExeSequential"),
        }
    }
}

/// Playbook execution definition for deployment
#[derive(Debug, Clone)]
pub enum ExePlaybook {
    Sequential(Vec<ExePlaybook>),
    Parallel(Vec<ExePlaybook>),
    Single(Box<Playbook>),
}

impl ExePlaybook {
    pub fn from_exe_play(name: &str, exe_play: ExePlay) -> Self {
        match exe_play {
            ExePlay::Sequential(plays) => Self::Sequential(
                plays
                    .into_iter()
                    .enumerate()
                    .map(|(i, exe_play)| Self::from_exe_play(&format!("{name}_seq{i}"), exe_play))
                    .collect(),
            ),
            ExePlay::Parallel(plays) => Self::Parallel(
                plays
                    .into_iter()
                    .enumerate()
                    .map(|(i, exe_play)| Self::from_exe_play(&format!("{name}_par{i}"), exe_play))
                    .collect(),
            ),
            ExePlay::Single(play) => Self::Single(Box::new(Playbook {
                name: format!(
                    "{name}_{}",
                    play.name.as_str().to_lowercase().replace(' ', "_")
                ),
                plays: vec![*play],
            })),
        }
    }
}
