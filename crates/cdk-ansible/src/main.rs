use anyhow::{bail, Result};
use cdk_ansible::{run, settings::SynthSettings, Synthesizer};
use cdk_ansible_core::core::{Inventory, Playbook};

struct UnimplementedCommander {}

impl Synthesizer for UnimplementedCommander {
    #[allow(unused_variables)]
    fn synth_playbooks(&self, synth_settings: &SynthSettings) -> Result<Vec<Playbook>> {
        bail!(
            "This is an original cdk-ansible command! 'synthesize' should be implemented in your custom app."
        );
    }

    #[allow(unused_variables)]
    fn synth_inventory(&self, synth_settings: &SynthSettings) -> Result<Inventory> {
        bail!(
            "This is an original cdk-ansible command! 'synthesize' should be implemented in your custom app."
        );
    }
}

fn main() -> Result<()> {
    run(std::env::args_os(), UnimplementedCommander {})?;
    Ok(())
}
