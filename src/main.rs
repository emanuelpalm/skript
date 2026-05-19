use std::io::Write;
use std::{env, io};

mod ast;
mod cli;
mod codegen;
mod vm;

const ROOT_PATTERN: cli::PatternSet<2> = cli::PatternSet {
    description: "The skript language toolkit.\n\nUsage: skript [options] <file>",
    flags: [
        cli::Flag {
            short: Some('h'),
            long: "help",
            description: "Prints this help message and exits.",
            kind: cli::FlagKind::Toggle,
        },
        cli::Flag {
            short: None,
            long: "version",
            description: "Prints the current application version and exits.",
            kind: cli::FlagKind::Toggle,
        },
    ],
};

fn main() {
    let matches = cli::parse(&ROOT_PATTERN, env::args());

    // Help
    if let cli::FlagValue::Toggled = matches.flags[0] {
        let mut buffer = String::new();
        cli::print(&ROOT_PATTERN, &mut buffer).unwrap();
        io::stdout().write_all(buffer.as_bytes()).unwrap();
        return;
    }

    // Version
    if let cli::FlagValue::Toggled = matches.flags[1] {
        println!("0.0.1");
        return;
    }
}
