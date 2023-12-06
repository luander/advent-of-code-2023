.PHONY: test tarpaulin clippy fmt audit deny clean watch build build-release all

all: test tarpaulin clippy fmt audit deny build-release

install-dependencies:
	cargo install cargo-audit \
		cargo-deny \
		cargo-expand \
		cargo-tarpaulin \
		cargo-watch \
		cargo-generate \
		cargo-udeps \
		flamegraph \
		cargo-criterion

test:
	cargo test

generate:
	cargo generate --path ./template --name "$(NAME)"

tarpaulin:
	cargo tarpaulin --ignore-tests

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt -- --check

audit:
	cargo audit

deny:
	cargo deny check

build:
	cargo build

build-release: fmt clippy audit tarpaulin test deny udeps
	cargo build --release

clean:
	cargo clean

watch:
	cargo watch -x clippy -x test -x run

udeps:
	cargo +nightly udeps --all-targets
