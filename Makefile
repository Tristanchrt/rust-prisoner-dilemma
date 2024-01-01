# Makefile for Rust project

# Directories
FRONT_DIR = front
BACK_DIR = back

# Targets
.PHONY: runf runb

runf:
	cd $(FRONT_DIR) && cargo build && cargo run

runb:
	cd $(BACK_DIR) && cargo build && cargo run
