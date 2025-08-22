"""
AlwaysOn Python - Python bindings for ThingWorx AlwaysOn protocol

This package provides Python bindings for the alwayson-codec Rust library,
enabling efficient encoding and decoding of ThingWorx AlwaysOn protocol messages.
"""

from ._native import __version__, BaseType, TwPrim, AlwaysOnError

__all__ = [
    "__version__",
    "BaseType",
    "TwPrim",
    "AlwaysOnError",
]