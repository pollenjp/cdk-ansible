#[cfg(test)]
mod test {
    use ::anyhow::Result;
    use ::cdk_ansible::{
        App, ExeParallel, ExePlay, ExeSequential, ExeSingle, Inventory, InventoryChild,
        InventoryRoot, OptU, Play, PlayOptions, Stack,
    };

    // Define a sample stack
    struct SampleStack {
        exe_play: ExePlay,
    }

    impl SampleStack {
        fn new(host: &str) -> Self {
            // Define a sample play
            let play = Box::new(Play {
                name: "sample".into(),
                hosts: vec![host.to_owned()].into(),
                options: PlayOptions::default(),
                tasks: vec![
                    // Add tasks later
                ],
            });

            Self {
                exe_play: ExeSequential(vec![
                    ExeSingle(play.clone()),
                    ExeSequential(vec![ExeSingle(play.clone()), ExeSingle(play.clone())]),
                    ExeParallel(vec![
                        ExeSequential(vec![ExeSingle(play.clone()), ExeSingle(play.clone())]),
                        ExeSingle(play.clone()),
                        ExeParallel(vec![ExeSingle(play.clone()), ExeSingle(play.clone())]),
                    ]),
                ]),
            }
        }
    }

    // Stack should implement the `Stack` trait
    impl Stack for SampleStack {
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

    fn run() -> Result<()> {
        let mut app = App::new(std::env::args().collect());
        let inventory = Inventory {
            name: "inventory".into(), // generate 'inventory.yaml' file
            root: InventoryRoot {
                all: InventoryChild {
                    hosts: OptU::Some([("localhost".into(), None)].into_iter().collect()),
                    ..Default::default()
                },
            },
        };

        app.add_inventory(inventory)?;
        app.add_stack(Box::new(SampleStack::new("localhost")))?;

        // app.run()?  // replace `Ok(())` with `app.run()`
        Ok(())
    }

    #[test]
    fn test_main() {
        assert!(run().is_ok());
    }
}
