use crate::{Play, Playbook};

/// Play execution definition for end users
///
/// ```rust
/// use cdk_ansible::{Play, PlayOptions, ExSequential, ExSingle, ExParallel};
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
/// let _play_exec = ExSequential(vec![
///     ExSingle(create_play_helper("sample1")),
///     ExSingle(create_play_helper("sample2")),
///     ExParallel(vec![
///         ExSingle(create_play_helper("sample3")),
///         ExSingle(create_play_helper("sample4")),
///     ]),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub enum ExPlay {
    /// Sequential execution
    Sequential(Vec<ExPlay>),
    /// Parallel execution
    Parallel(Vec<ExPlay>),
    /// Single Play
    Single(Box<Play>),
}

pub use ExPlay::Parallel as ExParallel;
pub use ExPlay::Sequential as ExSequential;
pub use ExPlay::Single as ExSingle;

/// Playbook execution definition for deployment
#[derive(Debug, Clone)]
pub enum ExPlaybook {
    Sequential(Vec<ExPlaybook>),
    Parallel(Vec<ExPlaybook>),
    Single(Box<Playbook>),
}

impl ExPlaybook {
    pub fn from_ex_play(name: &str, ex_play: ExPlay) -> Self {
        match ex_play {
            ExPlay::Sequential(plays) => Self::Sequential(
                plays
                    .into_iter()
                    .enumerate()
                    .map(|(i, ex_play)| Self::from_ex_play(&format!("{name}_seq{i}"), ex_play))
                    .collect(),
            ),
            ExPlay::Parallel(plays) => Self::Parallel(
                plays
                    .into_iter()
                    .enumerate()
                    .map(|(i, ex_play)| Self::from_ex_play(&format!("{name}_par{i}"), ex_play))
                    .collect(),
            ),
            ExPlay::Single(play) => Self::Single(Box::new(Playbook {
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
        let _play_exec = ExSingle(create_play_helper("sample"));
    }

    #[test]
    fn test_sequential_play_exec() {
        let _play_exec = ExSequential(vec![
            ExSingle(create_play_helper("sample1")),
            ExSingle(create_play_helper("sample2")),
            ExParallel(vec![
                ExSingle(create_play_helper("sample3")),
                ExSingle(create_play_helper("sample4")),
            ]),
        ]);
    }
}
