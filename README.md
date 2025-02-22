# stacrs

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/stac-utils/stacrs/ci.yaml?branch=main&style=for-the-badge)](https://github.com/stac-utils/stacrs/actions/workflows/ci.yaml)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/stac-utils/stacrs/docs.yaml?branch=main&style=for-the-badge&label=Docs)](https://stac-utils.github.io/stacrs/latest/)
[![PyPI - Version](https://img.shields.io/pypi/v/stacrs?style=for-the-badge)](https://pypi.org/project/stacrs)
[![Conda Downloads](https://img.shields.io/conda/d/conda-forge/stacrs?style=for-the-badge)](https://anaconda.org/conda-forge/stacrs)
![PyPI - License](https://img.shields.io/pypi/l/stacrs?style=for-the-badge)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](./CODE_OF_CONDUCT)

A small no-dependency Python package for [STAC](https://stacspec.org/), using Rust under the hood.

## Why?

Q: We already have [PySTAC](https://github.com/stac-utils/pystac), so why did we make a new STAC Python library?

A: We want to provide a couple of things in [stac-rs](https://github.com/stac-utils/stac-rs) (a collection of STAC Rust libraries) to the Python ecosystem:

- Read, write, and search [stac-geoparquet](https://github.com/stac-utils/stac-geoparquet)
- `async` functions

If you don't need those things, **stacrs** probably isn't for you — use **pystac** and its friend, [pystac-client](https://github.com/stac-utils/pystac-client), instead.

## Usage

Install via **pip**:

```shell
# basic
python -m pip install stacrs

# add methods to return arrow tables
python -m pip install 'stacrs[arrow]'
```

Or via **conda**:

```shell
conda install conda-forge::stacrs
```

Then:

```python
import stacrs

# Search a STAC API
items = await stacrs.search(
    "https://landsatlook.usgs.gov/stac-server",
    collections="landsat-c2l2-sr",
    intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
    sortby="-properties.datetime",
    max_items=100,
)

# If you installed with `pystac[arrow]`:
from geopandas import GeoDataFrame
table = await stacrs.search_to_arrow(...)
data_frame = GeoDataFrame.from_arrow(table)

# Write items to a stac-geoparquet file
await stacrs.write("items.parquet", items)

# Read items from a stac-geoparquet file as an item collection
item_collection = await stacrs.read("items.parquet")

# You can search geoparquet files using DuckDB
# If you want to search a file on s3, make sure to configure your AWS environment first
item_collection = await stacrs.search("s3://bucket/items.parquet", ...)

# Use `search_to` for better performance if you know you'll be writing the items
# to a file
await stacrs.search_to(
    "items.parquet",
    "https://landsatlook.usgs.gov/stac-server",
    collections="landsat-c2l2-sr",
    intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
    sortby="-properties.datetime",
    max_items=100,
)
```

See [the documentation](https://stac-utils.github.io/stacrs) for details.
In particular, our [example notebook](https://stac-utils.github.io/stacrs/latest/example/) demonstrates some of the more interesting features.

## CLI

**stacrs** comes with a CLI:

```shell
$ stacrs -h
stacrs: A command-line interface for the SpatioTemporal Asset Catalog (STAC)

Usage: stacrs [OPTIONS] <COMMAND>

Commands:
  translate  Translates STAC from one format to another
  search     Searches a STAC API or stac-geoparquet file
  serve      Serves a STAC API
  validate   Validates a STAC value
  help       Print this message or the help of the given subcommand(s)

Options:
  -i, --input-format <INPUT_FORMAT>
          The input format.
      --opt <OPTIONS>
          Options for getting and putting files from object storage.
  -o, --output-format <OUTPUT_FORMAT>
          The output format.
  -c, --compact-json <COMPACT_JSON>
          Whether to print compact JSON output [possible values: true, false]
      --parquet-compression <PARQUET_COMPRESSION>
          The parquet compression to use when writing stac-geoparquet.
  -h, --help
          Print help (see more with '--help')
```

> [!NOTE]
> Before **stacrs** v0.5.4, the CLI was its own PyPI package named **stacrs-cli**, which is no longer needed.

## Comparisons

This package (intentionally) has limited functionality, as it is _not_ intended to be a replacement for existing Python STAC packages.
[pystac](https://pystac.readthedocs.io) is a mature Python library with a significantly richer API for working with STAC objects.
For querying STAC APIs, [pystac-client](https://pystac-client.readthedocs.io) is more feature-rich than our simplistic `stacrs.search`.

That being said, it is hoped that **stacrs** will be a nice complement to the existing Python STAC ecosystem by providing a no-dependency package with unique capabilities, such as searching directly into a stac-geoparquet file.

### stac-geoparquet

**stacrs** also replicates much of the behavior in the [stac-geoparquet](https://github.com/stac-utils/stac-geoparquet) library, and even uses some of the same Rust dependencies.
We believe there are a couple of issues with **stac-geoparquet** that make **stacrs** a worthy replacement:

- The **stac-geoparquet** repo includes Python dependencies
- It doesn't have a nice one-shot API for reading and writing
- It includes some leftover code and logic from its genesis as a tool for the [Microsoft Planetary Computer](https://planetarycomputer.microsoft.com/)

We test to ensure [compatibility](https://github.com/stac-utils/stac-rs/blob/main/scripts/validate-stac-geoparquet) between the two libraries, and we intend to consolidate to a single "stac-geoparquet" library at some point in the future.

## Development

Get [Rust](https://rustup.rs/), [uv](https://docs.astral.sh/uv/getting-started/installation/), and [libduckdb](https://duckdb.org/docs/installation/index) (for more on setting up **libduckdb**, [see this](#duckdb)).
Then:

```shell
git clone git@github.com:stac-utils/stacrs.git
cd stacrs
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

> [!NOTE]
> We used to use the [bundled](https://github.com/duckdb/duckdb-rs?tab=readme-ov-file#notes-on-building-duckdb-and-libduckdb-sys) feature of DuckDB, but it was making our build times intolerably slow.

## License

**stacrs** is dual-licensed under both the MIT license and the Apache license (Version 2.0).
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
