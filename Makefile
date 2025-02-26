.PHONY: all clean build build_examples install test pip_install wheel compile_strategy run_momentum_example

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

# Compile a custom strategy file
compile_strategy:
	@echo "Usage: make compile_strategy STRATEGY=path/to/strategy.rs [OUTPUT=path/to/output]"
ifdef STRATEGY
	$(PYTHON) -m allocation_o2 compile $(STRATEGY) $(if $(OUTPUT),-o $(OUTPUT),)
else
	@echo "Error: STRATEGY parameter is required"
	@echo "Example: make compile_strategy STRATEGY=path/to/strategy.rs"
	@exit 1
endif

# Install the Python package in development mode (without examples)
install: build
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
