"""The power of Rust for the Python STAC ecosystem."""

from __future__ import annotations

from typing import Any, AsyncIterator, Literal, Optional, Tuple, TypedDict

import arro3.core

class Catalog(TypedDict):
    """A STAC Catalog object represents a logical group of other Catalog, Collection, and Item objects."""

    type: str = "Catalog"
    """Set to Catalog if this Catalog only implements the Catalog spec."""

    stac_version: str
    """The STAC version the Catalog implements."""

    stac_extensions: list[str] | None
    """A list of extension identifiers the Catalog implements."""

    id: str
    """Identifier for the Catalog."""

    title: str | None
    """A short descriptive one-line title for the Catalog."""

    description: str
    """Detailed multi-line description to fully explain the Catalog.
    
    CommonMark 0.29 syntax MAY be used for rich text representation."""

    links: list[Link]
    """A list of references to other documents."""

class Collection(TypedDict):
    """The STAC Collection Specification defines a set of common fields to describe a group of Items that share properties and metadata."""

    type: str = "Collection"
    """Must be set to Collection to be a valid Collection."""

    stac_version: str
    """The STAC version the Collection implements."""

    stac_extensions: list[str] | None
    """A list of extension identifiers the Collection implements."""

    id: str
    """Identifier for the Collection that is unique across all collections in the root catalog."""

    title: str | None
    """A short descriptive one-line title for the Collection."""

    description: str
    """Detailed multi-line description to fully explain the Collection.
    
    CommonMark 0.29 syntax MAY be used for rich text representation."""

    keywords: list[str] | None
    """List of keywords describing the Collection."""

    license: str
    """License(s) of the data collection as SPDX License identifier, SPDX License expression, or `other`."""

    providers: list[Provider] | None
    """A list of providers, which may include all organizations capturing or processing the data or the hosting provider."""

    extent: Extent
    """Spatial and temporal extents."""

    summaries: dict[str, Any]
    """A map of property summaries, either a set of values, a range of values or a JSON Schema."""

    links: list[Link]
    """A list of references to other documents."""

    assets: dict[str, Asset] | None
    """Dictionary of asset objects that can be downloaded, each with a unique key."""

    item_assets: dict[str, ItemAsset] | None
    """A dictionary of assets that can be found in member Items."""

class Provider(TypedDict):
    """A provider is any of the organizations that captures or processes the content of the Collection and therefore influences the data offered by this Collection."""

    name: str
    """The name of the organization or the individual."""

    description: str | None
    """Multi-line description to add further provider information such as processing details for processors and producers, hosting details for hosts or basic contact information.
    
    CommonMark 0.29 syntax MAY be used for rich text representation."""

    roles: list[
        Literal["licensor"]
        | Literal["producer"]
        | Literal["processor"]
        | Literal["host"]
    ]
    """Roles of the provider."""

    url: str | None
    """Homepage on which the provider describes the dataset and publishes contact information."""

class Extent(TypedDict):
    """The object describes the spatio-temporal extents of the Collection."""

    spatial: SpatialExtent
    """Potential spatial extents covered by the Collection."""

    temporal: TemporalExtent
    """Potential temporal extents covered by the Collection."""

class SpatialExtent(TypedDict):
    """The object describes the spatial extents of the Collection."""

    bbox: list[list[int | float]]
    """Potential spatial extents covered by the Collection."""

class TemporalExtent(TypedDict):
    """The object describes the temporal extents of the Collection."""

    bbox: list[list[str | None]]
    """Potential temporal extents covered by the Collection."""

class ItemAsset(TypedDict):
    """An Item Asset Object defined at the Collection level is nearly the same as the Asset Object in Items, except for two differences.

    The href field is not required, because Item Asset Definitions don't point to any data by themselves, but at least two other fields must be present."""

    title: str | None
    """The displayed title for clients and users."""

    description: str | None
    """A description of the Asset providing additional details, such as how it was processed or created.
    
    CommonMark 0.29 syntax MAY be used for rich text representation."""

    type: str | None
    """Media type of the asset."""

    roles: list[str] | None
    """The semantic roles of the asset, similar to the use of rel in links."""

