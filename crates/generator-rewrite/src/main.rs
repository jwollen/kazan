use std::path::{Path, PathBuf};

use anyhow::Result;

/// Return the workspace root directory.
///
/// The generator crate lives at `<workspace>/crates/generator-rewrite`, so the
/// workspace root is two levels up from `CARGO_MANIFEST_DIR`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn main() -> Result<()> {
    let root = workspace_root();
    let analysis = generator_rewrite::analysis::Analysis::new(
        root.join("crates/generator/external/Vulkan-Headers"),
    );

    generator_rewrite::generate(&analysis, &root)
}
