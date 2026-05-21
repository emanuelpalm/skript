use std::fmt;
use std::option::Option;

pub struct PatternSet<const F: usize> {
    pub description: &'static str,
    pub flags: [Flag; F],
}

pub struct Flag {
    pub short: Option<char>,
    pub long: &'static str,
    pub description: &'static str,
    pub kind: FlagKind,
}

pub enum FlagKind {
    Toggle,
    String,
}

pub enum FlagValue {
    None,
    Toggled,
    String(Option<String>),
}

pub struct MatchSet<const F: usize> {
    pub args: Vec<String>,
    pub flags: [FlagValue; F],
    pub remainder: Vec<String>,
}

pub fn parse<const F: usize>(
    ps: &PatternSet<F>,
    mut args: impl Iterator<Item = String>,
) -> MatchSet<F> {
    let mut match_set = MatchSet::<F> {
        args: Vec::new(),
        flags: [const { FlagValue::None }; F],
        remainder: Vec::new(),
    };

    'outer: while let Some(arg) = args.next() {
        if !arg.starts_with("-") {
            match_set.args.push(arg);
            continue;
        }

        if arg == "--" {
            args.for_each(|arg| match_set.args.push(arg));
            break;
        }

        for i in 0..F {
            let flag = &ps.flags[i];
            if matches(flag, &arg) {
                match_set.flags[i] = match flag.kind {
                    FlagKind::Toggle => FlagValue::Toggled,
                    FlagKind::String => FlagValue::String(match args.next() {
                        Some(arg) if !arg.starts_with("-") => Some(arg),
                        _ => None,
                    }),
                };
                continue 'outer;
            }
        }

        match_set.remainder.push(arg);
    }

    return match_set;

    fn matches(flag: &Flag, arg: &str) -> bool {
        arg.starts_with("--") && flag.long == &arg[2..]
            || arg.len() == 2 && flag.short == Some(arg.as_bytes()[0] as char)
    }
}

pub fn write<const F: usize, W: fmt::Write>(ps: &PatternSet<F>, mut w: W) -> fmt::Result {
    writeln!(w, "{}", ps.description)?;

    if !ps.flags.is_empty() {
        writeln!(w)?;
        writeln!(w, "Options:")?;
    }
    for i in 0..F {
        let flag = &ps.flags[i];
        w.write_str("  ")?;
        if let Some(short) = flag.short {
            w.write_char('-')?;
            w.write_char(short)?;
            w.write_char(',')?;
        } else {
            w.write_str("   ")?;
        }
        let value_str = match flag.kind {
            FlagKind::Toggle => " ",
            FlagKind::String => " <string>",
        };
        let value_len = 13 - value_str.len();

        writeln!(w, " --{:<value_len$}{} {}", flag.long, value_str, flag.description)?;
    }

    Ok(())
}
