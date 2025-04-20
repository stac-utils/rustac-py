# rustac

> The power of Rust for the Python STAC ecosystem.

**rustac** is a zero-dependency Python package for [STAC](https://stacspec.org/) using Rust under the hood.

## Installation

```shell
python -m pip install rustac
```

If you'd like to use `arrow` tables, e.g. to load into GeoPandas:

```shell
python -m pip install 'rustac[arrow]'
```

## Usage

```python
import asyncio
import rustac

def main() -> None:
    item = await rustac.read("item.json")

asyncio.run(main())
```

For more, see our [examples](./generated/gallery/index.md).
