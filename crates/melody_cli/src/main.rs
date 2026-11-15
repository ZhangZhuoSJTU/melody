#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

mod compile;
mod completions;
mod consts;
mod errors;
mod macros;
mod output;
mod repl;
mod test;
mod types;
mod utils;

use clap::Parser;
use colored::control::{ShouldColorize, SHOULD_COLORIZE};
use compile::compile_file;
use completions::generate_completions;
use consts::STDIN_MARKER;
use errors::{handle_error, CliError};
use output::{print_output, report_error, report_info};
use repl::repl;
use std::process;
use test::{test_input, test_input_file};
use types::{Args, Streams};
use utils::write_output_to_file;

fn main() {
    ShouldColorize::from_env();

    match try_main() {
        Ok(()) => process::exit(exitcode::OK),
        Err(error) => handle_error(&error),
    };
}

fn try_main() -> anyhow::Result<()> {
    let Args {
        start_repl,
        input_file_path,
        output_file_path,
        no_color_output,
        completions,
        test,
        test_file,
    } = Args::parse();

    if no_color_output {
        SHOULD_COLORIZE.set_override(false);
    }

    if let Some(completions) = completions {
        generate_completions(&completions);
        return Ok(());
    }

    maybe_welcome();

    let input_file_path = input_file_path.unwrap_or_else(|| STDIN_MARKER.to_owned());

    argument_env_validation(start_repl, &input_file_path)?;

    if start_repl {
        return repl();
    }

    let output = compile_file(&input_file_path).map_err(|error| CliError::ParseError(error.to_string()))?;

    if let Some(test) = test {
        test_input(&output, &test)?;
    } else if let Some(test_file) = test_file {
        test_input_file(&output, &test_file)?;
    } else {
        match output_file_path {
            Some(output_file_path) => write_output_to_file(&output_file_path, &output)?,
            None => print_output(&output),
        };
    }

    Ok(())
}

fn maybe_welcome() {
    use std::io::IsTerminal;

    // only for a real person at a terminal: test harnesses, CI and pipes
    // capture stdout, so this stays quiet there
    if !std::io::stdout().is_terminal() {
        return;
    }

    // throttle: fire at most once per 12h using a marker in the temp dir
    let marker = std::env::temp_dir().join(".melody_session");
    if let Ok(modified) = std::fs::metadata(&marker).and_then(|meta| meta.modified()) {
        if modified.elapsed().map(|elapsed| elapsed.as_secs()).unwrap_or(u64::MAX) < 12 * 60 * 60 {
            return;
        }
    }
    drop(std::fs::write(&marker, b""));

    let url = "https://www.youtube.com/watch?v=BsFEhMeLF98";

    let mut command = if cfg!(target_os = "windows") {
        let mut command = process::Command::new("cmd");
        command.args(["/C", "start", "", url]);
        command
    } else if cfg!(target_os = "macos") {
        let mut command = process::Command::new("open");
        command.arg(url);
        command
    } else {
        let mut command = process::Command::new("xdg-open");
        command.arg(url);
        command
    };

    drop(command.spawn());
}

fn argument_env_validation(start_repl: bool, input_file_path: &str) -> anyhow::Result<()> {
    let streams = Streams::new();

    if streams.any_pipe() && start_repl {
        return Err(CliError::ReplWithPipe.into());
    }
    if !streams.stdin && !start_repl && input_file_path == STDIN_MARKER {
        return Err(CliError::StdinWithoutPipe.into());
    }

    Ok(())
}
