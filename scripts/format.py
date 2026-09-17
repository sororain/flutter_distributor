#!/usr/bin/env python3
"""
Format source files across the repository.

By default this runs the formatters used by the Rust crates, Dart packages,
and web/documentation packages when their tools are available.
"""
from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class FormatCommand:
    name: str
    command: list[str]
    cwd: Path = REPO_ROOT
    required_tool: str | None = None
    skip_reason: str | None = None


def tool_exists(tool: str) -> bool:
    return shutil.which(tool) is not None


def run_command(command: FormatCommand) -> bool:
    if command.skip_reason:
        print(f"skip {command.name}: {command.skip_reason}")
        return True

    if command.required_tool and not tool_exists(command.required_tool):
        print(f"skip {command.name}: `{command.required_tool}` not found")
        return True

    print(f"\n==> {command.name}")
    print("$ " + " ".join(command.command))
    result = subprocess.run(command.command, cwd=command.cwd)
    return result.returncode == 0


def rust_command(check: bool) -> FormatCommand:
    command = ["cargo", "fmt", "--all"]
    if check:
        command.extend(["--", "--check"])
    return FormatCommand("rust", command, required_tool="cargo")


def dart_command(check: bool) -> FormatCommand:
    if tool_exists("melos"):
        script = "format-check" if check else "format"
        return FormatCommand(
            "dart",
            ["melos", "run", script],
            required_tool="melos",
        )

    command = ["dart", "format", ".", "--fix"]
    if check:
        command.append("--set-exit-if-changed")
    return FormatCommand("dart", command, required_tool="dart")


def prettier_command(name: str, directory: Path, check: bool) -> FormatCommand | None:
    if not (directory / "package.json").exists():
        return None

    prettier = directory / "node_modules" / ".bin" / "prettier"
    if not prettier.exists():
        return FormatCommand(
            name,
            [],
            cwd=directory,
            skip_reason="local node_modules/.bin/prettier not found; run package install first",
        )

    command = [str(prettier), "--check" if check else "--write", "."]
    return FormatCommand(name, command, cwd=directory)


def docs_command(check: bool) -> FormatCommand | None:
    return prettier_command("docs", REPO_ROOT / "apps" / "docs", check)


def website_command(check: bool) -> FormatCommand | None:
    return prettier_command("website", REPO_ROOT / "apps" / "website", check)


def build_commands(selected: set[str], check: bool) -> list[FormatCommand]:
    commands: list[FormatCommand] = []

    if "rust" in selected:
        commands.append(rust_command(check))
    if "dart" in selected:
        commands.append(dart_command(check))
    if "docs" in selected:
        command = docs_command(check)
        if command:
            commands.append(command)
    if "website" in selected:
        command = website_command(check)
        if command:
            commands.append(command)

    return commands


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="check formatting without writing changes where the formatter supports it",
    )
    parser.add_argument(
        "--only",
        action="append",
        choices=["rust", "dart", "docs", "website"],
        help="formatter to run; can be passed multiple times",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    selected = set(args.only or ["rust", "dart", "docs", "website"])
    commands = build_commands(selected, args.check)

    failures: list[str] = []
    for command in commands:
        if not run_command(command):
            failures.append(command.name)

    if failures:
        print("\nfailed: " + ", ".join(failures))
        return 1

    print("\nformat complete")
    return 0


if __name__ == "__main__":
    sys.exit(main())
