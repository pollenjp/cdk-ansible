PROJ_ROOT := $(shell pwd)
SAMPLE_ANSIBLE_ROOT := ${PROJ_ROOT}/tools/ansible
RS_OUT_DIR := ${PROJ_ROOT}/examples/cdk-ansible-sample-app/src/module
UV_RUN := uv --project "${SAMPLE_ANSIBLE_ROOT}" run

export


.PHONY: debug
debug:
	cd "${PROJ_ROOT}/crates/cdk-ansible" \
		&& ${UV_RUN} cargo run -- module --output-dir "${RS_OUT_DIR}" --module-name fortinet.fortimanager.fmgr_user_tacacs_dynamicmapping

.PHONY: debug-module
debug-module:
	cd "${PROJ_ROOT}/crates/cdk-ansible" && ${UV_RUN} cargo run -- module --output-dir "${RS_OUT_DIR}"

.PHONY: debug-synth
debug-synth:
	cd "${PROJ_ROOT}/crates/cdk-ansible-sample-app" && RUST_BACKTRACE=1 cargo run -- synth --output-dir "${SAMPLE_ANSIBLE_ROOT}"
# convert json to yaml by yq
	find "${SAMPLE_ANSIBLE_ROOT}/playbooks" "${SAMPLE_ANSIBLE_ROOT}/inventory" -name "*.json" \
		| xargs -I{} bash -c \
			'set -eu; \
			filepath_json={}; \
			filepath_yaml="$${filepath_json%.json}.yaml"; \
			yq -p json -o yaml "$${filepath_json}" > "$${filepath_yaml}"'
	$(MAKE) lint-ansible

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
