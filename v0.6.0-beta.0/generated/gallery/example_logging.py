# type: ignore
"""
Logging
"""

# %%
# **stacrs** can emit messages with the Python logging system.

import logging
import stacrs
import sys

logging.basicConfig(level=logging.DEBUG, stream=sys.stdout)
logger = logging.getLogger(__name__)
logger.error("Starting a search")
items = await stacrs.search("../../data/100-sentinel-2-items.parquet")
items

# %%
# Other text