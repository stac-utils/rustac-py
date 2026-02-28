"""Pre-built DuckDB extensions for rustac."""

from pathlib import Path


def extension_directory() -> Path:
    """Returns the path to the bundled DuckDB extensions directory."""
    return Path(__file__).parent / "extensions"
