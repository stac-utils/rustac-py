"""Builds a platform-specific wheel for rustac-duckdb-extensions.

Downloads the DuckDB extensions for the given platform, builds a wheel using
hatchling, and sets the appropriate platform tag.
"""

import subprocess
import sys
from pathlib import Path

from download_extensions import download_extensions

PLATFORM_MAP = {
    "linux_amd64": "manylinux_2_28_x86_64",
    "linux_arm64": "manylinux_2_28_aarch64",
    "osx_amd64": "macosx_11_0_x86_64",
    "osx_arm64": "macosx_11_0_arm64",
}

DUCKDB_EXTENSIONS_DIR = Path(__file__).parent.parent


def build_wheel(duckdb_platform: str) -> None:
    """Builds a platform-tagged wheel for the given DuckDB platform.

    Args:
        duckdb_platform: The DuckDB platform string (e.g. osx_arm64, linux_amd64).
    """
    wheel_platform = PLATFORM_MAP[duckdb_platform]

    download_extensions(duckdb_platform)

    dist_dir = DUCKDB_EXTENSIONS_DIR / "dist"
    subprocess.run(
        [sys.executable, "-m", "hatchling", "build", "-t", "wheel"],
        cwd=DUCKDB_EXTENSIONS_DIR,
        check=True,
    )

    for whl in dist_dir.glob("*.whl"):
        subprocess.run(
            [
                sys.executable,
                "-m",
                "wheel",
                "tags",
                "--remove",
                "--platform-tag",
                wheel_platform,
                str(whl),
            ],
            cwd=dist_dir,
            check=True,
        )


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <duckdb-platform>")
        print(f"Platforms: {', '.join(PLATFORM_MAP)}")
        sys.exit(1)
    duckdb_platform = sys.argv[1]
    if duckdb_platform not in PLATFORM_MAP:
        print(f"Unknown platform: {duckdb_platform}")
        print(f"Platforms: {', '.join(PLATFORM_MAP)}")
        sys.exit(1)
    build_wheel(duckdb_platform)
