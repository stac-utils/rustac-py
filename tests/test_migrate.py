from typing import Any

import rustac


def test_migrate(item: dict[str, Any]) -> None:
    item = rustac.migrate(item, version="1.1.0")
    assert item["stac_version"] == "1.1.0"
