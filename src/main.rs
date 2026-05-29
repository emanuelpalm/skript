use std::error::Error;
use std::io::{BufRead, Write};
use std::{env, io, process};

mod ast;
mod cli;
mod hir;
mod ops;
mod rvm;

const ROOT_PATTERN: cli::PatternSet<2> = cli::PatternSet {
    description: "The skript language toolkit.\n\nUsage: skript [options] <command> [parameters]",
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
    let result = (|| -> Result<(), Box<dyn Error>> {
        let matches = cli::parse(&ROOT_PATTERN, env::args().skip(1));

        // Help
        if let cli::FlagValue::Toggled = matches.flags[0] {
            return help();
        }

        // Version
        if let cli::FlagValue::Toggled = matches.flags[1] {
            return version();
        }

        let args: &[String] = &matches.args;
        let (command, tail) = match &args {
            &[command, tail @ ..] => (command, tail),
            &[] => return help(),
        };

        match command.as_str() {
            "repl" => repl(tail, &matches.remainder),
            command => {
                eprintln!("Unknown command: {}", command);
                eprintln!("Usage: skript [options] <command> [parameters]");
                process::exit(1);
            }
        }
    })();

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn help() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    cli::write(&ROOT_PATTERN, &mut buffer)?;
    io::stdout().write_all(buffer.as_bytes())?;
    Ok(())
}

fn repl(args: &[String], flags: &[String]) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut buffer = String::new();

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?; // Ensure the prompt is displayed immediately

        stdin.lock().read_line(&mut buffer)?;

        let input = buffer.trim();

        if input == "exit" {
            break;
        }

        let tree = ast::parse(input.as_bytes())?;
        let ir = ast::hir::lower(&tree);
        let code = hir::rvm::lower(&ir);

        println!("{:?}", code);

        let mut vm = rvm::VirtualMachine::new(&code);
        match vm.run() {
            Ok(value) => writeln!(stdout, "{}", value),
            Err(ref error) => writeln!(stdout, "{}", error),
        }?;

        buffer.clear();
    }

    writeln!(stdout, "Goodbye!")?;

    Ok(())
}

fn version() -> Result<(), Box<dyn Error>> {
    println!("0.0.1");
    Ok(())
}
