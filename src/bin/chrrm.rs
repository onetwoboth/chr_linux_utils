use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process;

#[derive(Default)]
struct Options {
    recursive: bool,
    force: bool,
}

// 打印用法
fn print_usage() {
    eprintln!("Usage: chrrm [OPTIONS] FILE");
    eprintln!("Options:");
    eprintln!("  --help       Print this help message");
    eprintln!("  -r  Remove directories and their contents recursively");
    eprintln!("  -f      Ignore nonexistent files and never prompt");
}

// 提示用户是否继续删除
fn confirm_delete(path: &Path) -> bool {
    print!("chrrm: remove write-protected file '{}'? [y/N] ", path.display());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    } else {
        false
    }
}

// chrrm 的核心逻辑
fn chrrm(path: &Path, options: &Options) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No such file or directory"));
    }

    let metadata = fs::metadata(path)?;

    let is_readonly = metadata.permissions().mode() & 0o200 == 0;

    if is_readonly && !options.force {
        if !confirm_delete(path) {
            eprintln!("chrrm: not removing '{}'", path.display());
            return Ok(());
        }
    }

    if metadata.is_dir() {
        if options.recursive {
            // 主要功能函数：递归删除目录及其内容
            fs::remove_dir_all(path)?;
        } else {
            // 主要功能函数：删除空目录
            fs::remove_dir(path)?;
        }
    } else {
        // 主要功能函数：删除文件
        fs::remove_file(path)?;
    }

    Ok(())
}

// 解析命令行参数
fn parse_args() -> (Vec<String>, Options) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(2);
    }

    if args.contains(&"--help".to_string()) {
        print_usage();
        process::exit(0);
    }

    let mut targets = Vec::new();
    let mut options = Options::default();

    for arg in &args[1..] {
        match arg.as_str() {
            "-r" => options.recursive = true,
            "-f" => options.force = true,
            _ if arg.starts_with("-") => {
                eprintln!("Unknown option: {}", arg);
                print_usage();
                process::exit(2);
            }
            _ => targets.push(arg.clone()),
        }
    }

    if targets.is_empty() {
        eprintln!("chrrm: missing operand");
        print_usage();
        process::exit(2);
    }

    (targets, options)
}

fn main() {
    let (targets, options) = parse_args();
    let mut exit_code = 0;

    for t in targets {
        let path = Path::new(&t);
        if let Err(e) = chrrm(path, &options) {
            if !options.force {
                eprintln!("chrrm: failed to remove '{}': {}", t, e);
            }
            exit_code = 1;
        }
    }

    process::exit(exit_code);
}
