#!/usr/bin/env python3
"""Structural checks for implementation_ledger.md. Stdlib only."""

from __future__ import annotations

import argparse
import re
import sys
from datetime import datetime
from pathlib import Path

STATUSES = frozenset({"pending", "in_progress", "blocked", "done", "superseded"})
ID_RE = re.compile(r"^[A-Z][A-Z0-9]*-\d+$")
ID_IN_TEXT_RE = re.compile(r"\b[A-Z][A-Z0-9]*-\d+\b")
LOG_HEADING_RE = re.compile(
    r"^### (\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z) — .+ \(([A-Z][A-Z0-9]*-\d+(?:, [A-Z][A-Z0-9]*-\d+)*)\)\s*$"
)
BUDGETS = {"Description": 120, "Acceptance condition": 200, "Evidence": 300}
EMPTY = frozenset({"", "-", "—", "–"})


def split_row(line: str) -> list[str]:
    line = line.rstrip("\n").rstrip("\r")
    if line.endswith("|") and not line.endswith("\\|"):
        line = line[:-1]
    if line.startswith("|"):
        line = line[1:]
    return [c.strip().replace("\\|", "|") for c in re.split(r"(?<!\\)\|", line)]


def is_separator(line: str) -> bool:
    cells = split_row(line)
    return bool(cells) and all(re.fullmatch(r":?-{3,}:?", c or "") for c in cells)


def section(text: str, heading: str) -> str:
    pattern = re.compile(rf"^## {re.escape(heading)}\s*$", re.M)
    match = pattern.search(text)
    if not match:
        return ""
    rest = text[match.end() :]
    nxt = re.search(r"^## ", rest, re.M)
    return rest[: nxt.start()] if nxt else rest


def parse_tasks(text: str) -> tuple[list[str], list[list[str]], list[str]]:
    errors: list[str] = []
    body = section(text, "Tasks")
    rows: list[list[str]] = []
    header: list[str] = []
    for raw in body.splitlines():
        if not raw.strip().startswith("|"):
            if header and raw.strip().startswith("##"):
                break
            continue
        cells = split_row(raw)
        if not header:
            header = cells
            continue
        if is_separator(raw):
            if len(cells) != len(header):
                errors.append(
                    f"separator has {len(cells)} columns, header has {len(header)}"
                )
            continue
        rows.append(cells)
    if not header:
        errors.append("no Tasks table header")
    return header, rows, errors


def empty(cell: str) -> bool:
    return cell.strip() in EMPTY


def check_rows(
    header: list[str], rows: list[list[str]]
) -> tuple[dict[str, list[str]], list[str], list[str]]:
    errors: list[str] = []
    warnings: list[str] = []
    by_id: dict[str, list[str]] = {}
    col = {name: i for i, name in enumerate(header)}
    needed = ["ID", "Description", "Dependencies", "Status", "Acceptance condition", "Evidence"]
    for name in needed:
        if name not in col:
            errors.append(f"missing column {name!r}")
            return by_id, errors, warnings

    for n, cells in enumerate(rows, start=1):
        loc = f"row {n}"
        if len(cells) != len(header):
            errors.append(f"{loc}: {len(cells)} columns, header has {len(header)}")
            continue
        task_id = cells[col["ID"]]
        if not ID_RE.fullmatch(task_id):
            errors.append(f"{loc}: bad ID {task_id!r}")
            continue
        if task_id in by_id:
            errors.append(f"{loc}: duplicate ID {task_id}")
        by_id[task_id] = cells
        status = cells[col["Status"]]
        if status not in STATUSES:
            errors.append(f"{task_id}: bad status {status!r}")
        evidence = cells[col["Evidence"]]
        if status in {"done", "blocked"} and empty(evidence):
            errors.append(f"{task_id}: {status} requires evidence")
        if status == "superseded":
            others = [
                m
                for i, cell in enumerate(cells)
                if i != col["ID"] and i != col["Dependencies"]
                for m in ID_IN_TEXT_RE.findall(cell)
                if m != task_id
            ]
            if not others:
                errors.append(f"{task_id}: superseded must name a replacement ID")
        for name, limit in BUDGETS.items():
            value = cells[col[name]]
            if len(value) > limit:
                warnings.append(f"{task_id}: {name} is {len(value)} chars (budget {limit})")

    skip_ids = {col["ID"], col["Dependencies"]}
    for task_id, cells in by_id.items():
        deps_raw = cells[col["Dependencies"]]
        if not empty(deps_raw):
            for dep in re.split(r"[,\s]+", deps_raw):
                if not dep:
                    continue
                if not ID_RE.fullmatch(dep):
                    errors.append(f"{task_id}: bad dependency {dep!r}")
                elif dep not in by_id:
                    errors.append(f"{task_id}: dependency {dep} is not in the table")
                elif cells[col["Status"]] == "done":
                    dep_status = by_id[dep][col["Status"]]
                    if dep_status not in {"done", "superseded"}:
                        warnings.append(
                            f"{task_id}: done depends on {dep} ({dep_status})"
                        )
        if cells[col["Status"]] == "superseded":
            named = " ".join(c for i, c in enumerate(cells) if i not in skip_ids)
            for other in ID_IN_TEXT_RE.findall(named):
                if other != task_id and other not in by_id:
                    errors.append(f"{task_id}: replacement {other} is not in the table")
                    break
    return by_id, errors, warnings


