PROJ_ROOT := $(shell pwd)
SAMPLE_ANSIBLE_ROOT := ${PROJ_ROOT}/tools/ansible
SAMPLE_APP_ROOT := ${PROJ_ROOT}/examples/cdk-ansible-sample-app
RS_OUT_DIR := ${PROJ_ROOT}/examples
UV_RUN := uv --project "${SAMPLE_ANSIBLE_ROOT}" run

export

.PHONY: debug
debug:
	${UV_RUN} cargo run --package cdk-ansible-cli -- module \
			--output-dir "${RS_OUT_DIR}" \
			--module-name-regex 'ansible.builtin.debug'
#			--module-name-regex 'ansible\.builtin\..*'

.PHONY: help
help:
#	cd "${PROJ_ROOT}/crates/cdk-ansible" && ${UV_RUN} cargo run -- --help
	RUST_BACKTRACE=1 cargo run --package simple-sample -- synth --help
#	${UV_RUN} cargo run --package cdk-ansible -- module --help

.PHONY: debug-module
debug-module:
	${UV_RUN} cargo run --package cdk-ansible -- module --output-dir "${RS_OUT_DIR}"
#	rsync -av --delete "${RS_OUT_DIR}/" "${SAMPLE_APP_ROOT}/src/module"

.PHONY: simple-sample
simple-sample:
	${UV_RUN} cargo run --package cdk-ansible-cli -- module --output-dir "${RS_OUT_DIR}" --module-name 'ansible.builtin.debug'
# Run 'synth' to generate playbooks and inventory
	RUST_BACKTRACE=1 cargo run --package simple-sample -- synth --output-dir "${SAMPLE_ANSIBLE_ROOT}"
# Convert json to yaml by yq
	find "${SAMPLE_ANSIBLE_ROOT}/playbooks" "${SAMPLE_ANSIBLE_ROOT}/inventory" -name "*.json" \
		| xargs -I{} bash -c \
			'set -eu; \
			filepath_json={}; \
			filepath_yaml="$${filepath_json%.json}.yaml"; \
			yq -p json -o yaml "$${filepath_json}" > "$${filepath_yaml}"'
# Run ansible-lint
	$(MAKE) lint-ansible

.PHONY: build
build:
# build packages one by one
	cargo build --package cdk-ansible-macro
	cargo build --package cdk-ansible-static
	cargo build --package cdk-ansible-core
	cargo build --package cdk-ansible-cli
	cargo build --package cdk-ansible
# examples
	cargo build --package cdkam_ansible
	cargo build --package simple-sample

.PHONY: build-release
build-release:
	PKG_NAME=cdk-ansible-cli BUILD_BINARY_TARGET=x86_64-unknown-linux-gnu ./tools/build/build.sh

.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	$(MAKE) lint-rust
	$(MAKE) lint-ansible

.PHONY: lint-rust
lint-rust:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint-ansible
lint-ansible:
	cd "${SAMPLE_ANSIBLE_ROOT}" && uv run ansible-lint -v

.PHONY: fmt
fmt:
	cargo fix --allow-staged
	cargo fmt --all

.PHONY: install-dev
install-dev:
	${MAKE} clean
	uv --project "${SAMPLE_ANSIBLE_ROOT}" sync
	cargo build

.PHONY: clean
clean:
	rm -rf target/

.PHONY: dist-gen
dist-gen:
#	dist generate
	dist build --tag=v0.0.10 --output-format=json "--artifacts=global"

.PHONY: publish
publish:
	./tools/publish/publish.sh
