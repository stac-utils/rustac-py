# rustac

![The rustac logo](./img/rustac-small.png)

The power of Rust for the Python [STAC](https://stacspec.org/) ecosystem.

<!-- markdownlint-disable MD046-->
!!! tip

    We pronounce **rustac** "ruh-stac"

!!! note

    Until 2025-04-17, this package was named **stacrs**.
    See [this RFC](https://github.com/stac-utils/rustac/issues/641) for context on the name change.

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

## Acknowledgements

We'd like to thank [@jkeifer](https://github.com/jkeifer), [@parksjr](https://github.com/parksjr), and Rob Gomez (all from [@Element84](https://github.com/Element84/)) for creating the rustac logo from an AI-generated image from the prompt "a crab, a cabin, and a glass of whisky".
