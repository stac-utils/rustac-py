from collections.abc import Callable, Iterator
from contextlib import contextmanager
from multiprocessing import get_context
from multiprocessing.connection import Connection
from pathlib import Path
import threading

import pytest
import rustac
from geopandas import GeoDataFrame
from pytest_httpserver import HTTPServer
from rustac import DuckdbClient, RustacError
from werkzeug import Request, Response


PARQUET_OPERATIONS = [
    pytest.param(
        "execute",
        "CREATE TEMP TABLE items AS SELECT * FROM read_parquet(?)",
        id="execute",
    ),
    pytest.param(
        "query_to_table", "SELECT id FROM read_parquet(?)", id="query_to_table"
    ),
    pytest.param("search", None, id="search"),
    pytest.param("search_to_arrow", None, id="search_to_arrow"),
    pytest.param("get_collections", None, id="get_collections"),
]


@pytest.fixture
def client() -> DuckdbClient:
    return DuckdbClient()


@pytest.fixture
def extension_directory() -> Path:
    return Path(__file__).parent / "duckdb-extensions"


@pytest.fixture
def gated_parquet_server(
    httpserver: HTTPServer,
) -> Iterator[tuple[str, threading.Event, threading.Event]]:
    """Hold a real Parquet response until the test allows the query to finish."""
    parquet = Path("data/100-sentinel-2-items.parquet").read_bytes()
    requested = threading.Event()
    release = threading.Event()

    def respond(request: Request) -> Response:
        requested.set()
        if not release.wait(timeout=30):
            return Response(status=504)
        return Response(parquet, content_type="application/octet-stream")

    httpserver.expect_request("/items.parquet").respond_with_handler(respond)
    try:
        yield httpserver.url_for("/items.parquet"), requested, release
    finally:
        release.set()


@contextmanager
def duckdb_process(target: Callable[..., None], *args: object) -> Iterator[Connection]:
    """Isolate potential GIL deadlocks and bound process cleanup."""
    context = get_context("spawn")
    parent, child = context.Pipe()
    process = context.Process(target=target, args=(child, *args))
    process.start()
    child.close()
    try:
        yield parent
        process.join(timeout=15)
        assert process.exitcode == 0, "DuckDB worker failed or did not finish"
    finally:
        if process.is_alive():
            process.terminate()
            process.join(timeout=5)
            if process.is_alive():
                process.kill()
                process.join()
        parent.close()
        process.close()


def query_with_python_progress(
    pipe: Connection, method: str, args: tuple[object, ...]
) -> None:
    """Run a Python thread only after the parent observes the native query."""
    client = DuckdbClient()

    def report_progress() -> None:
        pipe.recv()
        pipe.send("progress")

    thread = threading.Thread(target=report_progress, daemon=True)
    thread.start()
    getattr(client, method)(*args)
    thread.join(timeout=10)
    assert not thread.is_alive()


def query_with_independent_client(pipe: Connection, url: str) -> None:
    """Complete a write on a second client while the first waits for HTTP."""
    client = DuckdbClient()
    other = DuckdbClient()
    other.execute("CREATE TABLE progress (value INTEGER)")

    def write_independently() -> None:
        pipe.recv()
        assert other.execute("INSERT INTO progress VALUES (42)") == 1
        pipe.send("progress")

    thread = threading.Thread(target=write_independently, daemon=True)
    thread.start()
    assert len(client.search(url)) == 100
    thread.join(timeout=10)
    assert not thread.is_alive()


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


def test_query_to_table(client: DuckdbClient) -> None:
    pytest.importorskip("arro3.core")
    table = client.query_to_table(
        "SELECT id FROM read_parquet('data/100-sentinel-2-items.parquet')"
    )
    assert table.num_rows == 100
    assert table.column_names == ["id"]


@pytest.mark.parametrize("method, sql", PARQUET_OPERATIONS)
def test_duckdb_allows_python_thread_progress(
    method: str,
    sql: str | None,
    gated_parquet_server: tuple[str, threading.Event, threading.Event],
) -> None:
    """Require Python progress while each native method waits for an HTTP response."""
    if method in {"query_to_table", "search_to_arrow"}:
        pytest.importorskip("arro3.core")
    url, requested, release = gated_parquet_server
    args = (sql, [url]) if sql else (url,)

    with duckdb_process(query_with_python_progress, method, args) as pipe:
        assert requested.wait(timeout=20), "DuckDB never requested the fixture"
        pipe.send("run")
        assert pipe.poll(10), "Python thread could not run during the query"
        assert pipe.recv() == "progress"
        release.set()


