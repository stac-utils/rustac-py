from __future__ import annotations

from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from typing import Any

from obstore.store import ObjectStore

from . import store
from .rustac import *  # noqa: F403


@asynccontextmanager
async def geoparquet_writer(
    items: list[dict[str, Any]],
    path: str,
    drop_invalid_attributes: bool = True,
    store: store.Store | ObjectStore | None = None,
) -> AsyncGenerator[GeoparquetWriter]:  # noqa: F405
    """Open a geoparquet writer in a context manager.

    The items provided to the initial call will be used to build the geoparquet
    schema. All subsequent items must have the same schema.

    The underlying parquet writer will group batches of items into row groups
    based upon it's default configuration; the row groups are _not_ determined
    by the size of the item lists passed to the writer.

    Args:
        items: The STAC items
        path: The path for the stac-geoparquet file
        drop_invalid_attributes: If true, invalid attributes (e.g. an `id` in
            the `properties` field) will be dropped. If false, raise an error if
            an invalid attribute is encountered.
        store: The optional object store to use for writing the geoparquet file.

    Examples:

        >>> async with geoparquet_writer(item_batches[0], "out.parquet") as w:
        ...     for items in item_batches[1:]:
        ...         await w.write(items)
        ...
        >>>
    """
    writer = await GeoparquetWriter.open(items, path, drop_invalid_attributes, store)  # noqa: F405
    yield writer
    await writer.finish()


__doc__ = rustac.__doc__  # noqa: F405
if hasattr(rustac, "__all__"):  # noqa: F405
    __all__ = rustac.__all__  # noqa: F405
else:
    __all__ = []

__all__.append("store")
