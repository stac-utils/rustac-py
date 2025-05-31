from pathlib import Path
from typing import Any

import pytest
import rustac
from obstore.store import LocalStore as ObstoreLocalStore
from pystac import Item
from rustac.store import LocalStore


async def test_read(examples: Path) -> None:
    item = Item.from_dict(await rustac.read(str(examples / "simple-item.json")))
    item.validate()


async def test_asset_ordering(examples: Path) -> None:
    # Not a perfect test but should be good enough for https://github.com/stac-utils/rustac-py/issues/85
    lists = []
    for _ in range(10):
        item = await rustac.read(str(examples / "extended-item.json"))
        lists.append(list(item["assets"].keys()))
    for sublist in lists[1:]:
        assert lists[0] == sublist


async def test_read_proj_geometry(
    tmp_path: Path, maxar_items: list[dict[str, Any]]
) -> None:
    path = str(tmp_path / "out.parquet")
    await rustac.write(path, maxar_items)
    items = await rustac.read(path)
    assert len(items["features"]) == 2


async def test_read_store(examples: Path) -> None:
    store = LocalStore(prefix=str(examples))
    await rustac.read("simple-item.json", store=store)


async def test_read_external_store(examples: Path) -> None:
    store = ObstoreLocalStore(prefix=str(examples))
    with pytest.warns(
        RuntimeWarning,
        match="Successfully reconstructed a store defined in another Python module. Connection pooling will not be shared across store instances.",
    ):
        await rustac.read("simple-item.json", store=store)