class Item(TypedDict):
    """An Item is a GeoJSON Feature augmented with foreign members relevant to a STAC object."""

    type: str = "Feature"
    """Type of the GeoJSON Object. MUST be set to Feature."""

    stac_version: str
    """The STAC version the Item implements."""

    stac_extensions: list[str] | None
    """A list of extensions the Item implements."""

    id: str
    """Provider identifier. The ID should be unique within the Collection that contains the Item."""

    geometry: dict[str, Any] | None
    """Defines the full footprint of the asset represented by this item, formatted according to RFC 7946, section 3.1 if a geometry is provided or section 3.2 if no geometry is provided."""

    bbox: list[int | float] | None
    """REQUIRED if geometry is not null, prohibited if geometry is null.
    
    Bounding Box of the asset represented by this Item, formatted according to RFC 7946, section 5."""

    properties: Properties
    """A dictionary of additional metadata for the Item."""

    links: list[Link]
    """List of link objects to resources and related URLs.
    
    See the best practices for details on when the use self links is strongly recommended."""

    assets: dict[str, Asset]
    """Dictionary of asset objects that can be downloaded, each with a unique key."""

    collection: str | None
    """The id of the STAC Collection this Item references to.
    
    This field is required if a link with a collection relation type is present and is not allowed otherwise."""

class Properties(TypedDict):
    """Additional metadata fields can be added to the GeoJSON Object Properties."""

    datetime: str | None
    """The searchable date and time of the assets, which must be in UTC.
    
    It is formatted according to RFC 3339, section 5.6. null is allowed, but requires start_datetime and end_datetime from common metadata to be set."""

class Link(TypedDict):
    """This object describes a relationship with another entity.

    Data providers are advised to be liberal with the links section, to describe
    things like the Catalog an Item is in, related Items, parent or child Items
    (modeled in different ways, like an 'acquisition' or derived data)."""

    href: str
    """The actual link in the format of an URL.
    
    Relative and absolute links are both allowed. Trailing slashes are significant."""

    rel: str
    """Relationship between the current document and the linked document."""

    type: str | None
    """Media type of the referenced entity."""

    title: str | None
    """A human readable title to be used in rendered displays of the link."""

    method: str | None
    """The HTTP method that shall be used for the request to the target resource, in uppercase.
    
    GET by default"""

    headers: dict[str, str | list[str]] | None
    """The HTTP headers to be sent for the request to the target resource."""

    body: Any | None
    """The HTTP body to be sent to the target resource."""

class Asset(TypedDict):
    """An Asset is an object that contains a URI to data associated with the Item that can be downloaded or streamed.

    It is allowed to add additional fields."""

    href: str
    """URI to the asset object. Relative and absolute URI are both allowed. Trailing slashes are significant."""

    title: str | None
    """The displayed title for clients and users."""

    description: str | None
    """A description of the Asset providing additional details, such as how it was processed or created.
    
    CommonMark 0.29 syntax MAY be used for rich text representation."""

    type: str | None
    """Media type of the asset.
    
    See the common media types in the best practice doc for commonly used asset types."""

    roles: list[str] | None
    """The semantic roles of the asset, similar to the use of rel in links."""

class ItemCollection(TypedDict):
    """A GeoJSON feature collection of STAC Items."""

    features: list[Item]
    """STAC items."""

class RustacError(Exception):
    """A package-specific exception."""

