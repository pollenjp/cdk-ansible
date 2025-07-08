//! cdk-ansible is a development framework for defining Ansible applications, similar to AWS CDK.
//!
//! NOTE: The basic implementation is completely different from AWS CDK.
//!
//! cdk-ansible provides the following features:
//!
//! * Define Ansible Plays and Tasks using Rust code (wraps Ansible YAML files)
//! * Enable parallel execution with ease (wraps the `ansible-playbook` command)
//!
//! ## Example
//!
//! ```
//! use ::anyhow::Result;
//! use ::cdk_ansible::{
//!     DeployApp, DeployStack, ExeParallel, ExePlay, ExeSequential, ExeSingle, Inventory,
//!     InventoryChild, InventoryRoot, OptU, Play, PlayOptions,
//! };
//!
//! // Define a sample stack
//! struct SampleStack {
//!     exe_play: ExePlay,
//! }
//!
//! impl SampleStack {
//!     fn new(host: &str) -> Self {
//!         // Define a sample play
//!         let play = Box::new(Play {
//!             name: "sample".into(),
//!             hosts: vec![host.to_owned()].into(),
//!             options: PlayOptions::default(),
//!             tasks: vec![
//!                 // Add tasks later
//!             ],
//!         });
//!
//!         Self {
//!             exe_play: ExeSequential(vec![
//!                 ExeSingle(play.clone()),
//!                 ExeSequential(vec![ExeSingle(play.clone()), ExeSingle(play.clone())]),
//!                 ExeParallel(vec![
//!                     ExeSequential(vec![ExeSingle(play.clone()), ExeSingle(play.clone())]),
//!                     ExeSingle(play.clone()),
//!                     ExeParallel(vec![ExeSingle(play.clone()), ExeSingle(play.clone())]),
//!                 ]),
//!             ]),
//!         }
//!     }
//! }
//!
//! // Stack should implement the `DeployStack` trait
//! impl DeployStack for SampleStack {
//!     fn name(&self) -> &str {
//!         std::any::type_name::<Self>()
//!             .split("::")
//!             .last()
//!             .expect("Failed to get a stack name")
//!     }
//!
//!     fn exe_play(&self) -> &ExePlay {
//!         &self.exe_play
//!     }
//! }
//!
//! fn run() -> Result<()> {
//!     let mut app = DeployApp::new(std::env::args().collect());
//!     let inventory = Inventory {
//!         name: "inventory".into(), // generate 'inventory.yaml' file
//!         root: InventoryRoot {
//!             all: InventoryChild {
//!                 hosts: OptU::Some([("localhost".into(), None)].into_iter().collect()),
//!                 ..Default::default()
//!             },
//!         },
//!     };
//!
//!     app.add_inventory(inventory)?;
//!     app.add_stack(Box::new(SampleStack::new("localhost")))?;
//!
//!     // app.run()?  // replace `Ok(())` with `app.run()`
//!     Ok(())
//! }
//!
//! fn main() {
//!     if let Err(e) = run() {
//!         eprintln!("Error: {e:?}");
//!         std::process::exit(1);
//!     }
//! }
//! ```
//!
//! ## Tutorial
//!
//! ### Install cdk-ansible-cli
//!

pub mod arg;
pub mod settings;
pub use cdk_ansible_core::core::*;
mod deploy;
pub use deploy::*;
mod inventory;
pub use inventory::*;
mod utils;

// Re-export macros
pub use cdk_ansible_macro::*;
