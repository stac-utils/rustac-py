import subprocess
import sysconfig
from pathlib import Path

from pytest_httpserver import HTTPServer


def rustac_cli() -> str:
    return str(Path(sysconfig.get_path("scripts")) / "rustac")


def test_search_header(httpserver: HTTPServer) -> None:
    # https://github.com/stac-utils/rustac-py/issues/359
    httpserver.expect_request(
        "/search", headers={"x-rustac-test": "cli"}
    ).respond_with_json({"features": [], "links": []})
    subprocess.run(
        [
            rustac_cli(),
            "search",
            httpserver.url_for("/"),
            "--header",
            "x-rustac-test=cli",
        ],
        check=True,
        capture_output=True,
    )


def test_search_multiple_headers(httpserver: HTTPServer) -> None:
    httpserver.expect_request(
        "/search",
        headers={"x-rustac-test": "cli", "x-rustac-other-test": "cli-other"},
    ).respond_with_json({"features": [], "links": []})
    subprocess.run(
        [
            rustac_cli(),
            "search",
            httpserver.url_for("/"),
            "--header",
            "x-rustac-test=cli",
            "--header",
            "x-rustac-other-test=cli-other",
        ],
        check=True,
        capture_output=True,
    )
