use std::env;

mod bash;
mod cmd;
mod dotenv;
mod fish;
mod init;
mod pwsh;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).is_some_and(|arg| arg == "init") {
        match init::generate(&args[2..]) {
            Ok(output) => print!("{}", output),
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
        return;
    }

    if args.len() < 3 {
        eprintln!("Usage: dotsh <shell> <dotenv-content>");
        std::process::exit(1);
    }

    let shell = args[1].to_lowercase();
    let input = args[2..].join("\n");

    let mut lines = Vec::new();
    for (key, value) in dotenv::parse(&input) {
        let line = match format(&shell, &key, &value) {
            Ok(line) => line,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };

        lines.push(line);
    }

    print!("{}", lines.join("\n"));
}

fn format(shell: &str, key: &str, value: &str) -> Result<String, String> {
    match shell {
        "bash" | "zsh" | "sh" => Ok(bash::format(key, value)),
        "pwsh" | "powershell" => Ok(pwsh::format(key, value)),
        "fish" => Ok(fish::format(key, value)),
        "cmd" => Ok(cmd::format(key, value)),
        _ => Err(format!("Unsupported shell: {}", shell)),
    }
}
