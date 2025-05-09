from pathlib import Path
from typing import Any

import pyarrow.parquet
import rustac
import stac_geoparquet
from rustac.store import LocalStore


async def test_write(item: dict[str, Any], tmp_path: Path) -> None:
    path = str(tmp_path / "out.parquet")
    await rustac.write(path, [item])
    table = pyarrow.parquet.read_table(path)
    items = list(stac_geoparquet.arrow.stac_table_to_items(table))
    assert len(items) == 1


async def test_write_compressed(item: dict[str, Any], tmp_path: Path) -> None:
    path = str(tmp_path / "out.parquet")
    await rustac.write(path, [item])
    metadata = pyarrow.parquet.read_metadata(path)
    assert metadata.row_group(0).column(0).compression == "SNAPPY"


async def test_write_store(item: dict[str, Any], tmp_path: Path) -> None:
    store = LocalStore(prefix=tmp_path)
    await rustac.write("item.json", item, store=store)
    read_item = await rustac.read("item.json", store=store)
    assert item["id"] == read_item["id"]