class DuckdbClient:
    """A client for querying stac-geoparquet with DuckDB."""

    def __init__(
        self,
        *,
        use_s3_credential_chain: bool = True,
        use_azure_credential_chain: bool = True,
        use_httpfs: bool = True,
        use_hive_partitioning: bool = False,
        install_extensions: bool = True,
        custom_extension_repository: str | None = None,
        extension_directory: str | None = None,
    ) -> None:
        """Creates a new duckdb client.

        Args:
            use_s3_credential_chain: If true, configures DuckDB to correctly
                handle s3:// urls.
            use_azure_credential_chain: If true, configures DuckDB to correctly
                handle azure urls.
            use_httpfs: If true, configures DuckDB to correctly handle https
                urls.
            use_hive_partitioning: If true, enables queries on hive partitioned
                geoparquet files.
            install_extensions: If true, installs extensions before loading them.
            custom_extension_repository: A custom extension repository to use.
            extension_directory: A non-standard extension directory to use.
        """

    def search(
        self,
        href: str,
        *,
        ids: Optional[str | list[str]] = None,
        collections: Optional[str | list[str]] = None,
        intersects: Optional[str | dict[str, Any]] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        bbox: Optional[list[float]] = None,
        datetime: Optional[str] = None,
        include: Optional[str | list[str]] = None,
        exclude: Optional[str | list[str]] = None,
        sortby: Optional[str | list[str | dict[str, str]]] = None,
        filter: Optional[str | dict[str, Any]] = None,
        query: Optional[dict[str, Any]] = None,
        **kwargs: str,
    ) -> dict[str, Any]:
        """Search a stac-geoparquet file with duckdb, returning an item collection.

        Args:
            href: The stac-geoparquet file.
            ids: Array of Item ids to return.
            collections: Array of one or more Collection IDs that each matching
                Item must be in.
            intersects: Searches items by performing intersection between their
                geometry and provided GeoJSON geometry.
            limit: The number of items to return.
            offset: The number of items to skip before returning.
            bbox: Requested bounding box.
            datetime: Single date+time, or a range (`/` separator), formatted to
                RFC 3339, section 5.6.  Use double dots .. for open date ranges.
            include: fields to include in the response (see [the extension
                docs](https://github.com/stac-api-extensions/fields?tab=readme-ov-file#includeexclude-semantics))
                for more on the semantics).
            exclude: fields to exclude from the response (see [the extension
                docs](https://github.com/stac-api-extensions/fields?tab=readme-ov-file#includeexclude-semantics))
                for more on the semantics).
            sortby: Fields by which to sort results (use `-field` to sort descending).
            filter: CQL2 filter expression. Strings will be interpreted as
                cql2-text, dictionaries as cql2-json.
            query: Additional filtering based on properties.  It is recommended
                to use filter instead, if possible.
            kwargs: Additional parameters to pass in to the search.

        Returns:
            A feature collection of STAC items.
        """

    def search_to_arrow(
        self,
        href: str,
        *,
        ids: Optional[str | list[str]] = None,
        collections: Optional[str | list[str]] = None,
        intersects: Optional[str | dict[str, Any]] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        bbox: Optional[list[float]] = None,
        datetime: Optional[str] = None,
        include: Optional[str | list[str]] = None,
        exclude: Optional[str | list[str]] = None,
        sortby: Optional[str | list[str | dict[str, str]]] = None,
        filter: Optional[str | dict[str, Any]] = None,
        query: Optional[dict[str, Any]] = None,
        **kwargs: str,
    ) -> arro3.core.Table | None:
        """Search a stac-geoparquet file with duckdb, returning an arrow table
        suitable for loading into (e.g.) GeoPandas.

        **rustac** must be installed with the `arrow` extra, e.g. `python -m pip
        *install 'rustac[arrow]'.

        Because DuckDB has arrow as a core output format, this can be more
        performant than going through a JSON dictionary.

        Args:
            href: The stac-geoparquet file.
            ids: Array of Item ids to return.
            collections: Array of one or more Collection IDs that each matching
                Item must be in.
            intersects: Searches items by performing intersection between their
                geometry and provided GeoJSON geometry.
            limit: The number of items to return.
            offset: The number of items to skip before returning.
            bbox: Requested bounding box.
            datetime: Single date+time, or a range (`/` separator), formatted to
                RFC 3339, section 5.6.  Use double dots .. for open date ranges.
            include: fields to include in the response (see [the extension
                docs](https://github.com/stac-api-extensions/fields?tab=readme-ov-file#includeexclude-semantics))
                for more on the semantics).
            exclude: fields to exclude from the response (see [the extension
                docs](https://github.com/stac-api-extensions/fields?tab=readme-ov-file#includeexclude-semantics))
                for more on the semantics).
            sortby: Fields by which to sort results (use `-field` to sort descending).
            filter: CQL2 filter expression. Strings will be interpreted as
                cql2-text, dictionaries as cql2-json.
            query: Additional filtering based on properties.  It is recommended
                to use filter instead, if possible.
            kwargs: Additional parameters to pass in to the search.

        Returns:
            An arrow table, or none if no records were returned.

        Examples:
            >>> table = client.search_to_arrow("data/100-sentinel-2-items.parquet")
            >>> data_frame = GeoDataFrame.from_arrow(table)
        """

    def get_collections(self, href: str) -> list[Collection]:
        """Returns all collections in this stac-geoparquet file.

        These collections will be auto-generated from the STAC items, one
        collection per id in the `collections` column.

        Eventually, these collections might be stored in the stac-geoparquet
        metadata and retrieved from there, but that's not the case yet.

        Args:
            href: The stac-geoparquet file to build the collections from.

        Returns:
            A list of STAC Collections
        """

