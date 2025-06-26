#!/usr/bin/env bash
set -euCx -o pipefail

proj_root=$(git rev-parse --show-toplevel)
ansible_uv=(uv --project "${proj_root}/tools/ansible")

random_str=$(dd if=/dev/urandom bs=1024 count=1 2>/dev/null | tr -dc A-Za-z0-9 | fold -w 10 | awk 'NR<=1')
date_str=$(date +%Y%m%d-%H%M%S)
temp_dir=~/.cdk-ansible-tmp/"${date_str}-${random_str}"
mkdir -p "${temp_dir}"

cleanup() {
  local exit_code=$?
  echo "cleanup..."
  rm -rf "${temp_dir}"
  exit "${exit_code}"
}

trap cleanup EXIT

cat <<__EOF__ >"${temp_dir}/Cargo.toml"
[workspace]
members = ["crates/*"]

[workspace.dependencies]
cdk-ansible = { path = "${proj_root}/crates/cdk-ansible" }
__EOF__

pushd "${proj_root}"
"${ansible_uv[@]}" run cargo run --package cdk-ansible-cli -- module --output-dir "${temp_dir}/crates" --module-name 'ansible.builtin.service_facts'
popd

pushd "${temp_dir}/crates"
cargo build
popd
