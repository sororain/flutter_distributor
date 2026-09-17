#!/usr/bin/env python3
"""
Code generation orchestrator.

Discovers and executes all code generation scripts under the `generate/` directory.
Each script should be executable and self-contained.
"""
import subprocess
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent / "generate"


def discover_scripts() -> list[Path]:
    """Find all Python scripts in the generate/ directory."""
    if not SCRIPTS_DIR.exists():
        print(f"Generate directory not found: {SCRIPTS_DIR}")
        return []

    return sorted(
        p for p in SCRIPTS_DIR.iterdir()
        if p.suffix == ".py" and p.name != "__init__.py"
    )


def run_script(script: Path) -> bool:
    """Execute a single generation script."""
    print(f"\n{'='*60}")
    print(f"Running: {script.relative_to(SCRIPTS_DIR.parent)}")
    print(f"{'='*60}\n")

    result = subprocess.run(
        [sys.executable, str(script)],
        cwd=script.parent,
    )

    if result.returncode == 0:
        print(f"\n  ✓ {script.name} completed successfully")
        return True
    else:
        print(f"\n  ✗ {script.name} failed with exit code {result.returncode}")
        return False


def main():
    scripts = discover_scripts()

    if not scripts:
        print("No generation scripts found in scripts/generate/")
        sys.exit(0)

    print(f"Found {len(scripts)} generation script(s):")
    for s in scripts:
        print(f"  - {s.name}")

    success_count = 0
    failure_count = 0

    for script in scripts:
        ok = run_script(script)
        if ok:
            success_count += 1
        else:
            failure_count += 1

    print(f"\n{'='*60}")
    print(f"Summary: {success_count} succeeded, {failure_count} failed")
    print(f"{'='*60}")

    if failure_count > 0:
        sys.exit(1)


if __name__ == "__main__":
    main()
