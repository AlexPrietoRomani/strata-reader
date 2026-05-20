// Compile the strata.ia.v1 proto file at build time.
//
// Context7-verified for tonic 0.14.6: emit a FileDescriptorSet alongside the
// generated stubs so `tonic::include_file_descriptor_set!` can wire gRPC
// reflection when needed.
//
// See `docs/plan/plan_maestro.md` §11.T6.1 and ADR-pending §11.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/strata_ia.proto");

    #[allow(clippy::disallowed_methods)]
    let descriptor_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR")?).join("strata_ia_descriptor.bin");

    // Server stubs are generated too so the bench harness can spin up an
    // in-process echo server. Production code only ever uses the client.
    tonic_prost_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .build_client(true)
        .build_server(true)
        .compile_protos(&["proto/strata_ia.proto"], &["proto/"])?;

    Ok(())
}
