from __future__ import annotations

from .geoparquet import geoparquet_writer
from .rustac import *
from .duckdb import DuckdbClient  # noqa: E402

__doc__ = rustac.__doc__
__all__ = rustac.__all__ + ["geoparquet_writer"]  # pyright: ignore[reportUnsupportedDunderAll, reportAttributeAccessIssue]
