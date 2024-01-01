# Build
build:
	make build-rust
	make build-scheme

build-rust:
	cargo build

build-scheme:
	LD_LIBRARY_PATH=target/debug GUILE_LOAD_PATH=./scheme:${GUILE_LOAD_PATH} guild compile config.scm
	LD_LIBRARY_PATH=target/debug GUILE_LOAD_PATH=./scheme:${GUILE_LOAD_PATH} find scheme -type f -name "*.scm" -exec guild compile {} \;

# Run
run:
	LD_LIBRARY_PATH=target/debug GUILE_LOAD_PATH=./scheme:${GUILE_LOAD_PATH} guile -l config.scm -e main --listen

run-release:
	cargo build --release
	LD_LIBRARY_PATH=target/release GUILE_LOAD_PATH=./scheme:${GUILE_LOAD_PATH} guile -l config.scm -e main

flamegraph-profile:
	cargo build --release
	LD_LIBRARY_PATH=target/release GUILE_LOAD_PATH=./scheme:${GUILE_LOAD_PATH} flamegraph -- guile -l config.scm -e main

# Test
test:
	make test-rust
	make test-rustdoc
	make test-scheme

test-rust:
	cargo nextest run

test-rustdoc:
	cargo test --doc

test-scheme:
	LD_LIBRARY_PATH=target/debug GUILE_LOAD_PATH=./scheme:${GUILE_LOAD_PATH} find scheme/tests -type f -name "*.scm" -exec guile {} \;
