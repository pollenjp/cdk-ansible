//! # Example
//!
//! This is an example of how to use the `HostInventoryVarsGenerator` trait to generate inventory variables.
//! Defined at `test_macro.rs` in `test-suite` crate.
//!
//! ```
//! use ::anyhow::Result;
//! use ::cdk_ansible::{
//!     AllInventoryVarsGen, HostInventoryVars, HostInventoryVarsGenerator, Inventory,
//!     InventoryChild, InventoryRoot, OptU,
//! };
//! use ::indexmap::IndexMap;
//!
//! struct HostA {
//!     pub aaa: String,
//!     pub bbb: String,
//! }
//!
//! impl HostInventoryVarsGenerator for HostA {
//!     fn gen_host_vars(&self) -> Result<HostInventoryVars> {
//!         Ok(HostInventoryVars {
//!             ansible_host: "aaa.example.com".to_string(),
//!             inventory_vars: vec![
//!                 ("aaa".to_string(), self.aaa.clone().into()),
//!                 ("bbb".to_string(), self.bbb.clone().into()),
//!             ],
//!         })
//!     }
//! }
//!
//! struct HostB {
//!     pub aaa: String,
//!     pub bbb: String,
//! }
//!
//! impl HostInventoryVarsGenerator for HostB {
//!     fn gen_host_vars(&self) -> Result<HostInventoryVars> {
//!         Ok(HostInventoryVars {
//!             ansible_host: "bbb.example.com".to_string(),
//!             inventory_vars: vec![
//!                 ("aaa".to_string(), self.aaa.clone().into()),
//!                 ("bbb".to_string(), self.bbb.clone().into()),
//!             ],
//!         })
//!     }
//! }
//!
//! #[derive(AllInventoryVarsGen)]
//! struct HostPool {
//!     pub aaa: HostA,
//!     pub bbb: HostB,
//! }
//!
//! impl HostPool {
//!     pub fn to_inventory(&self) -> Result<Inventory> {
//!         let inventory = Inventory {
//!             name: "inventory".to_string(),
//!             root: InventoryRoot {
//!                 all: InventoryChild {
//!                     hosts: OptU::Some(
//!                         self.inventory_vars()?
//!                             .into_iter()
//!                             .map(|host_inventory_vars| {
//!                                 (
//!                                 host_inventory_vars.ansible_host,
//!                                 Some(
//!                                     host_inventory_vars
//!                                         .inventory_vars
//!                                         .into_iter()
//!                                         .collect::<IndexMap<String, ::serde_json::Value>>(),
//!                                 ),
//!                             )
//!                             })
//!                             .collect(),
//!                     ),
//!                     ..Default::default()
//!                 },
//!             },
//!         };
//!         Ok(inventory)
//!     }
//! }
//!
//! fn test_host_pool_to_inventory() {
//!     let host_pool = HostPool {
//!         aaa: HostA {
//!             aaa: "aaa".to_string(),
//!             bbb: "bbb".to_string(),
//!         },
//!         bbb: HostB {
//!             aaa: "aaa".to_string(),
//!             bbb: "bbb".to_string(),
//!         },
//!     };
//!
//!     assert_eq!(
//!         host_pool
//!             .to_inventory()
//!             .expect("failed to convert to inventory"),
//!         Inventory {
//!             name: "inventory".to_string(),
//!             root: InventoryRoot {
//!                 all: InventoryChild {
//!                     hosts: OptU::Some(IndexMap::from([
//!                         (
//!                             "aaa.example.com".to_string(),
//!                             Some(IndexMap::from([
//!                                 ("aaa".to_string(), "aaa".to_string().into()),
//!                                 ("bbb".to_string(), "bbb".to_string().into())
//!                             ]))
//!                         ),
//!                         (
//!                             "bbb.example.com".to_string(),
//!                             Some(IndexMap::from([
//!                                 ("aaa".to_string(), "aaa".to_string().into()),
//!                                 ("bbb".to_string(), "bbb".to_string().into())
//!                             ]))
//!                         ),
//!                     ])),
//!                     ..Default::default()
//!                 },
//!             },
//!         }
//!     );
//! }
//!
//! test_host_pool_to_inventory();
//! ```
//!

extern crate alloc;

use alloc::rc::Rc;
use anyhow::Result;
use cdk_ansible_core::core::{InventoryHosts, InventoryVars};
use core::cell::RefCell;

/// Define `inventory_vars()` method to a struct for pooling hosts.
/// This method returns a vector of `Result<HostInventoryVars>`.
/// Use from [`cdk_ansible_macro::AllInventoryVarsGen`] derive macro.
///
#[expect(
    clippy::exhaustive_structs,
    reason = "this struct is only used for generating inventory variables"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostInventoryVars {
    pub ansible_host: String,
    pub inventory_vars: Vec<(String, ::serde_json::Value)>,
}

impl FromIterator<HostInventoryVars> for InventoryHosts {
    fn from_iter<T: IntoIterator<Item = HostInventoryVars>>(iter: T) -> Self {
        iter.into_iter()
            .map(|host_inventory_vars| {
                (
                    host_inventory_vars.ansible_host,
                    if host_inventory_vars.inventory_vars.is_empty() {
                        None
                    } else {
                        Some(
                            host_inventory_vars
                                .inventory_vars
                                .into_iter()
                                .collect::<InventoryVars>(),
                        )
                    },
                )
            })
            .collect::<InventoryHosts>()
    }
}

/// This trait should be implemented by each host to generate inventory variables.
/// [`cdk_ansible_macro::AllInventoryVarsGen`] derive macro will use this trait to generate inventory variables.
#[diagnostic::on_unimplemented(
    note = "Each host should implement 'HostInventoryVarsGenerator' trait"
)]
pub trait HostInventoryVarsGenerator {
    fn gen_host_vars(&self) -> Result<HostInventoryVars>;
}

/// Generate a host inventory vars for ansible inventory
/// Should be used at [`cdk_ansible_macro::AllInventoryVarsGen`]
#[inline]
pub fn get_host_inventory_vars<T>(value: &T) -> Result<HostInventoryVars>
where
    T: ?Sized + HostInventoryVarsGenerator,
{
    value.gen_host_vars()
}

/// Generate a host inventory vars for ansible inventory
/// Should be used at [`cdk_ansible_macro::AllInventoryVarsGen`]
#[inline]
pub fn get_host_inventory_vars_rc<T>(value: &Rc<T>) -> Result<HostInventoryVars>
where
    T: ?Sized + HostInventoryVarsGenerator,
{
    value.gen_host_vars()
}

/// Generate a host inventory vars for ansible inventory
/// Should be used at [`cdk_ansible_macro::AllInventoryVarsGen`]
#[inline]
pub fn get_host_inventory_vars_ref_cell<T>(value: &RefCell<T>) -> Result<HostInventoryVars>
where
    T: ?Sized + HostInventoryVarsGenerator,
{
    value.borrow().gen_host_vars()
}
