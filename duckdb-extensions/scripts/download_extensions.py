"""Downloads DuckDB extensions for a given platform."""

import gzip
import sys
import urllib.request
from pathlib import Path

DUCKDB_VERSION = "v1.4.4"
EXTENSIONS = ["spatial", "icu", "parquet"]
BASE_URL = "http://extensions.duckdb.org"


def download_extensions(platform: str) -> None:
    """Downloads and decompresses DuckDB extensions for the given platform.

    Args:
        platform: The DuckDB platform string (e.g. osx_arm64, linux_amd64).
    """
    output_dir = (
        Path(__file__).parent.parent
        / "python"
        / "rustac_duckdb_extensions"
        / "extensions"
        / DUCKDB_VERSION
        / platform
    )
    output_dir.mkdir(parents=True, exist_ok=True)

    for ext in EXTENSIONS:
        url = f"{BASE_URL}/{DUCKDB_VERSION}/{platform}/{ext}.duckdb_extension.gz"
        output_path = output_dir / f"{ext}.duckdb_extension"
        print(f"Downloading {url}")
        response = urllib.request.urlopen(url)  # noqa: S310
        compressed = response.read()
        decompressed = gzip.decompress(compressed)
        output_path.write_bytes(decompressed)
        print(f"  -> {output_path} ({len(decompressed)} bytes)")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <platform>")
        print("Platforms: linux_amd64, linux_arm64, osx_amd64, osx_arm64")
        sys.exit(1)
    download_extensions(sys.argv[1])
