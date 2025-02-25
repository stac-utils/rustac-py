import pytest
import stacrs
from geopandas import GeoDataFrame
from stacrs import DuckdbClient


@pytest.fixture
def client() -> DuckdbClient:
    return DuckdbClient()


def test_search(client: DuckdbClient) -> None:
    item_collection = client.search("data/extended-item.parquet")
    assert len(item_collection["features"]) == 1


def test_search_offset(client: DuckdbClient) -> None:
    item_collection = client.search(
        "data/100-sentinel-2-items.parquet", offset=0, limit=1
    )
    assert (
        item_collection["features"][0]["id"]
        == "S2B_MSIL2A_20241203T174629_R098_T13TDE_20241203T211406"
    )

    item_collection = client.search(
        "data/100-sentinel-2-items.parquet", offset=1, limit=1
    )
    assert (
        item_collection["features"][0]["id"]
        == "S2A_MSIL2A_20241201T175721_R141_T13TDE_20241201T213150"
    )


def test_get_collections(client: DuckdbClient) -> None:
    collections = client.get_collections("data/100-sentinel-2-items.parquet")
    assert len(collections) == 1


def test_init_with_config() -> None:
    DuckdbClient(use_s3_credential_chain=True, use_hive_partitioning=True)


def test_search_to_arrow(client: DuckdbClient) -> None:
    pytest.importorskip("arro3.core")
    table = client.search_to_arrow("data/100-sentinel-2-items.parquet")
    data_frame = GeoDataFrame.from_arrow(table)
    assert len(data_frame) == 100
    data_frame_table = data_frame.to_arrow()
    item_collection = stacrs.from_arrow(data_frame_table)
    assert len(item_collection["features"]) == 100
