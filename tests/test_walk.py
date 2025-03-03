from pathlib import Path

import stacrs


async def test_walk(examples: Path) -> None:
    all_children = []
    all_items = []
    catalog = await stacrs.read(str(examples / "catalog.json"))
    async for _, children, items in stacrs.walk(catalog):
        all_children.extend(children)
        all_items.extend(items)

    assert len(all_children) == 3
    assert len(all_items) == 2
