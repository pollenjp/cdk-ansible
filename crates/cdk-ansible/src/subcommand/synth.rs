use crate::arg::SynthArgs;
use crate::settings;
use crate::{Inventory, Playbook, playbook_dump};
use anyhow::{Context, Result};

pub trait Synthesizer {
    fn synth_playbooks(&self, args: &settings::SynthSettings) -> Result<Vec<Playbook>>;
    fn synth_inventory(&self, args: &settings::SynthSettings) -> Result<Inventory>;
}

pub(crate) fn synth(synthesizer: &dyn Synthesizer, args: SynthArgs) -> Result<()> {
    let synth_settings = settings::SynthSettings::resolve(args);

    // Playbooks

    if synth_settings.playbook_dir.exists() {
        std::fs::remove_dir_all(&synth_settings.playbook_dir).with_context(|| {
            format!(
                "Failed to remove existing playbook directory: {:?}",
                &synth_settings.playbook_dir
            )
        })?;
    }
    let playbooks = synthesizer
        .synth_playbooks(&synth_settings)
        .with_context(|| {
            format!(
                "Failed to synthesize playbooks: {:?}",
                &synth_settings.playbook_dir
            )
        })?;
    for playbook in playbooks.iter() {
        playbook_dump(&synth_settings, playbook).with_context(|| {
            format!(
                "Failed to dump playbook: {:?}",
                &synth_settings
                    .playbook_dir
                    .join(format!("{}.json", playbook.name))
            )
        })?;
    }

    // Inventory

    let inventory = synthesizer.synth_inventory(&synth_settings)?;
    let inventory_file = synth_settings
        .inventory_dir
        .join(format!("{}.json", &inventory.name));
    std::fs::create_dir_all(inventory_file.parent().unwrap()).with_context(|| {
        format!(
            "Failed to create directory: {:?}",
            inventory_file.parent().unwrap()
        )
    })?;
    std::fs::write(
        &inventory_file,
        serde_json::to_string(&inventory.root)
            .with_context(|| format!("Failed to serialize inventory: {:?}", &inventory_file))?,
    )
    .with_context(|| format!("Failed to write inventory file: {:?}", &inventory_file))?;

    Ok(())
}
