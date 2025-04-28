from pathlib import Path

import pytest
import rustac
from geopandas import GeoDataFrame
from rustac import DuckdbClient, RustacError


@pytest.fixture
def client() -> DuckdbClient:
    return DuckdbClient()


@pytest.fixture
def extension_directory() -> Path:
    return Path(__file__).parent / "duckdb-extensions"


def test_search(client: DuckdbClient) -> None:
    items = client.search("data/extended-item.parquet")
    assert len(items) == 1


def test_search_empty_datetime_interval(client: DuckdbClient) -> None:
    # This used to cause an error until https://github.com/stac-utils/rustac/pull/715
    client.search("data/extended-item.parquet", datetime="2025-04-27T00:00:00Z/")


def test_search_missing_column(client: DuckdbClient) -> None:
    # https://github.com/stac-utils/rustac/pull/717
    client.search("data/100-sentinel-2-items.parquet", filter="foo:bar = 42")


def test_search_offset(client: DuckdbClient) -> None:
    items = client.search("data/100-sentinel-2-items.parquet", offset=0, limit=1)
    assert items[0]["id"] == "S2B_MSIL2A_20241203T174629_R098_T13TDE_20241203T211406"

    items = client.search("data/100-sentinel-2-items.parquet", offset=1, limit=1)
    assert items[0]["id"] == "S2A_MSIL2A_20241201T175721_R141_T13TDE_20241201T213150"


def test_get_collections(client: DuckdbClient) -> None:
    collections = client.get_collections("data/100-sentinel-2-items.parquet")
    assert len(collections) == 1


def test_search_to_arrow(client: DuckdbClient) -> None:
    pytest.importorskip("arro3.core")
    table = client.search_to_arrow("data/100-sentinel-2-items.parquet")
    data_frame = GeoDataFrame.from_arrow(table)
    assert len(data_frame) == 100
    data_frame_table = data_frame.to_arrow()
    item_collection = rustac.from_arrow(data_frame_table)
    assert len(item_collection["features"]) == 100


def test_custom_extension_directory(extension_directory: Path) -> None:
    client = DuckdbClient(extension_directory=extension_directory)
    # Search to ensure we trigger everything
    client.search("data/100-sentinel-2-items.parquet")


def test_no_install(tmp_path: Path) -> None:
    with pytest.raises(RustacError):
        DuckdbClient(extension_directory=tmp_path, install_extensions=False)


def test_extensions(extension_directory: Path, tmp_path: Path) -> None:
    # Ensure we've fetched the extension
    DuckdbClient(extension_directory=extension_directory)

    extensions = list(str(e) for e in extension_directory.glob("**/*.duckdb_extension"))
    client = DuckdbClient(
        extensions=extensions,
        extension_directory=tmp_path,
        install_extensions=False,
    )
    client.search("data/100-sentinel-2-items.parquet")


def test_execute(client: DuckdbClient, extension_directory: Path) -> None:
    # Just a smoke test
    client.execute("SET extension_directory = ?", [str(extension_directory)])


def test_load_spatial() -> None:
    DuckdbClient(extensions=["spatial"])


@pytest.mark.skip("slow")
def test_aws_credential_chain(client: DuckdbClient) -> None:
    client.execute("CREATE SECRET (TYPE S3, PROVIDER CREDENTIAL_CHAIN)")
