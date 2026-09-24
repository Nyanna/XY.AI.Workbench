# Implementation Prompt: Git-Based LLM Operation Snapshot & Review System

## Context

An Eclipse plugin runs an LLM agent that performs file operations
(create/edit/delete) directly on a Git-tracked workspace. Before/after every
LLM operation, we need a Human-in-the-Loop (HITL) review step: show the
exact diff, let the user accept or revert it — without polluting the real
Git history (no commits on the actual branch, no HEAD movement, no index
side effects visible to normal Git tooling).

The chosen mechanism uses Git's own object model (trees + commits) as an
out-of-band, GC-safe snapshot log under a dedicated ref namespace, wrapped
in commit objects so they remain visible in EGit's native UI (Additional
Refs, History view).

## Task

Implement this system as part of the existing Eclipse plugin, in Java using
JGit. The plugin already contains the hooks/triggers that fire immediately
before and after each LLM operation — integrate against those, do not
reinvent the triggering mechanism.

## Functional Requirements

1. **Snapshot creation (post-operation hook)**
   - After each LLM operation, capture the full working-tree state as a Git
     tree object and wrap it in a commit object.
   - Do not move HEAD, the current branch, or the real index in any
     user-visible way.
   - Commit message is fixed-format: Tool Call ID that caused the snapshot (no free-text
     description).
   - Store the resulting commit under a dedicated ref namespace
     (`refs/llm-ops/...`), never as a loose/unreferenced object.

2. **Chain logic**
   - Maintain a sequential parent chain: each new snapshot's parent is the
     previous snapshot in the same chain, not a fixed base commit.
   - Detect if HEAD has advanced since the last snapshot (i.e., a real
     commit happened on the branch in the meantime). If so, close the
     current chain and start a new one, rooted at the new HEAD. Do not
     delete or modify the old chain.
   - Guard against no-op snapshots: if the working tree is unchanged since
     the last snapshot, skip commit creation.

3. **Diff generation**
   - Produce a single aggregated unified diff for the entire operation
     (all changed files in one text block), equivalent to
     `git diff <parent> <commit>` — not per-file fragments.

4. **Revert**
   - Provide a revert operation that restores the working tree and index
     exactly to a given snapshot's tree state.
   - Must not use a mechanism that moves HEAD or the branch (i.e., no
     `git reset --hard` / JGit `ResetCommand`).
   - Must remove files created by the operation that are not part of the
     target tree (untracked-file cleanup after revert).

5. **Session inspection / export**
   - All snapshots across all chains must be listable (ref enumeration).
   - Support exporting the full snapshot history as a single portable file.
   - Support explicit cleanup (deleting refs) at session end, without
     affecting the real branch/remote in any way.
   - Confirm no snapshot refs are ever pushed to a remote by default.

6. **Eclipse UI integration**
   - A **singleton** panel (`ViewPart`, not `EditorPart`) that:
     - Shows all chains and their snapshots in a tree/list (top level:
       chain, children: individual operations).
     - On selection, displays the full aggregated unified diff for that
       operation in a dedicated diff area.
     - Vertical layout: ref/chain section on top, diff section below.
     - Provides a way to trigger revert to a selected snapshot from the
       panel.
   - Must not open a new editor tab per commit (this is why a custom
     singleton view is required instead of invoking EGit's built-in commit
     editor directly).
   - Reuse EGit/JGit facilities where possible instead of reimplementing
     diff computation or Git plumbing:
     - `DiffFormatter` for unified diff text generation.
     - `IHistoryView` / Additional Refs for cross-checking/inspection where
       useful, without depending on EGit's non-exported internal classes.

## Non-Functional / Robustness Requirements

- No dependency on Git CLI subprocess calls for the core snapshot/diff/
  revert logic — pure JGit, in-process.
- Every snapshot ref must point to a **commit object**, never a bare tree,
  so it renders correctly in EGit's Additional Refs / History view.
- Snapshot creation, diff generation, and revert must be robust against:
  - concurrent external Git operations happening outside the LLM
    operation's pre/post hook window (document the assumption if not
    handled),
  - large numbers of changed files per operation (avoid excessive per-file
    widget instantiation in the UI),
  - long-running sessions accumulating many refs (avoid unbounded resource
    growth without a cleanup path).
- Avoid dependence on internal/non-exported EGit packages
  (`org.eclipse.egit.ui.internal.*`); prefer public JGit API
  (`org.eclipse.jgit.*`) and public Eclipse Platform Team API
  (`org.eclipse.team.ui.history.*`) wherever functionality overlaps.
- Code must compile as part of an existing Eclipse plugin (OSGi bundle),
  with JGit available as a dependency via the EGit bundle or an explicit
  `org.eclipse.jgit` dependency in `MANIFEST.MF`.

## Explicitly Out of Scope

- No dry-run/sandboxed execution model — operations run for real; review
  happens after execution via diff + optional revert.
- No modification of the user's real branch, HEAD, index, or any pushed
  remote state.
- No requirement to reconstruct arbitrary point-in-time diffs across chains
  via patch replay — chain snapshots are stored as full tree/commit
  objects, not as sequential patches.
- No free-text commit messages or user-provided descriptions for
  snapshots.

## Reference Material

See the accompanying dossier (`ops-git-snapshot-dossier.md`) for:
- A working Bash/CLI reference implementation of the snapshot/revert logic.
- A JGit port of that logic (snapshot creation, diff generation, revert).
- Code for reading EGit's History panel and Additional Refs.
- A full singleton `ViewPart` implementation embedding the unified diff
  view, including `plugin.xml` registration.
- A pitfalls checklist (GC safety, chain parenting, HEAD safety, reflog
  misconceptions, push safety, no-op guards, concurrency assumptions).

## Deliverables Expected From Implementation

1. `OpSnapshotter` — snapshot creation with chain/base detection.
2. `OpDiffReader` — aggregated unified diff generation.
3. `OpReverter` — safe revert via `DirCacheCheckout` + clean.
4. `OpsPanel` (singleton `ViewPart`) — chain/snapshot tree + diff display
   + revert action, wired into the existing plugin's pre/post-operation
   hooks.
5. `plugin.xml` extension entries for the view registration.
