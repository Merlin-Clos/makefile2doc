.DEFAULT_GOAL := help

STEP = @printf '\n=== %s ===\n\n'
OK = @printf '\n✓ %s\n'

.PHONY: help check-tools check fmt-check fmt lint test test-unit test-one test-integration build-debug build-release docs

## @category Development

## @description Show the generated development command reference
help:
	$(STEP) "Development commands"
	@./scripts/show-markdown.sh MAKEFILE.md

## @description Verify that all tools required by the development workflow are available
check-tools:
	$(STEP) "Checking required tools"
	@missing=0; \
	for spec in \
		"cargo|https://rustup.rs" \
		"rustc|https://rustup.rs" \
		"rustfmt|https://rustup.rs" \
		"cargo-clippy|https://rustup.rs"; do \
		tool=$${spec%%|*}; \
		link=$${spec#*|}; \
		if ! command -v "$$tool" >/dev/null 2>&1; then \
			printf 'Missing tool: %s\nInstall instructions: %s\n' "$$tool" "$$link"; \
			missing=1; \
		fi; \
	done; \
	test "$$missing" -eq 0
	$(OK) "All required tools are available"
	@if command -v mdcat >/dev/null 2>&1; then \
		printf '✓ Recommended Markdown renderer is available: mdcat\n'; \
	else \
		printf 'Recommended: install mdcat for rendered help: https://github.com/BIRSAx2/mdcat\n'; \
	fi

## @category Quality

## @description Run every local quality check
## @depends fmt-check, lint, test
check: fmt-check lint test
	$(OK) "All local quality checks passed"

## @description Check Rust formatting without modifying files
fmt-check:
	$(STEP) "Checking Rust formatting"
	cargo fmt --all -- --check
	$(OK) "Rust formatting is valid"

## @description Format all Rust sources
fmt:
	$(STEP) "Formatting Rust sources"
	cargo fmt --all
	$(OK) "Rust sources are formatted"

## @description Run Clippy on every target and feature with warnings denied
lint:
	$(STEP) "Running enabled Clippy lints"
	cargo clippy --all-targets --all-features --locked -- -D warnings
	$(OK) "Clippy found no enabled lint violations"

## @description Run the complete Rust test suite
test:
	$(STEP) "Running all Rust tests"
	cargo test --locked
	$(OK) "All Rust tests passed"

## @description Run only library unit tests
test-unit:
	$(STEP) "Running library unit tests"
	cargo test --locked --lib
	$(OK) "All library unit tests passed"

## @description Run one exact library unit test; pass its full name with TEST=...
test-one:
	$(STEP) "Running one library unit test"
	@test -n "$(TEST)" || { printf 'Usage: make test-one TEST=module::tests::name\n'; exit 2; }
	cargo test --locked --lib "$(TEST)" -- --exact
	$(OK) "Selected library unit test passed"

## @description Run only the integration test target
test-integration:
	$(STEP) "Running integration tests"
	cargo test --locked --test integration
	$(OK) "All integration tests passed"

## @category Build

## @description Build the debug binary with development symbols
build-debug:
	$(STEP) "Building debug binary"
	cargo build --locked
	$(OK) "Debug binary is ready"

## @description Build the fully optimized and stripped release binary
build-release:
	$(STEP) "Building optimized release binary"
	cargo build --release --locked
	$(OK) "Release binary is ready"

## @category Documentation

## @description Generate MAKEFILE.md from this repository Makefile
docs:
	$(STEP) "Generating Makefile documentation"
	cargo run --locked -- --input Makefile --output MAKEFILE.md
