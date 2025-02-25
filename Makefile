.PHONY: all clean build build_examples install test pip_install wheel

# Default Python interpreter
PYTHON ?= python

# Default target
all: build

# Build the Rust library
build:
	cd rust_backend && cargo build --release
	cp rust_backend/target/release/liballocation_o2.so allocation_o2/allocation_o2.so

# Build examples (separately from the main library)
build_examples: build
	cd rust_backend && cargo build --release --example random_weight_strategy
	cp rust_backend/target/release/examples/librandom_weight_strategy.so examples/random_weight_strategy.so

# Install the Python package in development mode (without examples)
install: build
	$(PYTHON) -m pip install -e .

# Install via pip from the setup.py (without examples)
pip_install: build
	$(PYTHON) -m pip install -e .

# Build wheel package (without examples)
wheel: build
	$(PYTHON) -m pip install --upgrade build
	$(PYTHON) -m build --wheel

# Clean build artifacts
clean:
	cd rust_backend && cargo clean
	rm -f allocation_o2/allocation_o2.so
	rm -f examples/*.so
	find . -name "__pycache__" -type d -exec rm -rf {} +
	find . -name "*.pyc" -delete
	rm -rf dist build *.egg-info
