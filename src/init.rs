use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

pub fn generate(args: &[String]) -> Result<String, String> {
    if args.len() != 3 || args[1] != "--home" || args[2].is_empty() {
        return Err("Usage: dotsh init <shell> --home <directory>".to_string());
    }

    let shell = args[0].to_lowercase();
    if !matches!(
        shell.as_str(),
        "bash" | "zsh" | "sh" | "fish" | "pwsh" | "powershell" | "cmd"
    ) {
        return Err(format!("Unsupported shell: {}", shell));
    }

    let home = Path::new(&args[2]);
    if !home.is_dir() {
        return Err(format!("Not a directory: {}", home.display()));
    }

    let platform = platform()?;
    let mut paths = Vec::new();
    if let Some(input) = read_optional(&home.join(format!(".path.{}", platform)))? {
        for line in input.lines() {
            let path = line.trim_end_matches('\r');
            if !path.is_empty() && !paths.iter().any(|entry| entry == path) {
                paths.push(path.to_string());
            }
        }
    }

    if cfg!(windows)
        && matches!(shell.as_str(), "bash" | "zsh" | "sh" | "fish")
        && !paths.is_empty()
    {
        let output = Command::new("cygpath")
            .args(["-u", "--"])
            .args(&paths)
            .output()
            .map_err(|error| format!("Failed to run cygpath: {}", error))?;
        if !output.status.success() {
            return Err(format!(
                "cygpath failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        let converted = String::from_utf8(output.stdout)
            .map_err(|error| format!("Invalid cygpath output: {}", error))?;
        let converted: Vec<String> = converted.lines().map(str::to_string).collect();
        if converted.len() != paths.len() || converted.iter().any(String::is_empty) {
            return Err("cygpath did not return one path per entry".to_string());
        }
        paths = converted;
    }

    let mut lines = Vec::new();
    match shell.as_str() {
        "bash" | "zsh" | "sh" => {
            for path in &paths {
                let quoted = format!("'{}'", path.replace('\'', "'\\''"));
                lines.push(format!(
                    "case \":$PATH:\" in *\":\"{quoted}\":\"*) ;; *) export PATH=\"$PATH:\"{quoted} ;; esac"
                ));
            }
        }
        "fish" => {
            for path in &paths {
                let quoted = format!(
                    "\"{}\"",
                    path.replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('$', "\\$")
                );
                lines.push(format!(
                    "contains -- {quoted} $PATH; or set -gx PATH $PATH {quoted}"
                ));
            }
        }
        "pwsh" | "powershell" | "cmd" => {
            if !paths.is_empty() {
                let separator = if cfg!(windows) { ';' } else { ':' };
                let current = match env::var("PATH") {
                    Ok(value) => value,
                    Err(env::VarError::NotPresent) => String::new(),
                    Err(error) => return Err(format!("Cannot read PATH: {}", error)),
                };
                let mut entries: Vec<String> =
                    current.split(separator).map(str::to_string).collect();
                let mut changed = false;
                for path in paths {
                    if !entries.iter().any(|entry| {
                        if cfg!(windows) {
                            entry.eq_ignore_ascii_case(&path)
                        } else {
                            entry == &path
                        }
                    }) {
                        entries.push(path);
                        changed = true;
                    }
                }
                if changed {
                    lines.push(crate::format(
                        &shell,
                        "PATH",
                        &entries.join(&separator.to_string()),
                    )?);
                }
            }
        }
        _ => unreachable!(),
    }

    for file in [
        ".env".to_string(),
        format!(".env.{}", platform),
        ".local/.env".to_string(),
    ] {
        if let Some(input) = read_optional(&home.join(file))? {
            for (key, value) in crate::dotenv::parse(&input) {
                let mut characters = key.chars();
                if !characters
                    .next()
                    .is_some_and(|c| c == '_' || c.is_ascii_alphabetic())
                    || !characters.all(|c| c == '_' || c.is_ascii_alphanumeric())
                {
                    return Err(format!("Invalid environment variable name: {}", key));
                }
                lines.push(crate::format(&shell, &key, &value)?);
            }
        }
    }

    Ok(lines.join("\n"))
}

fn read_optional(path: &Path) -> Result<Option<String>, String> {
    match fs::read_to_string(path) {
        Ok(input) => Ok(Some(input)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("Cannot read {}: {}", path.display(), error)),
    }
}

fn platform() -> Result<&'static str, String> {
    if cfg!(windows) {
        return Ok("win");
    }
    if cfg!(target_os = "macos") {
        return Ok("mac");
    }
    if cfg!(target_os = "linux") {
        if env::var_os("WSL_DISTRO_NAME").is_some_and(|name| !name.is_empty()) {
            return Ok("wsl");
        }
        let release = fs::read_to_string("/proc/sys/kernel/osrelease")
            .map_err(|error| format!("Cannot detect WSL: {}", error))?;
        if release.to_ascii_lowercase().contains("microsoft") {
            return Ok("wsl");
        }
        return Ok("linux");
    }
    Err("Unsupported platform".to_string())
}
