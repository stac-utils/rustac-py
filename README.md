# stacrs

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/gadomski/stacrs/ci.yaml?branch=main&style=for-the-badge)](https://github.com/gadomski/stacrs/actions/workflows/ci.yaml)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/gadomski/stacrs/docs.yaml?branch=main&style=for-the-badge&label=Docs)](https://www.gadom.ski/stacrs/latest/)
[![PyPI - Version](https://img.shields.io/pypi/v/stacrs?style=for-the-badge)](https://pypi.org/project/stacrs)
[![Conda Downloads](https://img.shields.io/conda/d/conda-forge/stacrs?style=for-the-badge)](https://anaconda.org/conda-forge/stacrs)
![PyPI - License](https://img.shields.io/pypi/l/stacrs?style=for-the-badge)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](./CODE_OF_CONDUCT)

A small no-dependency Python package for [STAC](https://stacspec.org/), using Rust under the hood.

## Usage

Install via **pip**:

```shell
python -m pip install stacrs
```

Or via **conda**:

```shell
conda install conda-forge::stacrs
```

Then:

```python
import stacrs

# Search a STAC API
items = stacrs.search(
    "https://landsatlook.usgs.gov/stac-server",
    collections="landsat-c2l2-sr",
    intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
    sortby="-properties.datetime",
    max_items=100,
)

# Write items to a stac-geoparquet file
stacrs.write("items.parquet", items)

# Read items from a stac-geoparquet file as an item collection
item_collection = stacrs.read("items.parquet")

# Use `search_to` for better performance if you know you'll be writing the items
# to a file
stacrs.search_to(
    "items.parquet",
    "https://landsatlook.usgs.gov/stac-server",
    collections="landsat-c2l2-sr",
    intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
    sortby="-properties.datetime",
    max_items=100,
)
```

See [the documentation](https://www.gadom.ski/stacrs) for details.
In particular, our [example notebook](https://www.gadom.ski/stacrs/latest/example/) demonstrates some of the more interesting features.

## Comparisons

This package (intentionally) has limited functionality, as it is _not_ intended to be a replacement for existing Python STAC packages.
[pystac](https://pystac.readthedocs.io) is a mature Python library with a significantly richer API for working with STAC objects.
For querying STAC APIs, [pystac-client](https://pystac-client.readthedocs.io) is more feature-rich than our simplistic `stacrs.search`.

That being said, it is hoped that **stacrs** will be a nice complement to the existing Python STAC ecosystem by providing a no-dependency package with unique capabilities, such as searching directly into a stac-geoparquet file.

## Development

Get [Rust](https://rustup.rs/) and [uv](https://docs.astral.sh/uv/getting-started/installation/).
Then:

```shell
git clone git@github.com:gadomski/stacrs.git
cd stacrs
scripts/test  # This will take a little while while the Rust dependencies build, especially DuckDB
```

See [CONTRIBUTING.md](./CONTRIBUTING.md) for more information about contributing to this project.

## License

**stacrs** is dual-licensed under both the MIT license and the Apache license (Version 2.0).
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
