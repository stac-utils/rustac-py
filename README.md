# rustac

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/stac-utils/rustac-py/ci.yaml?branch=main&style=for-the-badge)](https://github.com/stac-utils/rustac-py/actions/workflows/ci.yaml)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/stac-utils/rustac-py/docs.yaml?branch=main&style=for-the-badge&label=Docs)](https://stac-utils.github.io/rustac-py/latest/)
[![PyPI - Version](https://img.shields.io/pypi/v/rustac?style=for-the-badge)](https://pypi.org/project/rustac)
[![Conda forge](https://img.shields.io/conda/v/conda-forge/rustac?style=for-the-badge)](https://anaconda.org/conda-forge/rustac)
![PyPI - License](https://img.shields.io/pypi/l/rustac?style=for-the-badge)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](./CODE_OF_CONDUCT)

![The rustac logo](./img/rustac-small.png)

The power of Rust for the Python [STAC](https://stacspec.org/) ecosystem.

<!-- markdownlint-disable MD028 -->
> [!TIP]
> We pronounce **rustac** "ruh-stac".

> [!NOTE]
> Until 2025-04-17, this package was named **stacrs**.
> See [this RFC](https://github.com/stac-utils/rustac/issues/641) for context on the name change.
<!-- markdownlint-enable MD028 -->

## Why?

Q: We already have [PySTAC](https://github.com/stac-utils/pystac), so why **rustac**?

A: **rustac** can

- Read, write, and search [stac-geoparquet](https://github.com/stac-utils/stac-geoparquet)
- Go to and from [arrow](https://arrow.apache.org/) tables, allowing easy interoperability with (e.g.) [GeoPandas](https://geopandas.org/en/stable/)
- `async`

If you don't need those things, **rustac** probably isn't for you — use **pystac** and its friend, [pystac-client](https://github.com/stac-utils/pystac-client).

## Installation

**rustac** has zero required dependencies.
Install via **pip**:

```shell
# basic
python -m pip install rustac

# support arrow tables
python -m pip install 'rustac[arrow]'
```

Or via **conda**:

```shell
conda install conda-forge::rustac
```

### From source

You'll need [Rust](https://rustup.rs/).
By default, **rustac** wants to find DuckDB on your system:

```shell
brew install duckdb  # if you're using Homebrew ... if not, get DuckDB another way
python -m pip install -U git+https://github.com/stac-utils/rustac-py
```

If you don't want to (or can't) install DuckDB,  _can_ build DuckDB as a "bundled" build (warning: it takes a while):

```shell
MATURIN_PEP517_ARGS="--features=duckdb-bundled" python -m pip install -U git+https://github.com/stac-utils/rustac-py
```

## Usage

```python
import asyncio
import rustac

async def main() -> None:
    # Search a STAC API
    items = await rustac.search(
        "https://landsatlook.usgs.gov/stac-server",
        collections="landsat-c2l2-sr",
        intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        sortby="-properties.datetime",
        max_items=100,
    )

    # If you installed with `pystac[arrow]`:
    from geopandas import GeoDataFrame

    table = rustac.to_arrow(items)
    data_frame = GeoDataFrame.from_arrow(table)
    items = rustac.from_arrow(data_frame.to_arrow())

    # Write items to a stac-geoparquet file
    await rustac.write("/tmp/items.parquet", items)

    # Read items from a stac-geoparquet file as an item collection
    item_collection = await rustac.read("/tmp/items.parquet")

    # Use `search_to` for better performance if you know you'll be writing the items
    # to a file
    await rustac.search_to(
        "/tmp/items.parquet",
        "https://landsatlook.usgs.gov/stac-server",
        collections="landsat-c2l2-sr",
        intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        sortby="-properties.datetime",
        max_items=100,
    )

asyncio.run(main())
```

See [the documentation](https://stac-utils.github.io/rustac-py) for details.
In particular, our [examples](https://stac-utils.github.io/rustac-py/latest/generated/gallery/) demonstrate some of the more interesting features.

## Command line interface (CLI)

**rustac** comes with a CLI:

```bash
rustac -h
```

## stac-geoparquet

**rustac** replicates much of the behavior in the [stac-geoparquet](https://github.com/stac-utils/stac-geoparquet) library, and even uses some of the same Rust dependencies.
We believe there are a couple of issues with **stac-geoparquet** that make **rustac** a worthy replacement:

- The **stac-geoparquet** repo includes Python dependencies
- It doesn't have a nice one-shot API for reading and writing
- It includes some leftover code and logic from its genesis as a tool for the [Microsoft Planetary Computer](https://planetarycomputer.microsoft.com/)

We test to ensure [compatibility](https://github.com/stac-utils/rustac/blob/main/scripts/validate-stac-geoparquet) between the two libraries, and we intend to consolidate to a single "stac-geoparquet" library at some point in the future.

## Development

Get [Rust](https://rustup.rs/), [uv](https://docs.astral.sh/uv/getting-started/installation/), and (optionally) [libduckdb](https://duckdb.org/docs/installation/index).
Then:

```shell
git clone git@github.com:stac-utils/rustac-py.git
cd rustac-py
scripts/test
```

See [CONTRIBUTING.md](./CONTRIBUTING.md) for more information about contributing to this project.

### DuckDB

By default, this package expects **libduckdb** to be present on your system.
If you get this sort of error when building:

```shell
  = note: ld: library 'duckdb' not found
```

Set your `DUCKDB_LIB_DIR` to point to your **libduckdb**.
If you're using [homebrew](https://brew.sh/), that might look like this:

```shell
export DUCKDB_LIB_DIR=/opt/homebrew/lib
```

Alternatively, you can use the `duckdb-bundled` feature to build DuckDB bindings into the Rust library:

```shell
maturin dev --uv -F duckdb-bundled && pytest
```

> [!WARNING]
> Building DuckDB [bundled](https://github.com/duckdb/duckdb-rs?tab=readme-ov-file#notes-on-building-duckdb-and-libduckdb-sys) takes a long while.

### Docs

If you want to run an off-cycle docs update (e.g. if you fixed something and want to post it without having to make a new release):

```shell
mike deploy [version] latest --push
```

## Acknowledgements

We'd like to thank [@jkeifer](https://github.com/jkeifer), [@parksjr](https://github.com/parksjr), and [@Xenocide122](https://github.com/Xenocide122) (all from [@Element84](https://github.com/Element84)) for creating the [rustac logo](https://raw.githubusercontent.com/stac-utils/rustac/refs/heads/main/img/rustac.svg) from an AI-generated image from this prompt:

> There is a library for working with STAC metadata that is written in rust called rustac: <https://github.com/stac-utils/rustac>. That name sounds like the word "rustic", and is meant to envoke (sic) an image of "a cabin and a glass of neat whisky".

## License

**rustac-py** is dual-licensed under both the MIT license and the Apache license (Version 2.0).
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
