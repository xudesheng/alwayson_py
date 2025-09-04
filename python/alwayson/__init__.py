"""
AlwaysOn Python - Python bindings for ThingWorx AlwaysOn protocol

This package provides Python bindings for the alwayson-codec Rust library,
enabling efficient encoding and decoding of ThingWorx AlwaysOn protocol messages.
"""

from ._native import (
    AlwaysOnError,
    BaseType,
    InfoTable,
    TwPrim,
    TwxEvent,
    TwxMessage,
    TwxProperty,
    TwxService,
    __version__,
)

__all__ = [
    "AlwaysOnError",
    "BaseType",
    "InfoTable",
    "TwPrim",
    "TwxEvent",
    "TwxMessage",
    "TwxProperty",
    "TwxService",
    "__version__",
]
