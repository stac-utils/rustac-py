from typing import Any

import pytest
import rustac
from geopandas import GeoDataFrame

pytest.importorskip("arro3.core")


def test_to_arrow(item: dict[str, Any]) -> None:
    table = rustac.to_arrow([item])
    data_frame = GeoDataFrame.from_arrow(table)
    assert len(data_frame) == 1
