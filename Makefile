PROJ_ROOT := $(shell pwd)
SAMPLE_ANSIBLE_ROOT := ${PROJ_ROOT}/tools/ansible
SAMPLE_APP_ROOT := ${PROJ_ROOT}/examples/cdk-ansible-sample-app
RS_OUT_DIR := ${PROJ_ROOT}/examples
UV_RUN := uv --project "${SAMPLE_ANSIBLE_ROOT}" run

export

.PHONY: debug
debug:
	${UV_RUN} cargo run --package cdk-ansible-cli -- module --help
#	${UV_RUN} cargo run --package cdk-ansible-cli -- module \
#			--output-dir "${RS_OUT_DIR}" \
#			--module-name-regex 'ansible.builtin.debug'
#			--module-name-regex 'ansible\.builtin\..*'

.PHONY: help
help:
#	cd "${PROJ_ROOT}/crates/cdk-ansible" && ${UV_RUN} cargo run -- --help
	RUST_BACKTRACE=1 cargo run --package simple-sample -- synth --help
#	${UV_RUN} cargo run --package cdk-ansible -- module --help

CDKAM_GEN_CMD := ${UV_RUN} cargo run --package cdk-ansible-cli \
	module \
	--output-dir './crates' \
	--pkg-unit 'none'

.PHONY: cdkam
cdkam:  ## utility to generate cdkam modules (manual editing required afterwards)
	${CDKAM_GEN_CMD} --module-name-regex 'ansible\.builtin\..*' \
		--module-name-exclude 'ansible\.builtin\.meta' \
		--module-name-exclude 'ansible\.builtin\.set_fact'
	${CDKAM_GEN_CMD} --module-name-regex 'ansible\.posix\..*'
	${CDKAM_GEN_CMD} --module-name-regex 'community\.general\..*'

.PHONY: cdkam-check
cdkam-check:
# re-write
	@echo "src/m/ansible/builtin/meta.rs"
	@echo "src/m/ansible/builtin/set_fact.rs"
	@find ./crates/cdkam -name "mod.rs"
	@find ./crates/cdkam -name "Cargo.toml"

.PHONY: build
build:
# FIXME: cargo-release may be better
# build packages one by one
	cargo build --package cdk-ansible-macro
	cargo build --package cdk-ansible-static
	cargo build --package cdk-ansible-core
	cargo build --package cdk-ansible-cli
	cargo build --package cdk-ansible
# include examples
	cargo build --workspace

.PHONY: build-release
build-release: ## local check
	PKG_NAME=cdk-ansible-cli BUILD_BINARY_TARGET=x86_64-unknown-linux-gnu ./tools/build/build.sh

.PHONY: test
test:
	${MAKE} test-cargo
	${MAKE} test-simple-sample
	${MAKE} test-under-tools

.PHONY: test-cargo
test-cargo:
	cargo test --all-features

.PHONY: test-simple-sample
test-simple-sample:
	${UV_RUN} cargo run --package cdk-ansible-cli -- module --pkg-prefix 'sample_cdkam' --output-dir "${RS_OUT_DIR}" --module-name 'ansible.builtin.debug'
	${UV_RUN} cargo run --package cdk-ansible-cli -- module --pkg-prefix 'sample_cdkam' --output-dir "${RS_OUT_DIR}" --module-name 'ansible.builtin.service_facts'
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

.PHONY: test-under-tools
test-under-tools:
	find "${PROJ_ROOT}/tools/test" -name "*.sh" -exec {} \;

.PHONY: lint
lint:
	$(MAKE) lint-rust
	$(MAKE) lint-ansible
	cargo hack check --rust-version --workspace --all-targets --ignore-private

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

.PHONY: prepare-release
prepare-release:
	cargo release changes

.PHONY: publish
publish:
	./tools/publish/publish.sh
