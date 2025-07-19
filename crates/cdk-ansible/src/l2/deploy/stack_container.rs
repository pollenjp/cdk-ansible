use crate::{
    l2::deploy::StackL2,
    types::{ExePlaybook, StackName},
};
use anyhow::Result;
use indexmap::IndexMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct StackContainer {
    /// key is an unique name of stack.
    stacks: IndexMap<StackName, Arc<dyn StackL2>>,
    /// Don't use this directly. Use [`App::exe_playbooks`] method.
    /// Memoization of ExePlaybooks.
    /// key is an unique name of stack. Forbidden to be duplicated.
    #[doc(hidden)]
    exe_playbooks: IndexMap<StackName, ExePlaybook>,
}

impl std::fmt::Debug for StackContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StackContainer")
            .field("stacks", &self.stacks.keys())
            .finish()
    }
}

impl StackContainer {
    pub fn new() -> Self {
        Self {
            stacks: IndexMap::new(),
            exe_playbooks: IndexMap::new(),
        }
    }

    pub fn stack(mut self, stack: Arc<dyn StackL2>) -> Result<Self> {
        // Memoization of ExePlaybook
        let old_exe_playbook = self.exe_playbooks.insert(
            stack.name().into(),
            ExePlaybook::from_exe_play(stack.name(), stack.exe_play().clone()),
        );
        if let Some(old_exe_playbook) = old_exe_playbook {
            anyhow::bail!(
                "conflicting stack name: {} ({:?})",
                stack.name(),
                old_exe_playbook
            );
        }

        let old_stack = self.stacks.insert(stack.name().into(), stack);
        if let Some(old_stack) = old_stack {
            anyhow::bail!("conflicting stack name: {}", old_stack.name());
        }
        Ok(self)
    }

    pub fn exe_playbooks(&self) -> &IndexMap<StackName, ExePlaybook> {
        &self.exe_playbooks
    }
}
