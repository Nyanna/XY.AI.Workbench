"""Writes rendered source text to files under the output directory."""

from pathlib import Path


class FileWriter:
    """Writes generated file contents to disk under a base output directory."""

    def __init__(self, output_dir: Path):
        self.output_dir = output_dir

    def write(self, relative_path: Path, content: str) -> None:
        """Write content to output_dir/relative_path, creating parent directories."""
        target = self.output_dir / relative_path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content, encoding="utf-8")
