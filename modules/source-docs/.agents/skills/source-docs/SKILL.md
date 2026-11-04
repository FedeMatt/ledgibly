---
name: source-docs
description: Bound product features to canonical vendor, spec, or protocol docs under docs/sources. Use when adding UI, HAL, drivers, API clients, or any behavior that a source document defines, or when asking what the system is allowed to do.
---

# Source docs

Authoritative sources under [`docs/sources/`](../../../docs/sources/README.md) (or the path in the `Sources` profile block) define what the software may implement. App work is a subset. Do not invent protocol, API, opcode, or hardware behavior.

## Before implementing

1. Open the sources index (`docs/sources/README.md`, or the profile `Sources` index).
2. Open the matching source file for this change.
3. Use the source's names in code and repo docs. Quote marketing or printed labels only when citing that source.
4. If two sources disagree, record both spellings and prefer a live-system response when one exists. Do not pick silently.

Host or device pattern, if the sources describe one: follow that pattern. Do not drive a process step-by-step from software when the source says the device or remote side owns the timer.

See [checklist.md](checklist.md).
