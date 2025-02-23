#!/usr/bin/env bash
set -eu -o pipefail

filepath=$(realpath "${BASH_SOURCE[0]}")
script_dir=$(dirname "$filepath")

repo_root=$(git -C "$script_dir" rev-parse --show-toplevel)

cd "$repo_root"
