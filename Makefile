#<target> : <prerequisites>
#[tab]  <commands>

# -------------------------------------development-------------------------------------
.PHONY: build
build:
	cargo build

.PHONY: install
install:
	cargo install --force --path .



.PHONY: test
test:
	SKIP_WASM_BUILD= cargo test --all

