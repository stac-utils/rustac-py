import json
from pathlib import Path
from typing import Any

import pystac
import pytest


@pytest.fixture
def root() -> Path:
    return Path(__file__).parents[1]


@pytest.fixture
def spec_examples(root: Path) -> Path:
    return root / "spec-examples"


@pytest.fixture
def examples(root: Path) -> Path:
    return root / "spec-examples" / "v1.1.0"


@pytest.fixture
def data(root: Path) -> Path:
    return root / "data"


@pytest.fixture
def item(examples: Path) -> dict[str, Any]:
    with open(examples / "simple-item.json") as f:
        return json.load(f)


@pytest.fixture
def maxar_items(root: Path) -> list[dict[str, Any]]:
    # https://github.com/stac-utils/rustac/issues/722
    directory = root / "tests" / "data" / "maxar-hurricane-ian-2022"
    item_a = pystac.read_file(directory / "031331303020" / "10300100DB064000.json")
    item_b = pystac.read_file(directory / "031331303211" / "10300100DB064000.json")
    return [
        item_a.to_dict(transform_hrefs=False),
        item_b.to_dict(transform_hrefs=False),
    ]
