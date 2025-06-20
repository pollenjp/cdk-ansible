#!/usr/bin/env bash
set -eu -o pipefail

pkgs=(
  cdk-ansible-core
  cdk-ansible-macro
  cdk-ansible-static
  cdk-ansible-cli
  cdk-ansible
  cdkam
)

for pkg in "${pkgs[@]}"; do
  echo "Publishing $pkg"
  random_s=$(dd if=/dev/urandom bs=1024 count=1 2>/dev/null | tr -dc A-Za-z0-9 | fold -w 10 | awk 'NR<=1')
  err_txt_file="/tmp/stderr_$random_s.txt"
  set +e
  cargo publish --package "$pkg" 2> "$err_txt_file"
  exit_code=$?
  err_msg=$(cat "$err_txt_file")
  rm "$err_txt_file"
  set -e
  if [[ "$exit_code" -ne 0 ]]; then
    if [[ "$err_msg" =~ already\ exists\ on\ crates\.io ]]; then
      echo "Package $pkg already published"
    else
      echo "$err_msg"
      echo "Failed to publish $pkg" 1>&2
      exit 1
    fi
  fi
done