def test_independent_client_progress(
    gated_parquet_server: tuple[str, threading.Event, threading.Event],
) -> None:
    """A blocked query must not prevent another client from completing a write."""
    url, requested, release = gated_parquet_server
    with duckdb_process(query_with_independent_client, url) as pipe:
        assert requested.wait(timeout=20), "DuckDB never requested the fixture"
        pipe.send("run")
        assert pipe.poll(10), "Independent client could not complete its write"
        assert pipe.recv() == "progress"
        release.set()


@pytest.mark.parametrize("method, sql", PARQUET_OPERATIONS)
def test_missing_file_leaves_client_usable(
    client: DuckdbClient, tmp_path: Path, method: str, sql: str | None
) -> None:
    """Release the client lock after a native file error in each entry point."""
    if method in {"query_to_table", "search_to_arrow"}:
        pytest.importorskip("arro3.core")
    missing = str(tmp_path / "missing.parquet")
    args = (sql, [missing]) if sql else (missing,)
    with pytest.raises(RustacError, match="missing.parquet"):
        getattr(client, method)(*args)

    assert len(client.search("data/100-sentinel-2-items.parquet")) == 100


def test_http_error_leaves_client_usable(
    client: DuckdbClient, httpserver: HTTPServer
) -> None:
    """Propagate an HTTP failure without leaving the client locked."""
    httpserver.expect_request("/missing.parquet").respond_with_data(status=404)
    with pytest.raises(RustacError):
        client.search(httpserver.url_for("/missing.parquet"))

    assert len(client.search("data/100-sentinel-2-items.parquet")) == 100


def test_query_to_table_empty(client: DuckdbClient) -> None:
    """An empty SQL result retains its column schema."""
    pytest.importorskip("arro3.core")
    table = client.query_to_table("SELECT 42 AS answer WHERE false")
    assert table.num_rows == 0
    assert table.column_names == ["answer"]


def test_search_to_arrow_empty(client: DuckdbClient) -> None:
    """A search with no matching items returns None."""
    pytest.importorskip("arro3.core")
    assert (
        client.search_to_arrow(
            "data/100-sentinel-2-items.parquet", ids=["nonexistent-item"]
        )
        is None
    )


def test_load_spatial() -> None:
    DuckdbClient(extensions=["spatial"])


@pytest.mark.skip("slow")
def test_aws_credential_chain(client: DuckdbClient) -> None:
    client.execute("CREATE SECRET (TYPE S3, PROVIDER CREDENTIAL_CHAIN)")


def test_search_union_by_name(client: DuckdbClient) -> None:
    items = client.search("data/*.parquet")
    assert len(items) == 102


def test_max_items_type(client: DuckdbClient) -> None:
    # https://github.com/stac-utils/rustac-py/issues/228
    items = client.search("data/*.parquet", max_items=1)
    assert len(items) == 1


@pytest.mark.parametrize(
    "datetime",
    [
        "2026-04-01",
        "2026-04-01T00:00:00Z",
        "2026-04-01T00:00:00.000000Z",
        "2026-04-01T00:00:00Z/2026-04-01T23:59:59Z",
        "2026-03-31/2026-04-02",
        "2026-04-01/2026-04-02",
        "2026-04-15",
        "2026-04-15/2026-04-16",
        "2026-04-01/2026-04-30",
        "2026-04-01T00:00:00Z/..",
    ],
)
def test_datetime_searching(client: DuckdbClient, datetime: str) -> None:
    # https://github.com/stac-utils/rustac/issues/1070
    items = client.search("data/sentinel-1-global-mosaics.parquet", datetime=datetime)
    assert len(items) == 1


def test_search_filter_and_fields(client: DuckdbClient) -> None:
    # https://github.com/stac-utils/rustac-py/issues/361
    items = client.search(
        "data/100-sentinel-2-items.parquet",
        filter='"eo:cloud_cover" < 20',
        fields="+id",
    )
    assert len(items) == 50
