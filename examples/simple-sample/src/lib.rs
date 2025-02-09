use cdk_ansible::settings;

pub mod inventory;
pub mod playbooks;

pub trait PlaybookGenArgs {
    fn get_synth_settings(&self) -> &settings::SynthSettings;
    fn get_hosts(&self) -> &inventory::Hosts;
}

/// A simple struct to implement [`PlaybookGenArgs`]
#[derive(Debug, Clone)]
pub struct PlaybookSynthConfig {
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
