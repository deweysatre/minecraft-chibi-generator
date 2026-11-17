.PHONY: all build build-release run run-release test check fmt clean count-chars clean pre-push

all: build-release

# --- BUILD ---
build:
	@echo "Building (dev)..."
	@cargo build

build-release:
	@echo "Building (release)..."
	@cargo build --release

# --- RUN ---
run:
	@echo "Running (dev)..."
	@cargo run

run-release:
	@echo "Running (release)..."
	@cargo run --release

# --- QUALITY & TESTS ---
test:
	@cargo test --workspace --color=always

check:
	@echo "Checking..."
	@cargo check --all

fmt:
	@echo "Formatting..."
	@cargo fmt --all

count-chars:
	@echo "The workspace have $$(cat $$(find src/ -name '*.*') | wc -m) chars."

clippy:
	@cargo clippy --workspace --all-targets -- -D warnings

pre-push: build check test fmt clippy
	@echo "All test succeeded, you can push"

# --- CLEAN ---
clean:
	@cargo clean
