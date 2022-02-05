TARGET ?= x86_64-unknown-linux-gnu
BIN ?= /usr/bin

.PHONY: install-rust-tools
install-rust-tools:
	rustup update
	rustup component add rustfmt clippy

.PHONY: lint
lint:
	# Format files in the current crate using rustfmt
	cargo fmt -- --check
	# Check all packages and tests in the current crate and fail on warnings
	cargo clippy --all --tests -- --no-deps -D warnings

.PHONY: test
test:
	cargo test --no-fail-fast

.PHONY: build
build:
	cargo build --target ${TARGET}

.PHONY: release
release:
	cargo build --release --target ${TARGET}

.PHONY: install
install: release
	sudo cp ./target/${TARGET}/release/lemurs ${BIN}
