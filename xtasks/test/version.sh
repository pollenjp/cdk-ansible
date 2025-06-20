#!/usr/bin/env bash
set -euCx -o pipefail

cargo run --package cdk-ansible-cli -- --version
