//! Command dispatch. The only module that prints and the only module that
//! decides exit codes. Every other module returns values or `anyhow::Result`.

use std::process::ExitCode;

use anyhow::Context;

use crate::cli::{Cli, Command};

/// The single place errors become output. Every command returns a Result and
/// its message is printed here, prefixed once, with the whole `anyhow` chain.
pub fn run(cli: Cli) -> ExitCode {
    match execute(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("renameme: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn execute(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Greet { name, shout } => {
            println!("{}", greeting(name.as_deref(), shout));
            Ok(())
        }
        Command::Count { file } => {
            let text = std::fs::read_to_string(&file)
                .with_context(|| format!("read {}", file.display()))?;
            println!("{}", line_count(&text));
            Ok(())
        }
    }
}

fn greeting(name: Option<&str>, shout: bool) -> String {
    let line = format!("hello, {}", name.unwrap_or("world"));
    if shout { line.to_uppercase() } else { line }
}

fn line_count(text: &str) -> usize {
    text.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_defaults_to_the_world() {
        assert_eq!(greeting(None, false), "hello, world");
        assert_eq!(greeting(Some("crab"), false), "hello, crab");
    }

    #[test]
    fn shout_uppercases_the_whole_line() {
        assert_eq!(greeting(Some("crab"), true), "HELLO, CRAB");
    }

    #[test]
    fn line_count_ignores_a_missing_final_newline() {
        assert_eq!(line_count(""), 0);
        assert_eq!(line_count("one\n"), 1);
        assert_eq!(line_count("one\ntwo"), 2);
        assert_eq!(line_count("one\ntwo\n"), 2);
    }
}