def check_log(text: str, by_id: dict[str, list[str]]) -> tuple[list[str], list[str]]:
    errors: list[str] = []
    warnings: list[str] = []
    body = section(text, "Chronological UTC Log")
    if not body.strip():
        errors.append("missing Chronological UTC Log section")
        return errors, warnings
    seen: list[datetime] = []
    for raw in body.splitlines():
        if not raw.startswith("### "):
            continue
        match = LOG_HEADING_RE.match(raw)
        if not match:
            warnings.append(f"log heading not in expected format: {raw[:80]}")
            continue
        stamp = datetime.strptime(match.group(1), "%Y-%m-%dT%H:%M:%SZ")
        if seen and stamp > seen[-1]:
            errors.append(
                f"log not newest-first: {match.group(1)} follows {seen[-1].strftime('%Y-%m-%dT%H:%M:%SZ')}"
            )
        seen.append(stamp)
        for task_id in match.group(2).split(", "):
            if task_id not in by_id:
                warnings.append(f"log cites {task_id} which is not in the Tasks table")
    return errors, warnings


def check_docs(docs: Path, by_id: dict[str, list[str]], header: list[str]) -> list[str]:
    warnings: list[str] = []
    if not docs.is_dir():
        warnings.append(f"docs dir missing: {docs}")
        return warnings
    blob = "\n".join(p.read_text(encoding="utf-8") for p in docs.rglob("*.md") if p.is_file())
    status_i = header.index("Status")
    for task_id, cells in by_id.items():
        if cells[status_i] == "done" and not re.search(rf"\b{re.escape(task_id)}\b", blob):
            warnings.append(f"{task_id} is done but not cited in docs/")
    return warnings


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".", help="repo root (default: cwd)")
    parser.add_argument("--ledger", default="implementation_ledger.md")
    parser.add_argument("--docs", default="docs")
    parser.add_argument("--threshold", type=int, default=100_000)
    args = parser.parse_args(argv)

    root = Path(args.root).resolve()
    ledger = (root / args.ledger).resolve()
    if not ledger.is_file():
        print(f"error: ledger not found: {ledger}", file=sys.stderr)
        return 1
    text = ledger.read_text(encoding="utf-8")
    errors: list[str] = []
    warnings: list[str] = []

    size = ledger.stat().st_size
    if size > args.threshold:
        warnings.append(f"ledger is {size} bytes (threshold {args.threshold}); propose archiving")

    header, rows, parse_errors = parse_tasks(text)
    errors.extend(parse_errors)
    by_id, row_errors, row_warnings = check_rows(header, rows) if header else ({}, [], [])
    errors.extend(row_errors)
    warnings.extend(row_warnings)
    log_errors, log_warnings = check_log(text, by_id)
    errors.extend(log_errors)
    warnings.extend(log_warnings)
    warnings.extend(check_docs(root / args.docs, by_id, header) if header else [])

    for item in warnings:
        print(f"warning: {item}", file=sys.stderr)
    for item in errors:
        print(f"error: {item}", file=sys.stderr)
    print(f"{len(errors)} errors, {len(warnings)} warnings, {len(by_id)} tasks")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
