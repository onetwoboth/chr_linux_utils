use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process;

#[derive(Default, Debug)]
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
fn parse_args<I>(args: I) -> Result<(Vec<String>, Options), i32>
where
    I: IntoIterator<Item = String>,
{
    let args: Vec<String> = args.into_iter().collect();

    if args.len() < 2 {
        return Err(2);
    }

    if args.contains(&"--help".to_string()) {
        return Err(0);
    }

    let mut targets = Vec::new();
    let mut options = Options::default();

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];

        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'r' => options.recursive = true,
                    'f' => options.force = true,
                    _ => return Err(2),
                }
            }
        } else {
            targets.push(arg.clone());
        }

        i += 1;
    }

    if targets.is_empty() {
        return Err(2);
    }

    Ok((targets, options))
}


fn main() {
    match parse_args(env::args()) {
        Ok((targets, options)) => {
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
        Err(code) => {
            if code == 0 {
                print_usage();
            } else {
                eprintln!("chrrm: invalid arguments");
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
    fn parse_single_target() {
        let args = vec![s("chrrm"), s("file.txt")];

        let (targets, options) = parse_args(args).unwrap();

        assert_eq!(targets, vec!["file.txt"]);
        assert!(!options.recursive);
        assert!(!options.force);
    }

    #[test]
    fn parse_recursive_force_combined() {
        let args = vec![s("chrrm"), s("-rf"), s("a"), s("b")];

        let (targets, options) = parse_args(args).unwrap();

        assert_eq!(targets, vec!["a", "b"]);
        assert!(options.recursive);
        assert!(options.force);
    }

    #[test]
    fn parse_separate_flags() {
        let args = vec![s("chrrm"), s("-r"), s("-f"), s("dir")];

        let (targets, options) = parse_args(args).unwrap();

        assert_eq!(targets, vec!["dir"]);
        assert!(options.recursive);
        assert!(options.force);
    }

    #[test]
    fn help_flag_returns_zero() {
        let args = vec![s("chrrm"), s("--help")];

        let err = parse_args(args).unwrap_err();
        assert_eq!(err, 0);
    }

    #[test]
    fn missing_operand_returns_error() {
        let args = vec![s("chrrm"), s("-r")];

        let err = parse_args(args).unwrap_err();
        assert_eq!(err, 2);
    }

    #[test]
    fn unknown_option_returns_error() {
        let args = vec![s("chrrm"), s("-x"), s("file")];

        let err = parse_args(args).unwrap_err();
        assert_eq!(err, 2);
    }

    #[test]
    fn dash_dash_treated_as_target() {
        let args = vec![s("chrrm"), s("--"), s("file")];

        let err = parse_args(args).unwrap_err();
        assert_eq!(err, 2);
    }
}
