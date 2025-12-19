use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::os::unix::fs::PermissionsExt;
use std::time::{UNIX_EPOCH, SystemTime};

/// 打印用法
fn print_usage() {
    eprintln!("Usage: chrls [OPTIONS] [PATH]");
    eprintln!("Options:");
    eprintln!("  -a        Show hidden files");
    eprintln!("  -l        Long format (permissions, size, modified time)");
    eprintln!("  --help    Display this help message");
}

/// 命令选项
#[derive(Default)]
struct Options {
    show_hidden: bool,
    long_format: bool,
}

/// 解析命令行参数
fn parse_args() -> (Options, PathBuf) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::default();
    let mut path = PathBuf::from(".");

    let mut i = 1; // args[0] 是命令名
    while i < args.len() {
        let arg = &args[i];
        if arg == "--help" {
            print_usage();
            process::exit(0);
        } else if arg.starts_with('-') && arg.len() > 1 {
            // 解析组合选项，例如 -al
            for ch in arg[1..].chars() {
                match ch {
                    'a' => opts.show_hidden = true,
                    'l' => opts.long_format = true,
                    _ => {
                        eprintln!("Unknown option: -{}", ch);
                        print_usage();
                        process::exit(2);
                    }
                }
            }
        } else {
            // 不是选项的参数，认为是路径
            path = PathBuf::from(arg);
        }
        i += 1;
    }
    (opts, path)
}

/// chrls 的核心逻辑
fn chrls(path: &PathBuf, opts: &Options) -> io::Result<()> {
    ensure_can_ls(path)?;
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with(".") && !opts.show_hidden {
            continue;
        }

        if opts.long_format {
            print_long_info(&entry)?;
        } else {
            println!("{}", name);
        }
    }

    Ok(())
}

/// 打印详细信息（类似 ls -l）
fn print_long_info(entry: &fs::DirEntry) -> io::Result<()> {
    let meta = entry.metadata()?;
    let permissions = meta.permissions();
    let mode = permissions.mode();

    // 简单显示 rwx 权限
    let file_type = if meta.is_dir() { 'd' } else { '-' };
    let perms = format!(
        "{}{}{}{}{}{}{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    );

    let size = meta.len();
    let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let duration = mtime.duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = duration.as_secs();

    println!("{} {} {:>8} {:>10} {}", file_type, perms, size, seconds, entry.file_name().to_string_lossy());
    Ok(())
}

// 确保目录可读且可执行
fn ensure_can_ls(path: &Path) -> io::Result<()> {
    let meta = fs::metadata(path)?;
    if !meta.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Not a directory",
        ));
    }

    let mode = meta.permissions().mode();
    /*
    Linux中的权限：rwxrwxrwx分为三组，每一组都使用三位01表示
    比如 100 100 100 也就是 0o444 表示所有用户都有读权限
     */
    let has_r = mode & 0o444 != 0;
    let has_x = mode & 0o111 != 0;

    if !has_r || !has_x {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Permission denied",
        ));
    }

    Ok(())
}

fn main() {
    let (opts, path) = parse_args();

    if let Err(err) = chrls(&path, &opts) {
        eprintln!("chrls: {}", err);
        process::exit(1);
    }
}
