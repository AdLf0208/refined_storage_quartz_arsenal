# Refined Storage - Quartz Arsenal — port notes

Ported from **Refined Storage - Quartz Arsenal 1.0.8**
(`refinedmods/refinedstorage-quartz-arsenal`, **MIT**) for Minecraft Java 26.3.
**0.1.0** was a `.hnmod` bundle at Horizon
**`1d63e0a5a37a7a708eb0b32b49bf0dfeb28ed8a9`** / horizon-mixins
**`5a64af938ec86b00d53d16a408aabc0038fb3e57`** (schema 6); **0.2.0** is the same
content as one flat SDK package at Horizon **`0b14f8acfd97fa2c750e42795249dcc981178ca4`**
/ horizon-mixins **`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`** (see
[`## Migration to the SDK format`](#migration-to-the-sdk-format-020-2026-11-18)).

**LOCAL-ONLY** until the maintainer approves publishing (ARCHITECTURE.md §8).
MIT allows derivatives; the approval is still required.

**Class + tier** (`ports/ledger.csv`; provenance
`ports/_shared/source-map/atm10-sources.csv`): **content / logic**, **add-on**
(parent `refinedstorage` is `ported-partial`). The tier's approach per
`horizon/docs/modpacks.md` is **carry-over**: the jar's `assets/` and `data/`
upgraded to 26.3, with the items declared from a vanilla template in
`data/<id>/horizon/item/`. There is **no behaviour** in this slice, so the
bundle is a test-only census model with **zero injections**; the items are
declared in `content/`.

**Licence: MIT → rank 0**, read from the jar's own `META-INF/neoforge.mods.toml`
(`license = "MIT"`) and `LICENSE.md` (Copyright © 2024 - 2025 Refined Mods).
`precheck-mod.py` → exit 0, `DISPATCHABLE`. Ported now because the licence-first
queue ranks a rank-0 content carry first and the parent (the #1 core base) is
already `ported-partial`.

Status: **0.2.0 is the SDK package; its content slice was verified in-game at
0.1.0.** The slice is the mod's entire content: two items, their models and
textures, 18 locales and one recipe. Everything else is the gap list below, not a
claim. The 0.2.0 package has **no in-game run** — `horizon-launcher` is not
installed here, so no game carrying a native `.hmod` can be built (gap 10).

## What this mod actually is (the first brief got it wrong)

The name and the README's "adds various new tools" suggest a quartz tool family.
The jar contains **no tool, weapon or block at any version**: `unzip -l` shows
two item families (`wireless_crafting_grid`, `creative_wireless_crafting_grid`),
4 item models, 2 textures, 18 lang files, 1 shaped recipe, and the `CHANGELOG`
from 0.1.0 to 2.0.6 lists only the Wireless Crafting Grid and its energy/config
knobs. That finding is recorded in
[`ports/_shared/evidence/refinedstorage_quartz_arsenal/brief-conflict.md`](../_shared/evidence/refinedstorage_quartz_arsenal/brief-conflict.md)
and became the campaign rule "a brief may not invent a mod's contents from its
name" (AGENTS.md). Slice 1 was re-scoped to the jar's real contents.

**Census corrections to the re-brief** (the artefact wins): the jar has **18**
lang files, not 19, and **4** item models, not 5 (the fifth file the brief counted
is the 26.x `assets/<ns>/items/` definition the repo grew after 1.21.1 — the pack's
1.21.1 jar has none). The port's tests and `tools/check_content.py` pin 18 and 4.

## Content carried (all of it)

| Part | Count | Detail |
|---|---:|---|
| Item declarations (`data/<ns>/horizon/item/`) | 2 | `minecraft:paper` template, `max_stack_size: 1`, no components |
| Item definitions (`assets/<ns>/items/`) | 2 | 26.3 `{"model":{"type":"minecraft:model","model":"<ns>:item/<n>"}}` |
| Item models | 4 | 2 base models + the `wireless_crafting_grid/{active,inactive}` pair, carried verbatim |
| Textures | 2 | `active.png` (567 B) and `inactive.png` (536 B), byte for byte |
| Lang | 18 | every locale byte for byte (22 keys each, same key set) |
| Recipes | 1 | `crafting_shaped` `EPE/EGE/EAE` → 1 grid |
| Manifest | 1 | API 0.2, `side = "both"`, capability `registries` |

**Copied verbatim vs upgraded:**

* **Verbatim:** the 2 textures, the 2 `active`/`inactive` variant models, and all
  18 lang files (their keys are already vanilla's `item.<ns>.<path>` shape).
* **Upgraded:** the 2 base models — 1.21.1's `overrides` block on the
  `refinedstorage:network_bound` predicate is not a 26.3 item-model node
  (`horizon-assets/src/item_model.rs`: 26.3 uses `condition` / `select` /
  `range_dispatch`), so both base models draw the carried `inactive` variant; the
  2 item definitions are new (26.3 binds the model through
  `assets/<ns>/items/<n>.json`); the recipe's `{"item": …}` / `{"tag": …}`
  ingredients became 26.3's `"id"` / `"#tag"`.

## Slice boundary — no behaviour, and why

1. **Opening the grid is not possible yet.** Upstream's item opens a 3×3
   crafting grid bound to an RS network. The parent port's slice 1 has a
   controller, cable, disk drive, 1k disk and a storage *grid* — no crafting
   grid, no crafting menu, no network-bound item and no crafting surface. A
   faithful menu needs the parent's crafting half first (`use_item`,
   `menu_extra_data` + `screen`, and the network's extract/insert).
2. **The active/inactive model switch is pending upstream.** The 26.x model
   selects on `refinedstorage:network_bound`, which a client answers through
   `Hooks::item_model_property`; the stamped catalogue at this pin lists it (API 1, listen) — an earlier reading called it pending, which is corrected here. Note
   `"hooks": 385` once this pending hook is stamped"). A pending hook may not be
   named, so the switch is **carried as files, not as behaviour**.
3. **The item's energy buffer** (`item_energy_buffer`, API 307 — stamped) belongs
   to the behaviour slice, not to a data carry.

So this slice is the content and nothing else: registration, item semantics,
assets, lang and the recipe. No injection, no `.rs` in the game beyond the
test-only census.

## Gaps (next slices)

1. **The recipe cannot match.** It names `#c:ender_pearls` and
   `#refinedstorage:crafting_grids`; 26.3 ships no `c:` tags and this pack has no
   `refinedstorage:crafting_grids`. An unresolved **tag** makes the ingredient
   empty and `Ingredient::parse` returns `None`, so the loader **drops the
   shaped recipe entirely** (`crafting.rs`) — it is not merely unmatchable.
2. **The parent ingredients do not exist here yet**: `refinedstorage:quartz_enriched_iron`
   and `refinedstorage:advanced_processor` are parent-port gap 3 (the parent's
   item chain is not ported; its items come from creative). Even with both tags,
   the recipe would need those two items.
3. **No behaviour**: right-click, the crafting menu, the FE buffer
   (`item_energy_buffer`), the network binding, the key binding
   (`open_wireless_crafting_grid`), the energy/crafting config knobs and the
   `active`/`inactive` model switch are all open.
4. **No creative tab.** Upstream's `CreativeModeTabItems` adds the items to a tab
   (its own or Refined Storage's); which one is not observable from the jar's
   data and the parent port declares no tab, so nothing was invented. Both items
   are `/give`-able and the search tab indexes them.
5. **Stack size 1 is an inference, pinned in-game.** The jar's data does not set
   it (the item class does). It is carried as 1 because the item holds per-stack
   state (FE in `custom_data`, a network binding); the e2e proves the effect
   (`/give 2` lands as two stacks of one). If upstream turned out to set 64, it
   is one line in `tools/gen_content.py` plus the e2e assertion.
6. **Lang is carried, not rendered.** All 18 files hold the two item keys and
   `tools/check_content.py` proves their key sets agree, but `horizon-ctl` has no
   path to a slot's display name, so the rendered name is not asserted.
7. **The `text.autoconfig.*` lang keys are inert** (no config screen).
8. **No texture/model animation or tint** beyond the two carried PNGs.
9. **`ported-from` is not a manifest field at this pin**
   (`horizon-mods/src/manifest.rs` is `deny_unknown_fields`), so provenance lives
   in `ports/_shared/carry-work/refinedstorage_quartz_arsenal/SOURCE.md` and in
   `LICENSE-NOTICE.md` (inside the package).
10. **No in-game run of the 0.2.0 package.** A native `.hmod` is loaded by a game the
    launcher built (`horizon-launcher native-build … --trust-native-code`), and the
    launcher is **not installed** in this environment. The 0.1.0 runs below stand as
    the behaviour's evidence; the packaging changed, not the content (29/29 carried
    files byte-identical), and nothing here claims otherwise.

## Migration to the SDK format (0.2.0, 2026-11-18)

Horizon's mod-API merge (#16941, launcher ≥ 0.2.70, PR #17992) retired `.hnmod`; the
registry closed every `.hnmod` submission on 2026-11-18 and an `.hnmod` no longer loads
(ARCHITECTURE.md §8). **This is a first publication** — this mod was never submitted, so
there is no closed PR to supersede: 0.2.0 is the same content as the one flat SDK package.

Pins: SDK **`0b14f8acfd97fa2c750e42795249dcc981178ca4`** (2026-11-18, `package_format` 6,
`content_api` "0.2"), mixins **`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`**. The pinned SDK is
cached in-tree at `../_shared/sdk/0b14f8a/` and was copied into `.horizon/sdk/` (gitignored) so
every build ran offline.

### What moved, and where the retired tree is

`../_shared/evidence/refinedstorage_quartz_arsenal/retired-hnmod-0.1.0/` holds the retired tree
(the move happened **before the first build**, trap 1; `sdk-retired-move.log`).

| 0.1.0 path | Now | Evidence |
|---|---|---|
| `mixins/refinedstorage_quartz_arsenal/runtime.rs` (10 tests) | `src/runtime.rs`, `cp` + `cmp`, **byte-identical** (sha256 `0aa2460a…e56d`) | `retired-hnmod-0.1.0/mixins/…/runtime.rs` |
| `mixins/…/horizon-mixin.toml` (1 `[[module]]`, **0 `[[injection]]`**) | no plan: `src/lib.rs` is the one `impl Hooks` | `retired-hnmod-0.1.0/mixins/` |
| `content/horizon-mod.toml` (1.0.8, no `[native]`) | the package root's `horizon-mod.toml` (0.2.0, `[native]`, description/authors/README added) | `retired-hnmod-0.1.0/horizon-mod.toml` |
| `content/assets/` + `content/data/` | `assets/` + `data/` at the package root — **29 files, byte-identical** | `retired-hnmod-0.1.0/refinedstorage_quartz_arsenal.hmod`, every entry compared (`sdk-carried-unchanged.log`) |
| `content/refinedstorage_quartz_arsenal.hmod` (16,346 B) | not needed: the content is inside the one package | same folder |
| `refinedstorage_quartz_arsenal.hnmod` (5,007 B) | `refinedstorage_quartz_arsenal.hmod` (**29,801 B**, native + content) | same folder |
| `compatibility.json`, `rust-toolchain.toml` | `[native]` in the manifest; the pinned commit decides the toolchain | same folder |
| `tools/horizon.py`, `tools/package.py`, `tools/package.sh` | deleted: staging and `.hnmod` packaging are the retired format | `retired-hnmod-0.1.0/tools/` |
| `tools/hmod-load.sh` | deleted: it proved the *content* `.hmod` loads as `--dev-mod`, which a native package cannot be; `tools/e2e.sh`'s default leg now covers the same claim on a game built with the package | `retired-hnmod-0.1.0/tools/` |
| `Cargo.lock` | not kept: it is packed and changes the artifact (trap 5) | not kept |

**Kept and re-pointed:** `tools/e2e.sh` (a launcher-built game, `GAME=`/`CONTROL_GAME=`,
`SELFTEST=1` still fail-capable), `tools/gen_content.py` (`--check` now re-derives `assets/` +
`data/` from the jar at the package root: **29 files, 0 stale, 0 extra**), `tools/check_content.py`
(the tree's cross-reference check at the root, pinning `version = "0.2.0"`). **New:**
`README.md` (written before the first build, trap 19), `LICENSE-NOTICE.md`.

### The wire: every old plan entry, mapped

The 0.1.0 plan carried **one `[[module]]` and zero `[[injection]]`**, because the mod is content
and the module was a test-only census. So **no helper became a hook and no hook was dropped**:

| 0.1.0 plan entry | 0.2.0 |
|---|---|
| `[[module]] runtime` → `crates/horizon-client/src/game.rs` | `pub mod runtime;` — an ordinary Rust module beside `src/lib.rs`, byte-identical, its 10 tests still green |
| no `support.rs`, no per-hook helper | none: `src/lib.rs` is the one `#[mod_api::horizon_mod] impl Hooks` with **no method**, because this slice has no behaviour |
| the two item declarations in `content/data/<ns>/horizon/item/` | the same files in `data/<ns>/horizon/item/`: the SDK's content API reads them; no `Registrar` call is needed |

**The behaviours this mod still owes, and the shape each one has in this SDK** (a gap with a
home, not a silent deletion):

| Upstream behaviour (0.1.0 gap) | Where it now lives | State |
|---|---|---|
| right-click a grid → a 3×3 crafting surface bound to an RS network | the item part (`part:item`) → `use_item`; the surface itself is a menu (`hook:menu_*`) or a block entity part (`part:block_entity` → `screen`) | **gap**: the parent port has no crafting half, no menu and no network binding |
| `active`/`inactive` model switch on `refinedstorage:network_bound` | `hook:item_model_property` — **present in this SDK's surface**; the 0.1.0 "pending hook" reason no longer holds, so this is now an implementation gap, not an API one | **gap**: needs the parent's property and the item's binding; both models still draw `inactive` |
| the item's FE buffer | `capability:horizon:item_energy` (`EnergyHandler`, `ItemComponentEnergy`) | **gap**: behaviour slice |
| key bind `open_wireless_crafting_grid` | `KeyMappingsRegistrar` + `hook:input_key` / `interaction_key_mapping_triggered` | **gap**: behaviour slice |
| creative tab | `data/<ns>/horizon/creative_tab/` (which tab upstream used is not observable from the jar) | **gap** (recorded at 0.1.0; nothing invented) |
| the recipe's two tags and two parent items | `data/<ns>/recipe/` | **gap** unchanged: an unresolved tag makes `Ingredient::parse` return `None`, so the loader drops the shaped recipe |

### The one number the census and the manifest disagree about, on purpose

`src/runtime.rs` is byte-identical, so its `the_manifest_values_the_hmod_must_declare` test still
pins `VERSION = "1.0.8"` — the **upstream jar's** version, which the 0.1.0 content manifest
declared. The 0.2.0 manifest declares **0.2.0**: a republish is a new port version (campaign rule),
and Cargo's version is not the mod's identity. The test passes because it compares constants, and
`tools/check_content.py` pins the manifest's real value. **Do not "fix" `runtime.rs` for this**:
byte-identity is the rule, and the divergence is recorded here.

### Traps measured in this migration (new to this port)

1. **Retired paths refuse the package.** `mixins/`, `content/`, the `.hnmod`, `compatibility.json`
   and `rust-toolchain.toml` were moved to the evidence folder *before* the first build; the
   package test asserts none of them returns, so a stray "content/" folder fails `cargo test`.
2. **The package root and the content root became the same folder.** `tools/gen_content.py` wrote
   `content/` and its `--check` scanned the whole folder; at the root that reported every document
   as `extra`. It now writes/compares `assets/` + `data/` only and no longer owns
   `horizon-mod.toml` (the package manifest is not carried content).
3. **`tools/check_content.py` hardcoded the old default.** After the move it died with
   `StopIteration` (no recipe found in the gone `content/`) — that red was the tree's own test of
   the move; re-pointing the default at the root and pinning `version = "0.2.0"` turned it green.
4. **`pub mod runtime;` is easy to lose.** Rewriting `lib.rs` for the glue dropped the module
   declaration and `cargo test` went from 13 tests to **3, still exiting 0** — the census tests
   silently vanished. Watch the *count*, not just the exit code (found and fixed here).
5. **`cargo horizon-mod check <f> --uses` prints the envelope, not the list.** The surface record
   is `uses.json` *inside* the zip (`unzip -p … uses.json`), 69 entries here.
6. **`--json`'s `capabilities` is `[]` and `side` is `null` for this package**, although the
   manifest says `["registries"]` / `both`. It is not a manifest check: `enderstorage` (which
   declares blocks, items, a component *and* `horizon:fluids`) reports exactly the same
   `"capabilities":[]`, `"side":null` at this pin, and native mods are not in the LAN mod list
   yet. Read the manifest, not those two fields, and do not "correct" it.
7. **`uses.json` lists 4 `hook:` entries** (`items`, `ingredient`, …) **with no hook implemented**:
   they are the trait's own default bodies that the macro's dispatch compiles. Count implemented
   methods in `src/lib.rs`, never from `uses.json`.
8. **Never run bare `cargo fmt` here.** `cargo fmt --check` reported two diffs in `lib.rs`; a bare
   `cargo fmt` would reformat `src/runtime.rs` as well and break byte-identity. Apply the printed
   diff by hand and re-check.
9. **A 38-hex pin reads as a missing pin** (`native requires game and exact revisions`). Both pins
   are full 40-hex commits and a unit test now fails if either is short.

### Changelog (0.2.0)

> **0.2.0 — first publication, as an SDK `.hmod`.** Quartz Arsenal for Horizon has not been
> published before, so this is a first publication and supersedes no closed PR. Horizon's mod-API
> merge (launcher ≥ 0.2.70) retired `.hnmod`, and this version is the one flat package the new
> format wants: `refinedstorage_quartz_arsenal.hmod` carries the code *and* the content — the two
> items (`wireless_crafting_grid`, `creative_wireless_crafting_grid`), four item models, two
> textures, eighteen locales and the shaped recipe, all byte-identical to 0.1.0 — with
> `src/lib.rs` as the one `impl mod_api::Hooks` (no hook: this is a content carry) and
> `src/runtime.rs` unchanged. What changed for `api = "0.2"`: the content now travels inside the
> package instead of a separate content `.hmod` loaded with `--dev-mod`; the manifest is the only
> identity and pins `[native] game 26.3 / horizon 0b14f8a / mixins e59da90`; there is no mixin
> plan, no `compatibility.json` and no `rust-toolchain.toml`; and the items' data is read by the
> SDK's content API. Behaviour is unchanged, because there is none yet: opening a grid, the
> energy buffer, the network binding, the model switch, the key bind and a creative tab remain
> open, exactly as listed in the gaps. Not run in a game — `horizon-launcher` is not installed
> here.

## Files (0.2.0 — the SDK package)

```
horizon-mod.toml                                the manifest and the only identity (0.2.0, [native])
Cargo.toml                                      one crate; the SDK's API crate from .horizon/sdk
.gitignore                                      /target, /.horizon, /Cargo.lock
LICENSE, LICENSE-NOTICE.md                      MIT text + provenance, inside the package
README.md                                       rewritten for the SDK package
notes.md                                        this file
refinedstorage_quartz_arsenal.hmod              the deliverable (29,801 B)
src/lib.rs                                      the one impl Hooks (no method) + 3 package tests
src/runtime.rs                                  the pure census, byte-identical, 10 tests
assets/refinedstorage_quartz_arsenal/items/      2 item definitions (26.3)
assets/…/models/item/                            4 item models
assets/…/textures/item/…                         2 textures
assets/…/lang/                                  18 locales
data/refinedstorage_quartz_arsenal/horizon/item/ 2 declarations
data/refinedstorage_quartz_arsenal/recipe/       1 recipe
tools/gen_content.py                            re-derives assets/+data/ from the jar (--check)
tools/check_content.py                          the tree's cross-reference check
tools/e2e.sh                                    the in-game scenario (launcher-built game)
```

The retired 0.1.0 tree (`mixins/`, `content/`, `.hnmod`, `compatibility.json`,
`rust-toolchain.toml`, `tools/horizon.py`, `tools/package.*`, `tools/hmod-load.sh`) is in
`../_shared/evidence/refinedstorage_quartz_arsenal/retired-hnmod-0.1.0/`.

## 26.3 carry traps hit

- **`overrides` is dead in 26.3 item models.** The jar's base models select a
  variant through `overrides` + a predicate; that node does not exist in 26.3's
  item-model tree, so carrying it verbatim would bake nothing. Rewritten, and the
  switch recorded against the `item_model_property` hook (stamped, API 1, client-side listen).
- **An unresolved tag drops the recipe, it does not just fail to match.** Worth
  knowing for every carry that ships NeoForge `#c:` or a mod tag.
- **`deny_unknown_fields` on the content manifest**: no `ported-from` key, so a
  provenance line must be a comment.
- **`ctl inventory` has no `slots` key** (`hotbar` + `main` + `armor` +
  `offhand` + `counts`); `slots` belongs to `ctl screen` with the inventory open.

Skills: superpowers/using-superpowers, brainstorming, writing-plans, test-driven-development, systematic-debugging, verification-before-completion; BeautifulRust/rust-best-practices, rust-port

## Verification (0.2.0 — the SDK package)

Pin **`0b14f8acfd97fa2c750e42795249dcc981178ca4`** / horizon-mixins
**`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`** (SDK `package_format` 6, `content_api` "0.2"),
2026-11-18, compiled with rustc 1.97.0 (the SDK declares 1.99.1; edition 2024). Every cargo step ran
through `ports/tools/build-slot.sh run --job refinedstorage_quartz_arsenal-sdk --port
ports/refinedstorage_quartz_arsenal --step …` with `CARGO_INCREMENTAL=0`, `BUILD_LOCAL=1` (the far
box has no copy of this port), no `--target-dir`; logs are in
[`ports/_shared/evidence/refinedstorage_quartz_arsenal/`](../_shared/evidence/refinedstorage_quartz_arsenal/).

| Check | Command | Result |
|---|---|---|
| red proof (glue) | `cargo test` with `src/lib.rs` + tests but no manifest | **2 failed of 3** (`horizon-mod.toml: No such file or directory`); the third (retired paths gone, carried tree present) passed — `sdk-test-red.log` |
| tests | `cargo test --offline` | **13 passed, 0 failed** (10 `runtime::`, 3 `package::`) — `sdk-test.log` |
| lints | `cargo clippy --all-targets --offline -- -D warnings` | exit 0, **0 warnings** — `sdk-clippy.log` |
| format | `cargo fmt --check` (**not** `--all`) | exit 0, empty diff — `sdk-fmt.log` |
| package | `rm -f Cargo.lock && cargo horizon-mod build` | `SDK 0b14f8a … in ./.horizon/sdk` / `Built ./target/horizon-mod/refinedstorage_quartz_arsenal.hmod` — `sdk-build.log` |
| reproducible | the same build again | `cmp` identical, same sha256 — `sdk-build-second.log`, `sdk-sha256.txt` |
| delivered file | `refinedstorage_quartz_arsenal.hmod` in the port root | **29,801 B**, sha256 `91e6edf809039aa7c1953b9030eb329ae95d01b2e44b0f029e6bab957a9f7409` |
| registry check | `cargo horizon-mod check refinedstorage_quartz_arsenal.hmod` | `refinedstorage_quartz_arsenal 0.2.0 (native-mod, format native-1/0b14f8ac…/e59da909…, for 26.3)` / `29801 bytes, SHA-256 91e6edf8…, unsigned` / *native source: unsandboxed; not executed by the registry check* — **EXIT 0** — `sdk-check.log` |
| machine form | `… check … --json` | `{"ok":true,…,"kind":"native-mod","sha256":"91e6edf8…","size":29801,"version":"0.2.0"}` — EXIT 0 — `sdk-check-json.log` |
| fits the pinned build | `cargo horizon-mod fits … --build nightly` | `Refined Storage - Quartz Arsenal:` / `0b14f8a  made for` — **EXIT 0** — `sdk-fits.log` |
| entries | `unzip -l …` | **37**: `horizon-mod.toml`, `Cargo.toml`, `LICENSE`, `LICENSE-NOTICE.md`, `README.md`, `uses.json`, 26 `assets/`, 3 `data/`, `src/lib.rs`, `src/runtime.rs`; **no `Cargo.lock`** — `sdk-entries.log` |
| runtime untouched | `cmp src/runtime.rs <retired>` and `unzip -p … src/runtime.rs \| sha256sum` | identical, `0aa2460a00ce93577c337b7e5b77ccdd96f7d92ccb97aadc9fd313284c5ee56d` all three — `sdk-runtime-identical.log` |
| carried files unchanged | retired 0.1.0 content package vs 0.2.0, entry by entry | **29/29 byte-identical**, 0 differ, 0 missing — `sdk-carried-unchanged.log` |
| content census | `python3 tools/check_content.py` | `PASS: content census 2 items, 4 models, 2 textures, 18 locales, 1 recipe; 0 failures` — `sdk-content-check.log` |
| carry re-derivation | `python3 tools/gen_content.py --check` | `29 files, 0 stale, 0 extra` — `sdk-gen-check.log` |
| what it uses | `unzip -p … uses.json` | 69 entries: 4 `hook:`, 5 `method:`, 1 `part:`, 6 `trait:`, 34 `type:`, 16 `handle:`, 2 `capability:` (`horizon:items`, `registries`), 1 `macro:` — `sdk-uses.json`, `sdk-uses-summary.log` |
| skills line | `python3 ports/tools/check-skills.py --notes ports/refinedstorage_quartz_arsenal --strict` | `Skills line present`, **SKILLS GATE: PASS** |

**Not run: the in-game e2e.** It needs a client the launcher built with this package
(`horizon-launcher native-build … --trust-native-code`) and `horizon-launcher` is not installed
here, so the 0.2.0 package has **no in-game claim** (gap 10). `tools/e2e.sh` is updated for that
game (`GAME=`, `CONTROL_GAME=`, `SELFTEST=1` still fail-capable) and the 0.1.0 logs below are
**historical**.

## Verification (0.1.0 — historical, `.hnmod` + content `--dev-mod`)

The two 0.1.0 packages named below (`content/refinedstorage_quartz_arsenal.hmod` and
`refinedstorage_quartz_arsenal.hnmod`) are in
`../_shared/evidence/refinedstorage_quartz_arsenal/retired-hnmod-0.1.0/`; nothing in this section
describes the 0.2.0 package.

Pin **`1d63e0a5a37a7a708eb0b32b49bf0dfeb28ed8a9`** / horizon-mixins
**`5a64af938ec86b00d53d16a408aabc0038fb3e57`** (schema 6), Rust 1.95.0, 26.3
prepared resources at `/home/adlf/Desktop/horizon/.local/horizon-client/26.3`.
Every cargo/ladder/e2e command ran on 2026-11-09 through
`ports/tools/build-slot.sh` (registry `ports/_shared/build-slots`,
`CARGO_INCREMENTAL=0`, no bare cargo, no `--target-dir`); logs are in
[`ports/_shared/evidence/refinedstorage_quartz_arsenal/`](../_shared/evidence/refinedstorage_quartz_arsenal/).

### Standalone runtime crate (test-only census)

| Check | Command | Result |
|---|---|---|
| red proof | `cargo test` with a deliberately wrong locale count | **1 failed** (`left: 18, right: 19`), exit 101 — `runtime-red-proof.log` |
| fmt | `cargo fmt --all -- --check` | `FMT_OK` |
| tests | `cargo test --locked --offline` | **10 passed, 0 failed** |
| clippy | `cargo clippy --all-targets --locked --offline -- -D warnings` | exit 0, **0 warnings** |

The red run also caught a real bug in the census itself (a `contains` that
matched the namespace's own suffix, where `starts_with` was meant); the test was
fixed and re-run red so only the intended failure remained.

### Staged ladder at `1d63e0a`

| Step | Result |
|---|---|
| stage | `stage_exit=0`, `stage_changed: true`, revision `1d63e0a5…`, 89.7 s |
| check | `cargo_exit: 0`, 76.9 s, **0 warnings** (`horizon-check.log`) |
| test | `cargo_exit: 0`, **10 passed; 0 failed; 1240 filtered** (`horizon-test.log`) |

`horizon-stage.json`, `horizon-check.json`, `horizon-test.json`. (`build` was not
run: the bundle has zero injections, so it contributes no code to the game
binary; the e2e uses the campaign's shared `1d63e0a5` game.)

### Packages (delivered in the port, not in `target/`)

| Package | Size | SHA-256 | Entries | Check |
|---|---:|---|---:|---|
| [`content/refinedstorage_quartz_arsenal.hmod`](content/refinedstorage_quartz_arsenal.hmod) | 16,346 B | `009a55a52c52f3d9fe8b1b376f9b717d58ddaebbba3c2176e7132eec2c20fec7` | 30 (26 assets, 3 data, 1 manifest) | `cargo-horizon-mod check` exit 0: `API 0.2, both`, **`no code: content only, nothing to run`**; a repeat build reproduces the same SHA-256 |
| [`refinedstorage_quartz_arsenal.hnmod`](refinedstorage_quartz_arsenal.hnmod) | 5,007 B | `75a81a0e754fd34856df3ca5def387616df9f86904ffb4c794b20ec82c9a46fb` | 3 (`horizon-native.json`, bundle toml + runtime.rs) | manifest pins `horizon 1d63e0a5…`, `mixins 5a64af93…`, schema 1, game 26.3; repack reproduces the same SHA-256 |

`package-hmod.log`, `package-hnmod.log`, `content-check.log`.

### In-game runs (shared unmodded `1d63e0a5` game)

| Run | Command | Result |
|---|---|---|
| default | `tools/e2e.sh` | **PASS + `EXIT:0`** — `e2e-default.log` |
| counterfactual | `COUNTERFACTUAL=1 tools/e2e.sh` | **PASS + `EXIT:0`** — `e2e-counterfactual.log` |
| negative | `NEGATIVE=1 tools/e2e.sh` | **FAIL + `EXIT:1`** as required — `e2e-negative.log` |
| selftest | `SELFTEST=1 tools/e2e.sh` | **FAIL + `EXIT:1`** as required — `e2e-selftest.log` |
| packaged `.hmod` | `tools/hmod-load.sh` | **PASS + `EXIT:0`** — `e2e-hmod-load.log` |

`python3 ports/tools/check-e2e-logs.py refinedstorage_quartz_arsenal --strict` →
exit 0: `logs=5 ok=5 missing-exit=0 bad-exit=0`.

The default run's transcript (quoted):

```
give refinedstorage_quartz_arsenal:wireless_crafting_grid -> inventory count 1
give refinedstorage_quartz_arsenal:creative_wireless_crafting_grid -> inventory count 1
three grids: total=3 slots-of-one=3 slots-of-two=0
give refinedstorage_quartz_arsenal:quartz_pickaxe -> refused, inventory count 0
minecraft:stick still gives in the same world
PASS: both wireless grids register and give (1 per slot, 3 grids in 3 slots), a plausible absent sibling id is refused while a vanilla stick still gives
EXIT:0
```

The controls (quoted):

```
vanilla minecraft:stick = 1
give …:wireless_crafting_grid without --dev-mod: refused, inventory count 0
give …:creative_wireless_crafting_grid without --dev-mod: refused, inventory count 0
PASS: counterfactual (no --dev-mod: both ids are unknown; minecraft:stick still gives)
EXIT:0

FAIL: NEGATIVE: the deliberately false assertion (the content is registered without --dev-mod) tripped, as it must (this exit 1 is the proof)
EXIT:1

SELFTEST: reading the deliberately wrong stack shape (one slot of 2)
FAIL: SELFTEST: read singles=3 doubles=0 where the deliberately wrong assertion wanted one slot of 2; the stack read-back is live (this exit 1 is the proof)
EXIT:1
```

The packaged-artifact run (quoted):

```
packaged give refinedstorage_quartz_arsenal:wireless_crafting_grid -> inventory count 1
packaged give refinedstorage_quartz_arsenal:creative_wireless_crafting_grid -> inventory count 1
control give refinedstorage_quartz_arsenal:wireless_crafting_grid -> refused, inventory count 0
control give refinedstorage_quartz_arsenal:creative_wireless_crafting_grid -> refused, inventory count 0
PASS: the .hmod registers the wireless crafting grid and its creative variant and both give; without it the same ids are refused
EXIT:0
```

### Harness facts learned here

- **A refusal is a non-zero `ctl give`**, and a vanilla id in the same world is
  the differential that stops "refused" from meaning "the command is broken".
- **`/give 2` of a `max_stack_size: 1` item lands in two slots** — the only way
  to observe a stack limit through `ctl inventory` (`counts` sums the stacks, so
  the per-slot shape comes from `hotbar`/`main`).
- The registry was saturated (12–14 waiters) for the packaged-load leg; it was
  batched with the other legs and then retried once with a longer
  `BUILD_WAIT_MAX`, and every leg still ran in a slot.
