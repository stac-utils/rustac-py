import json
from pathlib import Path
from typing import Any

import pyarrow.parquet
import pytest
import rustac
import stac_geoparquet.arrow
from rustac import RustacError
from rustac.store import MemoryStore


async def test_search() -> None:
    items = await rustac.search(
        "https://landsatlook.usgs.gov/stac-server",
        collections="landsat-c2l2-sr",
        intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        max_items=1,
    )
    assert len(items) == 1


async def test_search_to(tmp_path: Path) -> None:
    await rustac.search_to(
        str(tmp_path / "out.json"),
        "https://landsatlook.usgs.gov/stac-server",
        collections="landsat-c2l2-sr",
        intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        max_items=1,
    )
    with open(tmp_path / "out.json") as f:
        data = json.load(f)
    assert len(data["features"]) == 1


async def test_search_to_geoparquet(tmp_path: Path) -> None:
    count = await rustac.search_to(
        str(tmp_path / "out.parquet"),
        "https://landsatlook.usgs.gov/stac-server",
        collections="landsat-c2l2-sr",
        intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        max_items=1,
    )
    assert count == 1
    table = pyarrow.parquet.read_table(tmp_path / "out.parquet")
    items = list(stac_geoparquet.arrow.stac_table_to_items(table))
    assert len(items) == 1


async def test_search_geoparquet(data: Path) -> None:
    items = await rustac.search(str(data / "extended-item.parquet"))
    assert len(items) == 1


async def test_sortby_list_of_dict() -> None:
    items = await rustac.search(
        "https://landsatlook.usgs.gov/stac-server",
        collections="landsat-c2l2-sr",
        intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        sortby=[
            {"field": "properties.datetime", "direction": "asc"},
        ],
        max_items=1,
    )
    assert len(items) == 1


async def test_proj_geometry(maxar_items: list[dict[str, Any]], tmp_path: Path) -> None:
    await rustac.write(str(tmp_path / "out.parquet"), maxar_items)


async def test_search_to_store(data: Path) -> None:
    # https://github.com/stac-utils/rustac-py/issues/129
    store = MemoryStore()
    count = await rustac.search_to(
        "items.json", str(data / "100-sentinel-2-items.parquet"), store=store
    )
    assert count == 100
    item_collection = await rustac.read("items.json", store=store)
    assert len(item_collection["features"]) == 100


async def test_list_sortby(data: Path) -> None:
    # https://github.com/stac-utils/rustac-py/issues/80
    items = await rustac.search(
        str(data / "100-sentinel-2-items.parquet"),
        sortby=[
            {"field": "datetime", "direction": "desc"},
            {"field": "id", "direction": "asc"},
        ],
    )
    for first, second in zip(items, items[1:]):
        if first["properties"]["datetime"] == second["properties"]["datetime"]:
            assert first["id"] <= second["id"]
        else:
            assert first["properties"]["datetime"] >= second["properties"]["datetime"]


async def test_cql(data: Path) -> None:
    # https://github.com/stac-utils/rustac-py/issues/135
    with pytest.raises(RustacError, match="eq is not a valid operator"):
        await rustac.search(
            str(data / "100-sentinel-2-items.parquet"),
            filter={
                "op": "and",
                "args": [
                    # eq is cql, not cql2
                    {
                        "op": "eq",
                        "args": [{"property": "platform"}, "made-up-platform"],
                    },
                ],
            },
            max_items=1,
        )


async def test_iter_search() -> None:
    items = []
    search = await rustac.iter_search("https://landsatlook.usgs.gov/stac-server")
    async for item in search:
        items.append(item)
        if len(items) >= 10:
            break
