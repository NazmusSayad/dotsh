use std::env;
mod bash;
mod cmd;
mod fish;
mod pwsh;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: dotsh <shell> [line ...]");
        std::process::exit(1);
    }

    let shell = args[1].to_lowercase();
    let input = args[2].to_string();

    for item in dotenvy::from_read_iter(input.as_bytes()) {
        let (key, value) = item.expect("Failed to parse dotenv");
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
