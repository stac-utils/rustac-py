"""DuckdbClient wrapper with auto-detection of bundled extensions."""

from __future__ import annotations

from pathlib import Path

from rustac.rustac import DuckdbClient as _RustDuckdbClient


def _detect_extension_directory() -> Path | None:
    try:
        from rustac_duckdb_extensions import extension_directory

        return extension_directory()
    except ImportError:
        return None


class DuckdbClient(_RustDuckdbClient):
    """A client for querying stac-geoparquet with DuckDB.

    If ``rustac-duckdb-extensions`` is installed and no ``extension_directory``
    is provided, the bundled extensions will be used automatically and
    ``install_extensions`` will be set to ``False``.
    """

    def __init__(
        self,
        *,
        extension_directory: Path | None = None,
        extensions: list[str] | None = None,
        install_extensions: bool = True,
        use_hive_partitioning: bool = False,
    ) -> None:
        if extension_directory is None:
            detected = _detect_extension_directory()
            if detected is not None:
                extension_directory = detected
                install_extensions = False
        super().__init__(
            extension_directory=extension_directory,
            extensions=extensions or [],
            install_extensions=install_extensions,
            use_hive_partitioning=use_hive_partitioning,
        )
