use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process;
use std::os::unix::fs::PermissionsExt;

/// 打印用法
fn print_usage() {
    eprintln!("Usage: chrls [PATH]");
}

/// chrls 的核心逻辑
fn chrls(path: &Path) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    ensure_can_ls(path)?;
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();

        // Linux 文件名可能不是 UTF-8
        let name = file_name.to_string_lossy();
        println!("{}", name);
    }

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
    let args: Vec<String> = env::args().collect();

    // chrls [path]
    if args.len() > 2 {
        print_usage();
        process::exit(2);
    }

    let path = if args.len() == 2 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    if let Err(err) = chrls(path) {
        eprintln!("chrls: {}", err);
        process::exit(1);
    }
}