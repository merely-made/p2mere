# p2mere provenance and patch ledger

p2mere is the merely-made fork of [p2panda](https://github.com/p2panda/p2panda),
published as `p2mere-*` crates under the upstream licence (MIT OR Apache-2.0,
upstream authors kept). Library names are unchanged (`p2panda_core`, ...), so
consumer source code does not change; only manifests do.

## Recorded bases

- Fork base tag: `mere-p2panda-net-0.7.5-upstream-20261013` =
  `1f69c9bcb9f2a9d798b1c25b2ec2f47c0416bd65` (the family's pinned supplier).
- Upstream merge-base: `c99f5014836cb26ba96688baafad0d4cdb6e61d7`
  (2026-10-07, "node: Implement debouncer for repair task"). The fork contains
  upstream through this commit.
- Upstream release it descends from: `v0.7.1` (the last upstream tag; the
  merge-base is 0.7.1 plus later upstream main commits).
- `p2panda/p2panda` main at the previous ledger (2026-11-05):
  `d01898bfbce6af61abcb474c7b07f75eac5fbd69`, 59 commits past the merge-base.
  Taken in the intake below.
- Code differences from the merge-base to the base tag: 32 files, +1380 -125.

## Family patches over upstream (merge-base to base tag)

Non-merge commits, oldest first. All touch `p2panda-net` unless noted.

| Commit | Date | Purpose |
|---|---|---|
| 44e067d8 | 2026-07-29 | Unify on dalek 3 across the workspace. |
| c880761e | 2026-07-31 | Per-instance sync protocol id on the `LogSync` builder, so several sync instances can share a node. |
| 1a808d9d | 2026-08-16 | Make the address book store-agnostic. |
| 157b3998 | 2026-08-20 | Await `LogSync` store release on shutdown. |
| b837d53a | 2026-08-20 | Test for the shutdown store release. |
| 9f2c2a01 | 2026-08-23 | Admit exact external ALPN handlers (non-p2panda protocols on the endpoint). |
| e140e53b | 2026-09-09 | Release mere-p2panda-net 0.7.3. |
| 85f88345 | 2026-09-11 | Restore sync after rejoining existing peer overlays. |
| 3bb36461 | 2026-09-14 | Let an idle sync poller finish so shutdown drains promptly. |
| 0a54ab82 | 2026-09-14 | Release mere-p2panda-net 0.7.4. |
| d532713f | 2026-10-02 | Refresh topic watchers when a node's record is written. |
| 293dcafa | 2026-10-02 | rustfmt for the record-arrival gossip test. |
| 1bec457e | 2026-10-02 | Release mere-p2panda-net 0.7.5. |
| 64b4bcc8 | 2026-10-11 | Release 0.7.6 with current upstream and scoped network tests. |
| d8acabc9 | 2026-10-11 | Retain 0.7.5 package identity for pinned consumers. |

Merge commits (upstream intakes) in the same range: 4e80f5f9, 42ed21a0
(upstream v0.7.1), 9459fd41, b9cb6cc7, 94947fd1, 8efae5ff (upstream e11d590d),
91bafa2b, 3b1d8c0f, and the tag commit 1f69c9bc (upstream through 2026-10-13).

## Renames (2026-11-05, branch `p2mere`)

All packages move to version 0.8.0 (above every earlier 0.7.x of the fork and
upstream, so no confusion with `mere-p2panda-net` 0.7.x or upstream 0.7.1;
the new names start a new line). `repository` points at
`https://github.com/merely-made/p2mere`; descriptions end "(merely-made fork of
p2panda)". Workspace dependency keys keep the `p2panda-*` names with
`package = "p2mere-*"`, and each package sets `[lib] name` to its old name.

| Old package | New package | Library |
|---|---|---|
| p2panda | p2mere | p2panda |
| p2panda-auth | p2mere-auth | p2panda_auth |
| p2panda-blobs | p2mere-blobs | p2panda_blobs |
| p2panda-core | p2mere-core | p2panda_core |
| p2panda-discovery | p2mere-discovery | p2panda_discovery |
| p2panda-encryption | p2mere-encryption | p2panda_encryption |
| mere-p2panda-net | p2mere-net | p2panda_net |
| p2panda-spaces | p2mere-spaces | p2panda_spaces |
| p2panda-store | p2mere-store | p2panda_store |
| p2panda-stream | p2mere-stream | p2panda_stream |
| p2panda-sync | p2mere-sync | p2panda_sync |

`p2panda-fuzz` stays unpublished.

## Packaging fixes (2026-11-05, branch `p2mere`)

- `p2panda-sync` and `p2panda-net` enable `futures-channel`'s `sink` feature.
  Both use `mpsc::Sender` as a `Sink` but relied on `p2panda-discovery`
  enabling the feature through workspace unification, so each failed to
  compile when packaged alone. The same gap exists upstream; this one is worth
  offering back.

## Intake rule

Upstream work comes in by selective merge or cherry-pick, never a blind
`upstream/main` merge. Per release-log D7 and D8: wire compatibility with
upstream peers is kept where it costs nothing; a change that breaks
carrier-neutral use (the same sync, ordering and group machinery over
Reticulum, I2P, Tor and Noise/TCP as over iroh) is not taken as is. Family
changes are additive where possible so iroh peers still speak upstream's
protocols. Each intake appends its upstream revision and the commits taken or
skipped (with the reason) to this ledger. Changes worth offering back stay
licence-compatible and are sent upstream.

## Intake 2026-11-05 (branch `intake-20261105`, version 0.9.0)

Upstream revision taken: `d01898bfbce6af61abcb474c7b07f75eac5fbd69`
(2026-11-04, "node: Make members association a Processor impl"), merged whole
into the `p2mere-0.8.0` line: 59 upstream commits, 110 files. New merge-base is
that commit. Versions move to 0.9.0 in every package, because the
`p2mere-store` traits changed incompatibly (below) and `p2mere-spaces` was
reworked; consumers implementing store traits must change.

Taken:

- p2panda-spaces: read-modify-write state refactor, `StateChanges`, awaitable
  `PendingSpace` / `PendingGroup` events (`created()`, `joined()`), removed
  space members, filtering of application messages after removal, stricter
  key-bundle API, compact `AuthenticatedMember` encoding (a spaces wire change),
  `AuthMessage` moved into the message module.
- p2panda-store: every write is now an explicit `_tx` method (`insert_node_info_tx`,
  `set_topics_tx`, `prune_entries_tx`, `associate_tx`, `remove_tx`, ...), reads
  gain non-transactional forms, `KeySecretsStore` is `PreKeyBundlesStore`,
  event metadata is namespaced. This is the incompatible change.
- p2panda (high-level node): spaces and groups integrated into `Node`, a global
  `RepairTask` / `RepairHook`, member association as a `Processor`, example
  renames. The family does not link this crate; it is carried, not used.
- p2panda-net: iroh 1.4.1 and iroh-gossip 0.102.0, a flaky `e2e_log_sync` fix.
- p2panda-encryption: hpke-rs 0.8 (RUSTSEC-2026-0387).

Reconciled with family patches:

- Address book (`1a808d9d`, store-agnostic): kept. Upstream's actor uses
  `tx!` on `SqliteStore`; the actor keeps calling the store through
  `AddressBookStoreHandle`, whose `AddressBookStore` impl now carries the
  `_tx` method names and whose backends take a permit per write (`Transacted`)
  or none (`Direct`). Upstream's `tx!` use in the actor is declined: it binds
  SQLite, which the carrier-neutral path (D8) must not require.
- `LogSync` tests: upstream's rewritten `e2e_log_sync` taken; the family
  `ReleaseTrackedStore` (shutdown release test) renamed to the `_tx` store
  methods and its removed `get_latest_entry` dropped.
- `p2mere-*` package names with `p2panda_*` library names, the
  `futures-channel` `sink` fix, and per-package versions kept.

Declined: nothing else. No upstream change touched sync protocol ids, ALPN
handling or the encoding of non-spaces wire messages, so iroh peers still
speak upstream's protocols.

Tests: `test --workspace --exclude p2mere` passes (361 tests). The umbrella
`p2mere` (`tests/api.rs`) is excluded, as before.
