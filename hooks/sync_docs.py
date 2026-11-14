import os
import shutil
from pathlib import Path

def sync_all(root_dir: Path, docs_dir: Path):
    docs_dir.mkdir(exist_ok=True)

    excluded_dirs = {
        "docs", ".docs", "site", ".venv", ".git", ".github", ".cache",
        "__pycache__", "hooks", "assets", "javascripts", "stylesheets",
        ".vscode", "tmp"
    }

    # 1. Clean existing content in docs_dir
    for item in docs_dir.iterdir():
        if item.is_dir():
            shutil.rmtree(item)
        else:
            item.unlink()

    # 2. Copy root README.md to index.md if present
    readme = root_dir / "README.md"
    index_md = docs_dir / "index.md"
    if readme.exists():
        shutil.copy2(readme, index_md)
    else:
        index_md.write_text("# Systems & Robotics Engineering Notes\n\nWelcome to the knowledge base.\n\n- [Embedded Rust](embedded_rust/README.md)\n- [Hardware](hardware/hardware_theory.md)\n", encoding="utf-8")

    # 3. Copy top-level content directories that contain markdown files
    for item in root_dir.iterdir():
        if item.is_dir() and item.name not in excluded_dirs and not item.name.startswith("."):
            has_md = any(f.is_file() and f.suffix == ".md" for f in item.rglob("*"))
            if has_md:
                shutil.copytree(item, docs_dir / item.name)

    # 4. Copy assets, javascripts, stylesheets into docs_dir
    for folder in ["assets", "javascripts", "stylesheets"]:
        src_folder = root_dir / folder
        dst_folder = docs_dir / folder
        if src_folder.exists():
            shutil.copytree(src_folder, dst_folder)

def on_config(config):
    root_dir = Path(__file__).resolve().parent.parent
    docs_dir = root_dir / ".docs"
    sync_all(root_dir, docs_dir)
    return config

def on_pre_build(config):
    root_dir = Path(__file__).resolve().parent.parent
    docs_dir = root_dir / ".docs"
    sync_all(root_dir, docs_dir)
