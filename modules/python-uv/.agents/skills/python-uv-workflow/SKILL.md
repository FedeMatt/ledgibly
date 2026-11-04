---
name: python-uv-workflow
description: Standardizes Python operations using the uv package manager for speed and reliability.
---

## General Python Execution

- Always use `uv run` instead of `python` or `python3` to execute project code.
- Use `uv run -c "..."` for quick one-liners.
- If an operation requires a specific tool (like Ruff or Black), use `uvx <tool>` to run it in a temporary, isolated environment.

`scripts/ledger_check.py` is stdlib-only; `python3 scripts/ledger_check.py` is fine.

## Python project management

- Always use `uv add` and `uv remove` to manage dependencies.
- Never manually activate or manage virtual environments; use `uv run` for all commands.
- Use `[dependency-groups]` for dev/test/docs dependencies, not `[project.optional-dependencies]`.
- Use `uv add <package>` for production and `uv add --dev <package>` for development dependencies (like `pytest`).
- Let `uv` handle the `.venv` directory automatically.
- Run tests with `uv run pytest`.

## Script Workflow (PEP 723)

When creating standalone Python scripts:

1. Initialize: `uv init --script <script_name>.py`
2. Dependencies: `uv add --script <script_name>.py <package>`
3. Execution: `uv run <script_name>.py`
4. Metadata: use the PEP 723 inline metadata format (`# /// script`).

## Sandboxing & Cache

In a restricted environment, set `UV_CACHE_DIR` to a writable temporary directory.
