from pathlib import Path

import stacrs
from pystac import Item


async def test_read(examples: Path) -> None:
    item = Item.from_dict(await stacrs.read(str(examples / "simple-item.json")))
    item.validate()
