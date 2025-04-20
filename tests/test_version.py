import rustac


def test_version() -> None:
    assert rustac.version() is not None
    assert rustac.version("stac") is not None
    assert rustac.version("stac-api") is not None
    assert rustac.version("stac-duckdb") is not None


def test_sha() -> None:
    assert rustac.sha() is not None
