use crate::{l2::deploy::StackL2, types::StackName};
use anyhow::Result;
use indexmap::IndexMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct StackContainer {
    /// key is an unique name of stack.
    stacks: IndexMap<StackName, Arc<dyn StackL2>>,
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
        }
    }

    pub fn stack(mut self, stack: Arc<dyn StackL2>) -> Result<Self> {
        let old_stack = self.stacks.insert(stack.name().into(), stack);
        if let Some(old_stack) = old_stack {
            anyhow::bail!("conflicting stack name: {}", old_stack.name());
        }
        Ok(self)
    }

    pub fn get_stack(&self, name: &StackName) -> Option<Arc<dyn StackL2>> {
        self.stacks.get(name).map(Arc::clone)
    }

    pub fn get_stacks(&self) -> impl Iterator<Item = Arc<dyn StackL2>> {
        self.stacks.values().map(Arc::clone)
    }
}
