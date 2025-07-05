use ::anyhow::Result;
use ::cdk_ansible::{
    AllInventoryVarsGen, DeployApp, HostInventoryVars, HostInventoryVarsGenerator, Inventory,
    InventoryChild, InventoryRoot, OptU,
};

mod sample_stack;
use sample_stack::SampleStack;

#[inline]
pub fn run() -> Result<()> {
    let host_pool = HostPool {
        localhost: LocalHost {
            name: "localhost".into(),
        },
    };

    let mut app = DeployApp::new(std::env::args().collect());
    app.add_inventory(host_pool.to_inventory()?)?;
    app.add_stack(Box::new(SampleStack::new(&host_pool)))?;
    app.run()
}

#[derive(AllInventoryVarsGen)]
struct HostPool {
    pub localhost: LocalHost,
}

impl HostPool {
    fn to_inventory(&self) -> Result<Inventory> {
        Ok(Inventory {
            name: "inventory".into(), // generate 'inventory.yaml' file
            root: InventoryRoot {
                all: InventoryChild {
                    hosts: OptU::Some(self.inventory_vars()?.into_iter().collect()),
                    ..Default::default()
                },
            },
        })
    }
}

struct LocalHost {
    name: String,
}

impl HostInventoryVarsGenerator for LocalHost {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.name.clone(),
            inventory_vars: vec![],
        })
    }
}
