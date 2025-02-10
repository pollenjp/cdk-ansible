use anyhow::{bail, Result};
use cdk_ansible::{Child, Inventory, InventoryRoot, OptU};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostName {
    HostA,
}

use cdk_ansible_macro::FieldCount;

#[derive(Default, Debug, Clone, PartialEq, Eq, FieldCount)]
pub struct Hosts {
    pub host_a: HostA,
}

impl Hosts {
    /// Generate an inventory file for ansible
    ///
    /// ```yaml
    /// ---
    /// all:
    ///   hosts:
    ///     'fqdn': null
    /// ```
    ///
    pub fn to_inventory(&self) -> Result<Inventory> {
        let debian_ansible_python_interpreter_tuple = (
            "ansible_python_interpreter".to_string(),
            serde_json::Value::String("/usr/bin/python3".to_string()),
        );
        let inventory = Inventory {
            name: "inventory".to_string(),
            root: InventoryRoot {
                all: Child {
                    hosts: OptU::Some(
                        vec![(
                            self.host_a.fqdn.clone(),
                            Some(
                                vec![debian_ansible_python_interpreter_tuple.clone()]
                                    .into_iter()
                                    .collect::<serde_json::Map<String, serde_json::Value>>(),
                            ),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                    children: OptU::Unset,
                    vars: OptU::Unset,
                },
            },
        };

        let hosts = if let OptU::Some(hosts) = &inventory.root.all.hosts {
            hosts.clone()
        } else {
            bail!("hosts is not set");
        };

        // Validate: count Hosts' attributes
        if Hosts::field_count() != hosts.len() {
            bail!("Some hosts are not set");
        }

        Ok(inventory)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct HostA {
    pub name: String,
    pub fqdn: String,
}

pub fn get_hosts() -> Result<Hosts> {
    Ok(Hosts {
        host_a: {
            let name = "host-a".to_string();
            HostA {
                name: name.clone(),
                fqdn: format!("{}.example.com", &name),
            }
        },
    })
}
