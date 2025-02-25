.PHONY: all clean build build_examples install develop test pip_install wheel test_install

# Default Python interpreter
PYTHON ?= python

# Default target
all: build

# Build the Rust library
build:
	cd rust_backend && cargo build --release
	cp rust_backend/target/release/liballocation_o2.so allocation_o2/allocation_o2.so

# Build examples (отдельно от основной библиотеки)
build_examples: build
	cd rust_backend && cargo build --release --example random_weight_strategy
	cp rust_backend/target/release/examples/librandom_weight_strategy.so examples/random_weight_strategy.so

# Install the Python package in development mode (без примеров)
install: build
	$(PYTHON) -m pip install -e .

# Development mode installation (alias for install)
develop: build
	$(PYTHON) -m pip install -e .

# Install via pip from the setup.py (без примеров)
pip_install: build
	$(PYTHON) -m pip install -e .

# Build wheel package (без примеров)
wheel: build
	$(PYTHON) -m pip install --upgrade build
	$(PYTHON) -m build --wheel

# Test installation from wheel
test_install: wheel
	$(PYTHON) -m pip install --force-reinstall dist/allocation_o2-*.whl
	$(PYTHON) test_install.py

# Run tests
test:
	cd rust_backend && cargo test
	$(PYTHON) -m pytest allocation_o2/tests

# Clean build artifacts
clean:
	cd rust_backend && cargo clean
	rm -f allocation_o2/allocation_o2.so
	rm -f examples/*.so
	find . -name "__pycache__" -type d -exec rm -rf {} +
	find . -name "*.pyc" -delete
	rm -rf dist build *.egg-info

# Run the random weight example (требует предварительной сборки примеров)
run_random_example: build_examples
	PYTHONPATH=. $(PYTHON) examples/random_weight_example.py
