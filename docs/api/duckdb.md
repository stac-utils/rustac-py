---
description: Query stac-geoparquet with DuckDB
---

# DuckDB

## Extensions

DuckDB requires the **spatial**, **icu**, and **parquet** extensions.
By default, `DuckdbClient` downloads these at runtime via `INSTALL`.

To skip the runtime download, install the **rustac-duckdb-extensions** package, which ships pre-built extension binaries:

```shell
python -m pip install rustac-duckdb-extensions
```

Or as an extra:

```shell
python -m pip install 'rustac[duckdb-extensions]'
```

When `rustac-duckdb-extensions` is installed, `DuckdbClient` will automatically detect and use the bundled extensions — no configuration needed:

```python
import rustac

client = rustac.DuckdbClient()
items = client.search("data.parquet")
```

!!! tip

    This is especially useful in environments where network access is restricted or you want reproducible, hermetic builds.

`execute`, `query_to_table`, `search`, `search_to_arrow`, and `get_collections` are synchronous and
serialize through a client's single connection. They release the Python GIL during native query
work, allowing other Python threads to run, but calling them on an asyncio event-loop thread still
blocks that loop until the method returns. Use a worker thread when calling them from an event loop.

## API

::: rustac.DuckdbClient
