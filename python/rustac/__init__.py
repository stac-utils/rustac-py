from __future__ import annotations

from .duckdb import DuckdbClient
from .geoparquet import geoparquet_writer
from .rustac import *

__doc__ = rustac.__doc__  # pyright: ignore[reportAttributeAccessIssue]
__all__ = [
    "ApiClient",
    "DuckdbClient",
    "GeoparquetWriter",
    "RustacError",
    "collection_from_id_and_items",
    "exceptions",
    "from_arrow",
    "geoparquet_writer",
    "iter_search",
    "main",
    "migrate",
    "read",
    "read_sync",
    "search",
    "search_sync",
    "search_to",
    "sha",
    "store",
    "to_arrow",
    "version",
    "walk",
    "write",
    "write_sync",
]
