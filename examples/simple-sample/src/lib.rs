mod utils;
use ::anyhow::Result;
use ::cdk_ansible::{
    AllInventoryVarsGen, App, ExeParallel, ExePlay, ExeSequential, ExeSingle, HostInventoryVars,
    HostInventoryVarsGenerator, Inventory, InventoryChild, InventoryRoot, OptU, Stack,
};
pub use utils::*;

#[inline]
pub fn main() {
    if let Err(e) = main2() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

pub fn main2() -> Result<()> {
    let host_pool = HostPool {
        localhost: LocalHost {
            name: "localhost".into(),
        },
        host_a: HostA {
            name: "host_a".into(),
        },
    };

    let mut app = App::new(std::env::args().collect());
    app.add_inventory(host_pool.to_inventory()?)?;
    app.add_stack(Box::new(SampleStack::new(&host_pool)?))?;
    app.run()
}

struct SampleStack {
    exe_play: ExePlay,
}

impl SampleStack {
    fn new(hp: &HostPool) -> Result<Self> {
        let hosts = hp.localhost.name.as_str();

        Ok(Self {
            exe_play: ExeSequential(vec![
                ExeParallel(vec![
                    ExeParallel(vec![
                        ExeSingle(create_play_helper("sample0", hosts.into(), 5)?.into()),
                        ExeSingle(create_play_helper("sample1", hosts.into(), 10)?.into()),
                        ExeSingle(create_play_helper("sample2", hosts.into(), 15)?.into()),
                    ]),
                    ExeSequential(vec![
                        ExeSingle(create_play_helper("sample3", hosts.into(), 1)?.into()),
                        ExeSingle(create_play_helper("sample4", hosts.into(), 1)?.into()),
                        ExeSingle(create_play_helper("sample5", hosts.into(), 1)?.into()),
                    ]),
                    ExeSingle(create_play_helper("sample6", hosts.into(), 1)?.into()),
                ]),
                ExeSequential(vec![
                    ExeSingle(create_play_helper("sample7", hosts.into(), 1)?.into()),
                    ExeSingle(create_play_helper("sample8", hosts.into(), 1)?.into()),
                    ExeSingle(create_play_helper("sample9", hosts.into(), 1)?.into()),
                ]),
            ]),
        })
    }
}

impl Stack for SampleStack {
    /// TODO: May be converted to derive macro in the future
    #[expect(clippy::expect_used, reason = "Logical failure")]
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .expect("Failed to get a stack name")
    }

    fn exe_play(&self) -> &ExePlay {
        &self.exe_play
    }
}

#[derive(AllInventoryVarsGen)]
struct HostPool {
    pub localhost: LocalHost,
    pub host_a: HostA,
}

impl HostPool {
    fn to_inventory(&self) -> Result<Inventory> {
        Ok(Inventory {
            name: "dev".into(), // generate 'dev.yaml' file
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
            inventory_vars: vec![("ansible_connection".into(), "local".into())],
        })
    }
}

struct HostA {
    name: String,
}

impl HostInventoryVarsGenerator for HostA {
    fn gen_host_vars(&self) -> Result<HostInventoryVars> {
        Ok(HostInventoryVars {
            ansible_host: self.name.clone(),
            inventory_vars: vec![],
        })
    }
}
