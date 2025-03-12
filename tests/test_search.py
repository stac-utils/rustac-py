import json
from pathlib import Path

import pyarrow.parquet
import stac_geoparquet.arrow
import stacrs


async def test_search() -> None:
    items = await stacrs.search(
        "https://landsatlook.usgs.gov/stac-server",
        collections="landsat-c2l2-sr",
        intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        max_items=1,
    )
    assert len(items) == 1


async def test_search_to(tmp_path: Path) -> None:
    await stacrs.search_to(
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
    count = await stacrs.search_to(
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
    items = await stacrs.search(str(data / "extended-item.parquet"))
    assert len(items) == 1


async def test_sortby_list_of_dict() -> None:
    items = await stacrs.search(
        "https://landsatlook.usgs.gov/stac-server",
        collections="landsat-c2l2-sr",
        intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        sortby=[
            {"field": "properties.datetime", "direction": "asc"},
        ],
        max_items=1,
    )
    assert len(items) == 1
