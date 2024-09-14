use std::{
    env,
    path::{Path, PathBuf},
};

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(2)
    .unwrap()
    .to_path_buf()
}

fn main() {
    let root = project_root();
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir(root.join("crates/hop-scip/src/proto"))
        .compile(&[root.join("proto/scip.proto")], &[root.join("proto")])
        .unwrap();
}
