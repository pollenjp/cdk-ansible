#![allow(clippy::unwrap_used, reason = "test code")]
#![allow(clippy::expect_used, reason = "test code")]

#[cfg(test)]
mod test {
    extern crate alloc;
    use alloc::rc::{Rc, Weak};
    use anyhow::Result;
    use core::cell::RefCell;

    use cdk_ansible::{AllInventoryVarsGen, HostInventoryVars, HostInventoryVarsGenerator};

    mod simple_host {
        use super::*;

        #[derive(AllInventoryVarsGen)]
        struct HostPool {
            pub a: HostA,
            pub b: HostB,
            pub c: Rc<HostC>,
            pub d: RefCell<HostD>,
        }

        struct HostA {
            pub var1: String,
            pub var2: String,
        }

        impl HostInventoryVarsGenerator for HostA {
            fn gen_host_vars(&self) -> Result<HostInventoryVars> {
                Ok(HostInventoryVars {
                    ansible_host: "a.example.com".to_owned(),
                    inventory_vars: vec![
                        ("var1".to_owned(), self.var1.clone().into()),
                        ("var2".to_owned(), self.var2.clone().into()),
                    ],
                })
            }
        }

        struct HostB {
            pub var1: String,
            pub var2: String,
        }

        impl HostInventoryVarsGenerator for HostB {
            fn gen_host_vars(&self) -> Result<HostInventoryVars> {
                Ok(HostInventoryVars {
                    ansible_host: "b.example.com".to_owned(),
                    inventory_vars: vec![
                        ("var1".to_owned(), self.var1.clone().into()),
                        ("var2".to_owned(), self.var2.clone().into()),
                    ],
                })
            }
        }

        struct HostC {
            pub var1: String,
            pub var2: String,
        }

        impl HostInventoryVarsGenerator for HostC {
            fn gen_host_vars(&self) -> Result<HostInventoryVars> {
                Ok(HostInventoryVars {
                    ansible_host: "c.example.com".to_owned(),
                    inventory_vars: vec![
                        ("var1".to_owned(), self.var1.clone().into()),
                        ("var2".to_owned(), self.var2.clone().into()),
                    ],
                })
            }
        }

        struct HostD {
            pub var1: String,
            pub var2: String,
        }

        impl HostInventoryVarsGenerator for HostD {
            fn gen_host_vars(&self) -> Result<HostInventoryVars> {
                Ok(HostInventoryVars {
                    ansible_host: "d.example.com".to_owned(),
                    inventory_vars: vec![
                        ("var1".to_owned(), self.var1.clone().into()),
                        ("var2".to_owned(), self.var2.clone().into()),
                    ],
                })
            }
        }

        #[test]
        fn test_all_inventory_vars_gen_derive() {
            use simple_host::*;

            let d = Rc::new(HostC {
                var1: "xxx".to_owned(),
                var2: "yyy".to_owned(),
            });
            let host_pool = HostPool {
                a: HostA {
                    var1: "xxx".to_owned(),
                    var2: "yyy".to_owned(),
                },
                b: HostB {
                    var1: "xxx".to_owned(),
                    var2: "yyy".to_owned(),
                },
                c: d,
                d: RefCell::new(HostD {
                    var1: "xxx".to_owned(),
                    var2: "yyy".to_owned(),
                }),
            };

            assert_eq!(
                host_pool
                    .inventory_vars()
                    .expect("failed to collect inventory vars"),
                vec![
                    HostInventoryVars {
                        ansible_host: "a.example.com".to_owned(),
                        inventory_vars: vec![
                            ("var1".to_owned(), "xxx".into()),
                            ("var2".to_owned(), "yyy".into()),
                        ],
                    },
                    HostInventoryVars {
                        ansible_host: "b.example.com".to_owned(),
                        inventory_vars: vec![
                            ("var1".to_owned(), "xxx".into()),
                            ("var2".to_owned(), "yyy".into()),
                        ],
                    },
                    HostInventoryVars {
                        ansible_host: "c.example.com".to_owned(),
                        inventory_vars: vec![
                            ("var1".to_owned(), "xxx".into()),
                            ("var2".to_owned(), "yyy".into()),
                        ],
                    },
                    HostInventoryVars {
                        ansible_host: "d.example.com".to_owned(),
                        inventory_vars: vec![
                            ("var1".to_owned(), "xxx".into()),
                            ("var2".to_owned(), "yyy".into()),
                        ],
                    },
                ]
            );
        }
    }

    mod parent_child_host {
        use super::*;

        #[derive(AllInventoryVarsGen)]
        struct RcTestHostPool {
            pub a: Rc<RcTestHostA>,
            pub b: Rc<RcTestHostB>,
        }

        /// Base trait for Host
        pub trait RcTestHost {
            fn some_var(&self) -> String;
        }

        struct RcTestHostA {
            pub some_var: String,
            pub children: RefCell<Vec<Weak<dyn RcTestHost>>>,
        }

        impl RcTestHost for RcTestHostA {
            fn some_var(&self) -> String {
                self.some_var.clone()
            }
        }

        impl HostInventoryVarsGenerator for RcTestHostA {
            fn gen_host_vars(&self) -> Result<HostInventoryVars> {
                let some_var_val = self
                    .children
                    .borrow()
                    .first()
                    .ok_or_else(|| anyhow::anyhow!("no children"))?
                    .upgrade()
                    .ok_or_else(|| anyhow::anyhow!("can't upgrade"))?
                    .some_var();

                Ok(HostInventoryVars {
                    ansible_host: "A".to_owned(),
                    inventory_vars: vec![("B.some_var".to_owned(), some_var_val.into())],
                })
            }
        }

