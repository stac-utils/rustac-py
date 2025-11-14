from __future__ import annotations

from .rustac import *
from . import store

__doc__ = rustac.__doc__
if hasattr(rustac, "__all__"):
    __all__ = rustac.__all__
else:
    __all__ = []

__all__.append("store")
