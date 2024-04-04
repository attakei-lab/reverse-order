"""Test case for ``ypoc`` command."""

import shutil
from pathlib import Path

from . import root


def test_valid_env(cmd, bin_dir: Path, tmp_path: Path):
    """Run test caese on env having valid files."""
    env_path = root / "valid-patterns/single-file"
    from_ = tmp_path / "from"
    to_ = tmp_path / "to"
    shutil.copytree(env_path / "data", from_, dirs_exist_ok=True)
    to_.mkdir()
    proc = cmd("cp", f"{from_}/example.txt", f"{to_}/")
    assert proc.returncode == 0
    (to_ / "example.txt").write_text("example data.")
    proc = cmd(f"{bin_dir}/ypoc", f"{from_}/example.txt", f"{to_}/")
    assert proc.returncode == 0
    assert (from_ / "example.txt").read_text() == (to_ / "example.txt").read_text()
