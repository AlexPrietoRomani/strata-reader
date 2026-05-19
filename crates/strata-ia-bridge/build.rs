// Phase 0: proto file does not exist yet. Real implementation lands in T6.1 (A6.1.1).
// See docs/task/tareas.md and docs/plan/plan_maestro.md §11.
//
// When the proto is added, replace the body of `main` with the Context7-verified pattern:
//
//     let descriptor_path = std::path::PathBuf::from(std::env::var("OUT_DIR")?)
//         .join("strata_ia_descriptor.bin");
//     tonic_build::configure()
//         .file_descriptor_set_path(&descriptor_path)
//         .compile_protos(&["proto/strata_ia.proto"], &["proto/"])?;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto");
    Ok(())
}
