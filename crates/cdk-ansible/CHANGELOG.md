# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.7](https://github.com/pollenjp/cdk-ansible/compare/cdk-ansible-v0.2.6...cdk-ansible-v0.2.7) - 2025-07-24 (cdk-ansible)

### Other

- Add mermaid for L2 app

## [0.2.6](https://github.com/pollenjp/cdk-ansible/compare/cdk-ansible-v0.2.5...cdk-ansible-v0.2.6) - 2025-07-22 (cdk-ansible)

### Added

- add L2 features (AppL2, StackL2, LazyPlayL2, PlayL2) ([#89](https://github.com/pollenjp/cdk-ansible/pull/89))

### Other

- add image

## [0.2.5](https://github.com/pollenjp/cdk-ansible/compare/cdk-ansible-v0.2.4...cdk-ansible-v0.2.5) - 2025-07-17 (cdk-ansible)

### Other

- change push arg as ExePlay not Play

## [0.2.4](https://github.com/pollenjp/cdk-ansible/compare/cdk-ansible-v0.2.3...cdk-ansible-v0.2.4) - 2025-07-17 (cdk-ansible)

### Added

- IntoExePlaySequential and IntoExePlayParallel trait ([#64](https://github.com/pollenjp/cdk-ansible/pull/64))

## [0.2.3](https://github.com/pollenjp/cdk-ansible/compare/cdk-ansible-v0.2.2...cdk-ansible-v0.2.3) - 2025-07-16 (cdk-ansible)

### Added

- implement From trait for ExePlay

### Other

- version compatibility
- use .into in simple-sample
- change mermaid flow direction to LR
- Add warnings about alpha state and jsii

## [0.2.2](https://github.com/pollenjp/cdk-ansible/compare/cdk-ansible-v0.2.1...cdk-ansible-v0.2.2) - 2025-07-12 (cdk-ansible)

### Fixed

- --parallel arg to --max-procs

### Other

- use stateDiagram's forks instead of concurrency
- convert flowchart to stateDiagram
- modify mermaid
- README for v0.2
- update badge

## [0.2.0](https://github.com/pollenjp/cdk-ansible/compare/cdk-ansible-v0.1.5...cdk-ansible-v0.2.0) - 2025-07-10

### Other

- Rename to App from DeployApp and to Stack from DeployStack
- Add stack name argument
- Add path existence check
- synth_playbooks to synth in deploy subcommand
- Modify InventoryHosts's FromIterator
- use absolute()
- update docs
- Add top-page docs
- remove v0.1 features
- move version to each Cargo.toml
- [deploy subcommand]Add inventory
- Merge branch 'main' into feature/deploy-subcommand
- release v0.1.6
