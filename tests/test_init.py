import rustac
import rustac.rustac


def test_all_is_sorted_and_unique() -> None:
    assert rustac.__all__ == sorted(set(rustac.__all__))


def test_all_covers_the_rust_module() -> None:
    rust_all: list[str] = getattr(rustac.rustac, "__all__")
    assert set(rust_all) <= set(rustac.__all__)


def test_all_is_importable() -> None:
    for name in rustac.__all__:
        assert hasattr(rustac, name), name
