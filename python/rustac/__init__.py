from __future__ import annotations

from .rustac import *
from typing import TypedDict, Required, Any, Literal


class Catalog(TypedDict, total=False):
    """A STAC Catalog object represents a logical group of other Catalog, Collection, and Item objects."""

    type: Required[str]
    """Set to Catalog if this Catalog only implements the Catalog spec."""

    stac_version: Required[str]
    """The STAC version the Catalog implements."""

    stac_extensions: list[str] | None
    """A list of extension identifiers the Catalog implements."""

    id: Required[str]
    """Identifier for the Catalog."""

    title: str | None
    """A short descriptive one-line title for the Catalog."""

    description: Required[str]
    """Detailed multi-line description to fully explain the Catalog.
    
    CommonMark 0.29 syntax MAY be used for rich text representation."""

    links: Required[list[Link]]
    """A list of references to other documents."""

class Collection(TypedDict, total=False):
    """The STAC Collection Specification defines a set of common fields to describe a group of Items that share properties and metadata."""

    type: Required[str]
    """Must be set to Collection to be a valid Collection."""

    stac_version: Required[str]
    """The STAC version the Collection implements."""

    stac_extensions: list[str] | None
    """A list of extension identifiers the Collection implements."""

    id: Required[str]
    """Identifier for the Collection that is unique across all collections in the root catalog."""

    title: str | None
    """A short descriptive one-line title for the Collection."""

    description: Required[str]
    """Detailed multi-line description to fully explain the Collection.
    
    CommonMark 0.29 syntax MAY be used for rich text representation."""

    keywords: list[str] | None
    """List of keywords describing the Collection."""

    license: Required[str]
    """License(s) of the data collection as SPDX License identifier, SPDX License expression, or `other`."""

    providers: list[Provider] | None
    """A list of providers, which may include all organizations capturing or processing the data or the hosting provider."""

    extent: Required[Extent]
    """Spatial and temporal extents."""

    summaries: dict[str, Any] | None
    """A map of property summaries, either a set of values, a range of values or a JSON Schema."""

    links: Required[list[Link]]
    """A list of references to other documents."""

    assets: dict[str, Asset] | None
    """Dictionary of asset objects that can be downloaded, each with a unique key."""

    item_assets: dict[str, ItemAsset] | None
    """A dictionary of assets that can be found in member Items."""

class Provider(TypedDict, total=False):
    """A provider is any of the organizations that captures or processes the content of the Collection and therefore influences the data offered by this Collection."""

    name: Required[str]
    """The name of the organization or the individual."""

    description: str | None
    """Multi-line description to add further provider information such as processing details for processors and producers, hosting details for hosts or basic contact information.
    
    CommonMark 0.29 syntax MAY be used for rich text representation."""

    roles: list[
        Literal["licensor"]
        | Literal["producer"]
        | Literal["processor"]
        | Literal["host"]
    ] | None
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

class ItemAsset(TypedDict, total=False):
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

class Item(TypedDict, total=False):
    """An Item is a GeoJSON Feature augmented with foreign members relevant to a STAC object."""

    type: Required[str]
    """Type of the GeoJSON Object. MUST be set to Feature."""

    stac_version: Required[str]
    """The STAC version the Item implements."""

    stac_extensions: list[str] | None
    """A list of extensions the Item implements."""

    id: Required[str]
    """Provider identifier. The ID should be unique within the Collection that contains the Item."""

    geometry: dict[str, Any] | None
    """Defines the full footprint of the asset represented by this item, formatted according to RFC 7946, section 3.1 if a geometry is provided or section 3.2 if no geometry is provided."""

    bbox: list[int | float] | None
    """REQUIRED if geometry is not null, prohibited if geometry is null.
    
    Bounding Box of the asset represented by this Item, formatted according to RFC 7946, section 5."""

    properties: Required[Properties]
    """A dictionary of additional metadata for the Item."""

    links: Required[list[Link]]
    """List of link objects to resources and related URLs.
    
    See the best practices for details on when the use self links is strongly recommended."""

    assets: Required[dict[str, Asset]]
    """Dictionary of asset objects that can be downloaded, each with a unique key."""

    collection: str | None
    """The id of the STAC Collection this Item references to.
    
    This field is required if a link with a collection relation type is present and is not allowed otherwise."""

class Properties(TypedDict):
    """Additional metadata fields can be added to the GeoJSON Object Properties."""

    datetime: str | None
    """The searchable date and time of the assets, which must be in UTC.
    
    It is formatted according to RFC 3339, section 5.6. null is allowed, but requires start_datetime and end_datetime from common metadata to be set."""

class Link(TypedDict, total=False):
    """This object describes a relationship with another entity.

    Data providers are advised to be liberal with the links section, to describe
    things like the Catalog an Item is in, related Items, parent or child Items
    (modeled in different ways, like an 'acquisition' or derived data)."""

    href: Required[str]
    """The actual link in the format of an URL.
    
    Relative and absolute links are both allowed. Trailing slashes are significant."""

    rel: Required[str]
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

class Asset(TypedDict, total=False):
    """An Asset is an object that contains a URI to data associated with the Item that can be downloaded or streamed.

    It is allowed to add additional fields."""

    href: Required[str]
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

__doc__ = rustac.__doc__
if hasattr(rustac, "__all__"):
    __all__ = rustac.__all__
