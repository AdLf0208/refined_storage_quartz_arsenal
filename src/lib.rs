//! Refined Storage - Quartz Arsenal for Horizon: the wireless crafting grids.
//!
//! Upstream Quartz Arsenal 1.0.8 (`refinedmods/refinedstorage-quartz-arsenal`, MIT)
//! adds **no block, tool or weapon at all**: the jar holds two items — a wireless
//! crafting grid and its creative variant — with four item models, two textures,
//! eighteen locales and one shaped recipe. This port carries exactly that content
//! in `assets/` and `data/` beside this crate, and its items are declared from the
//! vanilla `minecraft:paper` template.
//!
//! There is no behaviour in this slice: opening a grid needs a crafting surface
//! bound to a Refined Storage network, and the parent port's crafting half does
//! not exist yet (`notes.md`, gaps). So this file is the one
//! `#[mod_api::horizon_mod] impl Hooks` with **no hook**, and the mod's whole
//! content is data. The numbers that must stay true are in [`runtime`], which is
//! byte-identical to the retired `.hnmod` port's file and tested there.
use mod_api::{self, Hooks};

/// The pure census: the ids, the carried model paths, the recipe, the locales
/// and the manifest values, unit-tested without the game.
pub mod runtime;

/// The mod: no state and no hook, because this slice is a content carry.
#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {}

#[cfg(test)]
mod package {
    //! What the package root must say about itself, so a moved file or a short
    //! pin fails here instead of in a registry check. The censused content itself
    //! is `runtime`'s job; this module pins the packaging the migration rewrites.

    use std::path::{Path, PathBuf};

    /// The package root: the crate's own directory, which is also the package's.
    fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// The one manifest, read as text: the schema refuses unknown fields, but the
    /// values below are the ones the registry and the carried tree agree on.
    fn manifest() -> String {
        let path = root().join("horizon-mod.toml");
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
    }

    /// The value of a top-level `key = "value"` line, if it is there.
    fn value(text: &str, key: &str) -> Option<String> {
        text.lines()
            .map(str::trim)
            .find_map(|line| line.strip_prefix(key)?.trim_start().strip_prefix('='))
            .map(|rest| rest.trim().trim_matches('"').to_owned())
    }

    #[test]
    fn the_manifest_declares_the_one_identity_and_the_shared_values() {
        let text = manifest();
        assert_eq!(
            value(&text, "id").as_deref(),
            Some("refinedstorage_quartz_arsenal"),
            "the id is the resource namespace the carried tree is named after"
        );
        assert_eq!(value(&text, "version").as_deref(), Some("0.2.0"));
        assert_eq!(value(&text, "api").as_deref(), Some("0.2"));
        // The installation side, not a hook's: the items are data both halves read.
        assert_eq!(value(&text, "side").as_deref(), Some("both"));
        // The one value the schema allows, even for a mod that registers no code.
        assert!(text.contains(r#"capabilities = ["registries"]"#), "{text}");
        assert!(text.contains("[native]"), "a package with src/ is native");
    }

    #[test]
    fn both_native_pins_are_full_forty_hex_commits() {
        let text = manifest();
        for key in ["horizon", "mixins"] {
            let pin = value(&text, key).unwrap_or_else(|| panic!("no {key} pin"));
            assert_eq!(pin.len(), 40, "{key} pin {pin:?} is not a full commit");
            assert!(
                pin.chars().all(|c| c.is_ascii_hexdigit()),
                "{key} pin {pin:?} is not hex"
            );
        }
        assert_eq!(value(&text, "game").as_deref(), Some("26.3"));
    }

    #[test]
    fn the_carried_tree_is_named_after_the_id_and_no_retired_path_survives() {
        let root = root();
        for path in [
            "assets/refinedstorage_quartz_arsenal",
            "data/refinedstorage_quartz_arsenal",
        ] {
            assert!(root.join(path).is_dir(), "missing carried {path}");
        }
        // The retired `.hnmod` container and its plan are not package paths: the
        // project root may hold package paths only, so their return is a failure.
        for retired in [
            "mixins",
            "compatibility.json",
            "rust-toolchain.toml",
            "refinedstorage_quartz_arsenal.hnmod",
            "content",
        ] {
            assert!(
                !root.join(retired).exists(),
                "{retired} is a retired path; the evidence folder holds it"
            );
        }
        assert!(
            Path::new(&root.join("src/runtime.rs")).is_file(),
            "the pure census did not travel"
        );
    }
}
