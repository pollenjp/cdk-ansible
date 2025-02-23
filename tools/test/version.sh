#!/usr/bin/env bash
set -euCx -o pipefail

proj_root=$(git rev-parse --show-toplevel)

pushd "${proj_root}"
cargo run --package cdk-ansible-cli -- --version
popd
