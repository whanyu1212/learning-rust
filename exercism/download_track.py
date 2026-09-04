#!/usr/bin/env python3
"""Download unlocked Rust exercises and group attempted vs not."""

from __future__ import annotations

import json
import re
import subprocess
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RUST = ROOT / "rust"
CONFIG_URL = "https://raw.githubusercontent.com/exercism/rust/main/config.json"


def fetch_slugs() -> list[str]:
    req = urllib.request.Request(CONFIG_URL, headers={"User-Agent": "learning-rust-setup"})
    with urllib.request.urlopen(req) as resp:
        cfg = json.load(resp)

    concept_order = [
        "lucians-luscious-lasagna",
        "assembly-line",
        "semi-structured-logs",
        "short-fibonacci",
        "resistor-color",
        "health-statistics",
        "low-power-embedded-game",
        "role-playing-game",
        "magazine-cutout",
        "rpn-calculator",
    ]
    practice: list[str] = []
    for ex in cfg["exercises"]["practice"]:
        if ex.get("status") == "deprecated":
            continue
        practice.append(ex["slug"])
    seen = set(concept_order)
    return concept_order + [s for s in practice if s not in seen]


def download(slug: str) -> str:
    dest = RUST / slug
    if (dest / "Cargo.toml").exists():
        return "skip"
    for attempt in range(8):
        proc = subprocess.run(
            ["exercism", "download", "--track", "rust", "--exercise", slug],
            capture_output=True,
            text=True,
        )
        out = (proc.stdout or "") + (proc.stderr or "")
        if proc.returncode == 0:
            return "ok"
        if "Too Many Requests" in out or "429" in out:
            wait = 20
            m = re.search(r"after (\d+) seconds", out)
            if m:
                wait = int(m.group(1)) + 2
            time.sleep(wait)
            continue
        if "not unlocked" in out.lower():
            return "locked"
        return f"fail:{out.strip().splitlines()[-1] if out.strip() else proc.returncode}"
    return "fail:rate-limit"


def is_attempted(slug: str) -> bool:
    lib = RUST / slug / "src" / "lib.rs"
    if not lib.exists():
        return False
    text = lib.read_text()
    return "todo!" not in text and "unimplemented!" not in text


def classify() -> tuple[list[str], list[str]]:
    attempted: list[str] = []
    not_attempted: list[str] = []
    for dest in sorted(RUST.iterdir()):
        if not dest.is_dir() or not (dest / "Cargo.toml").exists():
            continue
        slug = dest.name
        if is_attempted(slug):
            attempted.append(slug)
        else:
            not_attempted.append(slug)
    return attempted, not_attempted


def write_index(attempted: list[str], not_attempted: list[str], locked: list[str], failed: list[str]) -> None:
    lines = [
        "# Rust track — status",
        "",
        "Crates live at `rust/<slug>/` so `exercism submit` keeps working.",
        "",
        "An exercise counts as **attempted** when `src/lib.rs` no longer contains `todo!` / `unimplemented!`.",
        "",
        f"## Attempted ({len(attempted)})",
        "",
    ]
    for slug in attempted:
        lines.append(f"- [`{slug}`](./rust/{slug}/)")
    lines += ["", f"## Not attempted ({len(not_attempted)})", ""]
    for slug in not_attempted:
        lines.append(f"- [`{slug}`](./rust/{slug}/)")
    if locked:
        lines += ["", "## Still locked", ""]
        lines += [f"- `{s}`" for s in locked]
    if failed:
        lines += ["", "## Download failed", ""]
        lines += [f"- `{s}`" for s in failed]
    lines.append("")
    (ROOT / "STATUS.md").write_text("\n".join(lines))


def main() -> None:
    RUST.mkdir(parents=True, exist_ok=True)
    slugs = fetch_slugs()
    locked: list[str] = []
    failed: list[str] = []
    ok = skip = 0
    for i, slug in enumerate(slugs, 1):
        status = download(slug)
        print(f"[{i}/{len(slugs)}] {status:6} {slug}", flush=True)
        if status == "ok":
            ok += 1
            time.sleep(1.5)
        elif status == "skip":
            skip += 1
        elif status == "locked":
            locked.append(slug)
            time.sleep(0.4)
        else:
            failed.append(f"{slug} ({status})")
            time.sleep(1.0)
    attempted, not_attempted = classify()
    write_index(attempted, not_attempted, locked, failed)
    print(
        f"DONE ok={ok} skip={skip} locked={len(locked)} fail={len(failed)} "
        f"attempted={len(attempted)} not_attempted={len(not_attempted)}"
    )


if __name__ == "__main__":
    main()
