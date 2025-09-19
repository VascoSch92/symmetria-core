
# variables
PYPI_PASSWORD   ?=
SOURCEDIR		= symmetria-core

help:
	@echo "[HELP] Makefile commands:"
	@echo " * init: init the (dev) env"
	@echo " * lint-rust: lint rust code"
	@echo " * test-rust: test rust code"
	@echo " * pre-commit: run the pre-commit"
	@echo " * release: release a new version"

.PHONY: help Makefile

init:
	@echo "[INFO] Init the (dev) env"
	@uv --version || echo 'Please install uv: https://docs.astral.sh/uv/getting-started/installation/'
	@echo "[INFO] Cleaning..."
	@rm -rf .venv
	@echo "[INFO] Create env..."
	@uv venv
	@echo "[INFO] Sync (dev) deps..."
	@uv sync --all-groups
	@uv pip list
	@echo "[INFO] Install pre-commit into git-hooks"
	@uv run pre-commit install

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
	@LIBDIR=$$(uv run python -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))') && \
    	export LD_LIBRARY_PATH="$$LIBDIR:$$LD_LIBRARY_PATH" && \
    	cargo test

pre-commit:
	@echo "[INFO] Run pre-commit"
	@uv run pre-commit run --all-files

release:
	@echo "[INFO] Releasing a new version"
	@echo "[INFO] Cleaning..."
	@rm -rf target
	@echo "[INFO] Create and upload release..."
	@uv run maturin build --release && uv run maturin publish --username __token__ --password $(PYPI_PASSWORD)
