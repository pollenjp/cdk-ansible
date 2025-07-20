use ::anyhow::Result;
use ::cdk_ansible::{
    AllInventoryVarsGen, AppL2, ExeParallelL2, ExePlayL2, ExeSequentialL2, ExeSingleL2,
    HostInventoryVars, HostInventoryVarsGenerator, Inventory, InventoryChild, InventoryRoot, OptU,
    PlayL2, PlayOptions, StackL2, StringOrVecString, TaskOptions,
};
use simple_sample::main as simple_sample_main;

pub fn main() {
    if let Err(e) = main2() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

pub fn main2() -> Result<()> {
    // let host_pool = HostPool {
    //     localhost: LocalHost {
    //         name: "localhost".into(),
    //     },
    //     host_a: HostA {
    //         name: "host_a".into(),
    //     },
    // };

    // let mut app = AppL2::new(std::env::args().collect());
    // app.stack(Box::new(SampleStack::new(&host_pool)))?;
    // app.run()
    Ok(())
}
