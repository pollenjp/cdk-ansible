use anyhow::{bail, Result};
use cdk_ansible::{Child, Inventory, InventoryRoot, OptU};
use cdk_ansible_macro::FieldCount;
use indexmap::IndexMap;

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
            "ansible_python_interpreter".to_owned(),
            serde_json::Value::String("/usr/bin/python3".to_owned()),
        );
        let inventory = Inventory {
            name: "inventory".to_owned(),
            root: InventoryRoot {
                all: Child {
                    hosts: OptU::Some(
                        vec![(
                            self.host_a.fqdn.clone(),
                            Some(
                                vec![debian_ansible_python_interpreter_tuple]
                                    .into_iter()
                                    .collect::<IndexMap<String, serde_json::Value>>(),
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

        let OptU::Some(hosts) = inventory.root.all.hosts.clone() else {
            bail!("hosts is not set");
        };

        // Validate: count Hosts' attributes
        if hosts.len() != Self::field_count() {
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

#[expect(clippy::unnecessary_wraps, reason = "use anyhow::Result interface")]
pub fn get_hosts() -> Result<Hosts> {
    Ok(Hosts {
        host_a: {
            let name = "host-a".to_owned();
            HostA {
                name: name.clone(),
                fqdn: format!("{}.example.com", &name),
            }
        },
    })
}
