from __future__ import annotations

from .duckdb import DuckdbClient
from .geoparquet import geoparquet_writer
from .rustac import *

__doc__ = rustac.__doc__
__all__ = rustac.__all__ + ["geoparquet_writer", "DuckdbClient"]  # pyright: ignore[reportUnsupportedDunderAll, reportAttributeAccessIssue]
