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
#[derive(Default)]
struct Options {
    create_parents: bool,
}

/// 解析命令行参数
fn parse_args() -> (Options, PathBuf) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::default();
    let mut path = PathBuf::new();

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--help" => {
                print_usage();
                process::exit(0);
            },
            "-p" => {
                opts.create_parents = true;
            },
            _ if arg.starts_with("-") => {
                eprintln!("Unknown option: {}", arg);
                print_usage();
                process::exit(2);
            },
            _ => {
                path = PathBuf::from(arg);
            }
        }
        i += 1;
    }

    if path.as_os_str().is_empty() {
        eprintln!("Error: PATH is required.");
        print_usage();
        process::exit(2);
    }

    (opts, path)
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
    let (opts, path) = parse_args();

    if let Err(err) = chrmkdir(&path, &opts) {
        eprintln!("chrmkdir: {}", err);
        process::exit(1);
    }
}
