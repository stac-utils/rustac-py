import pytest
import rustac
from geopandas import GeoDataFrame
from rustac import Item

pytest.importorskip("arro3.core")


def test_to_arrow(item: Item) -> None:
    table = rustac.to_arrow([item])
    data_frame = GeoDataFrame.from_arrow(table)
    assert len(data_frame) == 1
