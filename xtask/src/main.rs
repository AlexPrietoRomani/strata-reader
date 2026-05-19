//! Repo-internal automation. Run with `cargo run -p xtask -- <subcommand>`.
//!
//! Phase 1 ships a single subcommand: `gen-schema`, which serializes the
//! [`strata_core::Document`] JSON Schema to `docs/schema/strata-document.schema.json`
//! (or compares the on-disk file against a freshly generated one with `--check`).

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use schemars::schema_for;
use strata_core::Document;

#[derive(Parser)]
#[command(name = "xtask", about = "Strata-Reader repo automation")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Generate (or verify) the public JSON Schema for `Document`.
    GenSchema {
        /// Output path (relative to the repo root).
        #[arg(long, default_value = "docs/schema/strata-document.schema.json")]
        output: PathBuf,
        /// Exit non-zero if the file on disk differs from the freshly generated one.
        #[arg(long)]
        check: bool,
    },
}

fn cmd_gen_schema(output: PathBuf, check: bool) -> Result<()> {
    let schema = schema_for!(Document);
    let mut text = serde_json::to_string_pretty(&schema)
        .context("schema serialization to JSON failed")?;
    text.push('\n'); // POSIX-friendly trailing newline.

    if check {
        let existing = std::fs::read_to_string(&output)
            .with_context(|| format!("cannot read {}", output.display()))?;
        if existing == text {
            eprintln!("ok  {} is up to date", output.display());
            Ok(())
        } else {
            anyhow::bail!(
                "{} is out of date — rerun `cargo run -p xtask -- gen-schema` to regenerate",
                output.display()
            )
        }
    } else {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("cannot create {}", parent.display()))?;
        }
        std::fs::write(&output, &text)
            .with_context(|| format!("cannot write {}", output.display()))?;
        eprintln!("wrote {}", output.display());
        Ok(())
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let res = match cli.cmd {
        Cmd::GenSchema { output, check } => cmd_gen_schema(output, check),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(1)
        }
    }
}
