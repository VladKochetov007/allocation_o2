"""
AllocationO2 - Tactical asset allocation with Rust backend

This package provides tools for tactical asset allocation with a high-performance
Rust backend. It includes both Python API and command-line tools.

Command-line usage:
    python -m allocation_o2 compile <rust_file.rs> [-o output_path]
"""

__version__ = "0.1.0" 

from .capital_allocator import CapitalAllocator
from .allocator_factory import create_allocator_class, RustStrategy

# Определяем только базовый интерфейс без конкретных реализаций
__all__ = ["CapitalAllocator", "create_allocator_class", "RustStrategy"] 