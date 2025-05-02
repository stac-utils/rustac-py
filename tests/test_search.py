import json
from pathlib import Path
from typing import Any

import pyarrow.parquet
import rustac
import stac_geoparquet.arrow


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
