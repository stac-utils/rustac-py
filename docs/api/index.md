---
description: The stacrs Python API
---

# Python API

API documentation for the **stacrs** Python package.

## Format

Several functions, including [stacrs.write][], take a `format` argument.
Valid values are:

- `json` or `geojson`: compact (no whitespace) JSON
- `json-pretty` or `geojson-pretty`: indented JSON
- `ndjson`: newline-delimited JSON (also known as `geojson-seq`)
- `parquet` or `geoparquet`: uncompressed geoparquet
- `parquet[{compression}]` or `geoparquet[{compression}]`: compressed parquet, where valid values for `compression` are the lowercase string versions of the values enumerated in [stac::geoparquet::Compression](https://docs.rs/stac/latest/stac/geoparquet/enum.Compression.html).

!!! tip

    If you're not sure which geoparquet compression to use, we suggest `parquet[snappy]`

!!! note

    The distinction between pretty and compact JSON, or compressed and uncompressed geoparquet, is only relevant on write.
    On read, the formats are treated the same.

Under the hood, the `format` argument is parsed into a [Format](https://docs.rs/stac/latest/stac/enum.Format.html) enum.
