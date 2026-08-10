//! The command line surface. Doc comments here are user-facing: clap prints
//! them as help text.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A placeholder command line tool.
#[derive(Debug, Parser)]
#[command(name = "renameme", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print a greeting.
    Greet {
        /// Who to greet. Omitted means the world.
        name: Option<String>,
        /// Uppercase the greeting.
        #[arg(long)]
        shout: bool,
    },
    /// Print the number of lines in a file.
    Count {
        /// The file to count.
        file: PathBuf,
    },
}

// These pin the argument shapes scripts depend on. A parsing change that
// breaks one of them breaks every caller silently.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_invocation_is_an_error() {
        assert!(Cli::try_parse_from(["renameme"]).is_err());
    }

    #[test]
    fn greet_parses_with_and_without_a_name() {
        assert!(Cli::try_parse_from(["renameme", "greet"]).is_ok());
        let cli = Cli::try_parse_from(["renameme", "greet", "crab", "--shout"]).expect("parse");
        match cli.command {
            Command::Greet { name, shout } => {
                assert_eq!(name.as_deref(), Some("crab"));
                assert!(shout);
            }
            other => panic!("expected greet, got {other:?}"),
        }
    }

    #[test]
    fn count_requires_a_file() {
        assert!(Cli::try_parse_from(["renameme", "count"]).is_err());
        assert!(Cli::try_parse_from(["renameme", "count", "notes.txt"]).is_ok());
    }

    #[test]
    fn unknown_subcommands_are_errors() {
        assert!(Cli::try_parse_from(["renameme", "frobnicate"]).is_err());
    }
}
