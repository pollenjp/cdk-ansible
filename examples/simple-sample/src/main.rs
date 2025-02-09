use anyhow::Result;
use cdk_ansible::{run, settings::SynthSettings, Synthesizer};
use cdk_ansible::{Inventory, Playbook};
use simple_sample::inventory::get_hosts;
use simple_sample::playbooks::generate_all_playbooks;
use simple_sample::PlaybookSynthConfig;

struct CustomSynthesizer {}

impl Synthesizer for CustomSynthesizer {
    fn synth_playbooks(&self, synth_settings: &SynthSettings) -> Result<Vec<Playbook>> {
        let playbook_gen_config = PlaybookSynthConfig {
            synth_settings: synth_settings.clone(),
            hosts: get_hosts()?,
        };
        let playbooks = generate_all_playbooks(&playbook_gen_config)?;
        Ok(playbooks)
    }

    fn synth_inventory(&self, synth_settings: &SynthSettings) -> Result<Inventory> {
        dbg!(&synth_settings);
        let hosts = get_hosts()?;
        let inventory = hosts.to_inventory()?;
        Ok(inventory)
    }
}

fn main() -> Result<()> {
    run(std::env::args_os(), CustomSynthesizer {})?;
    Ok(())
}
