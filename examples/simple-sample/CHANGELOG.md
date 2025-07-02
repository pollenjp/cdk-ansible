# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/pollenjp/cdk-ansible/releases/tag/simple-sample-v0.1.0) - 2025-07-02

### Fixed

- fix
- fix --version option

### Other

- Remove unused cdk-ansible-macro dependency and reorder imports in inventory module
- Add namespace and collection features
- introduce BoolOrString, StringOrVecString, BoolOrStringOrVecString type
- Use IndexMap and fix PlayOptions fields
- Use IndexMap instead of serde_json::Map to preserve order
- Add clippy cfg to simple-sample
- add check_module.rs
- Rename OptionUnset to OptU
- Add examples
