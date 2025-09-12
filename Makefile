
# variables
SOURCEDIR	= symmetria-core

help:
	@echo "[HELP] Makefile commands:"
	@echo " * lint-rust: lint rust code"
	@echo " * test-rust: test rust code"

.PHONY: help Makefile

lint-rust:
	@echo "[INFO] Lint Rust Code"
	@cargo --version
	@cargo fmt --version
	@cargo fmt --all -- --check
	@cargo clippy --version
	@cargo clippy -- -D warnings

test-rust:
	@echo "[INFO] Test Rust Code"
	@cargo --version
	@cargo test
