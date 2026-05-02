use std::process::Command;

fn dotsh() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dotsh"))
}

#[test]
fn bash_simple() {
    let output = dotsh()
        .args(["bash", "KEY=val"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "export KEY='val'");
}

#[test]
fn bash_empty_value() {
    let output = dotsh()
        .args(["bash", "KEY="])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "export KEY=''");
}

#[test]
fn pwsh_windows_path() {
    let output = dotsh()
        .args(["pwsh", r"CARGO_HOME=C:\Users\Sayad\.cargo"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        r"$env:CARGO_HOME = 'C:\Users\Sayad\.cargo'"
    );
}

#[test]
fn fish_multiple_args() {
    let output = dotsh()
        .args(["fish", "A=1", "B=2"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("set -gx A \"1\""));
    assert!(stdout.contains("set -gx B \"2\""));
}

#[test]
fn cmd_semicolon() {
    let output = dotsh()
        .args(["cmd", r"PATH=C:\Windows;C:\Users"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        r#"set "PATH=C:\Windows;C:\Users""#
    );
}

#[test]
fn empty_input_no_output() {
    let output = dotsh()
        .args(["bash", ""])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "");
}

#[test]
fn comments_skipped() {
    let output = dotsh()
        .args(["bash", "# comment", "KEY=val"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "export KEY='val'");
}

#[test]
fn missing_shell_arg() {
    let output = dotsh()
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
}

#[test]
fn missing_input_arg() {
    let output = dotsh()
        .arg("bash")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
}

#[test]
fn unsupported_shell() {
    let output = dotsh()
        .args(["ksh", "KEY=val"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unsupported shell"));
}

#[test]
fn multiple_args_joined() {
    let output = dotsh()
        .args(["bash", "A=1", "B=2", "C=3"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "export A='1'");
    assert_eq!(lines[1], "export B='2'");
    assert_eq!(lines[2], "export C='3'");
}

#[test]
fn shell_case_insensitive() {
    let output = dotsh()
        .args(["BASH", "KEY=val"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "export KEY='val'");
}

#[test]
fn zsh_alias() {
    let output = dotsh()
        .args(["zsh", "KEY=val"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "export KEY='val'");
}

#[test]
fn sh_alias() {
    let output = dotsh()
        .args(["sh", "KEY=val"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "export KEY='val'");
}

#[test]
fn powershell_alias() {
    let output = dotsh()
        .args(["powershell", "KEY=val"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "$env:KEY = 'val'");
}

#[test]
fn value_with_equals_preserved() {
    let output = dotsh()
        .args(["bash", "KEY=a=b=c"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "export KEY='a=b=c'");
}

#[test]
fn value_with_spaces_preserved() {
    let output = dotsh()
        .args(["pwsh", "KEY=val with spaces"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "$env:KEY = 'val with spaces'"
    );
}

#[test]
fn mixed_valid_invalid_lines() {
    let output = dotsh()
        .args(["bash", "A=1", "bad", "", "B=2"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "export A='1'");
    assert_eq!(lines[1], "export B='2'");
}
