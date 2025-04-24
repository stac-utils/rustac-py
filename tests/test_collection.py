from pathlib import Path

import rustac
from pystac import Collection


async def test_collection_from_id_and_items(data: Path) -> None:
    items = await rustac.read(str(data / "100-sentinel-2-items.parquet"))
    collection = Collection.from_dict(
        rustac.collection_from_id_and_items("a-collection", items["features"])  # type: ignore
    )
    collection.validate()
