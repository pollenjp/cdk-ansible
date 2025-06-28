use crate::Play;

/// Play execution definition
///
/// ```rust
/// use cdk_ansible::{Play, PlayOptions, ExSequential, ExSingle, ExParallel};
///
/// let play = Play {
///     name: "play1".into(),
///     hosts: "localhost".into(),
///     options: PlayOptions::default(),
///     tasks: vec![],
/// };
///
/// let _play_exec = ExSequential(vec![
///     Box::new(ExSingle(Box::new(play.clone()))),
///     Box::new(ExParallel(vec![
///         Box::new(ExSingle(Box::new(play.clone()))),
///         Box::new(ExSingle(Box::new(play.clone()))),
///     ])),
///     Box::new(ExSingle(Box::new(play.clone()))),
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

    /// Helper function to create sample plays
    fn create_sample_plays(num_plays: usize) -> Vec<Play> {
        (0..num_plays)
            .map(|i| Play {
                name: format!("play{}", i + 1),
                hosts: "localhost".into(),
                options: PlayOptions::default(),
                tasks: vec![],
            })
            .collect()
    }

    // #[test]
    // fn test_single_play_exec() {
    //     let mut plays = create_sample_plays(1);
    //     let _play_exec = ExSingle(Box::new(plays.pop().expect("play not found").clone()));
    // }

    // #[test]
    // fn test_sequential_play_exec() {
    //     let play = create_sample_plays(1).pop().expect("play not found");
    //     let _play_exec = ExSequential(vec![
    //         Box::new(ExSingle(Box::new(play.clone()))),
    //         Box::new(ExSingle(Box::new(play.clone()))),
    //         Box::new(ExParallel(vec![
    //             Box::new(ExSingle(Box::new(play.clone()))),
    //             Box::new(ExSingle(Box::new(play.clone()))),
    //         ])),
    //     ]);
    // }
}
