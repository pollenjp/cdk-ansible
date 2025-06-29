use crate::Play;

/// Play execution definition
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
