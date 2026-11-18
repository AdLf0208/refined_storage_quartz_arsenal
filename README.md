# Refined Storage - Quartz Arsenal for Horizon

A Horizon port of **Refined Storage - Quartz Arsenal 1.0.8**
(`refinedmods/refinedstorage-quartz-arsenal`, MIT) for Minecraft 26.3.

Despite the name, the addon contains **no tool, weapon or block**: its whole content is
two items — `refinedstorage_quartz_arsenal:wireless_crafting_grid` and
`refinedstorage_quartz_arsenal:creative_wireless_crafting_grid` — with four item models,
two textures, eighteen locales and one shaped recipe. That is what this port carries, and
nothing is invented around the name.

**Format: the SDK package (Horizon mod API 0.2).** One flat
`refinedstorage_quartz_arsenal.hmod` holds `horizon-mod.toml`, `Cargo.toml`,
`src/lib.rs` (the one `impl mod_api::Hooks` — no hook, because this slice is a content
carry), `src/runtime.rs` (the campaign's pure census, byte-identical to the retired
`.hnmod` port, 10 tests) and the content in `assets/` + `data/`. See
[notes.md § "Migration to the SDK format"](notes.md) for the file map, the census, the
gaps and the traps.

## What a player gets, and what they do not

Both grids register, give with `/give`, draw their carried models and occupy one slot each
(`max_stack_size: 1`, because the item holds per-stack state upstream). **Right-clicking one
does not open a crafting grid**: that needs a crafting surface bound to a Refined Storage
network, and the parent port (`refinedstorage`) has no crafting half yet. The
active/inactive model switch, the item's FE buffer, the network binding, the key bind, the
config knobs and a creative tab are all open; see `notes.md` for the full gap list.

## Build and test

No staging step and no mixin plan: the crate is compiled in place against the SDK the build
fetches into `.horizon/sdk/` (gitignored).

```sh
CARGO_INCREMENTAL=0 cargo test --offline                 # the census + the package test
CARGO_INCREMENTAL=0 cargo clippy --all-targets -- -D warnings
cargo fmt --check                                        # not --all: that checks the SDK
rm -f Cargo.lock                                         # a packed lock changes the artifact
CARGO_INCREMENTAL=0 cargo horizon-mod build              # -> target/horizon-mod/<id>.hmod
cp target/horizon-mod/refinedstorage_quartz_arsenal.hmod ./refinedstorage_quartz_arsenal.hmod
cargo horizon-mod check refinedstorage_quartz_arsenal.hmod
cargo horizon-mod fits refinedstorage_quartz_arsenal.hmod --build nightly
python3 tools/check_content.py                           # the carried tree's cross-reference check
sha256sum refinedstorage_quartz_arsenal.hmod
```

Every build/test step runs inside a campaign build slot
(`../../ports/tools/build-slot.sh run --job … --port ports/refinedstorage_quartz_arsenal
--step … -- bash -c 'cd ports/refinedstorage_quartz_arsenal && …'`).

## In-game check

`tools/e2e.sh` is the scenario: both ids give, `/give 2` lands as two stacks of one, a
plausible absent sibling id is refused while a vanilla stick still gives, and a
counterfactual run without the mod proves the differential. It needs a game the launcher
built with this package:

```sh
horizon-launcher native-build ports/refinedstorage_quartz_arsenal --instance <i> --trust-native-code
GAME=<that game>/horizon-client CTL=<that game>/horizon-ctl tools/e2e.sh
```

`horizon-launcher` is **not installed** in the campaign environment at this pin, so the
0.2.0 package has **not** been run in a game; the 0.1.0 runs are kept as history
(`../_shared/evidence/refinedstorage_quartz_arsenal/`).
