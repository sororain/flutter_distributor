#!/usr/bin/env python3
"""
Generate Google Play Console API Rust client (google_play_console) from Google's OpenAPI spec.

Steps:
1. Download the OpenAPI specification from the APIs.guru directory
2. Prune to only keep paths and schemas used by the codebase
3. Generate Rust client via cargo progenitor
4. Clean up temporary files
"""
import json
import subprocess
import sys
import urllib.request
from collections import deque
from pathlib import Path

# ── Paths ────────────────────────────────────────────────────────────────

ROOT = Path(__file__).resolve().parent.parent.parent
OUTPUT = ROOT / "crates/stores/google_play_console"
CACHE_DIR = ROOT / ".cache/google_play_console"


# ── Step 1: Download ─────────────────────────────────────────────────────

def ensure_spec() -> Path:
    """Return cached spec path, downloading if not present."""
    spec_path = CACHE_DIR / "openapi.yaml"
    if spec_path.exists():
        print(f"[1/4] Using cached spec: {spec_path.relative_to(ROOT)}")
        return spec_path

    url = "https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/googleapis.com/androidpublisher/v3/openapi.yaml"
    print(f"[1/4] Downloading OpenAPI spec from {url}...")

    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    urllib.request.urlretrieve(url, spec_path)
    print(f"       Downloaded to {spec_path.relative_to(ROOT)}")
    return spec_path


# ── Step 2: Prune ────────────────────────────────────────────────────────

def resolve_ref(ref, spec):
    parts = ref.lstrip('#/').split('/')
    obj = spec
    for part in parts:
        obj = obj.get(part, {})
    return obj


def collect_refs(obj, visited, spec):
    if isinstance(obj, dict):
        if '$ref' in obj:
            ref = obj['$ref']
            if ref not in visited:
                visited.add(ref)
                resolved = resolve_ref(ref, spec)
                collect_refs(resolved, visited, spec)
        for v in obj.values():
            collect_refs(v, visited, spec)
    elif isinstance(obj, list):
        for item in obj:
            collect_refs(item, visited, spec)


# Paths used by the codebase - edits, tracks (app releases), and listings
KEEP_PATHS = {
    '/androidpublisher/v3/applications/{packageName}/edits',
    '/androidpublisher/v3/applications/{packageName}/edits/{editId}:commit',
    '/androidpublisher/v3/applications/{packageName}/edits/{editId}/listings',
    '/androidpublisher/v3/applications/{packageName}/edits/{editId}/listings/{language}',
    '/androidpublisher/v3/applications/{packageName}/edits/{editId}/tracks',
    '/androidpublisher/v3/applications/{packageName}/edits/{editId}/tracks/{track}',
}

# Resource schemas whose `relationships`-like fields can be stripped
# (Google spec has different structure, this is for consistency)


