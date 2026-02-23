from setuptools.dist import Distribution
from setuptools_rust.setuptools_ext import pyprojecttoml_config as upstream_config


def pyprojecttoml_config(dist: Distribution) -> None:
    # Run the upstream setuptools-rust configuration
    upstream_config(dist)
    # Now monkeypatch the distribution to replace the output library
    # with rustfst-ffi (surely there must be a better way to do this?)
    for ext in dist.rust_extensions:
        if ext.name != "python-rustfst":
            continue
        metadata = ext.metadata(quiet=True)
        for p in metadata["packages"]:
            if p["name"] != "rustfst-ffi":
                continue
            package_id = p["id"]
        metadata["resolve"]["root"] = package_id
        ext.metadata = lambda _quiet: metadata
