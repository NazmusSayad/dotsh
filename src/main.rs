use std::env;

mod bash;
mod cmd;
mod dotenv;
mod fish;
mod pwsh;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: dotsh <shell> <dotenv-content>");
        std::process::exit(1);
    }

    let shell = args[1].to_lowercase();
    let input = args[2..].join("\n");

    for (key, value) in dotenv::parse(&input) {
        let output = match shell.as_str() {
            "bash" | "zsh" | "sh" => bash::format(&key, &value),
            "pwsh" | "powershell" => pwsh::format(&key, &value),
            "fish" => fish::format(&key, &value),
            "cmd" => cmd::format(&key, &value),
            _ => {
                eprintln!("Unsupported shell: {}", shell);
                std::process::exit(1);
            }
        };

        println!("{}", output);
    }
}
