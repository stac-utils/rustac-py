from pathlib import Path
from typing import Any

import pandas
import pyarrow.parquet
import rustac
import stac_geoparquet
from pyarrow.parquet import ParquetFile
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


async def test_write_includes_type(tmp_path: Path, item: dict[str, Any]) -> None:
    assert "type" in item
    await rustac.write(str(tmp_path / "out.parquet"), [item])
    data_frame = pandas.read_parquet(str(tmp_path / "out.parquet"))
    assert "type" in data_frame.columns


async def test_write_parquet_compression(tmp_path: Path, item: dict[str, Any]) -> None:
    await rustac.write(
        str(tmp_path / "out.parquet"), [item], parquet_compression="zstd(1)"
    )
    parquet_file = ParquetFile(tmp_path / "out.parquet")
    metadata = parquet_file.metadata
    for row_group in range(metadata.num_row_groups):
        for column in range(metadata.num_columns):
            assert metadata.row_group(row_group).column(column).compression == "ZSTD"