        struct RcTestHostB {
            pub some_var: String,
            pub parent: RefCell<Option<Weak<dyn RcTestHost>>>,
        }

        impl RcTestHost for RcTestHostB {
            fn some_var(&self) -> String {
                self.some_var.clone()
            }
        }

        impl HostInventoryVarsGenerator for RcTestHostB {
            fn gen_host_vars(&self) -> Result<HostInventoryVars> {
                let some_var_val = self
                    .parent
                    .borrow()
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("no parent"))?
                    .upgrade()
                    .ok_or_else(|| anyhow::anyhow!("can't upgrade"))?
                    .some_var();

                Ok(HostInventoryVars {
                    ansible_host: "B".to_owned(),
                    inventory_vars: vec![("A.some_var".to_owned(), some_var_val.into())],
                })
            }
        }

        #[expect(
            clippy::as_conversions,
            reason = "Need explicit upcasting for Weak<dyn Trait>"
        )]
        #[test]
        fn test_all_inventory_vars_gen_derive_rc() {
            use parent_child_host::*;

            let a = Rc::new(RcTestHostA {
                some_var: "xxx".to_owned(),
                children: RefCell::new(vec![]),
            });
            let b = Rc::new(RcTestHostB {
                some_var: "yyy".to_owned(),
                parent: RefCell::new(None),
            });
            a.children
                .borrow_mut()
                .push(Rc::downgrade(&b) as Weak<dyn RcTestHost>);
            b.parent
                .borrow_mut()
                .replace(Rc::downgrade(&a) as Weak<dyn RcTestHost>);

            let hosts = RcTestHostPool { a, b };

            assert_eq!(
                hosts
                    .inventory_vars()
                    .expect("failed to collect inventory vars"),
                vec![
                    HostInventoryVars {
                        ansible_host: "A".to_owned(),
                        inventory_vars: vec![("B.some_var".to_owned(), "yyy".into())],
                    },
                    HostInventoryVars {
                        ansible_host: "B".to_owned(),
                        inventory_vars: vec![("A.some_var".to_owned(), "xxx".into())],
                    },
                ]
            );
        }
    }

    mod host_pool_to_inventory {
        use ::anyhow::Result;
        use ::cdk_ansible::{
            AllInventoryVarsGen, HostInventoryVars, HostInventoryVarsGenerator, Inventory,
            InventoryChild, InventoryRoot, OptU,
        };
        use ::indexmap::IndexMap;

        struct HostA {
            pub aaa: String,
            pub bbb: String,
        }

        impl HostInventoryVarsGenerator for HostA {
            fn gen_host_vars(&self) -> Result<HostInventoryVars> {
                Ok(HostInventoryVars {
                    ansible_host: "aaa.example.com".to_string(),
                    inventory_vars: vec![
                        ("aaa".to_string(), self.aaa.clone().into()),
                        ("bbb".to_string(), self.bbb.clone().into()),
                    ],
                })
            }
        }

        struct HostB {
            pub aaa: String,
            pub bbb: String,
        }

        impl HostInventoryVarsGenerator for HostB {
            fn gen_host_vars(&self) -> Result<HostInventoryVars> {
                Ok(HostInventoryVars {
                    ansible_host: "bbb.example.com".to_string(),
                    inventory_vars: vec![
                        ("aaa".to_string(), self.aaa.clone().into()),
                        ("bbb".to_string(), self.bbb.clone().into()),
                    ],
                })
            }
        }

        #[derive(AllInventoryVarsGen)]
        struct HostPool {
            pub aaa: HostA,
            pub bbb: HostB,
        }

        impl HostPool {
            pub fn to_inventory(&self) -> Result<Inventory> {
                let inventory = Inventory {
                    name: "inventory".to_string(),
                    root: InventoryRoot {
                        all: InventoryChild {
                            hosts: OptU::Some(
                                self.inventory_vars()?
                                    .into_iter()
                                    .map(|host_inventory_vars| {
                                        (
                                        host_inventory_vars.ansible_host,
                                        Some(
                                            host_inventory_vars
                                                .inventory_vars
                                                .into_iter()
                                                .collect::<IndexMap<String, ::serde_json::Value>>(),
                                        ),
                                    )
                                    })
                                    .collect(),
                            ),
                            ..Default::default()
                        },
                    },
                };
                Ok(inventory)
            }
        }

        #[test]
        fn test_host_pool_to_inventory() {
            let host_pool = HostPool {
                aaa: HostA {
                    aaa: "aaa".to_string(),
                    bbb: "bbb".to_string(),
                },
                bbb: HostB {
                    aaa: "aaa".to_string(),
                    bbb: "bbb".to_string(),
                },
            };

            assert_eq!(
                host_pool
                    .to_inventory()
                    .expect("failed to convert to inventory"),
                Inventory {
                    name: "inventory".to_string(),
                    root: InventoryRoot {
                        all: InventoryChild {
                            hosts: OptU::Some(IndexMap::from([
                                (
                                    "aaa.example.com".to_string(),
                                    Some(IndexMap::from([
                                        ("aaa".to_string(), "aaa".to_string().into()),
                                        ("bbb".to_string(), "bbb".to_string().into())
                                    ]))
                                ),
                                (
                                    "bbb.example.com".to_string(),
                                    Some(IndexMap::from([
                                        ("aaa".to_string(), "aaa".to_string().into()),
                                        ("bbb".to_string(), "bbb".to_string().into())
                                    ]))
                                ),
                            ])),
                            ..Default::default()
                        },
                    },
                }
            );
        }
    }
}
