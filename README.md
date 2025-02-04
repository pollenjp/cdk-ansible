# CDK Ansible by Rust

[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/cdk-ansible.svg
[crates-url]: https://crates.io/crates/cdk-ansible

`cdk-ansible` is a CDK (Cloud Development Kit) for Ansible, and inspired by AWS CDK.

While Ansible's `playbook` and `inventory` files are written in YAML format, managing YAML templating can be challenging.
`cdk-ansible` enables you to generate Ansible files using **Rust** as a type-safe programming language.

## Features

- `cdk-ansible` generates Ansible **Playbook** and **Inventory** files.
