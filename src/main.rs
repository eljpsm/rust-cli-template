//! renameme is a placeholder command line tool.
//!
//! The pipeline: `cli` parses the subcommand and `app::run` executes it.
//! `app` is the only module that prints and the only module that decides
//! exit codes. The `greet` and `count` commands are scaffolding; replace
//! them with the real tool.

mod app;
mod cli;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    app::run(cli::Cli::parse())
}
