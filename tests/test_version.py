import stacrs


def test_version() -> None:
    assert stacrs.version() is not None
    assert stacrs.version("stac") is not None
    assert stacrs.version("stac-api") is not None
    assert stacrs.version("stac-duckdb") is not None