def migrate(value: dict[str, Any], version: Optional[str] = None) -> dict[str, Any]:
    """
    Migrates a STAC dictionary to another version.

    Migration can be as simple as updating the `stac_version` attribute, but
    sometimes can be more complicated. For example, when migrating to v1.1.0,
    [eo:bands and raster:bands should be consolidated to the new bands
    structure](https://github.com/radiantearth/stac-spec/releases/tag/v1.1.0-beta.1).

    See [the rustac
    documentation](https://docs.rs/stac/latest/stac/enum.Version.html) for
    supported versions.

    Args:
        value: The STAC value to migrate
        version: The version to migrate to. If not provided, the
            value will be migrated to the latest stable version.

    Returns:
        The migrated dictionary

    Examples:
        >>> with open("examples/simple-item.json") as f:
        >>>     item = json.load(f)
        >>> item = rustac.migrate(item, "1.1.0-beta.1")
        >>> assert item["stac_version"] == "1.1.0-beta.1"
    """

async def read(
    href: str,
    *,
    format: str | None = None,
    options: list[tuple[str, str]] | None = None,
    set_self_link: bool = True,
) -> dict[str, Any]:
    """
    Reads STAC from a href.

    Args:
        href: The href to write to
        format: The input format. If not provided, will be inferred
            from the href's extension.
        options: Options for configuring an
            object store, e.g. your AWS credentials.
        set_self_link: If True, set the `self` link to the value of `href`.

    Returns:
        The STAC value

    Examples:
        >>> item = await rustac.read("item.json")
    """

def from_arrow(
    table: arro3.core.Table,
) -> ItemCollection:
    """
    Converts an [arro3.core.Table][] to a STAC item collection.

    Requires **rustac** to be installed with the `arrow` extra.

    Args:
        table: The table

    Returns:
        The STAC item collection
    """

def to_arrow(
    items: list[Item] | ItemCollection,
) -> arro3.core.Table:
    """
    Converts items to an [arro3.core.Table][].

    Requires **rustac** to be installed with the `arrow` extra.

    Args:
        items: Either a list of items or a item collection

    Returns:
        The table
    """

async def search(
    href: str,
    *,
    intersects: Optional[str | dict[str, Any]] = None,
    ids: Optional[str | list[str]] = None,
    collections: Optional[str | list[str]] = None,
    max_items: Optional[int] = None,
    limit: Optional[int] = None,
    bbox: Optional[list[float]] = None,
    datetime: Optional[str] = None,
    include: Optional[str | list[str]] = None,
    exclude: Optional[str | list[str]] = None,
    sortby: Optional[str | list[str | dict[str, str]]] = None,
    filter: Optional[str | dict[str, Any]] = None,
    query: Optional[dict[str, Any]] = None,
    use_duckdb: Optional[bool] = None,
    **kwargs: str,
) -> dict[str, Any]:
    """
    Searches a STAC API server.

    Args:
        href: The STAC API to search.
        intersects: Searches items
            by performing intersection between their geometry and provided GeoJSON
            geometry.
        ids: Array of Item ids to return.
        collections: Array of one or more Collection IDs that
            each matching Item must be in.
        max_items: The maximum number of items to iterate through.
        limit: The page size returned from the server. Use
            `max_items` to actually limit the number of items returned from this
            function.
        bbox: Requested bounding box.
        datetime: Single date+time, or a range (`/` separator),
            formatted to RFC 3339, section 5.6.  Use double dots .. for open
            date ranges.
        include: fields to include in the response (see [the
            extension
            docs](https://github.com/stac-api-extensions/fields?tab=readme-ov-file#includeexclude-semantics))
            for more on the semantics).
        exclude: fields to exclude from the response (see [the
            extension
            docs](https://github.com/stac-api-extensions/fields?tab=readme-ov-file#includeexclude-semantics))
            for more on the semantics).
        sortby: Fields by which to sort results (use `-field` to sort descending).
        filter: CQL2 filter expression. Strings
            will be interpreted as cql2-text, dictionaries as cql2-json.
        query: Additional filtering based on properties.
            It is recommended to use filter instead, if possible.
        use_duckdb: Query with DuckDB. If None and the href has a
            'parquet' or 'geoparquet' extension, will be set to True. Defaults
            to None.
        kwargs: Additional parameters to pass in to the search.

    Returns:
        A feature collection of the returned STAC items.

    Examples:
        >>> item_collection = await rustac.search(
        ...     "https://landsatlook.usgs.gov/stac-server",
        ...     collections=["landsat-c2l2-sr"],
        ...     intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        ...     sortby="-properties.datetime",
        ...     max_items=1,
        ... )
    """

