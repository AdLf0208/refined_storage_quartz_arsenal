# Refined Storage - Quartz Arsenal — a Horizon port

**Refined Storage - Quartz Arsenal** is a Minecraft mod. Quartz Arsenal is an addon for Refined Storage that adds various new tools. This repository is a **port of it to [Horizon](https://github.com/horizon-rust/horizon)**, the Rust engine for Minecraft Java 26.3. It carries the complete source of the port plus the content files it ships, and installs as a single `.hmod` package.

Upstream: [refinedmods/refinedstorage-quartz-arsenal](https://github.com/refinedmods/refinedstorage-quartz-arsenal) — licence **MIT**.

| | |
|---|---|
| Upstream mod | Refined Storage - Quartz Arsenal ([source](https://github.com/refinedmods/refinedstorage-quartz-arsenal)) |
| Ported to | Horizon 0b14f8a, Minecraft Java 26.3, mod API `0.2` |
| Package | `refinedstorage_quartz_arsenal.hmod` (install it with the Horizon launcher) |
| Licence | **MIT** — the upstream mod's licence; see [Licence and credit](#licence-and-credit) |

## What this port implements

- **Opening the grid is not possible yet.** Upstream's item opens a 3×3
- **The active/inactive model switch is pending upstream.** The 26.x model
- **The item's energy buffer** (`item_energy_buffer`, API 307 — stamped) belongs

## Not yet ported

- **The recipe cannot match.** It names `#c:ender_pearls` and
- **The parent ingredients do not exist here yet**: `refinedstorage:quartz_enriched_iron`
- **No behaviour**: right-click, the crafting menu, the FE buffer
- **No creative tab.** Upstream's `CreativeModeTabItems` adds the items to a tab
- **Stack size 1 is an inference, pinned in-game.** The jar's data does not set
- **Lang is carried, not rendered.** All 18 files hold the two item keys and
- **The `text.autoconfig.*` lang keys are inert** (no config screen).
- **No texture/model animation or tint** beyond the two carried PNGs.

## Installing

1. Install the [Horizon launcher](https://github.com/horizon-rust/horizon/releases) (0.2.70 or newer).
2. Add `refinedstorage_quartz_arsenal.hmod` to the instance's mods.
3. The package is pinned to a specific Horizon build; the launcher checks that pin.

## Licence and credit

Refined Storage - Quartz Arsenal was created by its upstream authors; the code, models and assets in the original mod
remain theirs and are used here under **MIT**. The Rust in this repository is a clean-room
rewrite for Horizon, not a translation of the original source.

- `LICENSE` — the upstream licence text.
- `LICENSE-NOTICE.md` — what this port carries and where it came from.
- The upstream project: https://github.com/refinedmods/refinedstorage-quartz-arsenal

## Technical details

```
horizon-mod.toml   package manifest (id, version, api, [native] pin)
Cargo.toml         the crate; depends on `horizon-mod-api` from the fetched SDK
src/               lib.rs (the one `impl Hooks`), runtime.rs (pure logic + tests), glue
assets/ data/      the content the package ships
tools/             content generators and the e2e scenario
```

- Build: `cargo horizon-mod build` (packages; it does not compile against the game).
- Test: `cargo test`, `cargo clippy --all-targets -- -D warnings`.
- In-game: `ports/tools/launcher-game.sh run refinedstorage_quartz_arsenal` in the campaign tree, or install the
  `.hmod` and play; the harness scenario is `tools/e2e.sh`.

This repository is generated from the campaign's port at `ports/refinedstorage_quartz_arsenal/`; the port's own
`notes.md` is the full record (decisions, traps, evidence).
