import pytest
from rustac import ApiClient


@pytest.fixture
def api_client() -> ApiClient:
    return ApiClient("https://stac.eoapi.dev")


async def test_api_client_get_collection(api_client: ApiClient) -> None:
    assert await api_client.get_collection("openaerialmap")


async def test_api_client_get_collections(api_client: ApiClient) -> None:
    collections = await api_client.get_collections()
    assert len(collections) == 41
