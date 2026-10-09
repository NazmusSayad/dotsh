use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_HOME: AtomicU64 = AtomicU64::new(0);

struct TestHome(PathBuf);

impl TestHome {
    fn new() -> Self {
        let root = std::env::temp_dir().join("opencode");
        fs::create_dir_all(&root).unwrap();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = root.join(format!(
            "dotsh-init-{}-{}-{}",
            std::process::id(),
            timestamp,
            NEXT_HOME.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn write(&self, file: &str, content: &str) {
        let path = self.0.join(file);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn write_platform_env(&self, content: &str) {
        for suffix in ["win", "mac", "wsl", "linux"] {
            self.write(&format!(".env.{}", suffix), content);
        }
    }

    fn run(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_dotsh"))
            .args(["init", "bash", "--home"])
            .arg(&self.0)
            .output()
            .unwrap()
    }

    fn output(&self) -> String {
        let output = self.run();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        String::from_utf8(output.stdout).unwrap()
    }
}

impl Drop for TestHome {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn missing_files_produce_no_exports() {
    let home = TestHome::new();
    assert_eq!(home.output(), "");
}

#[test]
fn shared_values_load_without_override_files() {
    let home = TestHome::new();
    home.write(".env", "A=1\nB=2\n");
    assert_eq!(home.output(), "export A='1'\nexport B='2'");
}

#[test]
fn platform_overrides_shared_key_and_preserves_other_keys() {
    let home = TestHome::new();
    home.write(".env", "A=1\nB=2\n");
    home.write_platform_env("A=2\n");
    assert_eq!(home.output(), "export A='1'\nexport B='2'\nexport A='2'");
}

#[test]
fn platform_values_load_without_shared_file() {
    let home = TestHome::new();
    home.write_platform_env("A=2\n");
    assert_eq!(home.output(), "export A='2'");
}

#[test]
fn platform_can_add_new_keys() {
    let home = TestHome::new();
    home.write(".env", "A=1\n");
    home.write_platform_env("B=2\n");
    assert_eq!(home.output(), "export A='1'\nexport B='2'");
}

#[test]
fn local_overrides_shared_and_platform_values_last() {
    let home = TestHome::new();
    home.write(".env", "A=1\nB=2\n");
    home.write_platform_env("A=2\nC=3\n");
    home.write(".local/.env", "A=4\nB=5\n");
    assert_eq!(
        home.output(),
        "export A='1'\nexport B='2'\nexport A='2'\nexport C='3'\nexport A='4'\nexport B='5'"
    );
}

#[test]
fn local_values_load_without_shared_or_platform_files() {
    let home = TestHome::new();
    home.write(".local/.env", "A=3\n");
    assert_eq!(home.output(), "export A='3'");
}

#[test]
fn empty_platform_value_overrides_nonempty_shared_value() {
    let home = TestHome::new();
    home.write(".env", "A=1\nB=2\n");
    home.write_platform_env("A=\n");
    assert_eq!(home.output(), "export A='1'\nexport B='2'\nexport A=''");
}

#[test]
fn empty_local_value_overrides_platform_value() {
    let home = TestHome::new();
    home.write_platform_env("A=2\n");
    home.write(".local/.env", "A=\n");
    assert_eq!(home.output(), "export A='2'\nexport A=''");
}

#[test]
fn semicolon_in_shared_value_does_not_create_another_variable() {
    let home = TestHome::new();
    home.write(".env", "A=1;B=2\n");
    home.write_platform_env("A=2\n");
    assert_eq!(home.output(), "export A='1;B=2'\nexport A='2'");
}

#[test]
fn semicolon_in_platform_value_preserves_shared_second_variable() {
    let home = TestHome::new();
    home.write(".env", "A=1\nB=2\n");
    home.write_platform_env("A=3;B=4\n");
    assert_eq!(
        home.output(),
        "export A='1'\nexport B='2'\nexport A='3;B=4'"
    );
}

#[test]
fn duplicate_keys_keep_file_order_before_platform_override() {
    let home = TestHome::new();
    home.write(".env", "A=1\nA=2\n");
    home.write_platform_env("A=3\n");
    assert_eq!(home.output(), "export A='1'\nexport A='2'\nexport A='3'");
}

#[test]
fn overrides_handle_crlf_comments_and_missing_final_newline() {
    let home = TestHome::new();
    home.write(".env", "# shared\r\n\r\nA=1\r\nB=2\r\n");
    home.write_platform_env("# platform\r\nA=3");
    home.write(".local/.env", "A=4");
    assert_eq!(
        home.output(),
        "export A='1'\nexport B='2'\nexport A='3'\nexport A='4'"
    );
}

#[test]
fn unrecognized_platform_file_is_ignored() {
    let home = TestHome::new();
    home.write(".env", "A=1\n");
    home.write(".env.unsupported", "A=9\n");
    assert_eq!(home.output(), "export A='1'");
}

#[cfg(windows)]
#[test]
fn windows_loads_only_windows_overrides() {
    let home = TestHome::new();
    home.write(".env", "A=1\nB=2\n");
    home.write(".env.win", "A=3\n");
    home.write(".env.mac", "A=4\n");
    home.write(".env.wsl", "A=5\n");
    home.write(".env.linux", "A=6\n");
    assert_eq!(home.output(), "export A='1'\nexport B='2'\nexport A='3'");
}

#[test]
fn invalid_local_override_fails_without_partial_output() {
    let home = TestHome::new();
    home.write(".env", "A=1\n");
    home.write_platform_env("A=2\n");
    home.write(".local/.env", "INVALID-NAME=3\n");
    let output = home.run();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("Invalid environment variable name: INVALID-NAME")
    );
}
