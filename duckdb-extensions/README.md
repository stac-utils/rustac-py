# rustac-duckdb-extensions

Pre-built [DuckDB](https://duckdb.org/) extensions for [rustac](https://github.com/stac-utils/rustac-py).

This package bundles the **spatial**, **icu**, and **parquet** DuckDB extensions so that rustac can use them without downloading extensions at runtime.

## Usage

```python
import rustac_duckdb_extensions

path = rustac_duckdb_extensions.extension_directory()
```

## Building wheels locally

Install the build dependencies, then run the build script with a DuckDB platform target:

```shell
pip install hatchling wheel
cd duckdb-extensions/scripts
python build_wheel.py osx_arm64
```

Available platforms: `linux_amd64`, `linux_arm64`, `osx_amd64`, `osx_arm64`.

The built wheel will be in `duckdb-extensions/dist/`.
