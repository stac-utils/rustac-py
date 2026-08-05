"""Helpers for writing stac-geoparquet."""

from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from typing import TYPE_CHECKING, Any

from .rustac import GeoparquetWriter

if TYPE_CHECKING:
    from obstore.store import ObjectStore as ObstoreObjectStore

    from rustac.store import ObjectStore

    AnyObjectStore = ObjectStore | ObstoreObjectStore


@asynccontextmanager
async def geoparquet_writer(
    items: list[dict[str, Any]],
    path: str,
    drop_invalid_attributes: bool = True,
    store: "AnyObjectStore | None" = None,
) -> AsyncGenerator[GeoparquetWriter]:
    """Open a geoparquet writer in a context manager.

    The items provided to the initial call will be used to build the geoparquet
    schema. All subsequent items must have the same schema.

    The underlying parquet writer will group batches of items into row groups
    based upon its default configuration; the row groups are _not_ determined
    by the size of the item lists passed to the writer.

    The file is finished when the context manager exits.

    Args:
        items: The STAC items used to build the geoparquet schema.
        path: The path for the stac-geoparquet file.
        drop_invalid_attributes: If true, invalid attributes (e.g. an `id` in
            the `properties` field) will be dropped. If false, raise an error if
            an invalid attribute is encountered.
        store: The optional object store to use for writing the geoparquet file.
            If not provided, a local object store will be used.

    Yields:
        A [GeoparquetWriter][rustac.GeoparquetWriter] that can be used to write
        more items.

    Examples:

        >>> async with geoparquet_writer(item_batches[0], "out.parquet") as w:
        ...     for items in item_batches[1:]:
        ...         await w.write(items)
        ...
        >>>
    """
    writer = await GeoparquetWriter.open(items, path, drop_invalid_attributes, store)
    yield writer
    await writer.finish()
