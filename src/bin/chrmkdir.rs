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

/// 解析命令行参数
fn parse_args<I>(args: I) -> Result<(Options, PathBuf), i32>
where
    I: IntoIterator<Item = String>,
{
    let args: Vec<String> = args.into_iter().collect();
    let mut opts = Options::default();
    let mut path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--help" => return Err(0),
            "-p" => {
                opts.create_parents = true;
            }
            _ if arg.starts_with('-') => {
                return Err(2);
            }
            _ => {
                path = Some(PathBuf::from(arg));
            }
        }
        i += 1;
    }

    if let Some(p) = path {
        Ok((opts, p))
    } else {
        Err(2)
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
        }
        Err(code) => {
            if code == 0 {
                print_usage();
            } else {
                eprintln!("Error: invalid arguments");
                print_usage();
            }
            process::exit(code);
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
        let code = parse_args(args).unwrap_err();
        assert_eq!(code, 0);
    }

    #[test]
    fn missing_path() {
        let args = vec![s("chrmkdir"), s("-p")];
        let code = parse_args(args).unwrap_err();
        assert_eq!(code, 2);
    }

    #[test]
    fn invalid_flag() {
        let args = vec![s("chrmkdir"), s("-x"), s("dir")];
        let code = parse_args(args).unwrap_err();
        assert_eq!(code, 2);
    }
}
