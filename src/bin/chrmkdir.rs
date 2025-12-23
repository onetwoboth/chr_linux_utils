use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

/// 打印用法
fn print_usage() {
    eprintln!("Usage: chrmkdir [OPTIONS] PATH");
    eprintln!("Options:");
    eprintln!("  -p        Create parent directories as needed");
    eprintln!("  --help    Display this help message");
}

/// 命令选项
#[derive(Default, Debug, PartialEq, Eq)]
struct Options {
    create_parents: bool,
}

#[derive(PartialEq, Debug)]
enum ParseError {
    Help,
    UnknownOption(char),
    MissingOperand,
}

/// 解析命令行参数
fn parse_args<I>(args: I) -> Result<(Options, PathBuf), ParseError>
where
    I: IntoIterator<Item = String>,
{
    let args: Vec<String> = args.into_iter().collect();
    let mut opts = Options::default();
    let mut path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" {
            return Err(ParseError::Help);
        } else if arg.starts_with("-") {
            for ch in arg[1..].chars() {
                match ch {
                    'p' => opts.create_parents = true,
                    _ => return Err(ParseError::UnknownOption(ch)),
                }
            }
        } else {
            path = Some(PathBuf::from(arg));
        }
        i += 1;
    }

    if let Some(p) = path {
        Ok((opts, p))
    } else {
        Err(ParseError::MissingOperand)
    }
}


/// chrmkdir 核心逻辑
fn chrmkdir(path: &Path, opts: &Options) -> io::Result<()> {
    if opts.create_parents {
        fs::create_dir_all(path)?;
    } else {
        fs::create_dir(path)?;
    }
    Ok(())
}

fn main() {
    match parse_args(env::args()) {
        Ok((opts, path)) => {
            if let Err(err) = chrmkdir(&path, &opts) {
                eprintln!("chrmkdir: {}", err);
                process::exit(1);
            }
        },
        Err(ParseError::Help) => {
            print_usage();
            process::exit(0);
        },
        Err(ParseError::UnknownOption(ch)) => {
            eprintln!("chrmkdir: unknown option '-{}'", ch);
            print_usage();
            process::exit(2);
        },
        Err(ParseError::MissingOperand) => {
            eprintln!("chrmkdir: missing operand");
            print_usage();
            process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn parse_no_flag() {
        let args = vec![s("chrmkdir"), s("dir")];
        let (opts, path) = parse_args(args).unwrap();
        assert!(!opts.create_parents);
        assert_eq!(path, PathBuf::from("dir"));
    }

    #[test]
    fn parse_p_flag_and_path() {
        let args = vec![s("chrmkdir"), s("-p"), s("dir")];

        let (opts, path) = parse_args(args).unwrap();

        assert!(opts.create_parents);
        assert_eq!(path, PathBuf::from("dir"));
    }

    #[test]
    fn parse_help() {
        let args = vec![s("chrmkdir"), s("--help")];
        let error = parse_args(args).unwrap_err();
        assert_eq!(error, ParseError::Help);
    }

    #[test]
    fn missing_path() {
        let args = vec![s("chrmkdir"), s("-p")];
        let error = parse_args(args).unwrap_err();
        assert_eq!(error, ParseError::MissingOperand);
    }

    #[test]
    fn invalid_flag() {
        let args = vec![s("chrmkdir"), s("-x"), s("dir")];
        let error = parse_args(args).unwrap_err();
        assert_eq!(error, ParseError::UnknownOption('x'));
    }
}
