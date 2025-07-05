use crate::{Play, Playbook};

/// Play execution definition for end users
///
/// ```rust
/// use cdk_ansible::{Play, PlayOptions, ExeSequential, ExeSingle, ExeParallel};
///
/// /// Helper function to create sample play
/// fn create_play_helper(name: &str) -> Box<Play> {
///     Box::new(Play {
///         name: name.to_string(),
///         hosts: "localhost".into(),
///         options: PlayOptions::default(),
///         tasks: vec![],
///     })
/// }
///
/// let _play_exec = ExeSequential(vec![
///     ExeSingle(create_play_helper("sample1")),
///     ExeSingle(create_play_helper("sample2")),
///     ExeParallel(vec![
///         ExeSingle(create_play_helper("sample3")),
///         ExeSingle(create_play_helper("sample4")),
///     ]),
/// ]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use cdk_ansible_core::core::{Play, PlayOptions};

    /// Helper function to create sample play
    fn create_play_helper(name: &str) -> Box<Play> {
        Box::new(Play {
            name: name.to_string(),
            hosts: "localhost".into(),
            options: PlayOptions::default(),
            tasks: vec![],
        })
    }

    #[test]
    fn test_single_play_exec() {
        let _play_exec = ExeSingle(create_play_helper("sample"));
    }

    #[test]
    fn test_sequential_play_exec() {
        let _play_exec = ExeSequential(vec![
            ExeSingle(create_play_helper("sample1")),
            ExeSingle(create_play_helper("sample2")),
            ExeParallel(vec![
                ExeSingle(create_play_helper("sample3")),
                ExeSingle(create_play_helper("sample4")),
            ]),
        ]);
    }
}
