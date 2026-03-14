#![allow(warnings)]
#[macro_use]
extern crate failure;
extern crate ctap_hmac as ctap;

#[path = "src/cli_args/mod.rs"]
mod cli_args;
#[path = "src/error.rs"]
mod error;
#[path = "src/util.rs"]
mod util;

use cli_args::Args;
use std::env;
use std::str::FromStr;
use structopt::clap::Shell;
use structopt::StructOpt;

fn main() {
    // Apply ring patches before building
    apply_ring_patches();
    
    // generate completion scripts, zsh does panic for some reason
    for shell in Shell::variants().iter().filter(|shell| **shell != "zsh") {
        Args::clap().gen_completions(
            env!("CARGO_PKG_NAME"),
            Shell::from_str(shell).unwrap(),
            env!("CARGO_MANIFEST_DIR"),
        );
    }
}

fn apply_ring_patches() {
    use std::process::Command;
    
    // Always apply patches to ensure they're up to date
    println!("cargo:rerun-if-changed=apply-ring-patches.sh");
    println!("cargo:rerun-if-changed=patches/ring.patch");
    
    let status = Command::new("./apply-ring-patches.sh")
        .status()
        .expect("Failed to execute apply-ring-patches.sh");
    
    if !status.success() {
        panic!("Failed to apply ring patches");
    }
}
