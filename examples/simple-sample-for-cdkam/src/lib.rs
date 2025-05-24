use anyhow::Result;
use cdk_ansible::settings;
use cdk_ansible::{run, settings::SynthSettings, Synthesizer};
use cdk_ansible::{Inventory, Playbook};

mod inventory;
mod playbooks;

use inventory::get_hosts;
use playbooks::generate_all;

#[inline]
pub fn main() -> Result<()> {
    run(std::env::args_os(), CustomSynthesizer {})?;
    Ok(())
}

struct CustomSynthesizer;

impl Synthesizer for CustomSynthesizer {
    fn synth_playbooks(&self, args: &SynthSettings) -> Result<Vec<Playbook>> {
        let playbooks = generate_all(&PlaybookSynthConfig {
            synth_settings: args.clone(),
            hosts: get_hosts()?,
        })?;
        Ok(playbooks)
    }

    fn synth_inventory(&self, args: &SynthSettings) -> Result<Inventory> {
        dbg!(&args); // FIXME: remove this
        let hosts = get_hosts()?;
        let inventory = hosts.to_inventory()?;
        Ok(inventory)
    }
}

trait PlaybookGenArgs {
    #[expect(dead_code, reason = "not used yet")]
    fn get_synth_settings(&self) -> &settings::SynthSettings;
    fn get_hosts(&self) -> &inventory::Hosts;
}

/// A simple struct to implement [`PlaybookGenArgs`]
#[derive(Debug, Clone)]
struct PlaybookSynthConfig {
    #[expect(dead_code, reason = "not used yet")]
    pub synth_settings: settings::SynthSettings,
    pub hosts: inventory::Hosts,
}

impl PlaybookGenArgs for PlaybookSynthConfig {
    fn get_synth_settings(&self) -> &settings::SynthSettings {
        &self.synth_settings
    }

    fn get_hosts(&self) -> &inventory::Hosts {
        &self.hosts
    }
}