def prune(spec_path: Path) -> Path:
    print(f"[2/4] Pruning spec to only keep used paths and schemas...")

    import yaml
    with open(spec_path) as f:
        spec = yaml.safe_load(f)

    schemas = spec.get('components', {}).get('schemas', {})

    # Build pruned spec
    new_spec = {
        'openapi': spec['openapi'],
        'info': spec['info'],
        'servers': spec.get('servers', []),
        'paths': {},
        'components': {
            'schemas': {},
            'parameters': {},
            'securitySchemes': spec.get('components', {}).get('securitySchemes', {}),
        },
    }

    # Collect refs from kept paths
    refs = set()
    for path, methods in spec.get('paths', {}).items():
        if path in KEEP_PATHS:
            new_spec['paths'][path] = {}
            for method, operation in methods.items():
                if method == 'parameters':
                    new_spec['paths'][path][method] = operation
                else:
                    new_spec['paths'][path][method] = operation
                    collect_refs(operation, refs, spec)

    # Also collect params referenced in kept paths
    for path in KEEP_PATHS:
        if path in spec.get('paths', {}):
            params = spec['paths'][path].get('parameters', [])
            for param in params:
                collect_refs(param, refs, spec)

    # BFS transitive schema resolution
    schema_queue = deque()
    for ref in refs:
        parts = ref.lstrip('#/').split('/')
        if len(parts) >= 3 and parts[0] == 'components':
            name = parts[2]
            if name not in new_spec['components']['schemas']:
                schema_queue.append((parts[1], name))

    while schema_queue:
        kind, name = schema_queue.popleft()
        container = new_spec['components'].get(kind, {})
        if name in container:
            continue

        if kind == 'schemas':
            schema = schemas.get(name)
        elif kind == 'parameters':
            schema = spec.get('components', {}).get('parameters', {}).get(name)
        else:
            schema = None

        if schema is None:
            continue

        new_spec['components'][kind][name] = schema

        new_refs = set()
        collect_refs(schema, new_refs, spec)
        for ref in new_refs:
            parts = ref.lstrip('#/').split('/')
            if len(parts) >= 3 and parts[0] == 'components':
                dep_kind = parts[1]
                dep_name = parts[2]
                container = new_spec['components'].get(dep_kind, {})
                if dep_name not in container:
                    schema_queue.append((dep_kind, dep_name))

    pruned_path = spec_path.with_name("openapi.pruned.yaml")
    with open(pruned_path, 'w') as f:
        json.dump(new_spec, f, indent=2)

    old_paths = len(spec.get('paths', {}))
    old_schemas = len(schemas)
    new_paths = len(new_spec.get('paths', {}))
    new_schemas = len(new_spec['components'].get('schemas', {}))
    print(f"       Paths:  {old_paths:4d} → {new_paths:4d}  (removed {old_paths - new_paths})")
    print(f"       Schemas: {old_schemas:4d} → {new_schemas:4d}  (removed {old_schemas - new_schemas})")

    return pruned_path


# ── Step 3: Generate ─────────────────────────────────────────────────────

def generate(pruned_path: Path):
    print(f"[3/4] Generating Rust client via cargo progenitor...")
    if OUTPUT.exists():
        import shutil
        shutil.rmtree(OUTPUT)

    result = subprocess.run(
        [
            "cargo", "progenitor",
            "-i", str(pruned_path),
            "-o", str(OUTPUT),
            "--name", "fastforge_google_play_console",
            "--version", "0.1.0",
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )

    if result.returncode != 0:
        print(f"       Error: {result.stderr}")
        sys.exit(1)

    lib = OUTPUT / "src" / "lib.rs"
    client = OUTPUT / "src" / "client.rs"
    if lib.exists():
        lib.replace(client)
        lib.write_text("pub mod client;\n\npub use client::*;\n")
        lines = sum(1 for _ in client.open())
        size = client.stat().st_size
        print(f"       Generated {client.relative_to(ROOT)} ({lines} lines, {size / 1024:.0f} KB)")
        print(f"       Wrote {lib.relative_to(ROOT)}")

    # Fix up Cargo.toml metadata (progenitor overwrites it)
    cargo_toml = OUTPUT / "Cargo.toml"
    content = cargo_toml.read_text()
    content = content.replace(
        'version = "0.1.0"\nedition = "2024"\nlicense = "SPECIFY A LICENSE BEFORE PUBLISHING"',
        'version.workspace = true\nedition.workspace = true\nauthors.workspace = true\ndescription = "Google Play Console API client"\nlicense.workspace = true',
    )
    cargo_toml.write_text(content)
    print(f"       Fixed up {cargo_toml.relative_to(ROOT)}")


# ── Main ─────────────────────────────────────────────────────────────────

def main():
    try:
        import yaml
    except ImportError:
        print("Error: PyYAML is required. Install with: pip install pyyaml")
        sys.exit(1)

    spec_path = ensure_spec()
    pruned_path = prune(spec_path)
    generate(pruned_path)


if __name__ == "__main__":
    main()
