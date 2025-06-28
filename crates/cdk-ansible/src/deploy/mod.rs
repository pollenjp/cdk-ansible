use anyhow::Result;
use indexmap::IndexMap;

mod cli;
use cli::Cli;
mod types;
pub use types::*;

/// Main entry point for the cdk-ansible CLI.
///
/// ```rust
/// use cdk_ansible::SynthApp;
///
/// let app = SynthApp::new();
/// app.run(::std::env::args_os());
/// ```
#[derive(Debug)]
pub struct DeployApp {
    args: Vec<String>,
    stacks: IndexMap<String, ExPlay>,
}

impl DeployApp {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            stacks: IndexMap::new(),
        }
    }

    pub fn add_stack(&mut self, stack: Box<dyn DeployStack>) -> Result<()> {
        let old_value = self.stacks.insert(stack.name().to_owned(), stack.plays()?);
        if old_value.is_some() {
            anyhow::bail!("conflicting stack name: {}", stack.name());
        }
        Ok(())
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

pub trait DeployStack {
    fn name(&self) -> &str;
    fn plays(&self) -> Result<ExPlay>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use cdk_ansible_core::core::{Play, PlayOptions};

    /// Helper function to create sample play
    fn create_play(name: &str) -> Box<Play> {
        Box::new(Play {
            name: name.to_string(),
            hosts: "localhost".into(),
            options: PlayOptions::default(),
            tasks: vec![],
        })
    }

    #[test]
    fn test_sample_stack() {
        struct SampleStack;

        impl DeployStack for SampleStack {
            fn name(&self) -> &str {
                "sample"
            }
            fn plays(&self) -> Result<ExPlay> {
                Ok(ExPlay::Single(create_play("sample")))
            }
        }

        let mut app = DeployApp::new(vec!["help".to_string()]);
        app.add_stack(Box::new(SampleStack {}))
            .expect("Failed to add sample stack");
    }

    #[test]
    fn test_stack_name_confliction() {
        static STACK_NAME: &str = "sample";

        struct SampleStack1;

        impl DeployStack for SampleStack1 {
            fn name(&self) -> &str {
                STACK_NAME
            }
            fn plays(&self) -> Result<ExPlay> {
                Ok(ExPlay::Single(create_play("sample1")))
            }
        }

        struct SampleStack2;

        impl DeployStack for SampleStack2 {
            fn name(&self) -> &str {
                STACK_NAME
            }
            fn plays(&self) -> Result<ExPlay> {
                Ok(ExPlay::Single(create_play("sample2")))
            }
        }

        let mut app = DeployApp::new(vec!["help".to_string()]);
        app.add_stack(Box::new(SampleStack1 {}))
            .expect("Failed to add sample stack");
        app.add_stack(Box::new(SampleStack2 {}))
            .expect_err("should be error");
    }
}
