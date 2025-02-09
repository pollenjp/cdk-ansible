use anyhow::{bail, Result};
use cdk_ansible::Playbook;
use std::collections::HashSet;
mod playbook1;
mod playbook2;

use crate::PlaybookGenArgs;

/// Generate all playbooks and return them
pub fn generate_all_playbooks(args: &impl PlaybookGenArgs) -> Result<Vec<Playbook>> {
    let playbooks = vec![playbook1::playbook1(args)?, playbook2::playbook2(args)?];

    // validate playbook names are unique
    validate_playbook_names(&playbooks)?;

    Ok(playbooks)
}

/// Validate playbook names are unique
/// 'cdk-ansible' crate does not check this. (May future works)
fn validate_playbook_names(playbooks: &[Playbook]) -> Result<()> {
    let playbook_names = playbooks
        .iter()
        .map(|playbook| playbook.name.clone())
        .collect::<HashSet<_>>();
    if playbooks.len() != playbook_names.len() {
        bail!("Playbook names are not unique")
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_playbook_names() {
        let playbooks = vec![
            Playbook {
                name: "playbook".to_string(),
                plays: vec![],
            },
            Playbook {
                name: "playbook".to_string(),
                plays: vec![],
            },
        ];
        let err = validate_playbook_names(&playbooks).unwrap_err();
        assert_eq!(err.to_string(), "Playbook names are not unique");
    }
}