async def search_to(
    outfile: str,
    href: str,
    *,
    intersects: Optional[str | dict[str, Any]] = None,
    ids: Optional[str | list[str]] = None,
    collections: Optional[str | list[str]] = None,
    max_items: Optional[int] = None,
    limit: Optional[int] = None,
    bbox: Optional[list[float]] = None,
    datetime: Optional[str] = None,
    include: Optional[str | list[str]] = None,
    exclude: Optional[str | list[str]] = None,
    sortby: Optional[str | list[str | dict[str, str]]] = None,
    filter: Optional[str | dict[str, Any]] = None,
    query: Optional[dict[str, Any]] = None,
    format: Optional[str] = None,
    options: Optional[list[Tuple[str, str]]] = None,
    use_duckdb: Optional[bool] = None,
) -> int:
    """
    Searches a STAC API server and saves the result to an output file.

    Args:
        outfile: The output href. This can be a local file path, or any
            url scheme supported by [stac::object_store::write].
        href: The STAC API to search.
        intersects: Searches items
            by performing intersection between their geometry and provided GeoJSON
            geometry.
        ids: Array of Item ids to return.
        collections: Array of one or more Collection IDs that
            each matching Item must be in.
        max_items: The maximum number of items to iterate through.
        limit: The page size returned from the server. Use
            `max_items` to actually limit the number of items returned from this
            function.
        bbox: Requested bounding box.
        datetime: Single date+time, or a range ('/' separator),
            formatted to RFC 3339, section 5.6.  Use double dots .. for open
            date ranges.
        include: fields to include in the response (see [the
            extension
            docs](https://github.com/stac-api-extensions/fields?tab=readme-ov-file#includeexclude-semantics))
            for more on the semantics).
        exclude: fields to exclude from the response (see [the
            extension
            docs](https://github.com/stac-api-extensions/fields?tab=readme-ov-file#includeexclude-semantics))
            for more on the semantics).
        sortby: Fields by which to sort results (use `-field` to sort descending).
        filter: CQL2 filter expression. Strings
            will be interpreted as cql2-text, dictionaries as cql2-json.
        query: Additional filtering based on properties.
            It is recommended to use filter instead, if possible.
        format: The output format. If none, will be inferred from
            the outfile extension, and if that fails will fall back to compact JSON.
        options: Configuration values to pass to the object store backend.
        use_duckdb: Query with DuckDB. If None and the href has a
            'parquet' or 'geoparquet' extension, will be set to True. Defaults
            to None.

    Returns:
        The number of items written

    Examples:
        >>> count = await rustac.search_to("out.parquet",
        ...     "https://landsatlook.usgs.gov/stac-server",
        ...     collections=["landsat-c2l2-sr"],
        ...     intersects={"type": "Point", "coordinates": [-105.119, 40.173]},
        ...     sortby="-properties.datetime",
        ...     max_items=1,
        ... )
    """

def walk(
    container: dict[str, Any],
) -> AsyncIterator[tuple[Catalog | Collection, list[Catalog | Collection], list[Item]]]:
    """Recursively walks a STAC catalog or collection breadth-first.

    Args:
        container: A STAC catalog or collection.

    Yields:
        A three-tuple of the container, its children, and its items.

    Examples:
        >>> async for container, children, items in rustac.walk(catalog):
        ...     ...
    """

async def write(
    href: str,
    value: dict[str, Any] | list[dict[str, Any]],
    *,
    format: str | None = None,
    options: list[tuple[str, str]] | None = None,
) -> dict[str, str] | None:
    """
    Writes STAC to a href.

    Args:
        href: The href to write to
        value: The value to write. This
            can be a STAC dictionary or a list of items.
        format: The output format to write. If not provided, will be
            inferred from the href's extension.
        options: Options for configuring an
            object store, e.g. your AWS credentials.

    Returns:
        The result of putting data into an object store, e.g. the e_tag and the
            version. None is returned if the file was written locally.

    Examples:
        >>> with open("items.json") as f:
        ...     items = json.load(f)
        >>> await rustac.write("items.parquet", items)
    """

def version(
    name: Literal["stac"]
    | Literal["stac-api"]
    | Literal["stac-duckdb"]
    | Literal["duckdb"]
    | None = None,
) -> str | None:
    """
    Returns this package's version, or the version of a upstream.

    Args:
        name: The name of the upstream version to return. Valid
            values are "stac", "stac-api", "stac-duckdb", or "duckdb".

    Returns:
        The version, or None if the name is not recognized as an upstream.

    Examples:
        >>> rustac.version()
        "0.2.0"
        >>> rustac.version("duckdb")
        "1.0.0"
    """

def sha() -> str:
    """
    Returns the SHA of the underlying rustac crate.

    Examples:
        >>> rustac.sha()
        "4d6a60a3df1386922285191aba95a76ec704a8b4"
    """
