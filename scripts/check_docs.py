#!/usr/bin/env python3

from __future__ import annotations

import re
import sys
from os.path import relpath
from pathlib import Path
from urllib.parse import unquote


ROOT = Path(__file__).resolve().parents[1]
DOCS_ROOT = ROOT / "docs"
LOCALES = ("en", "zh-Hans")
MARKDOWN_LINK = re.compile(r"\[[^\]]*\]\(([^)]+)\)")


def markdown_files(locale: str) -> set[Path]:
    locale_root = DOCS_ROOT / locale
    return {
        path.relative_to(locale_root)
        for path in locale_root.rglob("*.md")
        if path.is_file()
    }


def check_locale_parity() -> list[str]:
    errors: list[str] = []
    reference_locale = LOCALES[0]
    reference_files = markdown_files(reference_locale)

    for locale in LOCALES[1:]:
        locale_files = markdown_files(locale)
        for path in sorted(reference_files - locale_files):
            errors.append(f"docs/{locale}/{path}: missing translation")
        for path in sorted(locale_files - reference_files):
            errors.append(
                f"docs/{locale}/{path}: no matching docs/{reference_locale}/{path}"
            )

    return errors


def check_local_links() -> list[str]:
    errors: list[str] = []

    for document in DOCS_ROOT.rglob("*.md"):
        content = document.read_text(encoding="utf-8")
        for raw_target in MARKDOWN_LINK.findall(content):
            target = raw_target.strip().split(maxsplit=1)[0].strip("<>")
            if not target or target.startswith(("#", "http://", "https://", "mailto:")):
                continue

            path_part = unquote(target.split("#", 1)[0])
            if not path_part:
                continue

            resolved = (document.parent / path_part).resolve()
            if not resolved.exists():
                relative_document = document.relative_to(ROOT)
                errors.append(f"{relative_document}: broken link {raw_target}")

    return errors


def check_language_switches() -> list[str]:
    errors: list[str] = []

    for locale in LOCALES:
        counterpart_locale = next(item for item in LOCALES if item != locale)
        locale_root = DOCS_ROOT / locale
        for document in locale_root.rglob("*.md"):
            relative_path = document.relative_to(locale_root)
            counterpart = DOCS_ROOT / counterpart_locale / relative_path
            expected_link = Path(relpath(counterpart, document.parent)).as_posix()
            header = "\n".join(document.read_text(encoding="utf-8").splitlines()[:8])
            if f"]({expected_link})" not in header:
                relative_document = document.relative_to(ROOT)
                errors.append(
                    f"{relative_document}: missing language switch to {expected_link}"
                )

    return errors


def main() -> int:
    errors = [
        *check_locale_parity(),
        *check_local_links(),
        *check_language_switches(),
    ]
    if errors:
        print("Documentation checks failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    count = len(markdown_files(LOCALES[0]))
    print(f"Documentation checks passed for {len(LOCALES)} locales and {count} pages.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
