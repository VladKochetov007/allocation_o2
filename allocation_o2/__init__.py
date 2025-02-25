"""
AllocationO2 - Tactical asset allocation with Rust backend
"""

__version__ = "0.1.0" 

from .capital_allocator import CapitalAllocator
from .allocator_factory import create_allocator_class, RustStrategy

# Определяем только базовый интерфейс без конкретных реализаций
__all__ = ["CapitalAllocator", "create_allocator_class", "RustStrategy"] 