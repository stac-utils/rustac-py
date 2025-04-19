# type: ignore
"""
Logging
"""

# %%
# **stacrs** can emit messages with the Python logging system.

import logging
import sys

import rustac

logging.basicConfig(level=logging.DEBUG, stream=sys.stdout)
logger = logging.getLogger(__name__)
logger.error("Starting a search")
items = await rustac.search("../../data/100-sentinel-2-items.parquet")
items

# %%
# Other text
