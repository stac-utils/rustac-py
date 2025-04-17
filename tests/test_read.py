from pathlib import Path

import rustac
from pystac import Item


async def test_read(examples: Path) -> None:
    item = Item.from_dict(await rustac.read(str(examples / "simple-item.json")))
    item.validate()
