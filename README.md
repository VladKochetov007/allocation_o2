# AllocationO2
Tactical asset allocation with Rust backend

## Project Structure

The project consists of two main parts:

1. **Rust Backend** (`rust_backend/`): Core functionality implemented in Rust
   - Fast and efficient portfolio optimization algorithms
   - Asset allocation strategies
   - Portfolio analytics

2. **Python Frontend** (`python_frontend/`): Python interface using PyO3
   - User-friendly API for Python users
   - Data visualization and analysis
   - Integration with data sources

## Development

This project uses PyO3 to create Python bindings for Rust code.

### Requirements

- Rust (latest stable)
- Python 3.8+
- setuptools-rust

### Building

To build the project:

```bash
make install
```
