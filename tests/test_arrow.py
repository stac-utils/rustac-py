from typing import Any

import pytest
import rustac
from geopandas import GeoDataFrame

pytest.importorskip("arro3.core")


def test_to_arrow(item: dict[str, Any]) -> None:
    table = rustac.to_arrow([item])
    data_frame = GeoDataFrame.from_arrow(table)
    assert len(data_frame) == 1


def test_from_arrow(item: dict[str, Any]) -> None:
    from arro3.core import Table

    data_frame = GeoDataFrame.from_features([item])
    item_collection = rustac.from_arrow(Table.from_arrow(data_frame.to_arrow()))
    assert len(item_collection["features"]) == 1
