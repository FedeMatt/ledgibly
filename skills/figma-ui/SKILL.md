---
name: figma-ui
description: Implement or restyle a screen, overlay, or component from the repo's Figma file without inventing layout. Use when building or changing any UI screen in a repo whose AGENTS.md "Agent profile" names a Figma fileKey, or when the user names a Figma frame or node ID.
---

# Figma UI

Per-repo Figma facts live in the `Figma` block of the `## Agent profile` section in AGENTS.md: fileKey, page, canvas size, inventory path, target stack, screen and config locations, behavior source, out-of-scope areas, and the visual check command. Read it first. If the profile has no Figma block, stop and ask; do not invent a fileKey.

## Find the frame cheaply

1. Grep the inventory, do not read it whole: `rg -n -i '<screen name>' <inventory>`.
2. Use the node ID, not the name (names repeat). Skip frames marked hidden.
3. If the frame is missing from the inventory, call `get_metadata` on the profile page once, find the frame, and add a row to the inventory.

## Pull one node at a time

1. If this harness has a `figma-design-to-code` skill, load it, then call `get_design_context` with the profile fileKey and the frame, or the smallest child node that covers the change. If that skill is absent, use the harness's Figma MCP or equivalent tools. If neither exists, stop and ask; do not invent layout.
2. Use `get_screenshot` on the same node as the visual reference.
3. Use `get_variable_defs` only if the repo maps Figma variables to tokens.
4. Never pull a whole page. Pull overlays and components only when the parent screen needs them.

## Adapt, do not paste

- Search the screen directory for existing components and reuse them before creating new ones.
- Translate the reference to the profile target stack. Never paste web or Tailwind output into a non-web stack.
- Do not invent layout, chrome, or copy. If the frame does not cover a state, ask.
- Behavior comes from the profile behavior source. If Figma conflicts with it, the behavior source wins; flag the conflict to the user.
- Do not implement out-of-scope areas or hidden frames.

## Verify and record

- Run the profile verify commands, and the visual check when its condition applies. Compare the result against the screenshot.
- Cite the node IDs in the ledger row evidence (see the `keep-implementation-ledger` skill). Update the inventory only when the Figma file gained frames.
- Do not write implementation status into the inventory or UI docs; the ledger owns status.

See [checklist.md](checklist.md).
