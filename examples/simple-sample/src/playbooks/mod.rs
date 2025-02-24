use anyhow::{bail, Result};
use cdk_ansible::Playbook;
use std::collections::HashSet;
mod playbook1;
mod playbook2;

use crate::PlaybookGenArgs;

/// Generate all playbooks and return them
pub fn generate_all<T: PlaybookGenArgs>(args: &T) -> Result<Vec<Playbook>> {
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
                name: "playbook".to_owned(),
                plays: vec![],
            },
            Playbook {
                name: "playbook".to_owned(),
                plays: vec![],
            },
        ];
        #[expect(clippy::unreachable, reason = "should be an error")]
        match validate_playbook_names(&playbooks) {
            Ok(()) => unreachable!("should be an error"),
            Err(e) => assert_eq!(e.to_string(), "Playbook names are not unique"),
        }
    }
}
