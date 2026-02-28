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

## API

::: rustac.DuckdbClient
